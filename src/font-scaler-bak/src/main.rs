use rusttype::{Font, Scale};

fn calculate_font_size(input_text: &str, container_width: f32, font: &Font) -> f32 {
    let mut min_size = 24.0;
    let mut max_size = 200.0;
    let mut width: f32 = 0.0;
    
    while width < container_width {
        let current_size = (min_size + max_size) / 2.0;
        let scale = Scale::uniform(current_size);
        width = font
            .layout(input_text, scale, rusttype::point(0.0, 0.0))
            .map(|g| g.pixel_bounding_box().map_or(0, |bb| bb.width()) as f32)
            .sum();
        
        println!("Current width: {}", width);

        if width > container_width {
            break;
        }
    }

    min_size
}

fn main() {
    let input_text = "ett rentekutt i år";
    let container_width = 660.0;
    let target_font_size = 81.0;
    let font_data = include_bytes!(".sample-fonts/E24Display-Bold-1.21.otf") as &[u8];
    let font = Font::try_from_bytes(&font_data).unwrap();

    let font_size = calculate_font_size(input_text, container_width, &font);

    assert_eq!(font_size, target_font_size);
}