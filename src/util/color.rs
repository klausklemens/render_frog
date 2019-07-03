pub struct ColorRGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorRGB {
    pub fn new(r: u8, g: u8, b: u8) -> ColorRGB {
        ColorRGB {
            r,
            g,
            b,
        }
    }

    pub fn clone(&self) -> ColorRGB {
        ColorRGB::new(self.r, self.g, self.b)
    }
}
