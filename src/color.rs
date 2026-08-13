use crossterm::style::Color;

pub fn get_title_color(title: &str) -> (u8, u8, u8) {
    let mut color: (u8, u8, u8) = (0, 0, 0);
    for (i, byte) in title.as_bytes().iter().enumerate() {
        if i < title.len() / 3 {
            color.0 = color.0.wrapping_add(*byte);
        } else if i < title.len() / 3 * 2 {
            color.1 = color.1.wrapping_add(*byte).wrapping_add(*byte);
        } else {
            color.2 = color
                .2
                .wrapping_add(*byte)
                .wrapping_add(*byte)
                .wrapping_add(*byte);
        }
    }

    while (color.0 as u32 + color.1 as u32 + color.2 as u32) < 400
        || (color.0 as u32 + color.1 as u32) < 150
    {
        color.0 = color.0.wrapping_add(71);
        color.1 = color.1.wrapping_add(111);
        color.2 = color.2.wrapping_add(241);
    }

    color
}

pub fn to_crossterm_color(color: (u8, u8, u8)) -> Color {
    Color::Rgb {
        r: color.0,
        g: color.1,
        b: color.2,
    }
}

pub fn get_lighter(color: (u8, u8, u8)) -> (u8, u8, u8) {
    let factor = 30;
    (
        color.0.min(255 - factor) + factor,
        color.1.min(255 - factor) + factor,
        color.2.min(255 - factor) + factor,
    )
}
