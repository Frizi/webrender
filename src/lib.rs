#![feature(drain)]
#![feature(hashmap_hasher)]
#![feature(vec_push_all)]
#![feature(step_by, convert, zero_one)]

#[macro_use]
extern crate lazy_static;

mod aabbtree;
mod batch;
mod batch_builder;
mod clipper;
mod device;
mod frame;
mod freelist;
mod internal_types;
mod layer;
mod node_compiler;
mod optimizer;
mod render_backend;
mod resource_cache;
mod resource_list;
mod scene;
mod tessellator;
mod texture_cache;
mod util;

mod platform {
    #[cfg(target_os="macos")]
    pub use platform::macos::font;
    #[cfg(any(target_os="linux", target_os="android"))]
    pub use platform::linux::font;

    #[cfg(target_os="macos")]
    pub mod macos {
        pub mod font;
    }
    #[cfg(any(target_os="linux", target_os="android"))]
    pub mod linux {
        pub mod font;
    }
}

pub mod renderer;

#[cfg(target_os="macos")]
extern crate core_graphics;
#[cfg(target_os="macos")]
extern crate core_text;

#[cfg(not(target_os="macos"))]
extern crate freetype;

extern crate app_units;
extern crate euclid;
extern crate fnv;
extern crate gleam;
extern crate ipc_channel;
extern crate scoped_threadpool;
extern crate simd;
extern crate time;
extern crate webrender_traits;

pub use renderer::Renderer;

#[doc(hidden)]
pub mod bench {
    // to make private modules available to the benchmarks
    pub use clipper::{clip_polygon, ClipBuffers, Polygon};
    pub use internal_types::WorkVertex;
}
