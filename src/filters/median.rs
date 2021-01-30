use super::{convolve, Df32, Filter};

pub fn median_filter<const N: usize>(mut height: usize, mut width: usize) -> Filter<N, N> {
    // make height width odd, else they create problems
    if height % 2 == 0 {
        height -= 1;
        if width % 2 == 0 {
            width -= 1;
        }
    }

    Filter {
        height,
        width,
        kernel: Box::new(move |m, idx| {
            let mut x = [0f32; N];
            let half_v = height / 2;
            let half_h = width / 2;
            let size = height * width;
            let mut v;
            for i in 0..N {
                v = m
                    .index((
                        idx.0 - half_h..idx.0 + half_h + 1,
                        idx.1 - half_v..idx.1 + half_v + 1,
                    ))
                    .iter()
                    .map(|elem| elem[i] as u8)
                    .collect::<Vec<u8>>();
                v.sort_unstable();
                x[i] = v[size / 2] as f32;
            }
            x
        }),
    }
}
pub fn median<const N: usize>(img: &Df32<N>, kernel_size: usize) -> Df32<N> {
    convolve(median_filter(kernel_size, kernel_size), img)
}
