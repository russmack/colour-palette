use std::cmp::Ordering;

/// HSVf uses f64 for all fields.
/// h is degrees 0.0 to 360.0
/// s is 0.0 to 1.0
/// v is 0.0 to 1.0
#[derive(Debug)]
pub struct HSVf {
    pub h: f64,
    pub s: f64,
    pub v: f64,
}

pub struct HSV {
    pub h: u16,
    pub s: u8,
    pub v: u8,
}

/// RGBf uses f64 for all fields.
/// There are methods for converting to other representations.
pub struct RGBf {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl HSVf {
    pub fn to_rgbf(&self) -> Result<RGBf, String> {
        if self.h < 0.0 || 
            self.h > 360.0 || 
                self.s < 0.0 || 
                self.s > 1.0 || 
                self.v < 0.0 || 
                self.v > 1.0 {
                    let e = format!("error: invalid HSV: {:?}", self);
                    return Err(e);
        }

        let h = match self.h {
            h if h.floor() as i64 == 360   => 0.0,
            h                              => h / 60.0,
        };

        let fraction: f64 = h - h.floor();

        let p: f64 = self.v * (1.0 - self.s);
        let q: f64 = self.v * (1.0 - self.s * fraction);
        let t: f64 = self.v * (1.0 - self.s * (1.0 - fraction));

        let rgb = match h {
            _ if 0.0 <= h && h < 1.0    => RGBf { r: self.v,    g: t,       b: p       },
            _ if 1.0 <= h && h < 2.0    => RGBf { r: q,         g: self.v,  b: p       },
            _ if 2.0 <= h && h < 3.0    => RGBf { r: p,         g: self.v,  b: t       },
            _ if 3.0 <= h && h < 4.0    => RGBf { r: p,         g: q,       b: self.v  },
            _ if 4.0 <= h && h < 5.0    => RGBf { r: t,         g: p,       b: self.v  },
            _ if 5.0 <= h && h < 6.0    => RGBf { r: self.v,    g: p,       b: q       },
            _                           => RGBf { r: 0.0,       g: 0.0,     b: 0.0     },
        };

        Ok(rgb)
    }
}

impl RGBf {
    pub fn to_u8(&self) -> RGB {
        RGB {
            r: (self.r * 255.0) as u8,
            g: (self.g * 255.0) as u8,
            b: (self.b * 255.0) as u8,
        }
    }
}

impl RGB {
    pub fn to_hsv(&self) -> HSV {
        let r: f32;
        let g: f32;
        let b: f32;

        let sorted_floats = {
            r = f32::from(self.r) / 255.0;
            g = f32::from(self.g) / 255.0;
            b = f32::from(self.b) / 255.0;

            let mut floats: Vec<f32> = vec![r, g, b];
            floats.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

            floats
        };

        let cmax = sorted_floats[2];
        let cmin = sorted_floats[0];
        let d = cmax - cmin;

        // Hue.
        let hue = match cmax {
            _ if r == cmax => (((g - b) / d) % 6.0) * 60.0,
            _ if g == cmax => (((b - r) / d) + 2.0) * 60.0,
            _ if b == cmax => (((r - g) / d) + 4.0) * 60.0,
            _ => 0.0,
        };

        // Saturation.
        let sat = match cmax {
            _ if cmax == 0.0 => 0.0,
            _ => d / cmax,
        };

        // Value / brightness.
        let val = cmax;

        HSV {
            h: hue as u16,
            s: (sat * 100.0) as u8,
            v: (val * 100.0) as u8,
        }
    }
}
 
#[cfg(test)]
mod tests {
    use crate::colour::{HSV, HSVf, RGB, RGBf};

    #[test]
    fn test_rgbf_to_u8() {
        struct Test {
            rgbf: RGBf, 
            rgb:  RGB,
        }

        let tests = vec![
            Test {
                rgbf: RGBf { r: 0.0, g: 0.0, b: 0.0, },
                rgb:  RGB  { r: 0,   g: 0,   b: 0 },
            },
            Test {
                rgbf: RGBf { r: 1.0, g: 1.0, b: 1.0, },
                rgb:  RGB  { r: 255, g: 255, b: 255 },
            },
            Test {
                rgbf: RGBf { r: 0.5, g: 0.5, b: 0.5, },
                rgb:  RGB  { r: 127, g: 127, b: 127 },
            },
            Test {
                rgbf: RGBf { r: 1.0,    g: 0.0, b: 0.0, },
                rgb:  RGB  { r: 255,    g: 0,   b: 0 },
            },
        ];

        for (i, j) in tests.iter().enumerate() {
            let res = j.rgbf.to_u8();

            assert_eq!(
                j.rgb.r, res.r, "case # {} ; for r expected {}, got {}", 
                i, j.rgb.r, res.r);
            assert_eq!(
                j.rgb.g, res.g, "case # {} ; for g expected {}, got {}", 
                i, j.rgb.g, res.g);
            assert_eq!(
                j.rgb.b, res.b, "case # {} ; for b expected {}, got {}", 
                i, j.rgb.b, res.b);
        }
    }


    #[test]
    fn test_hsv_hsvf_to_rgbf() {
        struct Test {
            hsv: HSVf,
            rgb: RGBf,
        }

        let tests = vec![
            Test {
                hsv: HSVf {
                    h: 0.0, // degrees as float
                    s: 0.0, // 0.0 to 1.0
                    v: 0.0, // 0.0 to 1.0
                },
                rgb: RGBf {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                },
            },
            Test {
                hsv: HSVf {
                    h: 90.0,
                    s: 1.0,
                    v: 1.0,
                },
                rgb: RGBf { // chartreuse
                    r: 0.5, // 127.0,
                    g: 1.0, // 255.0,
                    b: 0.0,
                },
            },
            Test {
                hsv: HSVf {
                    h: 60.0,
                    s: 1.0,
                    v: 0.5,
                },
                rgb: RGBf { // olive
                    r: 0.5, // 128.0,
                    g: 0.5, // 128.0,
                    b: 0.0,
                },
            },
        ];

        for (i, t) in tests.iter().enumerate() {

            let hsvf = HSVf { h: t.hsv.h, s: t.hsv.s, v: t.hsv.v };
            let res = match hsvf.to_rgbf() {
                Ok(v)   => v,
                Err(e)  => {
                    assert!(false, "error converting hsv to rgb: {}", e);
                    continue;
                },
            };

            assert_eq!(
                t.rgb.r, res.r, "case # {} ; for r expected {}, got {}", 
                i, t.rgb.r, res.r);
            assert_eq!(
                t.rgb.g, res.g, "case # {} ; for g expected {}, got {}", 
                i, t.rgb.g, res.g);
            assert_eq!(
                t.rgb.b, res.b, "case # {} ; for b expected {}, got {}", 
                i, t.rgb.b, res.b);
        }
    }

    #[test]
    fn test_rgb_to_hsv() {
        struct Test {
            rgb: RGB,
            hsv: HSV,
        }

        let tests = vec![
            Test {
                rgb: RGB {
                    r: 0,
                    g: 0,
                    b: 0,
                },
                hsv: HSV {
                    h: 0,
                    s: 0,
                    v: 0,
                },
            },
            Test {
                rgb: RGB {
                    r: 255,
                    g: 255,
                    b: 255,
                },
                hsv: HSV {
                    h: 0,   // 0
                    s: 0,   // 0.0
                    v: 100, // 100.0
                },
            },
            Test {
                rgb: RGB { // olive
                    r: 128,
                    g: 128,
                    b: 0,
                },
                hsv: HSV {
                    h: 60,
                    s: 100,
                    v: 50,  // 50.2
                },
            },
            Test {
                rgb: RGB { // chartreuse
                    r: 127,
                    g: 255,
                    b: 0,
                },
                hsv: HSV {
                    h: 90,
                    s: 100,
                    v: 100,
                },
            },
        ];

        for (i, t) in tests.iter().enumerate() {
            let res = RGB { r: t.rgb.r, g: t.rgb.g, b: t.rgb.b }.to_hsv();

            assert_eq!(t.hsv.h, res.h, "case # {} ; for r expected {}, got {}", i, t.hsv.h, res.h);
            assert_eq!(t.hsv.s, res.s, "case # {} ; for g expected {}, got {}", i, t.hsv.s, res.s);
            assert_eq!(t.hsv.v, res.v, "case # {} ; for b expected {}, got {}", i, t.hsv.v, res.v);
        }
    }
}

