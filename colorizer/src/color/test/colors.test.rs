#[cfg(test)]
pub mod tests {
    use crate::color::{Color, HSL, HSV, RGB};

    #[test]
    fn test_color_initialization() {
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

        // Variant colors

        let pink_rgb = RGB::new(255, 175, 204);
        let pink_hsl = HSL::new(338, 100, 84);

        assert_eq!(HSL::from(pink_rgb), pink_hsl);

        let orange_rgb = RGB::new(251, 133, 0);
        let orange_hsl = HSL::new(32, 100, 49);

        assert_eq!(HSL::from(orange_rgb), orange_hsl);

        let yellow_rgb = RGB::new(255, 214, 10);
        let yellow_hsl = HSL::new(50, 100, 52);

        assert_eq!(HSL::from(yellow_rgb), yellow_hsl);

        let purple_rgb = RGB::new(123, 44, 191);
        let purple_hsl = HSL::new(272, 63, 46);

        assert_eq!(HSL::from(purple_rgb), purple_hsl);
    }

    #[test]
    fn test_hsl_to_rgb() {
        let red_rgb = RGB::new(255, 0, 0);
        let red_hsl = HSL::new(0, 100, 50);

        assert_eq!(red_rgb, RGB::from(red_hsl));

        let green_rgb = RGB::new(0, 255, 0);
        let green_hsl = HSL::new(120, 100, 50);

        assert_eq!(green_rgb, RGB::from(green_hsl));

        let blue_rgb = RGB::new(0, 0, 255);
        let blue_hsl = HSL::new(240, 100, 50);

        assert_eq!(blue_rgb, RGB::from(blue_hsl));

        // Variant colors

        let pink_rgb = RGB::new(255, 173, 203);
        let pink_hsl = HSL::new(338, 100, 84);

        assert_eq!(pink_rgb, RGB::from(pink_hsl));

        let orange_rgb = RGB::new(250, 133, 0);
        let orange_hsl = HSL::new(32, 100, 49);

        assert_eq!(orange_rgb, RGB::from(orange_hsl));

        let yellow_rgb = RGB::new(255, 214, 10);
        let yellow_hsl = HSL::new(50, 100, 52);

        assert_eq!(yellow_rgb, RGB::from(yellow_hsl));

        let purple_rgb = RGB::new(122, 43, 191);
        let purple_hsl = HSL::new(272, 63, 46);

        assert_eq!(purple_rgb, RGB::from(purple_hsl));
    }

    #[test]
    fn test_basic_hex_convertion() {
        let red_color = Color::from(RGB::new(255, 0, 0));
        let green_color = Color::from(RGB::new(0, 255, 0));
        let blue_color = Color::from(RGB::new(0, 0, 255));

        assert_eq!(format!("{:X}", red_color), "#FF0000");
        assert_eq!(format!("{:X}", green_color), "#00FF00");
        assert_eq!(format!("{:X}", blue_color), "#0000FF");
    }

    #[test]
    fn test_complex_hex_convertion() {
        let color = Color::from(RGB::new(255, 183, 3));
        assert_eq!(format!("{:X}", color), "#FFB703");
        let color = Color::from(RGB::new(88, 129, 87));
        assert_eq!(format!("{:X}", color), "#588157");
        let color = Color::from(RGB::new(251, 133, 0));
        assert_eq!(format!("{:X}", color), "#FB8500");
        let color = Color::from(RGB::new(131, 56, 236));
        assert_eq!(format!("{:X}", color), "#8338EC");
        let color = Color::from(RGB::new(157, 129, 137));
        assert_eq!(format!("{:X}", color), "#9D8189");
    }

    #[test]
    fn test_hsv_from_rgb() {
        // Base colors
        let color = RGB::new(255, 0, 0);
        assert_eq!(HSV::from(color), HSV::new(0, 100, 100));
        let color = RGB::new(0, 255, 0);
        assert_eq!(HSV::from(color), HSV::new(120, 100, 100));
        let color = RGB::new(0, 0, 255);
        assert_eq!(HSV::from(color), HSV::new(240, 100, 100));

        // Complex colors
        let color = RGB::new(20, 240, 100);
        assert_eq!(HSV::from(color), HSV::new(142, 92, 94));
        let color = RGB::new(220, 10, 50);
        assert_eq!(HSV::from(color), HSV::new(349, 95, 86));
    }

    #[test]
    fn test_rgb_from_hsv() {
        // Base colors
        let color = HSV::new(0, 100, 100);
        assert_eq!(RGB::from(color), RGB::new(255, 0, 0));
        let color = HSV::new(120, 100, 100);
        assert_eq!(RGB::from(color), RGB::new(0, 255, 0));
        let color = HSV::new(240, 100, 100);
        assert_eq!(RGB::from(color), RGB::new(0, 0, 255));

        // Complex colors
        let color = HSV::new(349, 95, 86);
        assert_eq!(RGB::from(color), RGB::new(219, 11, 49));
        let color = HSV::new(142, 92, 94);
        assert_eq!(RGB::from(color), RGB::new(19, 240, 100));
    }

    #[test]
    fn test_rgb_string_parse() {
        // Base colors
        // HEX
        let color = "#FF0000";
        assert_eq!(RGB::from(color.to_string()), RGB::new(255, 0, 0));
        let color = "#00FF00";
        assert_eq!(RGB::from(color.to_string()), RGB::new(0, 255, 0));
        let color = "#0000FF";
        assert_eq!(RGB::from(color.to_string()), RGB::new(0, 0, 255));
        // RGB
        let color = "rgb(255 ,0,0)";
        assert_eq!(RGB::from(color.to_string()), RGB::new(255, 0, 0));
        let color = "rgb(0,255, 0)";
        assert_eq!(RGB::from(color.to_string()), RGB::new(0, 255, 0));
        let color = "rgb(0 , 0, 255)";
        assert_eq!(RGB::from(color.to_string()), RGB::new(0, 0, 255));

        // Complex colors
        // HEX
        let color = "#Ffb703";
        assert_eq!(RGB::from(color.to_string()), RGB::new(255, 183, 3));
        let color = "#588157";
        assert_eq!(RGB::from(color.to_string()), RGB::new(88, 129, 87));
        let color = "#fB8500";
        assert_eq!(RGB::from(color.to_string()), RGB::new(251, 133, 0));
        let color = "#8338eC";
        assert_eq!(RGB::from(color.to_string()), RGB::new(131, 56, 236));
        let color = "#9D8189";
        assert_eq!(RGB::from(color.to_string()), RGB::new(157, 129, 137));
        // RGB
        let color = "rgb(255, 183, 3)";
        assert_eq!(RGB::from(color.to_string()), RGB::new(255, 183, 3));
        let color = "rgb(88, 129, 87)";
        assert_eq!(RGB::from(color.to_string()), RGB::new(88, 129, 87));
        let color = "rgb(251, 133, 0)";
        assert_eq!(RGB::from(color.to_string()), RGB::new(251, 133, 0));
        let color = "rgb(131, 56, 236)";
        assert_eq!(RGB::from(color.to_string()), RGB::new(131, 56, 236));
        let color = "rgb(157, 129, 137)";
        assert_eq!(RGB::from(color.to_string()), RGB::new(157, 129, 137));
    }

    #[test]
    fn test_hsl_string_parse() {
        // Base colors
        let color = "hsl(0, 100, 50)";
        assert_eq!(HSL::from(color.to_string()), HSL::new(0, 100, 50));
        let color = "hsl(120, 100, 50)";
        assert_eq!(HSL::from(color.to_string()), HSL::new(120, 100, 50));
        let color = "hsl(240, 100, 50)";
        assert_eq!(HSL::from(color.to_string()), HSL::new(240, 100, 50));

        // Complex colors
        let color = "hsl(255 , 83,  3)";
        assert_eq!(HSL::from(color.to_string()), HSL::new(255, 83, 3));
        let color = "hsl(88 , 29,87)";
        assert_eq!(HSL::from(color.to_string()), HSL::new(88, 29, 87));
        let color = "hsl(251,33,0)";
        assert_eq!(HSL::from(color.to_string()), HSL::new(251, 33, 0));
        let color = "hsl(131,56,36)";
        assert_eq!(HSL::from(color.to_string()), HSL::new(131, 56, 36));
        let color = "hsl(157,29, 37 )";
        assert_eq!(HSL::from(color.to_string()), HSL::new(157, 29, 37));
    }

    #[test]
    fn test_hsv_string_parse() {
        // Base colors
        let color = "hsv(0, 100, 50)";
        assert_eq!(HSV::from(color.to_string()), HSV::new(0, 100, 50));
        let color = "hsv(120, 100, 50)";
        assert_eq!(HSV::from(color.to_string()), HSV::new(120, 100, 50));
        let color = "hsv(240, 100, 50)";
        assert_eq!(HSV::from(color.to_string()), HSV::new(240, 100, 50));

        // Complex colors
        let color = "hsv(255 , 83,  3)";
        assert_eq!(HSV::from(color.to_string()), HSV::new(255, 83, 3));
        let color = "hsv(88 , 29,87)";
        assert_eq!(HSV::from(color.to_string()), HSV::new(88, 29, 87));
        let color = "hsv(251,33,0)";
        assert_eq!(HSV::from(color.to_string()), HSV::new(251, 33, 0));
        let color = "hsv(131,56,36)";
        assert_eq!(HSV::from(color.to_string()), HSV::new(131, 56, 36));
        let color = "hsv(157,29, 37 )";
        assert_eq!(HSV::from(color.to_string()), HSV::new(157, 29, 37));
    }

    #[test]
    fn test_color_string_parse() {
        let input = "test1hsv(255 , 83,  3)test2";
        let result = Color::try_parse(input.to_string());
        assert_eq!(result, Ok(Color::from(HSV::from(input.to_string()))));

        let input = "test1hsj(255 , 83,  3)test2";
        let result = Color::try_parse(input.to_string());
        assert_eq!(result, Err(()));

        let input = "test1hsl(25a , 83,  3)test2";
        let result = Color::try_parse(input.to_string());
        assert_eq!(result, Err(()));

        let input = "#afj";
        let result = Color::try_parse(input.to_string());
        assert_eq!(result, Err(()));

        let input = "p#test2#010203j";
        let result = Color::try_parse(input.to_string());
        assert_eq!(result, Ok(Color::from(RGB::new(1, 2, 3))));
    }
}
