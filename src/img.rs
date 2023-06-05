#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::path::PathBuf;
use image::{ImageBuffer, Rgb, RgbImage, Pixel};

pub enum Fill {
    Solid(Rgb<u8>),
    Gradient(Rgb<u8>, Rgb<u8>, u32, u32),
}

pub struct WaveformImg {
    ibuf: RgbImage,
    width: u32,
    height: u32,
    vscale: f32,
    fg: Fill,
    bg: Rgb<u8>,
    gradient_table: Option<Vec<Rgb<u8>>>,
}

impl WaveformImg {
    pub fn new(width: u32, height: u32, vscale: f32, fg: Fill, bg: Rgb<u8>) -> Self {
        let gradient_table = match fg {
            Fill::Solid(_) => None,
            Fill::Gradient(rgb0, rgb1, start, end) => Self::populate_gtable(rgb0, rgb1, start, end, height)
        };
        WaveformImg {
            ibuf: ImageBuffer::new(width, height),
            width,
            height,
            vscale,
            fg,
            bg,
            gradient_table,
        }
    }

    fn populate_gtable(rgb0: Rgb<u8>, rgb1: Rgb<u8>, start: u32, end: u32, height_: u32) -> Option<Vec<Rgb<u8>>> {
        let height = height_ as usize;
        let Rgb([r0, g0, b0]) = rgb0;
        let Rgb([r1, g1, b1]) = rgb1;
        let distance = end - start;
        let rdiff = r1 as i16 - r0 as i16;
        let gdiff = g1 as i16 - g0 as i16;
        let bdiff = b1 as i16 - b0 as i16;
        let mut rr = vec![r0; height];
        let mut gg = vec![g0; height];
        let mut bb = vec![b0; height];

        let populate = |values: &mut Vec<u8>, c0: u8, c1: u8, cdiff: i16| {
            for y in start..end {
                let rel_y = y - start;
                let pct = rel_y as f32 / distance as f32;
                let delta = cdiff as f32 * pct;
                let cf = c0 as f32 + delta;
                values[y as usize] = cf as u8; 
            }
            for y in (end as usize)..height {
                values[y] = c1;
            }
        };

        match rdiff {
            0 => (),
            x => {
                populate(&mut rr, r0, r1, rdiff);
            },
        }

        match gdiff {
            0 => (),
            x => {
                populate(&mut gg, g0, g1, gdiff);
            },
        }

        match bdiff {
            0 => (),
            x => {
                populate(&mut bb, b0, b1, bdiff);
            },
        }

        let mut table = Vec::new();

        for y in 0..(height) {
            table.push(Rgb([rr[y], gg[y], bb[y]]));
        }

        Some(table)
    }

    pub fn draw(&mut self, data: Vec<(i16, i16)>) {
        for i in 0..data.len() {
            let (lo, hi) = data[i];
            self.draw_vline(i as u32, self.scale_y(hi), self.scale_y(lo));
        }
    }

    pub fn save(&self, path: &PathBuf) {
        let _ = self.ibuf.save(path);
    } 

    fn scale_y(&self, y: i16) -> u32 {
        let fmid = (self.height / 2) as f32;
        // let fy = y as f32;
        // Flip value because negative sample values should be in the lower half of the image, i.e. high y-coordinates
        let fy = -(y as f32);
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
            self.ibuf.put_pixel(x, y, self.bg);
        }
        match self.fg {
            Fill::Solid(color) => for y in y0..y1 {
                self.ibuf.put_pixel(x, y, color);
            },
            Fill::Gradient(color0, color1, start, end) => {
                let grad_colors = match &self.gradient_table {
                    None => {
                        panic!("ERROR: gradient table was not initialized");
                    },
                    Some(tbl) => {
                        for y in y0..y1 {
                            self.ibuf.put_pixel(x, y, tbl[y as usize]);
                        }
                    }
                };
            }
        }
        for y in y1..self.height {
            self.ibuf.put_pixel(x, y, self.bg);
        }
    }

    /*
    fn interpolate_rgb(&self, y: u32) -> Rgb<u8> {
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
                    let delta = (r1 as i16 - r0 as i16) as f32 * pct;
                    let rf = r0 as f32 + delta;
                    rf as u8
                };
                let g = if g0 == g1 {
                    g0
                } else {
                    let delta = (g1 as i16 - g0 as i16) as f32 * pct;
                    let gf = g0 as f32 + delta;
                    gf as u8
                };
                let b = if b0 == b1 {
                    b0
                } else {
                    let delta = (b1 as i16 - b0 as i16) as f32 * pct;
                    let bf = b0 as f32 + delta;
                    bf as u8
                };
                Rgb([r, g, b])
            },
            Fill::Solid(_) => panic!("This should never happen."),
        }
    }
    */
}
