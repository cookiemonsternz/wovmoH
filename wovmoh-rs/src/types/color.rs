use std::cmp::PartialEq;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use serde::Serialize;

pub enum ColorValue {
    RGB(f64, f64, f64),
    RGBA(f64, f64, f64, f64),
    HEX([char; 6]),
    HEXA([char; 8]),
}

#[derive(Copy, Clone, Default, Serialize, Debug)]
pub struct Color {
    e: [f64; 4],
}

// Basic Features
impl Color {
    pub fn new(v: ColorValue) -> Color {
        match v {
            ColorValue::RGB(r, g, b) => Color { e: [r, g, b, 1.0] },
            ColorValue::RGBA(r, g, b, a) => Color { e: [r, g, b, a] },
            ColorValue::HEX(hex) => {
                let (r, g, b, a) = hex_to_rgba(hex);
                Color { e: [r, g, b, a] }
            }
            ColorValue::HEXA(hex) => {
                let (r, g, b, a) = hexa_to_rgba(hex);
                Color { e: [r, g, b, a] }
            }
        }
    }

    pub const fn default() -> Color {
        Color {
            e: [0.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn r(&self) -> f64 {
        self.e[0]
    }

    pub fn g(&self) -> f64 {
        self.e[1]
    }

    pub fn b(&self) -> f64 {
        self.e[2]
    }

    pub fn a(&self) -> f64 {
        self.e[3]
    }

    pub fn rgb(&self) -> (f64, f64, f64) {
        (self.r(), self.g(), self.b())
    }

    pub fn rgba(&self) -> (f64, f64, f64, f64) {
        (self.r(), self.g(), self.b(), self.a())
    }

    pub fn hex(&self) -> String {
        format!(
            "{:02X}{:02X}{:02X}",
            (self.r() * 255.0) as u8,
            (self.g() * 255.0) as u8,
            (self.b() * 255.0) as u8,
        )
        .to_lowercase()
    }

    pub fn hexa(&self) -> String {
        format!(
            "{:02X}{:02X}{:02X}{:02X}",
            (self.r() * 255.0) as u8,
            (self.g() * 255.0) as u8,
            (self.b() * 255.0) as u8,
            (self.a() * 255.0) as u8,
        )
        .to_lowercase()
    }

    fn clamp(&self) -> Color {
        Color::new(ColorValue::RGBA(
            clamp_x(self.r()),
            clamp_x(self.g()),
            clamp_x(self.b()),
            clamp_x(self.a()),
        ))
    }
}

// Doesn't invert alpha (might want a seperate fn for this)
impl Neg for Color {
    type Output = Color;

    fn neg(self) -> Color {
        Color::new(ColorValue::RGBA(
            1.0 - self.r(),
            1.0 - self.g(),
            1.0 - self.b(),
            self.a(),
        ))
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Color) {
        *self = *self + rhs;
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, rhs: Color) {
        *self = *self - rhs;
    }
}

impl MulAssign for Color {
    fn mul_assign(&mut self, v: Color) {
        *self = *self * v;
    }
}

impl DivAssign<f64> for Color {
    fn div_assign(&mut self, t: f64) {
        *self = *self / t;
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color::new(ColorValue::RGBA(
            self.r() + rhs.r(),
            self.g() + rhs.g(),
            self.b() + rhs.b(),
            self.a() + rhs.a(),
        ))
        .clamp()
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Color {
        Color::new(ColorValue::RGBA(
            self.r() - rhs.r(),
            self.g() - rhs.g(),
            self.b() - rhs.b(),
            self.a() - rhs.a(),
        ))
        .clamp()
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color::new(ColorValue::RGBA(
            self.r() * rhs.r(),
            self.g() * rhs.g(),
            self.b() * rhs.b(),
            self.a() * rhs.a(),
        ))
        .clamp()
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color::new(ColorValue::RGBA(
            self * rhs.r(),
            self * rhs.g(),
            self * rhs.b(),
            self * rhs.a(),
        ))
        .clamp()
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Color {
        Color::new(ColorValue::RGBA(
            self.r() * rhs,
            self.g() * rhs,
            self.b() * rhs,
            self.a() * rhs,
        ))
        .clamp()
    }
}

impl Div for Color {
    type Output = Color;

    fn div(self, rhs: Color) -> Color {
        Color::new(ColorValue::RGBA(
            self.r() / rhs.r(),
            self.g() / rhs.g(),
            self.b() / rhs.b(),
            self.a() / rhs.a(),
        ))
        .clamp()
    }
}

impl Div<Color> for f64 {
    type Output = Color;

    fn div(self, rhs: Color) -> Color {
        Color::new(ColorValue::RGBA(
            self / rhs.r(),
            self / rhs.g(),
            self / rhs.b(),
            self / rhs.a(),
        ))
        .clamp()
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Color {
        Color::new(ColorValue::RGBA(
            self.r() / rhs,
            self.g() / rhs,
            self.b() / rhs,
            self.a() / rhs,
        ))
        .clamp()
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.e == other.e
    }
}

fn clamp_x(x: f64) -> f64 {
    x.min(1.0).max(0.0)
}

fn hex_to_rgba(hex: [char; 6]) -> (f64, f64, f64, f64) {
    let hex_str: String = hex.iter().collect();
    let r = u8::from_str_radix(&hex_str[0..2], 16)
        .map_err(|_| "Invalid hex color")
        .unwrap();
    let g = u8::from_str_radix(&hex_str[2..4], 16)
        .map_err(|_| "Invalid hex color")
        .unwrap();
    let b = u8::from_str_radix(&hex_str[4..6], 16)
        .map_err(|_| "Invalid hex color")
        .unwrap();
    (r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0, 1.0)
}

fn hexa_to_rgba(hex: [char; 8]) -> (f64, f64, f64, f64) {
    let hex_str: String = hex.iter().collect();
    let r = u8::from_str_radix(&hex_str[0..2], 16)
        .map_err(|_| "Invalid hex color")
        .unwrap();
    let g = u8::from_str_radix(&hex_str[2..4], 16)
        .map_err(|_| "Invalid hex color")
        .unwrap();
    let b = u8::from_str_radix(&hex_str[4..6], 16)
        .map_err(|_| "Invalid hex color")
        .unwrap();
    let a = u8::from_str_radix(&hex_str[6..8], 16)
        .map_err(|_| "Invalid hex color")
        .unwrap();
    (
        r as f64 / 255.0,
        g as f64 / 255.0,
        b as f64 / 255.0,
        a as f64 / 255.0,
    )
}
