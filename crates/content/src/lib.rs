#![forbid(unsafe_code)]

pub mod manifest;
pub mod pms;

pub(crate) fn is_safe_content_path(value: &str) -> bool {
    let normalized = value.replace('\\', "/");
    !normalized.starts_with('/')
        && !normalized.contains(':')
        && !normalized
            .split('/')
            .any(|segment| segment == "." || segment == ".." || segment.is_empty())
}
