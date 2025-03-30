#[test]
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
