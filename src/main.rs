mod mbdef;
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
