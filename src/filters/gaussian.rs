use super::{super::Df32, conv2d, Filter};
use nalgebra::DMatrix;

pub fn gaussian_kernel(dist: f32, sigma: f32) -> f32 {
    use std::f32::consts::PI;
    let sigma_pow = sigma.powi(2);
    (-dist / (2.0 * sigma_pow)).exp() / (2.0 * PI * sigma_pow)
}

// TODO: optimize to convolve 1 dimension at a time, reduce complextity
pub fn gaussian_filter<const N: usize>(
    mut height: usize,
    mut width: usize,
    sigma: [f32; N],
) -> Filter<N> {
    // make height width odd, else they create problems
    if height % 2 == 0 {
        height -= 1;
        if width % 2 == 0 {
            width -= 1;
        }
    }

    // getting the matrix with distances
    let (iheight, iwidth) = (height as isize, width as isize);
    let h = DMatrix::<f32>::from_fn(height, width, |x, y| {
        (x as isize - iheight / 2).pow(2) as f32 + (y as isize - iwidth / 2).pow(2) as f32
    });

    Filter {
        height,
        width,
        kernel: Box::new(move |m| {
            let mut x = [0f32; N];
            for i in 0..height {
                for j in 0..width {
                    x.iter_mut().enumerate().for_each(|(k, x)| {
                        *x += gaussian_kernel(h[(i, j)], sigma[k]) * m[(i, j)][k]
                    });
                }
            }
            x
        }),
    }
}

pub fn gaussian<const N: usize>(img: &Df32<N>, sigma: f32) -> Df32<N> {
    let size = (6f32 * sigma).ceil() as usize;
    let filter = gaussian_filter(size, size, [sigma; N]);
    conv2d(filter, img)
}
