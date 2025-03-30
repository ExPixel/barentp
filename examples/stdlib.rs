const DEFAULT_NTP_SERVER: &str = "pool.ntp.org";

pub fn main() {
    // Get the NTP server host name from the command line or just use the default:
    let ntp_server_arg = std::env::args().nth(1);
    let ntp_server = ntp_server_arg.unwrap_or_else(|| DEFAULT_NTP_SERVER.to_string());
    println!("Using NTP server: {}", ntp_server);

    // Lookup the IP address of the NTP server using dns_lookup:
    let ips = match dns_lookup::lookup_host(&ntp_server) {
        Ok(ips) => ips,
        Err(err) => {
            eprintln!("Failed to lookup host");
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };
    if ips.is_empty() {
        eprintln!("No IP addresses found for {}", ntp_server);
        std::process::exit(1);
    }
    let ip = ips[0];
    println!("Using NTP server IP address: {}", ip);

    let addr = std::net::SocketAddr::new(ip, 123);
    println!("Using NTP server socket address: {}", addr);

    // Create a UDP socket, bind it to an available IP and port, then connect it to the NTP server:
    let Ok(socket) = std::net::UdpSocket::bind("0.0.0.0:0") else {
        eprintln!("Failed to create UDP socket");
        std::process::exit(1);
    };
    if let Err(err) = socket.connect(addr) {
        eprintln!("Failed to connect UTP socket to {}", addr);
        eprintln!("{}", err);
        std::process::exit(1);
    }

    if let Err(err) = socket.set_read_timeout(Some(std::time::Duration::from_secs(5))) {
        eprintln!("Failed to set socket read timeout");
        eprintln!("{}", err);
        std::process::exit(1);
    }

    if let Err(err) = socket.set_write_timeout(Some(std::time::Duration::from_secs(5))) {
        eprintln!("Failed to set socket write timeout");
        eprintln!("{}", err);
        std::process::exit(1);
    }

    // Get a timestamp from the NTP server:
    println!("Fetching timestamp from NTP server...");
    let timestamp = match barentp::sntp_get_transmit_timestamp(&socket) {
        Ok(timestamp) => timestamp,
        Err(err) => {
            eprintln!("Failed to get timestamp from NTP server");
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let datetime_ntp = chrono::DateTime::<chrono::Utc>::from(timestamp);
    let datetime_sys = chrono::Utc::now();

    println!("NTP timestamp: {}", datetime_ntp);
    println!("System timestamp: {}", datetime_sys);

    let diff_time_delta = (datetime_ntp - datetime_sys).abs();
    let diff_duration = match diff_time_delta.to_std() {
        Ok(duration) => duration,
        Err(err) => {
            eprintln!("Failed to convert to std duration");
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };
    println!("Difference: {diff_duration:?}");
}
