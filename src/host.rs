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

impl From<&str> for Host {
    /// Creates a Host from a hosts line
    /// # Example
    /// ```
    /// hosts::Host::from("127.0.0.1 localhost");
    /// ```
    fn from(s: &str) -> Host {
        let values: Vec<&str> = s.split_whitespace().collect();
        Host::new(
            &values.get(0).expect("IP not found"),
            &values.get(1).expect("FQDN not found"),
            match values.get(2..) {
                Some([]) => None,
                Some(aliases) => Some(aliases.to_vec().iter().map(|v| v.to_string()).collect()),
                _ => None,
            }
        )
    }
}

impl fmt::Display for Host {
    /// Formats the host to output the hosts file standard.
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
    fn test_from() {
        let host = Host::new("127.0.0.1", "localhost", Some(vec!["test".to_string()]));
        assert_eq!(Host::from("127.0.0.1 localhost test"), host);

        let host = Host::new("127.0.0.1", "localhost", None);
        assert_eq!(Host::from("127.0.0.1 localhost"), host)
    }

    #[test]
    fn test_display() {
        let host = Host::new("127.0.0.1", "localhost", Some(vec!["test".to_string()]));
        assert_eq!(format!("{}", host), "127.0.0.1 localhost test");

        let host = Host::new("127.0.0.1", "localhost", None);
        assert_eq!(format!("{}", host), "127.0.0.1 localhost");
    }
}
