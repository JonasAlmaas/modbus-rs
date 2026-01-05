#[cfg(test)]
mod test {
    #[test]
    fn pdu_too_little_data() {
        let inst: mbrs::Instance = Default::default();

        let buf = [0x04];
        let mut res = [0; mbrs::pdu::SIZE_MAX];
        let res_len = mbrs::pdu::handle_req(&inst, &buf, &mut res);
        assert_eq!(res_len, 2);
        assert_eq!(res[0], 0x04 | 0x80); // Error response
        assert_eq!(res[1], 0x03); // Illegal data value
    }

    #[test]
    fn pdu_read_coil_works() {
        use mbrs::coil::Descriptor as CoilDesc;
        use mbrs::coil::ReadMethod as CoilReadMethod;

        let coil1 = false;
        let coil2 = true;

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
                read: Some(CoilReadMethod::Fn(Box::new(|| coil2))),
                ..Default::default()
            },
        ];
        let inst = mbrs::Instance {
            coils: Some(coils),
            ..Default::default()
        };

        let buf = [
            0x01, // Fc: Read coils
            0x00, 0x00, // Start address
            0x00, 0x03, // Quantity
        ];
        let mut res = [0; mbrs::pdu::SIZE_MAX];
        let res_len = mbrs::pdu::handle_req(&inst, &buf, &mut res);
        assert_eq!(res_len, 3);
        assert_eq!(res[0], 0x01);
        assert_eq!(res[1], 0x01);
        assert_eq!(res[2], 0b0101);
    }

    #[test]
    fn pdu_write_single_coil_fn_works() {
        use mbrs::coil::Descriptor as CoilDesc;
        use mbrs::coil::WriteMethod as CoilWriteMethod;

        let mut coil1 = false;

        let coils = &mbrs::asc![CoilDesc {
            address: 0x00,
            write: Some(CoilWriteMethod::Fn(Box::new(|v| coil1 = v))),
            ..Default::default()
        }];
        let _inst = mbrs::Instance {
            coils: Some(coils),
            ..Default::default()
        };

        // TODO
        /*let buf = [
            0x01, // Fc: Read coils
            0x00, 0x00, // Start address
            0x00, 0x03, // Quantity
        ];
        let mut res = [0; PDU_SIZE_MAX];
        let res_len = handle_req(&inst, &buf, &mut res);*/
    }
}
