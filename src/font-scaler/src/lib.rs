use wasm_bindgen::prelude::*;
use rusttype::{Font, Scale};

enum FontWeight {
    Thin,
    Normal,
    Medium,
    SemiBold,
    Bold,
    Breaking,
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn scaledFontSize(input_text: str, container_width: u16, font_weight: u16) -> f32 {
    let scale = Scale::uniform(24.0);
    let width = get_font(font_weight)
        .layout(input_text, scale, rusttype::point(0.0, 0.0))
        .map(|g| g.pixel_bounding_box().map_or(0, |bb| bb.width()) as f32)
        .sum();

    width
}

fn get_font(font_weight: u16) -> Font {
    match font_weight {
        400 => return load_font("E24_Display-Regular-web.woff2") as &[u8],
        500 => return load_font("E24_Display-Medium-web.woff2") as &[u8],
        600 => return load_font("E24_Display-Semi_Bold-web.woff2") as &[u8],
        700 => return load_font("E24_Display-Bold-web.woff2"),
        900 => return load_font("E24_Display-Breaking-web.woff2") as &[u8],
        _ => panic!("Unsupported font weight provided"),
    }
}

const FONT_BASE_PATH: str = "font-scaler/.sample-fonts/Web/woff2/Display";

fn load_font(filename: str) -> Font {
    let bytes = include_bytes!(&FONT_BASE_PATH + "/" + filename) as &[u8];

    Font::try_from_bytes(&bytes).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
