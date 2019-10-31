use std::fmt;
use std::net::IpAddr;

#[derive(Debug, PartialEq)]
pub struct Host {
    pub ip: IpAddr,
    pub fqdn: String,
    pub aliases: Option<Vec<String>>,
}

impl Host {
    pub fn new(ip: &str, fqdn: &str, aliases: Option<Vec<String>>) -> Host {
        Host {
            ip: ip.parse().expect("Invalid ip address"),
            fqdn: fqdn.to_string(),
            aliases,
        }
    }
}

impl fmt::Display for Host {
    /// Formats the host to comply with the hosts file standard.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut aliases = String::new();

        if self.aliases.is_some() {
            for alias in self.aliases.as_ref().unwrap() {
                aliases.push_str(&alias);
            }
            return write!(f, "{} {} {}", self.ip, self.fqdn, aliases);
        }

        write!(f, "{} {}", self.ip, self.fqdn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_bad_ip() {
        Host::new("1234.123.1233", "localhost", None);
    }

    #[test]
    fn test_display_with_aliases() {
        let host = Host::new("127.0.0.1", "localhost", Some(vec!["fennec".to_string()]));
        assert_eq!(format!("{}", host), "127.0.0.1 localhost fennec");
    }

    #[test]
    fn test_display_without_aliases() {
        let host = Host::new("127.0.0.1", "localhost", None);
        assert_eq!(format!("{}", host), "127.0.0.1 localhost");
    }
}
