#[cfg(test)]
mod test {
    use byteorder::{BigEndian, ByteOrder};

    #[test]
    fn adu_tcp_works() {
        use mbrs::coil::Descriptor as CoilDesc;
        use mbrs::coil::ReadMethod as CoilReadMethod;

        let coil1 = false;

        let coils = &mbrs::asc![
            CoilDesc {
                address: 0x00,
                read: Some(CoilReadMethod::Value(true)),
                ..Default::default()
            },
            CoilDesc {
                address: 0x01,
                read: Some(CoilReadMethod::Ref(&coil1)),
                ..Default::default()
            },
            CoilDesc {
                address: 0x02,
                read: Some(CoilReadMethod::Fn(Box::new(|| true))),
                ..Default::default()
            },
        ];
        let inst = mbrs::Instance {
            coils: Some(coils),
            ..Default::default()
        };

        let buf = [
            0x00, 0x01, // Transation id
            0x00, 0x00, // Protocol id
            0x00, 0x06, // Length
            0x01, // Unit id
            0x01, // Fc: Read coils
            0x00, 0x00, // Start address
            0x00, 0x03, // Quantity
        ];
        let mut res = [0; mbrs::adu_tcp::SIZE_MAX];
        let res_len = mbrs::adu_tcp::handle_req(&inst, &buf, &mut res);
        assert_eq!(res_len, 10);
        assert_eq!(BigEndian::read_u16(&res), 0x0001); // Transaction id
        assert_eq!(BigEndian::read_u16(&res[2..]), 0x0000); // Protocol id
        assert_eq!(BigEndian::read_u16(&res[4..]), 0x0004); // Length
        assert_eq!(res[6], 0x01); // Unit id
        assert_eq!(res[7], 0x01); // Functino code
        assert_eq!(res[8], 0x01); // Byte count
        assert_eq!(res[9], 0b0101); // Data
    }

    #[test]
    fn adu_tcp_undersized_request_fails() {
        let inst: mbrs::Instance = Default::default();

        let buf = [
            0x00, 0x01, // Transation id
            0x00, 0x00, // Protocol id
            0x00, 0x06, // Length
            0x01, // Unit id
                  // Missing function code and data
        ];
        let mut res = [0; mbrs::adu_tcp::SIZE_MAX];
        let res_len = mbrs::adu_tcp::handle_req(&inst, &buf, &mut res);
        assert_eq!(res_len, 0);
    }

    #[test]
    fn adu_tcp_invaid_protocol_id_fails() {
        let inst: mbrs::Instance = Default::default();

        let buf = [
            0x00, 0x01, // Transation id
            0x00, 0x01, // Invalid protocol id (should be 0x0000)
            0x00, 0x06, // Length
            0x01, // Unit id
            0x01, // Fc: Read coils
            0x00, 0x00, // Start address
            0x00, 0x03, // Quantity
        ];
        let mut res = [0; mbrs::adu_tcp::SIZE_MAX];
        let res_len = mbrs::adu_tcp::handle_req(&inst, &buf, &mut res);
        assert_eq!(res_len, 0);
    }

    #[test]
    fn adu_tcp_transaction_id_echoed_works() {
        let inst: mbrs::Instance = Default::default();

        let buf = [
            0x00, 0x01, // Transation id
            0x00, 0x00, // Protocol id
            0x00, 0x06, // Length
            0x01, // Unit id
            0x01, // Fc: Read coils
            0x00, 0x00, // Start address
            0x00, 0x03, // Quantity
        ];
        let mut res = [0; mbrs::adu_tcp::SIZE_MAX];
        let res_len = mbrs::adu_tcp::handle_req(&inst, &buf, &mut res);
        assert!(res_len > mbrs::adu_tcp::MBAP_SIZE);
        assert_eq!(BigEndian::read_u16(&res), 0x0001); // Transaction id
    }

    #[test]
    fn adu_tcp_protocol_id_echoed_works() {
        let inst: mbrs::Instance = Default::default();

        let buf = [
            0x00, 0x01, // Transation id
            0x00, 0x00, // Protocol id
            0x00, 0x06, // Length
            0x01, // Unit id
            0x01, // Fc: Read coils
            0x00, 0x00, // Start address
            0x00, 0x03, // Quantity
        ];
        let mut res = [0; mbrs::adu_tcp::SIZE_MAX];
        let res_len = mbrs::adu_tcp::handle_req(&inst, &buf, &mut res);
        assert!(res_len > mbrs::adu_tcp::MBAP_SIZE);
        assert_eq!(BigEndian::read_u16(&res[2..]), 0x0000); // Protocol id
    }

    #[test]
    fn adu_tcp_unit_id_echoed_works() {
        let inst: mbrs::Instance = Default::default();

        let buf = [
            0x00, 0x01, // Transation id
            0x00, 0x00, // Protocol id
            0x00, 0x06, // Length
            0x55, // Unit id
            0x01, // Fc: Read coils
            0x00, 0x00, // Start address
            0x00, 0x03, // Quantity
        ];
        let mut res = [0; mbrs::adu_tcp::SIZE_MAX];
        let res_len = mbrs::adu_tcp::handle_req(&inst, &buf, &mut res);
        assert!(res_len > mbrs::adu_tcp::MBAP_SIZE);
        assert_eq!(res[6], 0x55); // Unit id
    }

    #[test]
    fn adu_tcp_zero_size_fails() {
        let inst: mbrs::Instance = Default::default();

        let buf: &[u8] = &[];
        let mut res = [0; mbrs::adu_tcp::SIZE_MAX];
        let res_len = mbrs::adu_tcp::handle_req(&inst, buf, &mut res);
        assert_eq!(res_len, 0);
    }

    #[test]
    fn adu_tcp_min_size_works() {
        let inst: mbrs::Instance = Default::default();

        let buf = [
            0x00, 0x01, // Transation id
            0x00, 0x00, // Protocol id
            0x00, 0x06, // Length
            0x55, // Unit id
            0x01, // Fc: Read coils
        ];
        let mut res = [0; mbrs::adu_tcp::SIZE_MAX];
        let res_len = mbrs::adu_tcp::handle_req(&inst, &buf, &mut res);
        assert_eq!(res_len, 0);
    }
}
