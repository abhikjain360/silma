use super::{convolve, Df32, Filter};
use nalgebra::{Dynamic, Matrix, SliceStorage};

pub fn sobel_filter() -> Filter<1, 2> {
    type SobelKernel = Matrix<
        f32,
        Dynamic,
        Dynamic,
        SliceStorage<'static, f32, Dynamic, Dynamic, nalgebra::U1, Dynamic>,
    >;

    let g = SobelKernel::from_slice(
        &[1f32, 0f32, -1f32, 2f32, 0f32, -2f32, 1f32, 0f32, -1f32],
        3,
        3,
    );

    Filter {
        height: 3,
        width: 3,
        kernel: Box::new(move |m, idx| {
            let mut x = 0f32;
            let mut y = 0f32;
            for i in 0..3 {
                for j in 0..3 {
                    x += m[(idx.0 - 1 + i, idx.1 - 1 + j)][0] * g[(i, j)];
                    y += m[(idx.0 - 1 + i, idx.1 - 1 + j)][0] * g[(j, i)];
                }
            }
            [(x.powi(2) + y.powi(2)).sqrt(), y.atan2(x)]
        }),
    }
}

pub fn sobel(img: &Df32<1>) -> (Df32<1>, Df32<1>) {
    let x = convolve(sobel_filter(), img);
    (
        Df32::from_iterator(x.shape().0, x.shape().1, x.iter().map(|elem| [elem[0]])),
        Df32::from_iterator(x.shape().0, x.shape().1, x.iter().map(|elem| [elem[1]])),
    )
}
