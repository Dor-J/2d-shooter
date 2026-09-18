//! Rate limits and input feasibility. The simulation already refuses an illegal shot; this stops
//! a client from drowning the server before that check runs.

use std::collections::HashMap;
use std::net::IpAddr;
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct Bucket {
    tokens: f32,
    last: Instant,
    rate: f32,
    burst: f32,
}

impl Bucket {
    pub fn new(rate: f32, burst: f32) -> Self {
        Self {
            tokens: burst,
            last: Instant::now(),
            rate,
            burst,
        }
    }

    pub fn take(&mut self) -> bool {
        self.refill();
        if self.tokens < 1.0 {
            return false;
        }
        self.tokens -= 1.0;
        true
    }

    fn refill(&mut self) {
        let elapsed = self.last.elapsed().as_secs_f32();
        self.last = Instant::now();
        self.tokens = (self.tokens + elapsed * self.rate).min(self.burst);
    }
}

#[derive(Clone, Debug, Default)]
pub struct RateLimits {
    per_guest: HashMap<u32, Bucket>,
    per_ip: HashMap<IpAddr, Bucket>,
    rooms_per_ip: HashMap<IpAddr, Bucket>,
}

impl RateLimits {
    pub fn allow_message(&mut self, guest: u32) -> bool {
        self.per_guest
            .entry(guest)
            .or_insert_with(|| Bucket::new(MAX_INPUTS_PER_SECOND as f32, 40.0))
            .take()
    }

    pub fn allow_connect(&mut self, ip: IpAddr) -> bool {
        self.per_ip
            .entry(ip)
            .or_insert_with(|| Bucket::new(2.0, 6.0))
            .take()
    }

    pub fn allow_room(&mut self, ip: IpAddr) -> bool {
        self.rooms_per_ip
            .entry(ip)
            .or_insert_with(|| Bucket::new(0.2, 2.0))
            .take()
    }
}

/// Whether an input is even worth handing to the simulation.
pub fn input_feasible(seq: u32, last_seq: u32, aim_x: f32, aim_y: f32) -> bool {
    if seq <= last_seq {
        return false;
    }
    if seq - last_seq > 120 {
        return false;
    }
    aim_x.is_finite() && aim_y.is_finite() && aim_x.abs() <= 10_000.0 && aim_y.abs() <= 10_000.0
}

/// How many inputs one second may carry. Sixty is the tick; a little slack covers jitter.
pub const MAX_INPUTS_PER_SECOND: u32 = 72;

pub fn resume_ttl() -> Duration {
    Duration::from_secs(20)
}

/// Whether `socket` is a reverse proxy we may read `X-Forwarded-For` from.
pub fn is_trusted_proxy(socket: IpAddr, spec: &str) -> bool {
    let spec = spec.trim();
    if spec.is_empty() {
        return false;
    }
    if spec.eq_ignore_ascii_case("private") {
        return match socket {
            IpAddr::V4(ip) => ip.is_private() || ip.is_loopback(),
            IpAddr::V6(ip) => ip.is_loopback() || ip.is_unique_local(),
        };
    }
    spec.split(',')
        .filter_map(|part| part.trim().parse::<IpAddr>().ok())
        .any(|proxy| proxy == socket)
}

/// The client address used for bans and rate limits.
///
/// `X-Forwarded-For` is only believed when the TCP peer is a trusted proxy. The rightmost hop
/// is the one that proxy appended.
pub fn client_ip(socket: IpAddr, forwarded_for: Option<&str>, trusted_proxies: &str) -> IpAddr {
    if !is_trusted_proxy(socket, trusted_proxies) {
        return socket;
    }
    forwarded_for
        .and_then(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|hop| !hop.is_empty())
                .next_back()
        })
        .and_then(|hop| hop.parse().ok())
        .unwrap_or(socket)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    // Acceptance evidence: rust:security:limits
    fn a_bucket_gives_out_its_burst_and_then_refuses() {
        let mut bucket = Bucket::new(1.0, 2.0);
        assert!(bucket.take());
        assert!(bucket.take());
        assert!(!bucket.take());
    }

    #[test]
    fn a_stale_or_huge_aim_is_not_feasible() {
        assert!(!input_feasible(1, 1, 0.0, 0.0));
        assert!(!input_feasible(200, 1, 0.0, 0.0));
        assert!(!input_feasible(2, 1, f32::NAN, 0.0));
        assert!(input_feasible(2, 1, 10.0, 10.0));
    }

    #[test]
    // Acceptance evidence: rust:security:resume
    fn a_disconnected_guest_has_twenty_seconds_to_resume() {
        assert_eq!(resume_ttl(), Duration::from_secs(20));
    }

    #[test]
    fn room_creation_is_slower_than_ordinary_messages() {
        let mut limits = RateLimits::default();
        let ip = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        assert!(limits.allow_room(ip));
        assert!(limits.allow_room(ip));
        assert!(!limits.allow_room(ip));
    }

    #[test]
    // Acceptance evidence: rust:security:proxy
    fn forwarded_for_is_ignored_unless_the_socket_is_a_trusted_proxy() {
        let spoofed = "9.9.9.9, 10.0.0.2";
        let docker = IpAddr::V4(Ipv4Addr::new(172, 18, 0, 2));
        let direct = IpAddr::V4(Ipv4Addr::new(203, 0, 113, 5));
        assert_eq!(client_ip(direct, Some(spoofed), ""), direct);
        assert_eq!(
            client_ip(docker, Some(spoofed), "private"),
            IpAddr::V4(Ipv4Addr::new(10, 0, 0, 2))
        );
        assert_eq!(
            client_ip(docker, Some("1.2.3.4"), "8.8.8.8"),
            docker,
            "an unlisted proxy cannot mint a client address"
        );
    }
}
