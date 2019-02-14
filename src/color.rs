pub struct RGB {
        r: f64,
        g: f64,
        b: f64
    }


impl RGB {
    pub fn new(r: f64, g: f64, b: f64) -> RGB {
        RGB { r: r, g: g, b: b }
    }

    pub fn as_string(&self) -> String {
        format!("rgb({}, {}, {})", self.r.ceil(), self.g.ceil(), self.b.ceil())
    }

    pub fn darken(&mut self, percent: f64){
        self.r *= 1.0 - percent/100.0;
        self.g *= 1.0 - percent/100.0;
        self.b *= 1.0 - percent/100.0;
    }

    pub fn lighten(&mut self, percent: f64){
        self.r = self.r + (255.0 - self.r) * (percent/100.0);
        self.g = self.g + (255.0 - self.g) * (percent/100.0);
        self.b = self.b + (255.0 - self.b) * (percent/100.0);
    }

    pub fn to_hex_string(&self) -> String {
        let r_int = self.r.ceil() as i32;
        let g_int = self.g.ceil() as i32;
        let b_int = self.b.ceil() as i32;
        
        format!("{:02X}{:02X}{:02X}", r_int, g_int, b_int)
    }
}

pub struct RGBA {
    r: f64,
    g: f64,
    b: f64,
    a: f64
}

impl RGBA {
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> RGBA {
        RGBA { r: r, g: g, b: b, a: a }
    }
    
    pub fn lighten(&mut self, percent: f64){
        self.r = self.r.ceil() + (255.0 - self.r) * (percent/100.0) * self.a;
        self.g = self.g.ceil() + (255.0 - self.g) * (percent/100.0) * self.a;
        self.b = self.b.ceil() + (255.0 - self.b) * (percent/100.0) * self.a;
    }
}
