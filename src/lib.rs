pub mod edge_detection;
pub mod filters;

pub type Df32<const N: usize> = nalgebra::DMatrix<[f32; N]>;

mod util;
pub use util::*;
