use std::fmt::Error;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use std::path::Path;

pub struct Svg {
    height: u32,
    width_bars: u32
}

impl Svg {
    pub fn new(height: u32, width_bars: u32) -> Self {
        Svg {height, width_bars}    
    }

    pub fn generate(&self, encode: Vec<u8>) -> Result<String, Error> {
        let width = (encode.len() as u32) * self.width_bars;

        let bars: String = encode.iter()
            .enumerate()
            .filter(|&(_, &n)| n == 1)
            .map(|(pos, &color)| {
                
                let fill = match color {
                    1 => "000000",
                    _ => "ffffff"
                };

                format!(
                    "<rect x=\"{}\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"#{}\"/>",
                    pos as u32 * self.width_bars,
                    self.width_bars,
                    self.height,
                    fill
                )
            })
            .collect();

        Ok(format!(
            "<svg version=\"1.1\" viewBox=\"0 0 {} {}\">{}{}</svg>",
            width,
            self.height,
            format!(
                "<rect x=\"0\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"#{}\"/>",
                width,
                self.height,
                "ffffff"
            ),
            bars
        ))
    }

    pub fn save_to<P: AsRef<Path>>(&self, data: String, path: P) -> Result<(), Error> {
        let file = File::create(&path).unwrap();
        let mut writer = BufWriter::new(file);
        writer.write(data.as_bytes()).unwrap();

        Ok(())
    } 
}