pub mod mbadu;
pub mod mbcoil;
mod mbcrc;
pub mod mbdef;
mod mbfn_coils;
mod mbfn_regs;
pub mod mbinst;
pub mod mbpdu;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mbadu_works() {
        let inst = mbinst::Instance {
            descriptors: None,
            serial: Some(mbinst::SerialConfig { slave_addr: 1 }),
        };

        let mut buf = [
            0x01, // Slave addr
            0x03, // Read holding regs
            0x00, 0x00, // Start addr
            0x00, 0x01, // Quantity
            0x00, 0x00, // CRC
        ];
        let crc = mbcrc::crc16(&buf[..buf.len() - 2]);
        let crc = crc.to_le_bytes();
        match &mut buf {
            [.., crc_lo, crc_hi] => {
                *crc_lo = crc[0];
                *crc_hi = crc[1];
            }
        }

        let mut res = [0; mbadu::ADU_SIZE_MAX];
        let res_len = mbadu::handle_req(&inst, &buf, &mut res);

        assert_eq!(res_len, 7);
    }

    #[test]
    fn mbpdu_too_little_data() {
        let inst = mbinst::Instance {
            descriptors: None,
            serial: Some(mbinst::SerialConfig { slave_addr: 1 }),
        };

        let buf = [0x04];
        let mut res = [0; mbpdu::PDU_SIZE_MAX];
        let res_len = mbpdu::handle_req(&inst, &buf, &mut res);
        assert_eq!(res_len, 2);
        assert_eq!(res[0], 0x04 | 0x80); // Error response
        assert_eq!(res[1], 0x03); // Illegal data value
    }

    #[test]
    fn mbcrc16_known_values() {
        let buf = [0x55, 0xAA, 0x02, 0xF0];
        let crc = mbcrc::crc16(&buf);
        assert_eq!(crc, 0xEC30);
    }
}
