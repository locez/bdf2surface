use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;

/// A crate to convert bdf font to sdl2 surface
///
///  #Example
///
/// ```
/// use sdl2::surface::Surface;
/// use bdf2surface::{Color, Converter, Text};
///
///
/// let sdl_context = sdl2::init()?;
/// let _video_subsystem = sdl_context.video()?;
/// let path = "./resource/wqy_9pt.bdf";
/// let converter = Converter::new(path);
///
/// let texts = vec![
///     Text("你好，露西！".to_string(), Color::RGB(0, 0, 0)),
///     Text(
///         "Hello,                    ".to_string(),
///         Color::RGB(255, 0, 0),
///     ),
///     Text("Locez!".to_string(), Color::RGB(255, 73, 170)),
/// ];
/// let surface = converter.render(&texts, 60).unwrap();
/// surface.save_bmp("output.bmp")?;
/// ```
///

#[derive(Debug, Clone)]
pub enum Color {
    RGB(u8, u8, u8),
}

#[derive(Debug)]
pub struct Text(pub String, pub Color);

pub struct Converter {
    font_parser: bdf::Font,
}

impl Converter {
    pub fn new(font_path: &str) -> Converter {
        let font = bdf::open(font_path).unwrap();
        Converter { font_parser: font }
    }

    pub fn render(self: &Self, texts: &Vec<Text>, max_line_width: u32) -> Result<Surface, String> {
        // Calculate total width and max height
        let extra_spacing = 1;
        let mut max_height = 0;

        let mut lines = vec![];
        let mut current_line = String::new();
        let mut current_line_text = vec![];
        let mut current_width = 0;

        for text in texts {
            for ch in text.0.chars() {
                let glyph = self.font_parser.glyphs().get(&ch).unwrap();
                if current_width + glyph.width() + extra_spacing > max_line_width {
                    current_line_text.push((current_line.clone(), text.1.clone()));
                    lines.push((current_line_text.clone(), current_width));
                    current_line.clear();
                    current_width = 0;
                    current_line_text.clear();
                }
                current_line.push(ch);
                current_width += glyph.width() + extra_spacing;

                if glyph.height() > max_height {
                    max_height = glyph.height();
                }
            }
            current_line_text.push((current_line.clone(), text.1.clone()));
            current_line.clear();
        }
        lines.push((current_line_text.clone(), current_width));

        // Create an SDL2 surface
        let width = max_line_width;
        let height = lines.len() as u32 * max_height as u32;
        let mut surface = Surface::new(width, height, PixelFormatEnum::RGBA32)?;

        let mut y_offset = 0;
        // Render each character
        for (line, _line_width) in lines {
            let mut x_offset = 0;
            for text in line {
                for ch in text.0.chars() {
                    let bitmap = self.font_parser.glyphs().get(&ch).unwrap();
                    let y_start = y_offset + max_height - bitmap.height();
                    // Lock the surface to get access to its pixels
                    surface.with_lock_mut(|buffer: &mut [u8]| {
                        for y in 0..bitmap.height() {
                            for x in 0..bitmap.width() {
                                //let offset = (y * bitmap.width() + x) as usize;
                                let alpha = if bitmap.get(x, y) { 255 } else { 0 };
                                let buffer_offset =
                                    ((((y_start + y) * width) + x_offset + x) * 4) as usize;
                                buffer[buffer_offset + 3] = alpha; // A
                                match text.1 {
                                    Color::RGB(r, g, b) => {
                                        buffer[buffer_offset] = r; // R
                                        buffer[buffer_offset + 1] = g; // G
                                        buffer[buffer_offset + 2] = b; // B
                                    }
                                }
                            }
                        }
                    });

                    x_offset += bitmap.width() + extra_spacing;
                }
            }
            y_offset += max_height;
        }

        Ok(surface)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_test() -> Result<(), String> {
        let sdl_context = sdl2::init()?;
        let _video_subsystem = sdl_context.video()?;
        let path = "./resource/wqy_9pt.bdf";
        let converter = Converter::new(path);

        let texts = vec![
            Text("你好，露西！".to_string(), Color::RGB(0, 0, 0)),
            Text(
                "Hello,                    ".to_string(),
                Color::RGB(255, 0, 0),
            ),
            Text("Locez!".to_string(), Color::RGB(255, 73, 170)),
        ];
        let surface = converter.render(&texts, 60).unwrap();
        surface.save_bmp("output.bmp")?;
        Ok(())
    }
}
