use image;
use nalgebra as na;

use silma::filters::gaussian::{gaussian, gaussian_kernel};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open("1.jpeg")?.to_rgb8();

    let img_matrix = na::DMatrix::from_iterator(
        img.width() as usize,
        img.height() as usize,
        img.pixels()
            .map(|pixel| [pixel[0] as f32, pixel[1] as f32, pixel[2] as f32]),
    );

    let mut img = image::RgbImage::new(img_matrix.shape().0 as u32, img_matrix.shape().1 as u32);

    for i in 0..img_matrix.shape().0 {
        for j in 0..img_matrix.shape().1 {
            img.put_pixel(
                i as u32,
                j as u32,
                image::Rgb::from([
                    img_matrix[(i, j)][0] as u8,
                    img_matrix[(i, j)][1] as u8,
                    img_matrix[(i, j)][2] as u8,
                ]),
            );
        }
    }

    img.save("testing1.png")?;

    let sigma = 1.0;

    println!(
        "{} {} {}",
        gaussian_kernel(2f32, 1f32) * 255f32,
        gaussian_kernel(1f32, 2f32) * 255f32,
        gaussian_kernel(0f32, 2f32) * 255f32
    );

    let blurred = gaussian(&img_matrix, sigma);
    // let blurred = average(&img_matrix, 5);

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

    img.save("testing.png")?;

    Ok(())
}
