Asset Loading
=============

The [`assets`](../api/assets) crate loads various game assets: models, textures, shaders, etc. They can be compiled into the binary, loaded from disk, or both.

In the debug configuration, only disk loading is compiled in; in the release configuration, both are. If there are WebAssembly builds in the future, they will likely only have compiled-in assets supported.
