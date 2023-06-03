#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use image::{ImageBuffer, Rgb, RgbImage, Pixel};

pub enum Fill {
    Solid(Rgb),
    Gradient(Rgb, Rgb, u32, u32),
}

pub struct WaveformImg {
    ibuf: RgbImage,
    width: u32,
    height: u32,
    vscale: f32,
    fg: Fill,
    bg: Rgb,
}

impl WaveformImg {
    pub fn new(height: u32, width: u32, vscale: f32, fg: Fill, bg: Rgb) -> Self {
        WaveformImg {
            ibuf: ImageBuffer::new(width, height),
            width,
            height,
            vscale,
            fg,
            bg,
        }
    }

    fn scale_y(&self, y: i16) -> u32 {
        let fmid = (self.height / 2) as f32;
        let fy = y as f32;
        (fmid + fy * self.vscale) as u32
    }

    fn draw_vline(&mut self, x: u32, y0: u32, y1_: u32) {
        let y1 = if y1_ > self.height {
            println!("WARNING: invalid y-coordinate: {}", y1_);
            self.height
        } else {
            y1_
        };
        for y in 0..y0 {
            self.ibuf.put_pixel(x, y.into(), self.bg);
        }
        match self.fg {
            Fill::Solid(color) => for y in y0..y1 {
                self.ibuf.put_pixel(x, y.into(), color);
            },
            Gradient(color0, color1, start, end) => {
                let mut y = y0;
                while y < start {
                    self.ibuf.put_pixel(x, y,into(), color0);
                    y += 1;
                }
                while y < end {
                    let color = self.interpolate_rgb(y);
                    self.ibuf.put_pixel(x, y.into(), color);
                    y += 1;
                }
                while y < y1 {
                    self.ibuf.put_pixel(x, y,into(), color1);
                    y += 1;
                }
            }
        }
        for y in y1..self.height {
            self.ibuf.put_pixel(x, y.into(), self.bg);
        }
    }

    fn interpolate_rgb(&self, y: u32) -> Rgb {
        match self.fg {
            Fill::Gradient(color0, color1, start, end) => {
                let Rgb([r0, g0, b0]) = color0;
                let Rgb([r1, g1, b1]) = color1;
                let distance = end - start;
                let rel_y = y - start;
                let pct = rel_y as f32 / distance as f32;
                let r = if r0 == r1 {
                    r0
                } else {
                    let delta = (r1 - r0) as f32 / pct;
                    let rf = r0 as f32 + delta;
                    rf as u8
                };
                let g = if g0 == g1 {
                    g0
                } else {
                    let delta = (g1 - g0) as f32 / pct;
                    let gf = g0 as f32 + delta;
                    gf as u8
                };
                let b = if b0 == b1 {
                    b0
                } else {
                    let delta = (b1 - b0) as f32 / pct;
                    let bf = b0 as f32 + delta;
                    bf as u8
                };
                Rgb([r, g, b])
            },
            Fill::Solid(_) => panic!("This should never happen."),
        }
    }

    pub fn draw(&mut self, data: Vec<(i16, i16)>) {
        for i in 0..data.len() {
            let (y0, y1) = data[i];
            self.draw_vline(i as u32, self.scale_y(y0), self.scale_y(y1));
        }
        img
    }
}
