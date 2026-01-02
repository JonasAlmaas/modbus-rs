const fn gen_lookup_table() -> [u16; 256] {
    const POLYNOMIAL: u16 = 0xA001; // Reversed polynomial for LSB-first calculation

    let mut table = [0; 256];

    let mut i = 0usize;
    while i < 256 {
        let mut crc = i as u16;

        let mut j = 0;
        while j < 8 {
            if (crc & 0x0001) != 0 {
                crc >>= 1;
                crc ^= POLYNOMIAL;
            } else {
                crc >>= 1;
            }
            j += 1;
        }

        table[i] = crc;
        i += 1;
    }

    table
}

const LOOKUP: [u16; 256] = gen_lookup_table();

pub fn crc16(buf: &[u8]) -> u16 {
    buf.iter().fold(0xFFFF, |crc, v| {
        (crc >> 8) ^ LOOKUP[((crc ^ *v as u16) & 0xFF) as usize]
    })
}

pub fn print_lookup() {
    for row_ix in 0..32 {
        for col_ix in 0..8 {
            let ix = row_ix * 8 + col_ix;
            if col_ix > 0 {
                print!(" ");
            }
            print!("0x{:04X}", LOOKUP[ix]);
        }
        println!();
    }
}
