pub mod average;
pub mod gaussian;
pub mod median;

use super::Df32;

pub struct Filter<const N: usize> {
    pub height: usize,
    pub width: usize,
    pub kernel: Box<dyn Fn(&Df32<N>, (usize, usize)) -> [f32; N]>
}

pub fn convolve<const N: usize>(filter: Filter<N>, img: &Df32<N>) -> Df32<N> {
    use crate::util::pad;
    let img = pad(img, filter.height, filter.width);
    let mut h = img.clone();

    let half_v = filter.height / 2;
    let half_h = filter.width / 2;

    for i in half_v..img.shape().0 - half_v {
        for j in half_h..img.shape().1 - half_h {
            h[(i, j)] =
                (filter.kernel)(&img, (i, j));
        }
    }

    h
}

