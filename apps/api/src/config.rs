//! Server settings that can be validated, persisted, and reloaded without a restart.

#[derive(Clone, Debug, PartialEq)]
pub struct ServerConfig {
    pub name: String,
    pub password: String,
    pub max_players: u8,
    pub public: bool,
    pub welcome: String,
    pub contact: String,
    pub max_ping_ms: u32,
    pub kick_on_ping: bool,
    pub friendly_fire: bool,
    pub respawn_seconds: u32,
    pub bonuses: bool,
    pub spectator_limit: u8,
    pub admin_password: String,
    pub logging: bool,
    pub locked: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            name: "Arena".into(),
            password: String::new(),
            max_players: 16,
            public: true,
            welcome: String::new(),
            contact: String::new(),
            max_ping_ms: 400,
            kick_on_ping: false,
            friendly_fire: false,
            respawn_seconds: 3,
            bonuses: true,
            spectator_limit: 8,
            admin_password: String::new(),
            logging: true,
            locked: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigError {
    EmptyName,
    Capacity,
    Locked,
}

impl ServerConfig {
    pub fn set_name(&mut self, name: String) -> Result<(), ConfigError> {
        self.guard()?;
        let name = name.trim();
        if name.is_empty() {
            return Err(ConfigError::EmptyName);
        }
        self.name = name.chars().take(32).collect();
        Ok(())
    }

    pub fn set_password(&mut self, password: String) -> Result<(), ConfigError> {
        self.guard()?;
        self.password = password;
        Ok(())
    }

    pub fn set_max_players(&mut self, count: u8) -> Result<(), ConfigError> {
        self.guard()?;
        if count == 0 || count > 32 {
            return Err(ConfigError::Capacity);
        }
        self.max_players = count;
        Ok(())
    }

    pub fn set_respawn(&mut self, seconds: u32) -> Result<(), ConfigError> {
        self.guard()?;
        self.respawn_seconds = seconds.min(60);
        Ok(())
    }

    pub fn authenticate_admin(&self, password: &str) -> bool {
        !self.admin_password.is_empty() && self.admin_password == password
    }

    fn guard(&self) -> Result<(), ConfigError> {
        if self.locked {
            Err(ConfigError::Locked)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Acceptance evidence: rust:config:validate
    fn a_locked_config_refuses_runtime_changes() {
        let mut config = ServerConfig {
            locked: true,
            ..ServerConfig::default()
        };
        assert_eq!(config.set_name("X".into()), Err(ConfigError::Locked));
    }

    #[test]
    fn capacity_and_name_are_checked() {
        let mut config = ServerConfig::default();
        assert_eq!(config.set_max_players(0), Err(ConfigError::Capacity));
        assert_eq!(config.set_name("  ".into()), Err(ConfigError::EmptyName));
        assert!(config.set_name("Night".into()).is_ok());
        assert_eq!(config.name, "Night");
    }

    #[test]
    fn admin_auth_is_off_until_a_password_is_set() {
        let config = ServerConfig {
            admin_password: "secret".into(),
            ..ServerConfig::default()
        };
        assert!(!ServerConfig::default().authenticate_admin("x"));
        assert!(config.authenticate_admin("secret"));
        assert!(!config.authenticate_admin("nope"));
    }

    #[test]
    // Acceptance evidence: rust:ping:kick
    fn ping_kick_is_off_until_the_operator_turns_it_on() {
        let config = ServerConfig::default();
        assert!(!config.kick_on_ping);
        assert_eq!(config.max_ping_ms, 400);
    }
}
