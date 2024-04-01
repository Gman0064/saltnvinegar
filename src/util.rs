pub fn print_help() {
    print!("Usage: saltnvinegar [CHIP8_FILE]\n");
}

pub fn u8_tuple_to_u16(u8_in: (u8, u8)) -> u16 {
    let head_byte: u16 = u16::from(u8_in.0) << 8;
    let tail_byte: u16 = u16::from(u8_in.1);
    return head_byte + tail_byte;
}