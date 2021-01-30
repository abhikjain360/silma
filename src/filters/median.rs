use super::{convolve, Df32, Filter};

pub fn median_filter<const N: usize>(mut height: usize, mut width: usize) -> Filter<N> {
    Filter {
        height,
        width,
        kernel: Box::new(move |m, idx| {
            let mut x = [0f32; N];
            let size = (height * width) as f32;

        })
    }
}

pub fn triangular_filter<const N: usize>(mut height: usize, mut width: usize) -> Filter<N> {
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
        kernel: Box::new(move |m| {
            let mut x = [0f32; N];
            let size = (height * width) as f32;
            for i in 0..N {
                x[i] = m.iter().map(|elem| elem[0]).sum::<f32>() / size;
            }
            x
        }),
    }
}

pub fn average<const N: usize>(img: &Df32<N>, kernel_size: usize) -> Df32<N> {
    convolve(triangular_filter(kernel_size, kernel_size), img)
}
