pub mod request_formatter_test {
    use std::str::FromStr;

    use bigdecimal::BigDecimal;

    use iyzipay_rust::requests::RequestFormatter;

    #[test]
    fn should_format_price_given_string_value() {
        assert_eq!("0.0", RequestFormatter::format_price(&BigDecimal::from_str("0").unwrap()));
        assert_eq!("0.0", RequestFormatter::format_price(&BigDecimal::from_str("00000").unwrap()));
        assert_eq!("0.0", RequestFormatter::format_price(&BigDecimal::from_str("0.0").unwrap()));
        assert_eq!("0.0", RequestFormatter::format_price(&BigDecimal::from_str("00000.0000").unwrap()));
        assert_eq!("1.0", RequestFormatter::format_price(&BigDecimal::from_str("1").unwrap()));
        assert_eq!("1.0", RequestFormatter::format_price(&BigDecimal::from_str("1.0").unwrap()));
        assert_eq!("1.0", RequestFormatter::format_price(&BigDecimal::from_str("1.000").unwrap()));
        assert_eq!("1.0", RequestFormatter::format_price(&BigDecimal::from_str("00001.000").unwrap()));
        assert_eq!("-1.0", RequestFormatter::format_price(&BigDecimal::from_str("-1").unwrap()));
        assert_eq!("-1.0", RequestFormatter::format_price(&BigDecimal::from_str("-1.0").unwrap()));
        assert_eq!("-1.0", RequestFormatter::format_price(&BigDecimal::from_str("-1.000").unwrap()));
        assert_eq!("0.3", RequestFormatter::format_price(&BigDecimal::from_str("0.3").unwrap()));
        assert_eq!("-0.3", RequestFormatter::format_price(&BigDecimal::from_str("-0.3").unwrap()));
        assert_eq!("1100000.3", RequestFormatter::format_price(&BigDecimal::from_str("00001100000.3000000").unwrap()));
        assert_eq!("-1.0", RequestFormatter::format_price(&BigDecimal::from_str("-00001.000").unwrap()));
        assert_eq!("10000.0", RequestFormatter::format_price(&BigDecimal::from_str("10000").unwrap()));
        assert_eq!("-10000.0", RequestFormatter::format_price(&BigDecimal::from_str("-10000").unwrap()));
        assert_eq!("33001.00044", RequestFormatter::format_price(&BigDecimal::from_str("0033001.0004400").unwrap()));
        assert_eq!("-33001.00044", RequestFormatter::format_price(&BigDecimal::from_str("-0033001.0004400").unwrap()));
        assert_eq!("99999999999999999999999999.99999999999999999999999", RequestFormatter::format_price(&BigDecimal::from_str("99999999999999999999999999.99999999999999999999999").unwrap()));
        assert_eq!("-99999999999999999999999999.99999999999999999999999", RequestFormatter::format_price(&BigDecimal::from_str("-99999999999999999999999999.99999999999999999999999").unwrap()));
        assert_eq!("99999999999999999999999999.99999999999999999999999", RequestFormatter::format_price(&BigDecimal::from_str("0099999999999999999999999999.9999999999999999999999900").unwrap()));
        assert_eq!("-99999999999999999999999999.99999999999999999999999", RequestFormatter::format_price(&BigDecimal::from_str("-0099999999999999999999999999.9999999999999999999999900").unwrap()));
    }
}