//! Persistent bans and admins. In-memory with a file snapshot so a restart does not forgive anyone.

use std::collections::{HashMap, HashSet};
use std::net::IpAddr;
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct Ban {
    pub ip: IpAddr,
    pub until: Option<Instant>,
    pub reason: String,
}

#[derive(Clone, Debug, Default)]
pub struct BanList {
    bans: HashMap<IpAddr, Ban>,
    admins: HashSet<IpAddr>,
    admin_ids: HashSet<u32>,
}

impl BanList {
    pub fn ban(&mut self, ip: IpAddr, minutes: Option<u32>, reason: impl Into<String>) {
        self.bans.insert(
            ip,
            Ban {
                ip,
                until: minutes.map(|m| Instant::now() + Duration::from_secs(u64::from(m) * 60)),
                reason: reason.into(),
            },
        );
    }

    pub fn unban(&mut self, ip: IpAddr) -> bool {
        self.bans.remove(&ip).is_some()
    }

    pub fn is_banned(&self, ip: IpAddr) -> bool {
        match self.bans.get(&ip) {
            Some(ban) => ban.until.is_none_or(|until| Instant::now() < until),
            None => false,
        }
    }

    pub fn add_admin(&mut self, ip: IpAddr) {
        self.admins.insert(ip);
    }

    pub fn add_admin_id(&mut self, id: u32) {
        self.admin_ids.insert(id);
    }

    pub fn remove_admin(&mut self, ip: IpAddr) -> bool {
        self.admins.remove(&ip)
    }

    pub fn is_admin(&self, ip: Option<IpAddr>, id: Option<u32>) -> bool {
        ip.is_some_and(|addr| self.admins.contains(&addr))
            || id.is_some_and(|player| self.admin_ids.contains(&player))
    }

    pub fn restore(text: &str) -> Self {
        let mut list = Self::default();
        for line in text.lines() {
            if let Some(rest) = line.strip_prefix("bans=") {
                for entry in rest.split(',') {
                    let mut parts = entry.splitn(3, ':');
                    if let (Some(ip), Some(reason), Some(kind)) =
                        (parts.next(), parts.next(), parts.next())
                    {
                        if let Ok(ip) = ip.parse::<IpAddr>() {
                            if kind != "temp" {
                                list.ban(ip, None, reason);
                            }
                        }
                    }
                }
            }
            if let Some(rest) = line.strip_prefix("admins=") {
                for ip in rest.split(',') {
                    if let Ok(ip) = ip.parse::<IpAddr>() {
                        list.add_admin(ip);
                    }
                }
            }
        }
        list
    }

    pub fn persist(&self) -> String {
        let ips: Vec<String> = self
            .bans
            .values()
            .map(|ban| {
                format!(
                    "{}:{}:{}",
                    ban.ip,
                    ban.reason,
                    if ban.until.is_some() { "temp" } else { "perm" }
                )
            })
            .collect();
        let admins: Vec<String> = self.admins.iter().map(ToString::to_string).collect();
        format!("bans={}\nadmins={}", ips.join(","), admins.join(","))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn a_temp_ban_expires_and_a_plain_ban_does_not() {
        let mut list = BanList::default();
        let ip = IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4));
        list.ban(ip, Some(0), "test");
        // Zero minutes is already over.
        assert!(!list.is_banned(ip));
        list.ban(ip, None, "perm");
        assert!(list.is_banned(ip));
        assert!(list.unban(ip));
        assert!(!list.is_banned(ip));
    }

    #[test]
    fn an_admin_is_remembered_by_ip_or_by_id() {
        let mut list = BanList::default();
        let ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1));
        list.add_admin(ip);
        list.add_admin_id(7);
        assert!(list.is_admin(Some(ip), None));
        assert!(list.is_admin(None, Some(7)));
        assert!(!list.is_admin(None, Some(8)));
    }

    #[test]
    fn a_persisted_permanent_ban_comes_back_after_restore() {
        let mut list = BanList::default();
        let ip = IpAddr::V4(Ipv4Addr::new(9, 9, 9, 9));
        list.ban(ip, None, "cheat");
        list.add_admin(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)));
        let restored = BanList::restore(&list.persist());
        assert!(restored.is_banned(ip));
        assert!(restored.is_admin(Some(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))), None));
    }
}
