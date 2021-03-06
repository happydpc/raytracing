use crate::color::{color, Color, BLACK};
use std::io::Write;

pub fn canvas(w: u32, h: u32) -> Canvas {
    Canvas::new(w, h)
}

pub struct Canvas {
    width: u32,
    height: u32,
    data: Vec<Color>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Canvas {
            width,
            height,
            data: vec![color(0, 0, 0); (width * height) as usize],
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn average_brightness(&self) -> Color {
        let mut sum = BLACK;
        for &c in &self.data {
            sum = sum + c;
        }
        sum / (self.width * self.height)
    }

    pub fn clear(&mut self, c: Color) {
        self.data.clear();
        self.data.resize((self.width * self.height) as usize, c);
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, c: Color) {
        self.rows_mut().skip(y as usize).next().unwrap()[x as usize] = c;
    }

    pub fn add_to_pixel(&mut self, x: u32, y: u32, c: Color) {
        let pix = &mut self.rows_mut().skip(y as usize).next().unwrap()[x as usize];
        *pix = *pix + c;
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        self.rows().skip(y as usize).next().unwrap()[x as usize]
    }

    pub fn rows(&self) -> impl Iterator<Item = &[Color]> + '_ {
        self.data.chunks_exact(self.width as usize)
    }

    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut [Color]> + '_ {
        self.data.chunks_exact_mut(self.width as usize)
    }

    pub fn flat(&self) -> impl Iterator<Item = Color> + '_ {
        self.data.iter().copied()
    }

    pub fn write_ppm(&self, writer: &mut impl Write) -> std::io::Result<()> {
        let mut line_guard = MaxWidthWriter::new(70, writer);
        self.write_ppm_header(&mut line_guard)?;
        self.write_ppm_data(&mut line_guard)
    }

    fn write_ppm_header(&self, writer: &mut impl Write) -> std::io::Result<()> {
        write!(writer, "P3\n{} {}\n255\n", self.width, self.height)
    }

    fn write_ppm_data(&self, writer: &mut impl Write) -> std::io::Result<()> {
        for row in self.rows() {
            for (i, pixel) in row.iter().enumerate() {
                if i > 0 {
                    write!(writer, " ")?;
                }
                let (r, g, b) = pixel.to_u8();
                write!(writer, "{}", r)?;
                write!(writer, " {}", g)?;
                write!(writer, " {}", b)?;
            }
            write!(writer, "\n")?;
        }
        Ok(())
    }

    pub fn write_png(&self, writer: &mut impl Write) -> std::io::Result<()> {
        let mut encoder = png::Encoder::new(writer, self.width, self.height);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        let data: Vec<_> = self
            .data
            .iter()
            .map(|pixel| pixel.to_u8())
            .map(|(r, g, b)| vec![r, g, b])
            .flatten()
            .collect();
        writer.write_image_data(&data).unwrap();
        Ok(())
    }
}

struct MaxWidthWriter<'a, T: Write> {
    writer: &'a mut T,
    line_buffer: Vec<u8>,
    n_cols: usize,
}

impl<'a, T: Write> MaxWidthWriter<'a, T> {
    pub fn new(n_cols: usize, writer: &'a mut T) -> Self {
        MaxWidthWriter {
            writer,
            line_buffer: vec![],
            n_cols,
        }
    }

    pub fn flush_line(&mut self) -> std::io::Result<()> {
        if self.line_buffer.len() < self.n_cols {
            return Ok(());
        }

        let mut i = self.n_cols;
        loop {
            if self.line_buffer[i] == b' ' {
                return self.flush_partial(i);
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }

        for i in self.n_cols..self.line_buffer.len() {
            if self.line_buffer[i] == b' ' {
                return self.flush_partial(i);
            }
        }

        Ok(())
    }

    fn flush_partial(&mut self, i: usize) -> std::io::Result<()> {
        self.line_buffer[i] = b'\n';
        self.writer.write(&self.line_buffer[..=i])?;
        self.line_buffer.copy_within(i + 1.., 0);
        self.line_buffer.truncate(self.line_buffer.len() - i - 1);
        Ok(())
    }
}

impl<T: Write> Write for MaxWidthWriter<'_, T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.line_buffer.extend_from_slice(buf);

        while let Some(i) = self.line_buffer.iter().position(|&b| b == b'\n') {
            self.flush_partial(i)?;
        }

        while self.line_buffer.len() > self.n_cols {
            self.flush_line()?;
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.write(&self.line_buffer)?;
        self.line_buffer.clear();
        self.writer.flush()
    }
}

impl<T: Write> Drop for MaxWidthWriter<'_, T> {
    fn drop(&mut self) {
        self.flush().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Creating a canvas
    #[test]
    fn test_canvas() {
        let c = canvas(10, 20);
        assert_eq!(c.width(), 10);
        assert_eq!(c.height(), 20);
        assert!(c.flat().all(|pixel| pixel == color(0, 0, 0)));
    }

    /// Writing pixels to a canvas
    #[test]
    fn set_pixel() {
        let mut c = canvas(10, 20);
        let red = color(1, 0, 0);
        c.set_pixel(2, 3, red);
        assert_eq!(c.get_pixel(2, 3), red);
    }

    /// Constructing the PPM header
    #[test]
    fn ppm_header() {
        let c = canvas(5, 3);
        let mut buf = vec![];
        c.write_ppm(&mut buf).unwrap();
        let ppm = String::from_utf8(buf).unwrap();
        assert_eq!(
            ppm.lines().take(3).collect::<Vec<_>>().join("\n"),
            "\
P3
5 3
255"
        );
    }

    /// Constructing the PPM pixel data
    #[test]
    fn ppm_data() {
        let mut c = canvas(5, 3);
        c.set_pixel(0, 0, color(1.5, 0, 0));
        c.set_pixel(2, 1, color(0, 0.5, 0));
        c.set_pixel(4, 2, color(-0.5, 0, 1));
        let mut buf = vec![];
        c.write_ppm(&mut buf).unwrap();
        let ppm = String::from_utf8(buf).unwrap();
        assert_eq!(
            ppm.lines().skip(3).take(3).collect::<Vec<_>>().join("\n"),
            "\
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"
        );
    }

    /// Splitting long lines in PPM files
    #[test]
    fn ppm_linesplit() {
        let mut c = canvas(10, 2);
        c.clear(color(1, 0.8, 0.6));
        let mut buf = vec![];
        c.write_ppm(&mut buf).unwrap();
        let ppm = String::from_utf8(buf).unwrap();
        assert_eq!(
            ppm.lines().skip(3).take(4).collect::<Vec<_>>().join("\n"),
            "\
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
    }

    /// PPM files end with a newline character
    #[test]
    fn ppm_end_newline() {
        let c = canvas(5, 3);
        let mut buf = vec![];
        c.write_ppm(&mut buf).unwrap();
        assert_eq!(buf.last(), Some(&b'\n'))
    }
}
