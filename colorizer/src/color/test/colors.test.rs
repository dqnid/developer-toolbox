#[cfg(test)]
pub mod tests {
    use crate::color::{Color, HSL, RGB};

    #[test]
    fn test_conversion() {
        let hsl_color = Color::from(HSL::new(193, 67, 28));
        let rgb_color = Color::from(RGB::from(HSL::new(193, 67, 28)));
        assert_eq!(hsl_color, rgb_color);
    }

    #[test]
    fn test_rgb_to_hsl() {
        let red_rgb = RGB::new(255, 0, 0);
        let red_hsl = HSL::new(0, 100, 50);

        assert_eq!(HSL::from(red_rgb), red_hsl);

        let green_rgb = RGB::new(0, 255, 0);
        let green_hsl = HSL::new(120, 100, 50);

        assert_eq!(HSL::from(green_rgb), green_hsl);

        let blue_rgb = RGB::new(0, 0, 255);
        let blue_hsl = HSL::new(240, 100, 50);

        assert_eq!(HSL::from(blue_rgb), blue_hsl);
    }

    #[test]
    fn test_hsl_to_rgb() {
        let red_hsl = Color::from(HSL::new(0, 100, 50));
        let red_rgb = Color::from(RGB::new(255, 0, 0));
        assert_eq!(red_hsl, red_rgb);

        let green_hsl = Color::from(HSL::new(120, 100, 50));
        let green_rgb = Color::from(RGB::new(0, 255, 0));
        assert_eq!(green_hsl, green_rgb);

        let blue_hsl = Color::from(HSL::new(240, 100, 50));
        let blue_rgb = Color::from(RGB::new(0, 0, 255));
        assert_eq!(blue_hsl, blue_rgb);
    }
}
