use crate::gfx::fonts;

#[allow(dead_code)]
pub static FONT: fonts::Font = fonts::Font {
    glyph_width: 7,
    glyph_height: 8,
    left_bearing: 1,
    top_bearing: 1,
    advance_width: 8,
    line_height: 9,
    glyph_data: &GLYPHS,
    name: "cbmii",
};

#[cfg_attr(rustfmt, rustfmt_skip)]
static GLYPHS: [u8; 95 * 8] = [
    // space
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,

    // !
    0b_0001000_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0000000_0,
    0b_0001000_0,
    0b_0000000_0,

    // "
    0b_0010100_0,
    0b_0010100_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,

    // #
    0b_0100100_0,
    0b_0100100_0,
    0b_1111110_0,
    0b_0100100_0,
    0b_1111110_0,
    0b_0100100_0,
    0b_0100100_0,
    0b_0000000_0,

    // $
    0b_0001000_0,
    0b_0011110_0,
    0b_0101000_0,
    0b_0011100_0,
    0b_0001010_0,
    0b_0111100_0,
    0b_0001000_0,
    0b_0000000_0,

    // %
    0b_0000000_0,
    0b_1100010_0,
    0b_1100100_0,
    0b_0001000_0,
    0b_0010000_0,
    0b_0100110_0,
    0b_1000110_0,
    0b_0000000_0,

    // &
    0b_0111000_0,
    0b_1000100_0,
    0b_1000100_0,
    0b_0111000_0,
    0b_1000101_0,
    0b_1000010_0,
    0b_0111101_0,
    0b_0000000_0,

    // '
    0b_0000100_0,
    0b_0001000_0,
    0b_0010000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,

    // (
    0b_0000100_0,
    0b_0001000_0,
    0b_0010000_0,
    0b_0010000_0,
    0b_0010000_0,
    0b_0001000_0,
    0b_0000100_0,
    0b_0000000_0,

    // )
    0b_0001000_0,
    0b_0000100_0,
    0b_0000010_0,
    0b_0000010_0,
    0b_0000010_0,
    0b_0000100_0,
    0b_0001000_0,
    0b_0000000_0,

    // *
    0b_0001000_0,
    0b_0101010_0,
    0b_0011100_0,
    0b_0111110_0,
    0b_0011100_0,
    0b_0101010_0,
    0b_0001000_0,
    0b_0000000_0,

    // +
    0b_0000000_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0111110_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0000000_0,
    0b_0000000_0,

    // ,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0010000_0,

    // -
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_1111110_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,

    // .
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0011000_0,
    0b_0011000_0,
    0b_0000000_0,

    // /
    0b_0000000_0,
    0b_0000010_0,
    0b_0000100_0,
    0b_0001000_0,
    0b_0010000_0,
    0b_0100000_0,
    0b_1000000_0,
    0b_0000000_0,

    // 0
    0b_0111100_0,
    0b_1000010_0,
    0b_1000110_0,
    0b_1011010_0,
    0b_1100010_0,
    0b_1000010_0,
    0b_0111100_0,
    0b_0000000_0,

    // 1
    0b_0001000_0,
    0b_0011000_0,
    0b_0101000_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0111110_0,
    0b_0000000_0,

    // 2
    0b_0111100_0,
    0b_1000010_0,
    0b_0000010_0,
    0b_0001100_0,
    0b_0110000_0,
    0b_1000000_0,
    0b_1111110_0,
    0b_0000000_0,

    // 3
    0b_0111100_0,
    0b_1000010_0,
    0b_0000010_0,
    0b_0011100_0,
    0b_0000010_0,
    0b_1000010_0,
    0b_0111100_0,
    0b_0000000_0,

    // 4
    0b_0000100_0,
    0b_0001100_0,
    0b_0010100_0,
    0b_0100100_0,
    0b_1111110_0,
    0b_0000100_0,
    0b_0000100_0,
    0b_0000000_0,

    // 5
    0b_1111110_0,
    0b_1000000_0,
    0b_1111000_0,
    0b_0000100_0,
    0b_0000010_0,
    0b_1000100_0,
    0b_0111000_0,
    0b_0000000_0,

    // 6
    0b_0011110_0,
    0b_0100000_0,
    0b_1000000_0,
    0b_1111100_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_0111100_0,
    0b_0000000_0,

    // 7
    0b_1111110_0,
    0b_1000010_0,
    0b_0000100_0,
    0b_0001000_0,
    0b_0010000_0,
    0b_0010000_0,
    0b_0010000_0,
    0b_0000000_0,

    // 8
    0b_0111100_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_0111100_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_0111100_0,
    0b_0000000_0,

    // 9
    0b_0111100_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_0111110_0,
    0b_0000010_0,
    0b_0000100_0,
    0b_0111000_0,
    0b_0000000_0,

    // :
    0b_0000000_0,
    0b_0000000_0,
    0b_0001000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0001000_0,
    0b_0000000_0,
    0b_0000000_0,

    // ;
    0b_0000000_0,
    0b_0000000_0,
    0b_0001000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0010000_0,

    // <
    0b_0000110_0,
    0b_0001100_0,
    0b_0011000_0,
    0b_0110000_0,
    0b_0011000_0,
    0b_0001100_0,
    0b_0000110_0,
    0b_0000000_0,

    // =
    0b_0000000_0,
    0b_0000000_0,
    0b_1111110_0,
    0b_0000000_0,
    0b_1111110_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,

    // >
    0b_0110000_0,
    0b_0011000_0,
    0b_0001100_0,
    0b_0000110_0,
    0b_0001100_0,
    0b_0011000_0,
    0b_0110000_0,
    0b_0000000_0,

    // ?
    0b_0111100_0,
    0b_1000010_0,
    0b_0000010_0,
    0b_0001100_0,
    0b_0010000_0,
    0b_0000000_0,
    0b_0010000_0,
    0b_0000000_0,

    // @
    0b_0011100_0,
    0b_0100010_0,
    0b_1001010_0,
    0b_1010110_0,
    0b_1001000_0,
    0b_0100000_0,
    0b_0011110_0,
    0b_0000000_0,

    // A
    0b_0011000_0,
    0b_0100100_0,
    0b_1000010_0,
    0b_1111110_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_0000000_0,


    // B
    0b_1111100_0,
    0b_0100010_0,
    0b_0100010_0,
    0b_0111100_0,
    0b_0100010_0,
    0b_0100010_0,
    0b_1111100_0,
    0b_0000000_0,

    // C
    0b_0011100_0,
    0b_0100010_0,
    0b_1000000_0,
    0b_1000000_0,
    0b_1000000_0,
    0b_0100010_0,
    0b_0011100_0,
    0b_0000000_0,

    // D
    0b_1111000_0,
    0b_0100100_0,
    0b_0100010_0,
    0b_0100010_0,
    0b_0100010_0,
    0b_0100100_0,
    0b_1111000_0,
    0b_0000000_0,

    // E
    0b_1111110_0,
    0b_1000000_0,
    0b_1000000_0,
    0b_1111100_0,
    0b_1000000_0,
    0b_1000000_0,
    0b_1111110_0,
    0b_0000000_0,

    // F
    0b_1111110_0,
    0b_1000000_0,
    0b_1000000_0,
    0b_1111100_0,
    0b_1000000_0,
    0b_1000000_0,
    0b_1000000_0,
    0b_0000000_0,

    // G
    0b_0011100_0,
    0b_0100010_0,
    0b_1000000_0,
    0b_1001110_0,
    0b_1000010_0,
    0b_0100010_0,
    0b_0011100_0,
    0b_0000000_0,

    // H
    0b_1000010_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_1111110_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_0000000_0,

    // I
    0b_0111110_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0111110_0,
    0b_0000000_0,

    // J
    0b_0001110_0,
    0b_0000100_0,
    0b_0000100_0,
    0b_0000100_0,
    0b_0000100_0,
    0b_1000100_0,
    0b_0111000_0,
    0b_0000000_0,

    // K
    0b_1000010_0,
    0b_1000100_0,
    0b_1001000_0,
    0b_1110000_0,
    0b_1001000_0,
    0b_1000100_0,
    0b_1000010_0,
    0b_0000000_0,

    // L
    0b_1000000_0,
    0b_1000000_0,
    0b_1000000_0,
    0b_1000000_0,
    0b_1000000_0,
    0b_1000000_0,
    0b_1111110_0,
    0b_0000000_0,

    // M
    0b_1000010_0,
    0b_1100110_0,
    0b_1011010_0,
    0b_1011010_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_0000000_0,

    // N
    0b_1000010_0,
    0b_1100010_0,
    0b_1010010_0,
    0b_1001010_0,
    0b_1000110_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_0000000_0,

    // O
    0b_0011000_0,
    0b_0100100_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_0100100_0,
    0b_0011000_0,
    0b_0000000_0,

    // P
    0b_1111100_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_1111100_0,
    0b_1000000_0,
    0b_1000000_0,
    0b_1000000_0,
    0b_0000000_0,

    // Q
    0b_0011000_0,
    0b_0100100_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_1001010_0,
    0b_0100100_0,
    0b_0011010_0,
    0b_0000000_0,

    // R
    0b_1111100_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_1111100_0,
    0b_1001000_0,
    0b_1000100_0,
    0b_1000010_0,
    0b_0000000_0,

    // S
    0b_0111100_0,
    0b_1000010_0,
    0b_1000000_0,
    0b_0111100_0,
    0b_0000010_0,
    0b_1000010_0,
    0b_0111100_0,
    0b_0000000_0,

    // T
    0b_0111110_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0000000_0,

    // U
    0b_1000010_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_0111100_0,
    0b_0000000_0,

    // V
    0b_1000010_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_0100100_0,
    0b_0100100_0,
    0b_0011000_0,
    0b_0000000_0,

    // W
    0b_1000010_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_1011010_0,
    0b_1011010_0,
    0b_1100110_0,
    0b_1000010_0,
    0b_0000000_0,

    // X
    0b_1000010_0,
    0b_1000010_0,
    0b_0100100_0,
    0b_0011000_0,
    0b_0100100_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_0000000_0,

    // Y
    0b_0100010_0,
    0b_0100010_0,
    0b_0100010_0,
    0b_0011100_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0000000_0,

    // Z
    0b_1111110_0,
    0b_0000010_0,
    0b_0000100_0,
    0b_0011000_0,
    0b_0100000_0,
    0b_1000000_0,
    0b_1111110_0,
    0b_0000000_0,

    // [
    0b_1111110_0,
    0b_1000000_0,
    0b_1000000_0,
    0b_1000000_0,
    0b_1000000_0,
    0b_1000000_0,
    0b_1111110_0,
    0b_0000000_0,

    // \
    0b_0000000_0,
    0b_1000000_0,
    0b_0100000_0,
    0b_0010000_0,
    0b_0001000_0,
    0b_0000100_0,
    0b_0000010_0,
    0b_0000000_0,

    // ]
    0b_1111110_0,
    0b_0000010_0,
    0b_0000010_0,
    0b_0000010_0,
    0b_0000010_0,
    0b_0000010_0,
    0b_1111110_0,
    0b_0000000_0,

    // ^
    0b_0001000_0,
    0b_0010100_0,
    0b_0100010_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,

    // _
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_1111111_0,
    0b_0000000_0,

    // `
    0b_0010000_0,
    0b_0001000_0,
    0b_0000100_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,

    // a
    0b_0000000_0,
    0b_0000000_0,
    0b_0111000_0,
    0b_0000100_0,
    0b_0111100_0,
    0b_1000100_0,
    0b_0111010_0,
    0b_0000000_0,

    // b
    0b_1000000_0,
    0b_1000000_0,
    0b_1011100_0,
    0b_1100010_0,
    0b_1000010_0,
    0b_1100010_0,
    0b_1011100_0,
    0b_0000000_0,

    // c
    0b_0000000_0,
    0b_0000000_0,
    0b_0111100_0,
    0b_1000010_0,
    0b_1000000_0,
    0b_1000010_0,
    0b_0111100_0,
    0b_0000000_0,

    // d
    0b_0000010_0,
    0b_0000010_0,
    0b_0111010_0,
    0b_1000110_0,
    0b_1000010_0,
    0b_1000110_0,
    0b_0111010_0,
    0b_0000000_0,

    // e
    0b_0000000_0,
    0b_0000000_0,
    0b_0111100_0,
    0b_1000010_0,
    0b_1111110_0,
    0b_1000000_0,
    0b_0111100_0,
    0b_0000000_0,

    // f
    0b_0001100_0,
    0b_0010010_0,
    0b_0010000_0,
    0b_1111100_0,
    0b_0010000_0,
    0b_0010000_0,
    0b_0010000_0,
    0b_0000000_0,

    // g
    0b_0000000_0,
    0b_0000000_0,
    0b_0111010_0,
    0b_1000110_0,
    0b_1000110_0,
    0b_0111010_0,
    0b_0000010_0,
    0b_0111100_0,

    // h
    0b_1000000_0,
    0b_1000000_0,
    0b_1011100_0,
    0b_1100010_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_0000000_0,

    // i
    0b_0001000_0,
    0b_0000000_0,
    0b_0011000_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0011100_0,
    0b_0000000_0,

    // j
    0b_0000100_0,
    0b_0000000_0,
    0b_0001100_0,
    0b_0000100_0,
    0b_0000100_0,
    0b_0000100_0,
    0b_0100100_0,
    0b_0011000_0,

    // k
    0b_1000000_0,
    0b_1000000_0,
    0b_1000100_0,
    0b_1001000_0,
    0b_1010000_0,
    0b_1101000_0,
    0b_1000100_0,
    0b_0000000_0,

    // l
    0b_0011000_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0011100_0,
    0b_0000000_0,

    // m
    0b_0000000_0,
    0b_0000000_0,
    0b_1110110_0,
    0b_1001001_0,
    0b_1001001_0,
    0b_1001001_0,
    0b_1001001_0,
    0b_0000000_0,

    // n
    0b_0000000_0,
    0b_0000000_0,
    0b_1011100_0,
    0b_1100010_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_0000000_0,

    // o
    0b_0000000_0,
    0b_0000000_0,
    0b_0111100_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_0111100_0,
    0b_0000000_0,

    // p
    0b_0000000_0,
    0b_0000000_0,
    0b_1011100_0,
    0b_1100010_0,
    0b_1100010_0,
    0b_1011100_0,
    0b_1000000_0,
    0b_1000000_0,

    // q
    0b_0000000_0,
    0b_0000000_0,
    0b_0111010_0,
    0b_1000110_0,
    0b_1000110_0,
    0b_0111010_0,
    0b_0000010_0,
    0b_0000010_0,

    // r
    0b_0000000_0,
    0b_0000000_0,
    0b_1011100_0,
    0b_1100010_0,
    0b_1000000_0,
    0b_1000000_0,
    0b_1000000_0,
    0b_0000000_0,

    // s
    0b_0000000_0,
    0b_0000000_0,
    0b_0111110_0,
    0b_1000000_0,
    0b_0111100_0,
    0b_0000010_0,
    0b_1111100_0,
    0b_0000000_0,

    // t
    0b_0001000_0,
    0b_0001000_0,
    0b_0111110_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0001010_0,
    0b_0000100_0,
    0b_0000000_0,

    // u
    0b_0000000_0,
    0b_0000000_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_1000110_0,
    0b_0111010_0,
    0b_0000000_0,

    // v
    0b_0000000_0,
    0b_0000000_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_0100100_0,
    0b_0011000_0,
    0b_0000000_0,

    // w
    0b_0000000_0,
    0b_0000000_0,
    0b_1000001_0,
    0b_1001001_0,
    0b_1001001_0,
    0b_1001001_0,
    0b_0110110_0,
    0b_0000000_0,

    // x
    0b_0000000_0,
    0b_0000000_0,
    0b_1000010_0,
    0b_0100100_0,
    0b_0011000_0,
    0b_0100100_0,
    0b_1000010_0,
    0b_0000000_0,

    // y
    0b_0000000_0,
    0b_0000000_0,
    0b_1000010_0,
    0b_1000010_0,
    0b_1000110_0,
    0b_0111010_0,
    0b_0000010_0,
    0b_0111100_0,

    // z
    0b_0000000_0,
    0b_0000000_0,
    0b_1111110_0,
    0b_0000100_0,
    0b_0011000_0,
    0b_0100000_0,
    0b_1111110_0,
    0b_0000000_0,

    // {
    0b_0001100_0,
    0b_0010000_0,
    0b_0010000_0,
    0b_1100000_0,
    0b_0010000_0,
    0b_0010000_0,
    0b_0001100_0,
    0b_0000000_0,

    // |
    0b_0001000_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0001000_0,

    // }
    0b_0110000_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0000110_0,
    0b_0001000_0,
    0b_0001000_0,
    0b_0110000_0,
    0b_0000000_0,

    // ~
    0b_0110001_0,
    0b_1001001_0,
    0b_1000110_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,
    0b_0000000_0,
];