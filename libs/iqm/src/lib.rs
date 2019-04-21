//! A parser for the IQM format used by Cube 2 games.
//!
//! See [here](http://sauerbraten.org/iqm/) for more information about the format, or the `iqm.txt`
//! file stored adjacently.
#![deny(
    bad_style,
    bare_trait_objects,
    const_err,
    dead_code,
    improper_ctypes,
    legacy_directory_ownership,
    missing_debug_implementations,
    missing_docs,
    no_mangle_generic_items,
    non_shorthand_field_patterns,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    plugin_as_library,
    private_in_public,
    safe_extern_statics,
    trivial_casts,
    trivial_numeric_casts,
    unconditional_recursion,
    unions_with_drop_fields,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_extern_crates,
    unused_import_braces,
    unused_parens,
    unused_qualifications,
    unused_results,
    while_true
)]

#[cfg(test)]
mod tests;

use byteorder::{ByteOrder, LittleEndian};
use log::warn;
use std::{ops::Range, str::from_utf8};

/// The data stored within an IQM file.
#[derive(Clone, Debug)]
pub struct IQM {
    /// Text entries.
    pub text: Vec<String>,

    /// Mesh entries.
    pub meshes: Vec<Mesh>,
    // TODO: vertex_arrays: (u32, u32, u32),
    // TODO: triangles: (u32, u32, u32),
    // TODO: joints: (u32, u32),
    // TODO: poses: (u32, u32),
    // TODO: animations: (u32, u32),
    // TODO: frames: (u32, u32, u32, u32),
    /// Comment entries.
    pub comments: Vec<String>,
}

impl IQM {
    /// Attempts to read the data from the contents of an IQM file.
    pub fn parse_from(bs: &[u8]) -> Option<IQM> {
        let header = Header::parse_from(bs)?;
        if header.extensions.0 != 0 || header.extensions.1 != 0 {
            warn!("Extensions are not yet supported");
        }

        let index = |n, (l, o)| &bs[o as usize..][..n * l as usize];

        Some(IQM {
            text: parse_texts(index(1, header.text))?,
            meshes: parse_meshes(index(24, header.meshes))?,
            comments: parse_texts(index(1, header.comments))?,
        })
    }
}

/// A single mesh.
#[derive(Clone, Debug)]
pub struct Mesh {
    /// The name of the mesh. This is an index into the text entries.
    pub name: Option<usize>,

    /// The material to be used. TODO: Where do these end up?
    pub material: u32,

    /// The range of vertices in the vertex entries that correspond to this mesh.
    pub vertices: Range<usize>,

    /// The range of triangles in the triangle entries that correspond to this mesh.
    pub triangles: Range<usize>,
}

/// The header of the IQM file.
#[derive(Clone, Copy, Debug)]
struct Header {
    /// The magic identifier. Should always be "INTERQUAKEMODEL\0".
    magic: [u8; 16],

    /// The version number. Should always be 2.
    version: u32,

    /// The size of the file, including the header.
    filesize: u32,

    /// The flags. TODO: What are these?
    flags: u32,

    /// The number and offset of text values.
    text: (u32, u32),

    /// The number and offset of the meshes.
    meshes: (u32, u32),

    /// TODO: uint num_vertexarrays, num_vertexes, ofs_vertexarrays;
    vertex_arrays: (u32, u32, u32),

    /// TODO: uint num_triangles, ofs_triangles, ofs_adjacency;
    triangles: (u32, u32, u32),

    /// The number and offset of the joints.
    joints: (u32, u32),

    /// The number and offset of the poses.
    poses: (u32, u32),

    /// The number and offset of the animations.
    animations: (u32, u32),

    /// TODO: uint num_frames, num_framechannels, ofs_frames, ofs_bounds;
    frames: (u32, u32, u32, u32),

    /// The number and offset of the comments.
    comments: (u32, u32),

    /// The number of extensions and the offset of the first one. Note that this is semantically
    /// different from the other fields.
    extensions: (u32, u32),
}

impl Header {
    /// Attempts to read the header from the contents of an IQM file.
    pub fn parse_from(bs: &[u8]) -> Option<Header> {
        const HEADER_SIZE: usize = std::mem::size_of::<Header>();
        assert_eq!(HEADER_SIZE, 124);
        if bs.len() < HEADER_SIZE {
            return None;
        }

        macro_rules! u32_at {
            ($off:expr) => {
                LittleEndian::read_u32(&bs[$off..($off + 4)])
            };
            (2 x $off:expr) => {
                (u32_at!($off), u32_at!($off + 4))
            };
            (3 x $off:expr) => {
                (u32_at!($off), u32_at!($off + 4), u32_at!($off + 8))
            };
            (4 x $off:expr) => {
                (
                    u32_at!($off),
                    u32_at!($off + 4),
                    u32_at!($off + 8),
                    u32_at!($off + 12),
                )
            };
        }

        let mut magic = [0; 16];
        magic.copy_from_slice(&bs[..16]);
        let header = Header {
            magic,
            version: u32_at!(16),
            filesize: u32_at!(20),
            flags: u32_at!(24),
            text: u32_at!(2 x 28),
            meshes: u32_at!(2 x 36),
            vertex_arrays: u32_at!(3 x 44),
            triangles: u32_at!(3 x 56),
            joints: u32_at!(2 x 68),
            poses: u32_at!(2 x 76),
            animations: u32_at!(2 x 84),
            frames: u32_at!(4 x 92),
            comments: u32_at!(2 x 108),
            extensions: u32_at!(2 x 116),
        };

        if header.filesize as usize == bs.len() && header.validate() {
            Some(header)
        } else {
            None
        }
    }

    /// Validates that a header is reasonable. Also check that `self.filesize == file.len()`!
    fn validate(&self) -> bool {
        if &self.magic != b"INTERQUAKEMODEL\0" || self.version != 2 {
            return false;
        }

        let check_filesize = |n: u32, (l, o): (u32, u32)| {
            let l = l.checked_mul(n).and_then(|l| l.checked_add(o));
            match l {
                Some(l) => l <= self.filesize,
                None => false,
            }
        };

        check_filesize(1, self.text) &&
        check_filesize(24, self.meshes) &&

        // TODO: vertex_arrays: (u32, u32, u32),
        // TODO: triangles: (u32, u32, u32),

        check_filesize(1, self.joints) && // TODO: Fixme
        check_filesize(1, self.poses) && // TODO: Fixme
        check_filesize(1, self.animations) && // TODO: Fixme

        // TODO: frames: (u32, u32, u32, u32),

        check_filesize(1, self.comments) &&
        check_filesize(1, self.extensions)
    }
}

fn parse_meshes(bs: &[u8]) -> Option<Vec<Mesh>> {
    fn range(n: u32, l: u32) -> Range<usize> {
        let n = n as usize;
        let l = l as usize;
        n..(n + l)
    }

    if bs.len() % 24 != 0 {
        return None;
    }
    let meshes = (0..bs.len() / 24)
        .into_iter()
        .map(|n| n * 24)
        .map(|n| &bs[n..][..24])
        .map(|bs| Mesh {
            name: match LittleEndian::read_u32(&bs[0..4]) {
                0 => None,
                n => Some(n as usize - 1),
            },
            material: LittleEndian::read_u32(&bs[4..8]),
            vertices: range(
                LittleEndian::read_u32(&bs[8..12]),
                LittleEndian::read_u32(&bs[12..16]),
            ),
            triangles: range(
                LittleEndian::read_u32(&bs[16..20]),
                LittleEndian::read_u32(&bs[20..24]),
            ),
        })
        .collect();
    Some(meshes)
}

fn parse_texts(mut bs: &[u8]) -> Option<Vec<String>> {
    let mut texts = Vec::new();
    while !bs.is_empty() {
        let len = bs.iter().cloned().position(|b| b == 0)?;
        let s = from_utf8(&bs[..len]).ok()?;
        debug_assert_eq!(bs[len], 0);
        bs = &bs[len + 1..];
        texts.push(s.to_string());
    }
    if !texts.is_empty() {
        if !texts[0].is_empty() {
            return None;
        }
        let _ = texts.remove(0);
    }
    Some(texts)
}
