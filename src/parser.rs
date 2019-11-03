use crate::host::Host;
use std::fmt;
use std::fs;

pub struct HostsFile {
    pub hosts: Vec<Host>,
}

impl From<&str> for HostsFile {
    /// Parses a hosts file.
    ///
    /// Ignores comments.
    ///
    /// # Example
    /// ```
    /// let hosts = "
    /// 127.0.0.1	localhost
    /// ::1		localhost
    /// 127.0.1.1	test.localdomain test";
    ///
    /// hosts::HostsFile::from(hosts);
    /// ```
    fn from(s: &str) -> HostsFile {
        let mut hosts: Vec<Host> = vec![];

        for line in s.lines() {
            if line.contains("#") || line.is_empty() {
                continue;
            }

            hosts.push(Host::from(line))
        }
        return HostsFile { hosts };
    }
}

impl From<Vec<Host>> for HostsFile {
    fn from(hosts: Vec<Host>) -> HostsFile {
        HostsFile { hosts }
    }
}

impl fmt::Display for HostsFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut hosts_file = String::new();

        for host in self.hosts.clone() {
            hosts_file.push_str(&format!("{}\n", host))
        }

        write!(f, "{}", hosts_file)
    }
}

impl HostsFile {
    /// Create empty HostsFile
    pub fn new() -> HostsFile {
        HostsFile { hosts: Vec::new() }
    }

    pub fn from_file(path: &str) -> HostsFile {
        HostsFile::from(
            fs::read_to_string(path)
                .expect("Invalid file path")
                .as_str(),
        )
    }

    /// Writes hosts to a hosts file.
    ///
    /// # Example
    /// ```
    /// let hosts_file = hosts::HostsFile::from("127.0.0.1 localhost");
    /// hosts_file.write("hosts");
    /// # std::fs::remove_file("hosts");
    /// ```
    pub fn write(self, path: &str) {
        fs::write(path, format!("{}", self)).expect("Invalid path");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let hosts_str = "127.0.0.1 localhost\n::1 localhost\n127.0.1.1 foxtrot.localdomain foxtrot";

        let hosts_file = HostsFile::from(hosts_str);

        for (i, line) in hosts_str.lines().enumerate() {
            assert_eq!(format!("{}", hosts_file.hosts[i]), line)
        }
    }
}
