#[cfg(test)]
mod test {
    #[test]
    fn adu_works() {
        let inst = mbrs::Instance {
            serial: Some(mbrs::SerialConfig { slave_addr: 1 }),
            ..Default::default()
        };

        let mut buf = [
            0x01, // Slave addr
            0x03, // Read holding regs
            0x00, 0x00, // Start addr
            0x00, 0x01, // Quantity
            0x00, 0x00, // CRC
        ];
        let crc = mbrs::crc::crc16(&buf[..buf.len() - 2]);
        let crc = crc.to_le_bytes();
        match &mut buf {
            [.., crc_lo, crc_hi] => {
                *crc_lo = crc[0];
                *crc_hi = crc[1];
            }
        }

        let mut res = [0; mbrs::adu::SIZE_MAX];
        let res_len = mbrs::adu::handle_req(&inst, &buf, &mut res);

        assert_eq!(res_len, 7);
    }
}
