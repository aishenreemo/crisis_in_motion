use bevy::prelude::*;

#[allow(dead_code)]
#[derive(Resource)]
pub struct ColorPalette {
    pub fg: Color,
    pub bg: Color,
    pub black: Color,
    pub red: Color,
    pub green: Color,
    pub orange: Color,
    pub blue: Color,
    pub magenta: Color,
    pub cyan: Color,
    pub white: Color,
}

impl ColorPalette {
    pub const KIZU: ColorPalette = ColorPalette {
        fg: hex_to_srgb("#C5C8C9"),
        bg: hex_to_srgb("#0B0F10"),
        black: hex_to_srgb("#131718"),
        red: hex_to_srgb("#DF5B61"),
        green: hex_to_srgb("#87C7A1"),
        orange: hex_to_srgb("#DE8F78"),
        blue: hex_to_srgb("#6791C9"),
        magenta: hex_to_srgb("#BC83E3"),
        cyan: hex_to_srgb("#70B9CC"),
        white: hex_to_srgb("#C4C4C4"),
    };
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self::KIZU
    }
}

const fn hex_to_srgb(hex: &'static str) -> Color {
    let hex = hex.as_bytes();

    if hex.len() != 7 || hex[0] != b'#' {
        panic!("Invalid hex string.")
    }

    let r = hex_byte_to_u8(hex[1]) * 16 + hex_byte_to_u8(hex[2]);
    let g = hex_byte_to_u8(hex[3]) * 16 + hex_byte_to_u8(hex[4]);
    let b = hex_byte_to_u8(hex[5]) * 16 + hex_byte_to_u8(hex[6]);

    Color::Srgba(Srgba::new(
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        1.,
    ))
}

const fn hex_byte_to_u8(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        _ => panic!("Invalid hex char."),
    }
}
