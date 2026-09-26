pub mod color;
pub mod data_type;
pub mod vector;

// Tests
#[cfg(test)]
pub mod tests {
    use super::color::*;
    use super::vector::*;

    #[test]
    fn create_color() {
        let rgb_val = ColorValue::RGB(0.5, 0.25, 0.0);
        let rgba_val = ColorValue::RGBA(0.5, 0.25, 0.0, 0.75);
        let hex_chars: [char; 6] = ['1', '9', '3', '2', '9', '6'];
        let hex_val = ColorValue::HEX(hex_chars);
        let hexa_chars: [char; 8] = ['1', '9', '3', '2', '9', '6', 'C', '8'];
        let hexa_val = ColorValue::HEXA(hexa_chars);

        let rgb_col = Color::new(rgb_val);
        let rgba_col = Color::new(rgba_val);
        let hex_col = Color::new(hex_val);
        let hexa_col = Color::new(hexa_val);

        assert_eq!((0.5, 0.25, 0.0), rgb_col.rgb());
        assert_eq!((0.5, 0.25, 0.0, 0.75), rgba_col.rgba());
        assert_eq!("193296", hex_col.hex());
        assert_eq!("193296c8", hexa_col.hexa());
    }

    #[test]
    fn neg_color() {
        let rgba_val = ColorValue::RGBA(0.5, 0.25, 0.0, 1.0);
        let rgba_col = Color::new(rgba_val);
        assert_eq!((0.5, 0.75, 1.0, 1.0), (-rgba_col).rgba());
    }

    #[test]
    fn color_ops() {
        let mut add_col = Color::new(ColorValue::RGBA(0.25, 0.0, 0.5, 1.0));
        add_col += add_col;
        assert_eq!((0.5, 0.0, 1.0, 1.0), add_col.rgba());

        let mut sub_col = Color::new(ColorValue::RGBA(0.25, 0.0, 0.5, 1.0));
        sub_col -= Color::new(ColorValue::RGBA(0.1, 0.0, 0.25, 0.5));
        assert_eq!((0.15, 0.0, 0.25, 0.5), sub_col.rgba());

        let mut mul_col = Color::new(ColorValue::RGBA(0.25, 0.0, 0.5, 1.0));
        mul_col *= mul_col;
        assert_eq!((0.0625, 0.0, 0.25, 1.0), mul_col.rgba());

        let mut div_col = Color::new(ColorValue::RGBA(0.25, 0.0, 0.5, 1.0));
        div_col /= 2.0;
        assert_eq!((0.125, 0.0, 0.25, 0.5), div_col.rgba());
    }

    #[test]
    fn vector_ops() {
        let mut add_vec = Vec3::new(0.1, 0.25, 0.5);
        add_vec += add_vec;
        assert_eq!(Vec3::new(0.2, 0.5, 1.0), add_vec);

        let mut sub_vec = Vec3::new(0.1, 0.25, 0.5);
        sub_vec -= Vec3::new(0.05, 0.1, 0.25);
        assert_eq!(Vec3::new(0.05, 0.15, 0.25), sub_vec);

        let mut mul_vec = Vec3::new(0.1, 0.25, 0.5);
        mul_vec *= Vec3::new(0.1, 0.1, 0.25);
        assert_eq!(Vec3::new(0.01, 0.025, 0.125), mul_vec);

        let mut div_vec = Vec3::new(0.1, 0.25, 0.5);
        div_vec /= 2.0;
        assert_eq!(Vec3::new(0.05, 0.125, 0.25), div_vec);
    }
}
