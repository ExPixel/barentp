#[test]
#[ignore]
pub fn compare_to_system_time() {
    let ntp_server =
        std::env::var("NTP_TEST_SERVER").unwrap_or_else(|_| "pool.ntp.org".to_string());
    println!("Using NTP server: {}", ntp_server);

    // Lookup the IP address of the NTP server using dns_lookup:
    let ips = dns_lookup::lookup_host(&ntp_server).expect("failed to lookup host");
    assert!(!ips.is_empty(), "no IP addresses found for {}", ntp_server);
    let ip = ips[0];
    println!("Using NTP server IP address: {}", ip);

    let addr = std::net::SocketAddr::new(ip, 123);
    println!("Using NTP server socket address: {}", addr);

    // Create a UDP socket, bind it to an available IP and port, then connect it to the NTP server:
    let socket = std::net::UdpSocket::bind("0.0.0.0:0").expect("could not found to UDP socket");

    socket
        .set_read_timeout(Some(std::time::Duration::from_secs(5)))
        .expect("failed to set socket read timeout");

    socket
        .set_write_timeout(Some(std::time::Duration::from_secs(5)))
        .expect("failed to set socket write timeout");

    socket
        .connect(addr)
        .expect("failed to connect UDP socket to NTP server");

    // Get a timestamp from the NTP server:
    println!("Fetching timestamp from NTP server...");
    let timestamp = barentp::sntp_get_transmit_timestamp(&socket).expect("failed to get timestamp");

    let datetime_ntp = chrono::DateTime::<chrono::Utc>::from(timestamp);
    let datetime_sys = chrono::Utc::now();

    println!("NTP timestamp: {}", datetime_ntp);
    println!("System timestamp: {}", datetime_sys);

    let diff_time_delta = (datetime_ntp - datetime_sys).abs();
    let diff_duration = diff_time_delta
        .to_std()
        .expect("failed to convert to std duration");
    println!("Difference: {diff_duration:?}");

    assert!(
        diff_duration < std::time::Duration::from_secs(5),
        "difference between NTP and system time is too large"
    );
}

#[test]
fn test_first_timestamp_msb_set_micros() {
    let timestamp = barentp::Timestamp::new(0x80000000, 0);
    let actual_utc = timestamp.utc_micros();

    // First timestamp with MSB set:
    //     Saturday, January 20, 1968 | 03:14:08 AM UTC
    let expected = chrono::NaiveDateTime::new(
        chrono::NaiveDate::from_ymd_opt(1968, 1, 20).unwrap(),
        chrono::NaiveTime::from_hms_opt(3, 14, 8).unwrap(),
    );
    let expected_utc = expected.and_utc().timestamp_micros();

    assert_eq!(actual_utc, expected_utc);
}

#[test]
fn test_first_timestamp_msb_clear_micros() {
    let timestamp = barentp::Timestamp::new(0, 0);
    let actual_utc = timestamp.utc_micros();

    // First timestamp with MSB clear:
    //     Thursday, February 07, 2036 | 06:28:16 AM UTC
    let expected = chrono::NaiveDateTime::new(
        chrono::NaiveDate::from_ymd_opt(2036, 2, 7).unwrap(),
        chrono::NaiveTime::from_hms_opt(6, 28, 16).unwrap(),
    );
    let expected_utc = expected.and_utc().timestamp_micros();

    assert_eq!(actual_utc, expected_utc);
}

#[test]
fn test_first_timestamp_msb_set_millis() {
    let timestamp = barentp::Timestamp::new(0x80000000, 0);
    let actual_utc = timestamp.utc_millis();

    // First timestamp with MSB set:
    //     Saturday, January 20, 1968 | 03:14:08 AM UTC
    let expected = chrono::NaiveDateTime::new(
        chrono::NaiveDate::from_ymd_opt(1968, 1, 20).unwrap(),
        chrono::NaiveTime::from_hms_opt(3, 14, 8).unwrap(),
    );
    let expected_utc = expected.and_utc().timestamp_millis();

    assert_eq!(actual_utc, expected_utc);
}

#[test]
fn test_first_timestamp_msb_clear_millis() {
    let timestamp = barentp::Timestamp::new(0, 0);
    let actual_utc = timestamp.utc_millis();

    // First timestamp with MSB clear:
    //     Thursday, February 07, 2036 | 06:28:16 AM UTC
    let expected = chrono::NaiveDateTime::new(
        chrono::NaiveDate::from_ymd_opt(2036, 2, 7).unwrap(),
        chrono::NaiveTime::from_hms_opt(6, 28, 16).unwrap(),
    );
    let expected_utc = expected.and_utc().timestamp_millis();

    assert_eq!(actual_utc, expected_utc);
}

#[test]
fn test_first_timestamp_msb_set_seconds() {
    let timestamp = barentp::Timestamp::new(0x80000000, 0);
    let actual_utc = timestamp.utc_seconds();

    // First timestamp with MSB set:
    //     Saturday, January 20, 1968 | 03:14:08 AM UTC
    let expected = chrono::NaiveDateTime::new(
        chrono::NaiveDate::from_ymd_opt(1968, 1, 20).unwrap(),
        chrono::NaiveTime::from_hms_opt(3, 14, 8).unwrap(),
    );
    let expected_utc = expected.and_utc().timestamp();

    assert_eq!(actual_utc, expected_utc);
}

#[test]
fn test_first_timestamp_msb_clear_seconds() {
    let timestamp = barentp::Timestamp::new(0, 0);
    let actual_utc = timestamp.utc_seconds();

    // First timestamp with MSB clear:
    //     Thursday, February 07, 2036 | 06:28:16 AM UTC
    let expected = chrono::NaiveDateTime::new(
        chrono::NaiveDate::from_ymd_opt(2036, 2, 7).unwrap(),
        chrono::NaiveTime::from_hms_opt(6, 28, 16).unwrap(),
    );
    let expected_utc = expected.and_utc().timestamp();

    assert_eq!(actual_utc, expected_utc);
}
