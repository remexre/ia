use crate::Asset;
use derivative::Derivative;
use ecstasy::{Component, ComponentStore, SystemMut};
use log::{debug, error};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::{
    collections::HashMap,
    fs::read,
    marker::PhantomData,
    path::PathBuf,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc,
    },
    thread::{spawn, JoinHandle},
};
use typemap::{Key, TypeMap};

/// A `SystemMut` for loading assets. See the "Asset Loading" chapter of the documentation for more
/// information.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Loader {
    #[derivative(Debug = "ignore")]
    cache: TypeMap,
    // An Option so it can be dropped; this lets thread_fn block while waiting for new requests,
    // while also quickly exiting in the case of a requested stop.
    file_reqs: Option<Sender<(AssetKind, PathBuf)>>,
    file_resps: Receiver<(AssetKind, PathBuf, Vec<u8>)>,
    thread: Option<JoinHandle<()>>,
}

impl Loader {
    /// Creates a new `Loader`.
    pub fn new() -> Loader {
        Loader::default()
    }
}

impl Default for Loader {
    fn default() -> Loader {
        let (file_reqs_send, file_reqs_recv) = channel();
        let (file_resps_send, file_resps_recv) = channel();
        let thread = spawn(move || {
            while let Ok((kind, path)) = file_reqs_recv.recv() {
                if let Ok(bytes) = read(&path) {
                    file_resps_send.send((kind, path, bytes)).unwrap();
                }
            }
        });
        Loader {
            cache: TypeMap::new(),
            file_reqs: Some(file_reqs_send),
            file_resps: file_resps_recv,
            thread: Some(thread),
        }
    }
}

impl Drop for Loader {
    fn drop(&mut self) {
        drop(self.file_reqs.take());
        let _ = self
            .thread
            .take()
            .expect("double-dropped assets::Loader")
            .join();
    }
}

impl SystemMut for Loader {
    fn run(&mut self, cs: &mut ComponentStore, _dt: f32) {
        macro_rules! match_with {
            ($kind:ident as $ty:ident $body:block) => {
                match $kind {
                    AssetKind::Model => {
                        type $ty = crate::Model;
                        $body
                    }
                    AssetKind::Texture => {
                        type $ty = crate::Texture;
                        $body
                    }
                }
            };
        }

        cs.iter_entities().for_each(|entity| {
            if let Some(reqs) = cs.get_mut_component::<AssetRequests>(entity).as_mut() {
                for req in reqs.0.iter_mut() {
                    // TODO: Check cache.
                    if !req.loading {
                        debug!("Loading {}", req.path.display());
                        let _ = self
                            .file_reqs
                            .as_mut()
                            .unwrap()
                            .send((req.kind, req.path.clone()));
                        req.loading = true;
                    }
                }
            }
        });

        while let Ok((kind, path, data)) = self.file_resps.try_recv() {
            match_with!(kind as T {
                // We check the cache again; in all likelihood, the same asset has been requested
                // more than once at the same time.
                match <T as Asset>::load_from(&data) {
                    Ok(asset) => {
                        let c = self.cache.entry::<CacheEntry<T>>()
                            .or_insert_with(HashMap::new)
                            .entry(path)
                            .or_insert_with(|| Arc::new(asset))
                            .clone();
                        cs.iter_entities().for_each(|entity| {
                            cs.set_component::<<T as Asset>::Component>(entity, c.clone().into());
                        });
                    }
                    Err(err) => {
                        error!("Could not load {}: {}", path.display(), err)
                    }
                }
            });
        }
    }
}

/// The kind of an asset.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum AssetKind {
    /// A model, loaded as an IQM file.
    Model,

    /// A texture, loaded as a JPEG or PNG.
    Texture,
}

/// A request for an asset to be loaded, converted to the appropriate component, and attached.
#[derive(Debug, Deserialize, Serialize)]
pub struct AssetRequest {
    pub kind: AssetKind,
    pub path: PathBuf,
    pub loading: bool,
}

/// A `Component` for `AssetRequest`s.
#[derive(Component, Debug, Deserialize, Serialize)]
pub struct AssetRequests(pub SmallVec<[AssetRequest; 4]>);
// TODO(perf): The maximum capacity in practice would be a fine thing to measure; I'd be shocked if
// it were >8, and mildly surprised if it were >4. I think it'd be fine to decrease this to 2 in
// the name of common-case performance (or even Box<SmallVec<[AssetRequest; 8]>> or summat); this
// /shouldn't/ be any sort of bottleneck though.

struct CacheEntry<T>(PhantomData<T>);

impl<T: Asset> Key for CacheEntry<T> {
    type Value = HashMap<PathBuf, Arc<T::Inner>>;
}
