use super::gaussian_kernel;
use super::{super::Df32, convolve, Filter};
use nalgebra::DMatrix;

// TODO: optimize to convolve 1 dimension at a time, reduce complextity
pub fn bilateral_filter<const N: usize>(
    mut height: usize,
    mut width: usize,
    sigma_s: [f32; N],
    sigma_r: [f32; N],
) -> Filter<N, N> {
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
        kernel: Box::new(move |m, idx| {
            let mut x = [0f32; N];
            let diff_h = idx.0 - (height / 2);
            let diff_v = idx.1 - (width / 2);
            let mut w = [0f32; N];
            for i in 0..height {
                for j in 0..width {
                    x.iter_mut().enumerate().for_each(|(k, x)| {
                        let temp = gaussian_kernel(h[(i, j)], sigma_s[k])
                            * gaussian_kernel(
                                m[(diff_h + i, diff_v + j)][k] * m[(i, j)][k],
                                sigma_r[k],
                            );
                        *x += m[(diff_h + i, diff_v + j)][k] * temp;
                        w[k] += temp;
                    });
                }
            }
            x.iter_mut()
                .enumerate()
                .for_each(|(k, elem)| *elem = *elem / w[k]);
            x
        }),
    }
}

pub fn bilateral<const N: usize>(img: &Df32<N>, sigma_s: f32, sigma_r: f32) -> Df32<N> {
    let size = (6f32 * sigma_s).ceil() as usize;
    let filter = bilateral_filter(size, size, [sigma_s; N], [sigma_r; N]);
    convolve(filter, img)
}
