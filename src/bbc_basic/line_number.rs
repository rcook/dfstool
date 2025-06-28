// https://xania.org/200711/bbc-basic-line-number-format

pub const fn encode_line_number(line_number: u16) -> (u8, u8, u8) {
    let hi = (line_number >> 8) as u8;
    let lo = (line_number & 0xff) as u8;
    let top_bits = ((lo & 0b11000000) >> 2) + ((hi & 0b11000000) >> 4);
    (top_bits ^ 0x54, (lo & 0x3f) | 0x40, (hi & 0x3f) | 0x40)
}

pub const fn decode_line_number(b0: u8, b1: u8, b2: u8) -> u16 {
    let t0 = b0 ^ 0x54;
    let ll = (t0 & 0b00110000) >> 4;
    let hh = (t0 & 0b00001100) >> 2;

    let t1 = b1 & 0b00111111;
    let lo = t1 + (ll << 6);

    let t2 = b2 & 0b00111111;
    let hi = t2 + (hh << 6);

    ((hi as u16) << 8) + lo as u16
}

#[cfg(test)]
mod tests {
    use crate::bbc_basic::{decode_line_number, encode_line_number};
    use rstest::rstest;

    #[rstest]
    #[case((0x74, 0x4c, 0x40), 140)]
    #[case((0x44, 0x78, 0x45), 1400)]
    #[case((0x74, 0x70, 0x76), 14000)]
    #[case((0x48, 0x60, 0x6a), 60000)] // Not actually a valid line number!
    fn basics(#[case] (b0, b1, b2): (u8, u8, u8), #[case] line_number: u16) {
        assert_eq!((b0, b1, b2), encode_line_number(line_number));
        assert_eq!(line_number, decode_line_number(b0, b1, b2));
    }
}
