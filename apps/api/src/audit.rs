//! Append-only, redacted audit records. A password never lands in the log.

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuditRecord {
    pub at: u64,
    pub actor: String,
    pub action: String,
    pub detail: String,
}

#[derive(Clone, Debug, Default)]
pub struct AuditLog {
    records: Vec<AuditRecord>,
}

impl AuditLog {
    pub fn record(
        &mut self,
        actor: impl Into<String>,
        action: impl Into<String>,
        detail: impl Into<String>,
    ) {
        self.records.push(AuditRecord {
            at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            actor: actor.into(),
            action: action.into(),
            detail: redact(&detail.into()),
        });
    }

    pub fn last(&self) -> Option<&AuditRecord> {
        self.records.last()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }
}

fn redact(detail: &str) -> String {
    let mut out = detail.to_string();
    for key in ["password=", "password:", "token="] {
        if let Some(start) = out.to_ascii_lowercase().find(key) {
            let after = start + key.len();
            let end = out[after..]
                .find(|c: char| c.is_whitespace() || c == ',' || c == ';')
                .map_or(out.len(), |i| after + i);
            out.replace_range(after..end, "***");
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_password_is_redacted_in_the_log() {
        let mut log = AuditLog::default();
        log.record("1", "password", "password=secret123 kept");
        assert!(log.last().unwrap().detail.contains("***"));
        assert!(!log.last().unwrap().detail.contains("secret123"));
    }
}
