use super::{conv2d, Df32, Filter};

pub fn average(img: &Df32<3>, kernel_size: usize) -> Df32<3> {
    let filter = Filter {
        height: kernel_size,
        width: kernel_size,
        kernel: Box::new(move |m| {
            [
                m.iter().map(|elem| elem[0]).sum::<f32>() / (kernel_size * kernel_size) as f32,
                m.iter().map(|elem| elem[1]).sum::<f32>() / (kernel_size * kernel_size) as f32,
                m.iter().map(|elem| elem[2]).sum::<f32>() / (kernel_size * kernel_size) as f32,
            ]
        }),
    };

    conv2d(filter, img)
}
