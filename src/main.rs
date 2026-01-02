mod mbcrc;
mod mbdef;
mod mbfn_coils;
mod mbfn_regs;
mod mbpdu;

fn print_buf(buf: &[u8]) {
    print!("[");
    for (ix, v) in buf.iter().enumerate() {
        if ix > 0 {
            print!(", ");
        }
        print!("0x{:02X}", v);
    }
    println!("]");
}

fn main() {
    println!("CRC lookup table");
    mbcrc::print_crc_table();

    println!();
    println!("-------------------");
    println!();

    println!(
        "CRC test Expected<0x{:04X}> Actual<0x{:04X}>",
        0x2590,
        mbcrc::crc16(&[0x12, 0x34, 0x56, 0x78, 0x09])
    );

    println!();
    println!("-------------------");
    println!();

    let req = [
        0x04, // Functino code (Read input registers)
        0x00, 0x00, // Start address
        0x00, 0x02, // Quantity of registers to read
    ];
    print!("Request: ");
    print_buf(&req);

    let mut resp = [0u8; mbpdu::PDU_SIZE_MAX];
    let resp_len = mbpdu::handle_req(&req, &mut resp);

    println!("Response length: {}", resp_len);
    print_buf(&resp[..resp_len]);

    println!();

    let req = [0x04];
    print!("Failing request: ");
    print_buf(&req);
    let resp_len = mbpdu::handle_req(&req, &mut resp);

    println!("Response length: {}", resp_len);
    print_buf(&resp[..resp_len]);
}
