#[derive(Clone, Copy, Debug)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    pub const ZERO: Self = Self {
        width: 0,
        height: 0,
    };

    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn clamp(&self, min: Size, max: Size) -> Size {
        Self {
            width: self.width.clamp(min.width, max.width),
            height: self.height.clamp(min.height, max.height),
        }
    }
}

impl From<(u16, u16)> for Size {
    fn from(s: (u16, u16)) -> Self {
        Self {
            width: s.0 as usize,
            height: s.1 as usize,
        }
    }
}
