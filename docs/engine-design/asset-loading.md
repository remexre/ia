Asset Loading
=============

The [`assets`](../api/assets) crate loads various game assets: models, sounds, shaders, textures, etc. They can be compiled into the binary, loaded from disk, or both.

In the debug configuration, only disk loading is compiled in; in the release configuration, both are. If there are WebAssembly builds in the future, they will likely only have compiled-in assets supported.

Assets are stored in IRB (Ia Resource Bundle) files. The [`ia-asset-tool`](../api/ia_asset_tool) binary can be used to manipulate these files. Inside the archive, the following formats are used:

-	Models: Custom
-	Sounds: [Ogg Vorbis](https://en.wikipedia.org/wiki/Vorbis)
-	Shaders: [SPIR-V](https://en.wikipedia.org/wiki/Standard_Portable_Intermediate_Representation)
-	Textures: [JPEG](https://en.wikipedia.org/wiki/JPEG) or [PNG](https://en.wikipedia.org/wiki/Portable_Network_Graphics)

The bundles themselves are zstd-compressed bincode values; see the API docs for the [`assets::irb`](../api/assets/irb) module for more information.
