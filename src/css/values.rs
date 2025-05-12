use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    Percentage(f32),
    Color(Color),
    Url(String),
    String(String),
    Function(String, Vec<Value>),
    Rect(Rect),
    Initial,
    Inherit,
    Unset,
    CurrentColor,
    Auto,
    None,
    LinearGradient(Box<LinearGradient>), // Boxed to prevent infinite size
    List(Vec<Value>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Unit {
    Px,
    Em,
    Rem,
    Ex,
    Ch,
    Vw,
    Vh,
    Vmin,
    Vmax,
    Pt,
    Pc,
    In,
    Cm,
    Mm,
    Q,
    Deg,
    Rad,
    Grad,
    Turn,
    S,
    Ms,
    Hz,
    Khz,
    Dpi,
    Dpcm,
    Dppx,
    Fr,
    Percent,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rect {
    pub top: Box<Value>,
    pub right: Box<Value>,
    pub bottom: Box<Value>,
    pub left: Box<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LinearGradient {
    pub direction: Box<Value>,
    pub stops: Vec<GradientStop>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GradientStop {
    pub color: Color,
    pub position: Option<Value>,
}

impl Value {
    pub fn to_px(&self, base_font_size: f32) -> f32 {
        match self {
            Value::Length(n, Unit::Px) => *n,
            Value::Length(n, Unit::Em) => *n * base_font_size,
            Value::Length(n, Unit::Rem) => *n * base_font_size,
            Value::Percentage(p) => *p / 100.0 * base_font_size,
            _ => 0.0,
        }
    }

    pub fn to_color(&self) -> Option<Color> {
        match self {
            Value::Color(c) => Some(c.clone()),
            Value::CurrentColor => None,
            _ => None,
        }
    }
}

impl FromStr for Unit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "px" => Ok(Unit::Px),
            "em" => Ok(Unit::Em),
            "rem" => Ok(Unit::Rem),
            "ex" => Ok(Unit::Ex),
            "ch" => Ok(Unit::Ch),
            "vw" => Ok(Unit::Vw),
            "vh" => Ok(Unit::Vh),
            "vmin" => Ok(Unit::Vmin),
            "vmax" => Ok(Unit::Vmax),
            "pt" => Ok(Unit::Pt),
            "pc" => Ok(Unit::Pc),
            "in" => Ok(Unit::In),
            "cm" => Ok(Unit::Cm),
            "mm" => Ok(Unit::Mm),
            "q" => Ok(Unit::Q),
            "deg" => Ok(Unit::Deg),
            "rad" => Ok(Unit::Rad),
            "grad" => Ok(Unit::Grad),
            "turn" => Ok(Unit::Turn),
            "s" => Ok(Unit::S),
            "ms" => Ok(Unit::Ms),
            "hz" => Ok(Unit::Hz),
            "khz" => Ok(Unit::Khz),
            "dpi" => Ok(Unit::Dpi),
            "dpcm" => Ok(Unit::Dpcm),
            "dppx" => Ok(Unit::Dppx),
            "fr" => Ok(Unit::Fr),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Unit::Px => "px",
                Unit::Em => "em",
                Unit::Rem => "rem",
                Unit::Ex => "ex",
                Unit::Ch => "ch",
                Unit::Vw => "vw",
                Unit::Vh => "vh",
                Unit::Vmin => "vmin",
                Unit::Vmax => "vmax",
                Unit::Pt => "pt",
                Unit::Pc => "pc",
                Unit::In => "in",
                Unit::Cm => "cm",
                Unit::Mm => "mm",
                Unit::Q => "q",
                Unit::Percent => "%",
                Unit::Deg => "deg",
                Unit::Rad => "rad",
                Unit::Grad => "grad",
                Unit::Turn => "turn",
                Unit::S => "s",
                Unit::Ms => "ms",
                Unit::Hz => "Hz",
                Unit::Khz => "kHz",
                Unit::Dpi => "dpi",
                Unit::Dpcm => "dpcm",
                Unit::Dppx => "dppx",
                Unit::Fr => "fr",
            }
        )
    }
}

impl Color {
    /// Parses hex color formats: #rgb, #rgba, #rrggbb, #rrggbbaa
    pub fn parse_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
                Some(Color { r, g, b, a: 1.0 })
            }
            4 => {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
                let a = u8::from_str_radix(&hex[3..4].repeat(2), 16).ok()? as f32 / 255.0;
                Some(Color { r, g, b, a })
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some(Color { r, g, b, a: 1.0 })
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()? as f32 / 255.0;
                Some(Color { r, g, b, a })
            }
            _ => None,
        }
    }

    /// Parses named colors according to CSS Color Module Level 4
    pub fn parse_named(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            // Basic colors
            "black" => Some(Color {
                r: 0,
                g: 0,
                b: 0,
                a: 1.0,
            }),
            "silver" => Some(Color {
                r: 192,
                g: 192,
                b: 192,
                a: 1.0,
            }),
            "gray" => Some(Color {
                r: 128,
                g: 128,
                b: 128,
                a: 1.0,
            }),
            "white" => Some(Color {
                r: 255,
                g: 255,
                b: 255,
                a: 1.0,
            }),
            "maroon" => Some(Color {
                r: 128,
                g: 0,
                b: 0,
                a: 1.0,
            }),
            "red" => Some(Color {
                r: 255,
                g: 0,
                b: 0,
                a: 1.0,
            }),
            "purple" => Some(Color {
                r: 128,
                g: 0,
                b: 128,
                a: 1.0,
            }),
            "fuchsia" => Some(Color {
                r: 255,
                g: 0,
                b: 255,
                a: 1.0,
            }),
            "green" => Some(Color {
                r: 0,
                g: 128,
                b: 0,
                a: 1.0,
            }),
            "lime" => Some(Color {
                r: 0,
                g: 255,
                b: 0,
                a: 1.0,
            }),
            "olive" => Some(Color {
                r: 128,
                g: 128,
                b: 0,
                a: 1.0,
            }),
            "yellow" => Some(Color {
                r: 255,
                g: 255,
                b: 0,
                a: 1.0,
            }),
            "navy" => Some(Color {
                r: 0,
                g: 0,
                b: 128,
                a: 1.0,
            }),
            "blue" => Some(Color {
                r: 0,
                g: 0,
                b: 255,
                a: 1.0,
            }),
            "teal" => Some(Color {
                r: 0,
                g: 128,
                b: 128,
                a: 1.0,
            }),
            "aqua" => Some(Color {
                r: 0,
                g: 255,
                b: 255,
                a: 1.0,
            }),

            // Extended colors
            "aliceblue" => Some(Color {
                r: 240,
                g: 248,
                b: 255,
                a: 1.0,
            }),
            "antiquewhite" => Some(Color {
                r: 250,
                g: 235,
                b: 215,
                a: 1.0,
            }),
            // TODO (include all 140+ named colors from CSS Color Module)

            // Special cases
            "transparent" => Some(Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0.0,
            }),
            "currentcolor" => None, // Special keyword handled during style calculation

            _ => None,
        }
    }

    /// Parses rgb() and rgba() functions
    pub fn parse_rgb(rgb: &str) -> Option<Self> {
        let rgb = rgb.trim();
        if !rgb.starts_with("rgb(") && !rgb.starts_with("rgba(") {
            return None;
        }

        let is_rgba = rgb.starts_with("rgba(");
        let content = rgb
            .trim_start_matches("rgb(")
            .trim_start_matches("rgba(")
            .trim_end_matches(')');

        let components: Vec<&str> = content.split(',').map(|s| s.trim()).collect();

        if (is_rgba && components.len() != 4) || (!is_rgba && components.len() != 3) {
            return None;
        }

        let parse_component = |s: &str| -> Option<f32> {
            if s.ends_with('%') {
                s.trim_end_matches('%')
                    .parse::<f32>()
                    .ok()
                    .map(|p| p * 255.0 / 100.0)
            } else {
                s.parse::<f32>().ok()
            }
        };

        let r = parse_component(components[0])?.clamp(0.0, 255.0) as u8;
        let g = parse_component(components[1])?.clamp(0.0, 255.0) as u8;
        let b = parse_component(components[2])?.clamp(0.0, 255.0) as u8;
        let a = if is_rgba {
            components[3].parse::<f32>().ok()?.clamp(0.0, 1.0)
        } else {
            1.0
        };

        Some(Color { r, g, b, a })
    }

    /// Parses hsl() and hsla() functions
    pub fn parse_hsl(hsl: &str) -> Option<Self> {
        let hsl = hsl.trim();
        if !hsl.starts_with("hsl(") && !hsl.starts_with("hsla(") {
            return None;
        }

        let is_hsla = hsl.starts_with("hsla(");
        let content = hsl
            .trim_start_matches("hsl(")
            .trim_start_matches("hsla(")
            .trim_end_matches(')');

        let components: Vec<&str> = content.split(',').map(|s| s.trim()).collect();

        if (is_hsla && components.len() != 4) || (!is_hsla && components.len() != 3) {
            return None;
        }

        let h = components[0]
            .trim_end_matches("deg")
            .trim_end_matches("rad")
            .trim_end_matches("grad")
            .trim_end_matches("turn")
            .parse::<f32>()
            .ok()?;

        let s = components[1].trim_end_matches('%').parse::<f32>().ok()? / 100.0;
        let l = components[2].trim_end_matches('%').parse::<f32>().ok()? / 100.0;
        let a = if is_hsla {
            components[3].parse::<f32>().ok()?.clamp(0.0, 1.0)
        } else {
            1.0
        };

        // Convert HSL to RGB
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;

        let (r, g, b) = match (h / 60.0).floor() as i32 % 6 {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        Some(Color {
            r: ((r + m) * 255.0).round() as u8,
            g: ((g + m) * 255.0).round() as u8,
            b: ((b + m) * 255.0).round() as u8,
            a,
        })
    }
}
