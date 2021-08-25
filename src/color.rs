#[derive(Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub(crate) fn new(r: u8, g: u8, b: u8) -> Self{
        Self {
            r,
            g,
            b,
        }
    }

    pub(crate) fn get_rgb(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    pub(crate) fn set_rgb(&mut self, rgb: (u8, u8, u8)) {
        self.r = rgb.0;
        self.g = rgb.1;
        self.b = rgb.2;
    }

    pub(crate) fn rgb_to_hex(rgb: (u8, u8, u8)) -> String {
        let to_hex = |s: u8| format!("{s:0>width$}", s=format!("{:x}", s), width=2);
        format!("{}{}{}", to_hex(rgb.0), to_hex(rgb.1), to_hex(rgb.2))
    }

    pub(crate) fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
        (
            u8::from_str_radix(&hex[0..2], 16).unwrap(),
            u8::from_str_radix(&hex[2..4], 16).unwrap(),
            u8::from_str_radix(&hex[4..6], 16).unwrap(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;

    #[test]
    fn rgb_to_hex() {
        assert_eq!("08120c", Color::rgb_to_hex((8, 18, 12)));
    }
}
