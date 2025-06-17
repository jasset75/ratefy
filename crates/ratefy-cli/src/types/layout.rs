/// Define behaviour of input on horizontal layout
#[allow(dead_code)]
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
    Full,
}

/// Define behaviour of input on vertical layout
#[allow(dead_code)]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
    Full,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_align_variants() {
        let variants = [
            HorizontalAlign::Left,
            HorizontalAlign::Center,
            HorizontalAlign::Right,
            HorizontalAlign::Full,
        ];
        for variant in variants {
            match variant {
                HorizontalAlign::Left => assert!(true),
                HorizontalAlign::Center => assert!(true),
                HorizontalAlign::Right => assert!(true),
                HorizontalAlign::Full => assert!(true),
            }
        }
    }

    #[test]
    fn test_vertical_align_variants() {
        let variants = [
            VerticalAlign::Top,
            VerticalAlign::Middle,
            VerticalAlign::Bottom,
            VerticalAlign::Full,
        ];
        for variant in variants {
            match variant {
                VerticalAlign::Top => assert!(true),
                VerticalAlign::Middle => assert!(true),
                VerticalAlign::Bottom => assert!(true),
                VerticalAlign::Full => assert!(true),
            }
        }
    }
}