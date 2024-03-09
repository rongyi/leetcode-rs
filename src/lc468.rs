struct Solution;

impl Solution {
    pub fn valid_ip_address(query_ip: String) -> String {
        // ipv6path
        if query_ip.contains(":") {
            let chunks: Vec<&str> = query_ip.split(':').collect();
            if chunks.len() != 8 {
                return "Neither".to_string();
            }
            for &ip in &chunks {
                if ip.len() > 4 {
                    return "Neither".to_string();
                }
                let num = u16::from_str_radix(ip, 16).ok();
                if num.is_none() {
                    return "Neither".to_string();
                }
            }

            "IPv6".to_string()
        } else {
            // ipv4
            let chunks: Vec<&str> = query_ip.split('.').collect();
            if chunks.len() != 4 {
                return "Neither".to_string();
            }

            for &ip in &chunks {
                if ip.len() > 3 {
                    return "Neither".to_string();
                }
                if ip.len() > 1 && ip.chars().nth(0).unwrap() == '0' {
                    return "Neither".to_string();
                }
                let num = u8::from_str_radix(ip, 10).ok();
                if num.is_none() {
                    return "Neither".to_string();
                }
            }

            "IPv4".to_string()
        }
    }
}

fn main() {}
