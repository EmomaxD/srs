use url::Url;

pub fn parse_address(address_str: &str) -> Result<(String, u16), String> {
    // Try to parse as URL
    if let Ok(url) = Url::parse(&format!("http://{}", address_str)) {
        let host = url.host_str().ok_or("Missing host in URL")?;
        let ip_str = if host.starts_with("www.") {
            &host[4..] // Remove "www." prefix
        } else {
            host
        };
        let port = url.port().unwrap_or(80); // Default to 80 for HTTP
        return Ok((ip_str.to_string(), port));
    }

    // Try to parse as IP:PORT
    if let Some((ip_str, port_str)) = address_str.split_once(':') {
        // Validate IP address format
        let ip_parts: Vec<&str> = ip_str.split('.').collect();
        if ip_parts.len() == 4 {
            for part in ip_parts {
                let _octet: u8 = match part.parse() {
                    Ok(value) if value <= 255 => value,
                    _ => return Err("Invalid IP address octet".to_string()),
                };
            }
            let port: u16 = match port_str.parse() {
                Ok(value) if value <= 65535 => value,
                _ => return Err("Invalid port number".to_string()),
            };
            return Ok((ip_str.to_string(), port));
        } else {
            return Err("Invalid IP address format".to_string());
        }
    }

    Err("Invalid address format".to_string())
}
