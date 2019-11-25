//! A piet backend appropriate for the current platform.
//!
//! This crate reexports the [piet crate][piet], alongside an appropriate backend
//! for the given platform. It also exposes [kurbo][], which defines shape and
//! curve types useful in drawing.
//!
//! The intention of this crate is to provide a single dependency that handles
//! the common piet use-case. If you have more complicated needs (such as
//! supporting multiple backends simultaneously) you should use crates such as
//! [piet][] and [piet-cairo][] directly.
//!
//! The associated types for brushes, text, and images are exported as type
//! definitions (resolving to concrete types within the backend), so they can
//! be used directly. The text-related types are prefixed with "Piet" to avoid
//! conflict with the text traits that would otherwise have the same name.
//!
//! Also note that all public types for the specific backend are re-exported,
//! but have their docs hidden here. These types can be useful for platform
//! integration, and also potentially to access extensions specific to the
//! backend. The types documented below can be used portable across all
//! backends.
//!
//! [piet]: https://crates.io/crates/piet
//! [kurbo]: https://crates.io/crates/kurbo
//! [piet-cairo]: https://crates.io/crates/piet-cairo

pub use piet::*;

#[doc(hidden)]
pub use piet::kurbo;

#[cfg(any(
    feature = "cairo",
    not(any(target_arch = "wasm32", target_os = "windows", feature = "direct2d", feature = "embedded-graphics"))
))]
#[path = "cairo_back.rs"]
mod backend;

#[cfg(any(feature = "d2d", all(target_os = "windows", not(feature = "cairo"))))]
#[path = "direct2d_back.rs"]
mod backend;

#[cfg(any(feature = "web", target_arch = "wasm32"))]
#[path = "web_back.rs"]
mod backend;

#[cfg(any(feature = "embedded-graphics", all(target_arch = "arm", target_os = "none")))]
#[path = "embedded_graphics_back.rs"]
mod backend;

pub use backend::*;
