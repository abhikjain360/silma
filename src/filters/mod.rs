mod average;
pub use average::*;
mod gaussian;
pub use gaussian::*;
mod median;
pub use median::*;
mod bilateral;
pub use bilateral::*;
mod sobel;
pub use sobel::*;

use super::Df32;

pub struct Filter<const N: usize, const M: usize> {
    pub height: usize,
    pub width: usize,
    pub kernel: Box<dyn Fn(&Df32<N>, (usize, usize)) -> [f32; M]>,
}

pub fn convolve<const N: usize, const M: usize>(filter: Filter<N, M>, img: &Df32<N>) -> Df32<M> {
    use crate::util::pad;
    let img = pad(img, filter.height, filter.width);
    let zero = [0f32; M];
    let mut h = Df32::<M>::from_iterator(
        img.shape().0,
        img.shape().1,
        (0..img.shape().0 * img.shape().1).map(|_| zero),
    );

    let half_v = filter.height / 2;
    let half_h = filter.width / 2;

    for i in half_v..img.shape().0 - half_v {
        for j in half_h..img.shape().1 - half_h {
            h[(i, j)] = (filter.kernel)(&img, (i, j));
        }
    }

    h
}
