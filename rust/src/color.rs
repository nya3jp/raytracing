#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl std::ops::Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl std::ops::Sub<Color> for Color {
    type Output = Color;
    fn sub(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Color;
    fn mul(self, m: f64) -> Self::Output {
        Color {
            r: self.r * m,
            g: self.g * m,
            b: self.b * m,
        }
    }
}

impl std::ops::Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, c: Color) -> Self::Output {
        c * self
    }
}

impl std::ops::Div<f64> for Color {
    type Output = Color;
    fn div(self, m: f64) -> Self::Output {
        Color {
            r: self.r / m,
            g: self.g / m,
            b: self.b / m,
        }
    }
}

impl Color {
    pub const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };
    pub const WHITE: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };

    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }

    pub fn encode(self) -> [u8; 3] {
        [
            (self.r * 255.999) as u8,
            (self.g * 255.999) as u8,
            (self.b * 255.999) as u8,
        ]
    }
}
