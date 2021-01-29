use image;

use super::Df32;

pub fn open_luma<P>(path: P) -> Result<Df32<1>, Box<dyn std::error::Error>>
where
    P: AsRef<std::path::Path>,
{
    let img = image::open(path)?;
    unimplemented!();
}

pub fn pad<const N: usize>(m: &Df32<N>, vertical: usize, horizontal: usize) -> Df32<N> {
    let zero = [0.0; N];

    let (height, width) = (m.shape().0 + vertical, m.shape().1 + horizontal);
    let mut h = Df32::<N>::from_iterator(height, width, (0..height * width).map(|_| zero));

    let (half_v, half_h) = (vertical / 2, horizontal / 2);

    for x in 0..m.shape().0 {
        for y in 0..m.shape().1 {
            h[(x + half_v, y + half_h)] = m[(x, y)];
        }
    }

    h
}
