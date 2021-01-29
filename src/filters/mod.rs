pub mod average;
pub mod gaussian;

use na::{Dynamic, MatrixSlice};
use nalgebra as na;

use super::Df32;

pub struct Filter<const N: usize> {
    pub kernel:
        Box<dyn Fn(MatrixSlice<'_, [f32; N], Dynamic, Dynamic, na::U1, Dynamic>) -> [f32; N]>,
    pub height: usize,
    pub width: usize,
}

pub fn conv2d<const N: usize>(filter: Filter<N>, img: &Df32<N>) -> Df32<N> {
    use crate::util::pad;
    let img = pad(img, filter.height, filter.width);
    let mut h = img.clone();

    let half_v = filter.height / 2;
    let half_h = filter.width / 2;

    for i in half_v..img.shape().0 - half_v {
        for j in half_h..img.shape().1 - half_h {
            h[(i, j)] =
                (filter.kernel)(img.index((i - half_v..i + half_v + 1, j - half_h..j + half_h + 1)));
        }
    }

    h
}
