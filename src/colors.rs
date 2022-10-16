/// A color. Contains r, g, b, and a values.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Color {
    /// Red (#FF0000).
    pub const RED: Color = Color::rgb(255, 0, 0);
    /// Green (#00FF00).
    pub const GREEN: Color = Color::rgb(0, 255, 0);
    /// Blue (#0000FF).
    pub const BLUE: Color = Color::rgb(0, 0, 255);
    /// Yellow (#FFFF00).
    pub const YELLOW: Color = Color::rgb(255, 255, 0);
    /// Cyan (#00FFFF).
    pub const CYAN: Color = Color::rgb(0, 255, 255);
    /// Magenta (#FF00FF).
    pub const MAGENTA: Color = Color::rgb(255, 0, 255);
    /// White (#FFFFFF).
    pub const WHITE: Color = Color::rgb(255, 255, 255);
    /// Black (#000000).
    pub const BLACK: Color = Color::rgb(0, 0, 0);
    /// Transparent (#00000000).
    pub const TRANSPARENT: Color = Color::rgba(0, 0, 0, 0);

    /// Creates a new `Color` from the given RGB values.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
    /// Creates a new `Color` from the given RGBA values.
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    /// Creates a new `Color` from the given HSL values (note: this is not free, unlike `rgb` and `rgba`).
    pub fn hsl(h: u16, s: f32, l: f32) -> Self {
        Self::hsla(h, s, l, 255)
    }
    /// Creates a new `Color` from the given HSLA values (note: this is not free, unlike `rgb` and `rgba`).
    pub fn hsla(h: u16, s: f32, l: f32, a: u8) -> Self {
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h as f32 / 60.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;
        let (rprime, gprime, bprime) = if h <= 60 {
            (c, x, 0.0)
        } else if h < 120 {
            (x, c, 0.0)
        } else if h < 180 {
            (0.0, c, x)
        } else if h < 240 {
            (0.0, x, c)
        } else if h < 300 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };
        Self {
            r: ((rprime + m) * 255.0) as u8,
            g: ((gprime + m) * 255.0) as u8,
            b: ((bprime + m) * 255.0) as u8,
            a,
        }
    }
    /// Creates a new `Color` from the given hex string.
    pub fn from_hex(hex: &str) -> Self {
        let mut hex = hex;
        if hex.starts_with('#') {
            hex = &hex[1..];
        }
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
        let a = if hex.len() == 8 {
            u8::from_str_radix(&hex[6..8], 16).unwrap()
        } else {
            255
        };
        Self { r, g, b, a }
    }
    /// Outputs the color as a hex string.
    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
    }
}
impl From<u32> for Color {
    /// Creates a new `Color` from a 64-bit integer. Bit interpretation is as follows:
    /// - 0..8: Alpha
    /// - 8..16: Blue
    /// - 16..24: Green
    /// - 24..32: Red
    /// This means that `0xFF000000` is 100% red.
    fn from(color: u32) -> Self {
        Self {
            r: ((color >> 24) & 0xFF) as u8,
            g: ((color >> 16) & 0xFF) as u8,
            b: ((color >> 8) & 0xFF) as u8,
            a: (color & 0xFF) as u8,
        }
    }
}
