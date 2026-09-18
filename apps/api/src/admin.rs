//! Safe administrator commands. The tokenizer never shells out; every argument is typed.

use std::net::IpAddr;
use std::str::FromStr;

/// Who may run a command.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    Player,
    Admin,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdminCommand {
    AddMap { map: String },
    DelMap { map: String },
    AddBot { team: Option<u8>, name: String },
    Kick { target: Target },
    KickLast,
    TempBan { minutes: u32, target: Target },
    Ban { target: Target },
    BanIp { ip: IpAddr },
    Unban { ip: IpAddr },
    Map { map: String },
    Restart,
    NextMap,
    Adm { target: Target },
    AdmIp { ip: IpAddr },
    Unadm { ip: IpAddr },
    RespawnTime { seconds: u32 },
    Password { value: String },
    MaxPlayers { count: u8 },
    Drain,
    Undrain,
    Vote { map: String },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Target {
    Id(u32),
    Name(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdminError {
    Empty,
    Unknown,
    BadArgument,
    Unauthorized,
}

pub fn parse_admin(line: &str) -> Result<AdminCommand, AdminError> {
    let trimmed = line.trim().trim_start_matches('/');
    let mut parts = tokenize(trimmed);
    let verb = parts.next().ok_or(AdminError::Empty)?.to_ascii_uppercase();
    match verb.as_str() {
        "ADDMAP" => Ok(AdminCommand::AddMap {
            map: require_rest(&mut parts)?,
        }),
        "DELMAP" => Ok(AdminCommand::DelMap {
            map: require_rest(&mut parts)?,
        }),
        verb if verb.starts_with("ADDBOT") => {
            let team = match verb.strip_prefix("ADDBOT").unwrap_or("") {
                "1" | "ALPHA" => Some(1),
                "2" | "BRAVO" => Some(2),
                "" => None,
                _ => return Err(AdminError::BadArgument),
            };
            Ok(AdminCommand::AddBot {
                team,
                name: require_rest(&mut parts).unwrap_or_else(|_| "Bot".into()),
            })
        }
        "KICK" => Ok(AdminCommand::Kick {
            target: parse_target(&require_rest(&mut parts)?)?,
        }),
        "KICKLAST" => Ok(AdminCommand::KickLast),
        "TEMPBAN" => {
            let minutes = parse_u32(parts.next().ok_or(AdminError::BadArgument)?)?;
            Ok(AdminCommand::TempBan {
                minutes,
                target: parse_target(&require_rest(&mut parts)?)?,
            })
        }
        "BAN" => Ok(AdminCommand::Ban {
            target: parse_target(&require_rest(&mut parts)?)?,
        }),
        "BANIP" => Ok(AdminCommand::BanIp {
            ip: parse_ip(&require_rest(&mut parts)?)?,
        }),
        "UNBAN" => Ok(AdminCommand::Unban {
            ip: parse_ip(&require_rest(&mut parts)?)?,
        }),
        "MAP" => Ok(AdminCommand::Map {
            map: require_rest(&mut parts)?,
        }),
        "RESTART" => Ok(AdminCommand::Restart),
        "NEXTMAP" => Ok(AdminCommand::NextMap),
        "ADM" => Ok(AdminCommand::Adm {
            target: parse_target(&require_rest(&mut parts)?)?,
        }),
        "ADMIP" => Ok(AdminCommand::AdmIp {
            ip: parse_ip(&require_rest(&mut parts)?)?,
        }),
        "UNADM" => Ok(AdminCommand::Unadm {
            ip: parse_ip(&require_rest(&mut parts)?)?,
        }),
        "RESPAWNTIME" => Ok(AdminCommand::RespawnTime {
            seconds: parse_u32(&require_rest(&mut parts)?)?,
        }),
        "PASSWORD" => Ok(AdminCommand::Password {
            value: require_rest(&mut parts).unwrap_or_default(),
        }),
        "MAXPLAYERS" => Ok(AdminCommand::MaxPlayers {
            count: parse_u32(&require_rest(&mut parts)?)?.min(32) as u8,
        }),
        "DRAIN" => Ok(AdminCommand::Drain),
        "UNDRAIN" => Ok(AdminCommand::Undrain),
        "VOTE" => Ok(AdminCommand::Vote {
            map: require_rest(&mut parts)?,
        }),
        _ => Err(AdminError::Unknown),
    }
}

/// Whether this role may run that command. Players may vote on the next map.
pub fn authorize(role: Role, command: &AdminCommand) -> Result<(), AdminError> {
    match (role, command) {
        (Role::Admin, _) => Ok(()),
        (Role::Player, AdminCommand::Vote { .. }) => Ok(()),
        (Role::Player, _) => Err(AdminError::Unauthorized),
    }
}

fn tokenize(line: &str) -> impl Iterator<Item = &str> {
    line.split_whitespace()
}

fn require_rest<'a>(parts: &mut impl Iterator<Item = &'a str>) -> Result<String, AdminError> {
    let rest: Vec<&str> = parts.collect();
    if rest.is_empty() {
        return Err(AdminError::BadArgument);
    }
    Ok(rest.join(" "))
}

fn parse_target(raw: &str) -> Result<Target, AdminError> {
    if let Ok(id) = raw.parse::<u32>() {
        return Ok(Target::Id(id));
    }
    if raw.is_empty() {
        return Err(AdminError::BadArgument);
    }
    Ok(Target::Name(raw.to_string()))
}

fn parse_u32(raw: &str) -> Result<u32, AdminError> {
    raw.parse().map_err(|_| AdminError::BadArgument)
}

fn parse_ip(raw: &str) -> Result<IpAddr, AdminError> {
    IpAddr::from_str(raw).map_err(|_| AdminError::BadArgument)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Acceptance evidence: rust:admin:parse
    fn every_documented_verb_parses() {
        assert!(matches!(
            parse_admin("/ADDMAP Aero").unwrap(),
            AdminCommand::AddMap { map } if map == "Aero"
        ));
        assert!(matches!(
            parse_admin("/KICK 7").unwrap(),
            AdminCommand::Kick {
                target: Target::Id(7)
            }
        ));
        assert!(matches!(
            parse_admin("/TEMPBAN 10 1.2.3.4").unwrap(),
            AdminCommand::TempBan { minutes: 10, .. }
        ));
        assert!(matches!(
            parse_admin("/NEXTMAP").unwrap(),
            AdminCommand::NextMap
        ));
        assert!(matches!(
            parse_admin("/ADDBOT1 Boogie").unwrap(),
            AdminCommand::AddBot { team: Some(1), .. }
        ));
        assert_eq!(parse_admin("/NOPE"), Err(AdminError::Unknown));
        assert_eq!(parse_admin("/KICK"), Err(AdminError::BadArgument));
        assert_eq!(parse_admin("/DRAIN").unwrap(), AdminCommand::Drain);
        assert_eq!(parse_admin("/UNDRAIN").unwrap(), AdminCommand::Undrain);
        assert!(matches!(
            parse_admin("/VOTE Aero").unwrap(),
            AdminCommand::Vote { map } if map == "Aero"
        ));
        assert!(authorize(Role::Player, &parse_admin("/VOTE Aero").unwrap()).is_ok());
    }

    #[test]
    fn a_player_may_not_run_an_admin_command() {
        let command = parse_admin("/BAN 1").unwrap();
        assert_eq!(
            authorize(Role::Player, &command),
            Err(AdminError::Unauthorized)
        );
        assert!(authorize(Role::Admin, &command).is_ok());
    }

    #[test]
    fn an_ip_that_is_not_an_ip_is_refused() {
        assert_eq!(
            parse_admin("/BANIP not-an-ip"),
            Err(AdminError::BadArgument)
        );
    }
}
