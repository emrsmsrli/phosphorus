use std::{self, ops, path::Path, fs::File, io::Write};

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl ops::Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        Color::new(self.r / rhs, self.g  / rhs, self.b / rhs)
    }
}

pub struct Writer {
    data: Vec<Color>,
    width: usize,
    height: usize,
}

impl Writer {
    pub fn new(width: usize, height: usize) -> Self {
        Writer {
            data: vec![Color::new(0.0, 0.0, 0.0); width * height],
            width,
            height,
        }
    }

    pub fn save_to_file(&self, path: &Path) {
        let mut f = File::create(path)
            .expect("could not create file at");

        f.write_fmt(format_args!("P3\n{} {}\n{}\n", self.width, self.height, 255))
            .expect("error while writing ppm header");
        for color in self.data.iter() {
            f.write_fmt(format_args!("{} {} {}\n",
                                     (255.0 * color.r) as u8,
                                     (255.0 * color.g) as u8,
                                     (255.0 * color.b) as u8))
                .expect("error while writing ppm color");
        }
    }
}

impl ops::IndexMut<(usize, usize)> for Writer {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.1 * self.width as usize + index.0]
    }
}

impl ops::Index<(usize, usize)> for Writer {
    type Output = Color;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.1 * self.width as usize + index.0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color() {
        let c = Color { r: 2.0, g: 5.0, b: 10.0 };
        assert_eq!(c.r, 2.0);
        assert_eq!(c.g, 5.0);
        assert_eq!(c.b, 10.0);
    }

    #[test]
    fn test_writer() {
        let mut c = Writer::new(100, 100);
        assert_eq!(c[(0, 0)].r, 0.0);

        c[(0, 0)] = Color { r: 1.0, g: 1.0, b: 1.0 };
        assert_eq!(c[(0, 0)].r, 1.0);
        assert_eq!(c[(0, 1)].r, 0.0);
    }
}
