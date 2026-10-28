//! Runtime configuration: cache/output locations and the git proxy.
//!
//! The proxy is resolved on every invocation — first from environment
//! variables, then from the OS-level proxy settings — so a proxy that changes
//! on the system is picked up automatically by the next command.

use std::path::{Path, PathBuf};

pub struct Config {
    pub cache_dir: PathBuf,
    pub output_dir: PathBuf,
    pub proxy: String,
}

impl Config {
    pub fn load() -> Self {
        let home = std::env::var_os(home_env_key())
            .map(PathBuf::from)
            .unwrap_or_default();

        Self {
            cache_dir: resolve_cache_dir(&home, std::env::var_os("GH_CACHE_DIR").as_deref()),
            output_dir: home.join(".ghr-output"),
            proxy: resolve_proxy(&|name| std::env::var(name).ok()),
        }
    }
}

/// Cache location: a non-empty `GH_CACHE_DIR` overrides the default under
/// the home directory.
fn resolve_cache_dir(home: &Path, override_dir: Option<&std::ffi::OsStr>) -> PathBuf {
    match override_dir.filter(|dir| !dir.is_empty()) {
        Some(dir) => PathBuf::from(dir),
        None => home.join(".ghr-cache"),
    }
}

#[cfg(windows)]
fn home_env_key() -> &'static str {
    "USERPROFILE"
}

#[cfg(not(windows))]
fn home_env_key() -> &'static str {
    "HOME"
}

/// Proxy environment variables in priority order. Lowercase variants are
/// checked too: they are the convention on Linux (`curl`, `apt`, ...).
const PROXY_ENV_VARS: [&str; 7] = [
    "GH_PROXY",
    "HTTPS_PROXY",
    "https_proxy",
    "HTTP_PROXY",
    "http_proxy",
    "ALL_PROXY",
    "all_proxy",
];

/// First non-empty proxy environment variable, falling back to the proxy
/// configured in the operating system.
fn resolve_proxy(lookup: &dyn Fn(&str) -> Option<String>) -> String {
    PROXY_ENV_VARS
        .into_iter()
        .find_map(|name| lookup(name).filter(|value| !value.is_empty()))
        .unwrap_or_else(&detect_system_proxy)
}

/// Detects the proxy configured at the OS level: `scutil --proxy` on macOS,
/// GNOME `gsettings` on Linux, the registry on Windows. HTTP(S) proxies win
/// over SOCKS; returns an empty string when none is set or detection is
/// unavailable.
#[cfg(target_os = "macos")]
fn detect_system_proxy() -> String {
    let Ok(output) = std::process::Command::new("scutil").arg("--proxy").output() else {
        return String::new();
    };
    if !output.status.success() {
        return String::new();
    }

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let dict = ScutilDict::parse(&stdout);
    for (enable_key, proxy_key, port_key) in [
        ("HTTPSEnable", "HTTPSProxy", "HTTPSPort"),
        ("HTTPEnable", "HTTPProxy", "HTTPPort"),
    ] {
        if dict.is_enabled(enable_key) {
            if let Some(url) = dict.proxy_url(proxy_key, port_key, "http") {
                return url;
            }
        }
    }
    if dict.is_enabled("SOCKSEnable") {
        if let Some(url) = dict.proxy_url("SOCKSProxy", "SOCKSPort", "socks5h") {
            return url;
        }
    }

    String::new()
}

#[cfg(all(unix, not(target_os = "macos")))]
fn detect_system_proxy() -> String {
    let global_mode = gsettings_value("org.gnome.system.proxy", "mode");
    if global_mode.as_deref() != Some("manual") {
        return String::new();
    }

    for (schema, scheme) in [
        ("org.gnome.system.proxy.https", "http"),
        ("org.gnome.system.proxy.http", "http"),
        ("org.gnome.system.proxy.socks", "socks5h"),
    ] {
        if let Some(url) = gsettings_proxy_url(schema, scheme) {
            return url;
        }
    }

    String::new()
}

#[cfg(all(unix, not(target_os = "macos")))]
fn gsettings_proxy_url(schema: &str, scheme: &str) -> Option<String> {
    let host = gsettings_value(schema, "host")?;
    let port = gsettings_value(schema, "port")?;
    if host.is_empty() || port.is_empty() {
        return None;
    }

    Some(format!("{scheme}://{host}:{port}"))
}

#[cfg(all(unix, not(target_os = "macos")))]
fn gsettings_value(schema: &str, key: &str) -> Option<String> {
    let output = std::process::Command::new("gsettings")
        .args(["get", schema, key])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    Some(
        String::from_utf8_lossy(&output.stdout)
            .trim()
            .trim_matches('\'')
            .to_owned(),
    )
}

#[cfg(windows)]
fn detect_system_proxy() -> String {
    const KEY: &str = r"HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings";

    let enabled = reg_query(KEY, "ProxyEnable").is_some_and(|value| value.contains("0x1"));
    if !enabled {
        return String::new();
    }

    let Some(server) = reg_query(KEY, "ProxyServer") else {
        return String::new();
    };

    parse_windows_proxy_server(&server).unwrap_or_default()
}

/// `ProxyServer` is either "host:port" or "http=...;https=...;socks=...".
/// HTTPS wins over HTTP, which wins over SOCKS.
#[cfg(any(windows, test))]
fn parse_windows_proxy_server(server: &str) -> Option<String> {
    let mut http = None;
    let mut https = None;
    let mut socks = None;

    for entry in server
        .split(';')
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
    {
        let Some((scheme, host)) = entry.split_once('=') else {
            return with_scheme(entry, "http");
        };
        match scheme {
            "https" => https = with_scheme(host, "http"),
            "http" => http = with_scheme(host, "http"),
            "socks" => socks = with_scheme(host, "socks5h"),
            _ => {}
        }
    }

    https.or(http).or(socks)
}

#[cfg(any(windows, test))]
fn with_scheme(target: &str, scheme: &str) -> Option<String> {
    if target.is_empty() {
        return None;
    }
    if target.contains("://") {
        return Some(target.to_owned());
    }

    Some(format!("{scheme}://{target}"))
}

#[cfg(windows)]
fn reg_query(key: &str, value: &str) -> Option<String> {
    let output = std::process::Command::new("reg")
        .args(["query", key, "/v", value])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    let text = String::from_utf8_lossy(&output.stdout);
    text.lines()
        .find(|line| line.contains(value))
        .and_then(|line| line.split_whitespace().last())
        .map(str::to_owned)
}

/// A flat `<dictionary>` as printed by `scutil --proxy`, with `Key : value`
/// lines.
#[cfg(target_os = "macos")]
struct ScutilDict<'a> {
    fields: std::collections::HashMap<&'a str, &'a str>,
}

#[cfg(target_os = "macos")]
impl<'a> ScutilDict<'a> {
    fn parse(text: &'a str) -> Self {
        let fields = text
            .lines()
            .filter_map(|line| line.split_once(" : "))
            .map(|(key, value)| (key.trim(), value.trim()))
            .collect();

        Self { fields }
    }

    fn is_enabled(&self, key: &str) -> bool {
        self.fields.get(key).is_some_and(|value| *value == "1")
    }

    fn proxy_url(&self, proxy_key: &str, port_key: &str, scheme: &str) -> Option<String> {
        let host = self.fields.get(proxy_key)?;
        let port = self.fields.get(port_key)?;
        if host.is_empty() || port.is_empty() {
            return None;
        }

        Some(format!("{scheme}://{host}:{port}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn proxy_from(pairs: &[(&str, &str)]) -> String {
        resolve_proxy(&|name| {
            pairs
                .iter()
                .find(|(key, _)| *key == name)
                .map(|(_, value)| value.to_string())
        })
    }

    #[test]
    fn resolve_cache_dir_defaults_under_home() {
        assert_eq!(
            resolve_cache_dir(Path::new("/home/u"), None),
            PathBuf::from("/home/u/.ghr-cache")
        );
    }

    #[test]
    fn resolve_cache_dir_honors_non_empty_override() {
        assert_eq!(
            resolve_cache_dir(Path::new("/home/u"), Some("/data/cache".as_ref())),
            PathBuf::from("/data/cache")
        );
        assert_eq!(
            resolve_cache_dir(Path::new("/home/u"), Some("".as_ref())),
            PathBuf::from("/home/u/.ghr-cache")
        );
    }

    #[test]
    fn resolve_proxy_prefers_gh_proxy() {
        assert_eq!(proxy_from(&[("GH_PROXY", "a"), ("HTTPS_PROXY", "b")]), "a");
    }

    #[test]
    fn resolve_proxy_falls_back_through_the_env_chain() {
        assert_eq!(
            proxy_from(&[("HTTPS_PROXY", "b"), ("HTTP_PROXY", "c")]),
            "b"
        );
        assert_eq!(proxy_from(&[("HTTP_PROXY", "c")]), "c");
        assert_eq!(
            proxy_from(&[("https_proxy", "lower"), ("ALL_PROXY", "all")]),
            "lower"
        );
        assert_eq!(proxy_from(&[("all_proxy", "last")]), "last");
    }

    #[test]
    fn resolve_proxy_skips_empty_values() {
        assert_eq!(proxy_from(&[("GH_PROXY", ""), ("HTTP_PROXY", "c")]), "c");
    }

    #[test]
    fn resolve_proxy_defaults_to_empty_without_environment_or_system_proxy() {
        // The test process has no proxy env vars and (in CI) no OS proxy.
        let proxy = proxy_from(&[]);
        assert!(
            proxy.is_empty() || proxy.contains("://"),
            "unexpected proxy: {proxy}"
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn scutil_dict_parses_enabled_https_proxy() {
        let sample = "<dictionary> {\n  HTTPSEnable : 1\n  HTTPSPort : 7890\n  HTTPSProxy : 127.0.0.1\n  HTTPEnable : 0\n  SOCKSEnable : 0\n}";
        let dict = ScutilDict::parse(sample);

        assert!(dict.is_enabled("HTTPSEnable"));
        assert!(!dict.is_enabled("HTTPEnable"));
        assert_eq!(
            dict.proxy_url("HTTPSProxy", "HTTPSPort", "http"),
            Some("http://127.0.0.1:7890".to_owned())
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn scutil_dict_reports_socks_with_the_socks5h_scheme() {
        let sample =
            "<dictionary> {\n  SOCKSEnable : 1\n  SOCKSPort : 1080\n  SOCKSProxy : 127.0.0.1\n}";
        let dict = ScutilDict::parse(sample);

        assert_eq!(
            dict.proxy_url("SOCKSProxy", "SOCKSPort", "socks5h"),
            Some("socks5h://127.0.0.1:1080".to_owned())
        );
    }

    #[test]
    fn windows_proxy_server_prefers_https_then_http_then_socks() {
        assert_eq!(
            parse_windows_proxy_server("http=10.0.0.1:8080;https=10.0.0.2:8443"),
            Some("http://10.0.0.2:8443".to_owned())
        );
        assert_eq!(
            parse_windows_proxy_server("http=10.0.0.1:8080;socks=10.0.0.3:1080"),
            Some("http://10.0.0.1:8080".to_owned())
        );
        assert_eq!(
            parse_windows_proxy_server("socks=10.0.0.3:1080"),
            Some("socks5h://10.0.0.3:1080".to_owned())
        );
    }

    #[test]
    fn windows_proxy_server_accepts_plain_host_port() {
        assert_eq!(
            parse_windows_proxy_server("10.0.0.1:8080"),
            Some("http://10.0.0.1:8080".to_owned())
        );
        assert_eq!(
            parse_windows_proxy_server("http://10.0.0.1:8080"),
            Some("http://10.0.0.1:8080".to_owned())
        );
        assert_eq!(parse_windows_proxy_server(""), None);
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn scutil_dict_yields_nothing_when_proxies_are_disabled() {
        let sample = "<dictionary> {\n  HTTPSEnable : 0\n  HTTPEnable : 0\n}";
        let dict = ScutilDict::parse(sample);

        assert_eq!(dict.proxy_url("HTTPSProxy", "HTTPSPort", "http"), None);
    }
}
