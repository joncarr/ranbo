pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> RGB {
        RGB { r: r, g: g, b: b }
    }

    pub fn as_string(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }

    pub fn darken(&mut self, percent: u8) {
        let rf = self.r as f64 * 1.0 - percent as f64 / 100.0;
        let gf = self.g as f64 * 1.0 - percent as f64 / 100.0;
        let bf = self.b as f64 * 1.0 - percent as f64 / 100.0;

        self.r = rf.ceil() as u8;
        self.g = gf.ceil() as u8;
        self.b = bf.ceil() as u8;
    }

    pub fn lighten(&mut self, percent: u8) {
        let rf = self.r as f64 + (255.0 - self.r as f64) * (percent as f64 / 100.0);
        let gf = self.g as f64 + (255.0 - self.g as f64) * (percent as f64 / 100.0);
        let bf = self.b as f64 + (255.0 - self.b as f64) * (percent as f64 / 100.0);

        self.r = rf.ceil() as u8;
        self.g = gf.ceil() as u8;
        self.b = bf.ceil() as u8;
    }

    pub fn to_hex_string(&self) -> String {
        format!("{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

pub struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl RGBA {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> RGBA {
        RGBA {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }

    pub fn lighten(&mut self, percent: u8) {
        let rf = self.r as f64 + (255.0 - self.r as f64) * (percent as f64 / 100.0) * self.a as f64;
        let gf = self.g as f64 + (255.0 - self.g as f64) * (percent as f64 / 100.0) * self.a as f64;
        let bf = self.b as f64 + (255.0 - self.b as f64) * (percent as f64 / 100.0) * self.a as f64;

        self.r = rf.ceil() as u8;
        self.g = gf.ceil() as u8;
        self.b = bf.ceil() as u8;
    }
}
