use image;
use nalgebra as na;

use silma::filters::average;

#[test]
fn blur_test() -> Result<(), Box<dyn std::error::Error>> {
    let data_iter = std::fs::read_dir("imgs/chessboard")?.into_iter();
    for img_file in data_iter {
        let img_file = img_file?;
        let img = image::open(img_file.path())?.to_rgb8();

        let img_matrix = na::DMatrix::from_iterator(
            img.width() as usize,
            img.height() as usize,
            img.pixels()
                .map(|pixel| [pixel[0] as f32, pixel[1] as f32, pixel[2] as f32]),
        );

        let blurred = average(&img_matrix, 5);

        let mut img = image::RgbImage::new(blurred.shape().0 as u32, blurred.shape().1 as u32);

        for i in 0..blurred.shape().0 {
            for j in 0..blurred.shape().1 {
                img.put_pixel(
                    i as u32,
                    j as u32,
                    image::Rgb::from([
                        blurred[(i, j)][0] as u8,
                        blurred[(i, j)][1] as u8,
                        blurred[(i, j)][2] as u8,
                    ]),
                );
            }
        }

        img.save(format!(
            "temp/{}",
            img_file.file_name().into_string().unwrap()
        ))?;
    }

    Ok(())
}
