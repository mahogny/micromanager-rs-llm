#[cfg(feature = "spot")]
mod ffi;

#[cfg(feature = "spot")]
pub mod camera;

#[cfg(feature = "spot")]
pub use camera::SpotCamera;
