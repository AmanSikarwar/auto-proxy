use core::fmt;
use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum ProxyProtocol {
    Http,
    Https,
    Ftp,
    Socks,
}

impl ProxyProtocol {
    pub fn iter() -> impl Iterator<Item = &'static str> {
        ["http", "https", "ftp", "socks5"].iter().copied()
    }

    pub fn all<'a>() -> &'a [ProxyProtocol] {
        &[
            ProxyProtocol::Http,
            ProxyProtocol::Https,
            ProxyProtocol::Ftp,
            ProxyProtocol::Socks,
        ]
    }
}

impl fmt::Display for ProxyProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProxyProtocol::Http => write!(f, "http"),
            ProxyProtocol::Https => write!(f, "https"),
            ProxyProtocol::Ftp => write!(f, "ftp"),
            ProxyProtocol::Socks => write!(f, "socks5"),
        }
    }
}

impl Clone for ProxyProtocol {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for ProxyProtocol {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProxyAuth {
    pub username: String,
    pub password: String,
}

impl ProxyAuth {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

impl Clone for ProxyAuth {
    fn clone(&self) -> Self {
        ProxyAuth {
            username: self.username.clone(),
            password: self.password.clone(),
        }
    }
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct ProxySettings {
    pub host: String,
    pub port: String,
    pub auth: Option<ProxyAuth>,
    pub protocols: Vec<ProxyProtocol>,
    pub no_proxy: Vec<String>,
}

impl ProxySettings {
    pub fn new(
        host: String,
        port: String,
        auth: Option<ProxyAuth>,
        protocols: Vec<ProxyProtocol>,
        no_proxy: Vec<String>,
    ) -> Self {
        Self {
            host,
            port,
            auth,
            protocols,
            no_proxy,
        }
    }
}

impl fmt::Display for ProxySettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut proxy = String::new();
        if self.auth.is_some() {
            for protocol in &self.protocols {
                match protocol {
                    ProxyProtocol::Http => {
                        proxy.push_str(&format!(
                            "http://{}:{}@{}:{}\n",
                            self.auth.as_ref().unwrap().username,
                            self.auth.as_ref().unwrap().password,
                            self.host,
                            self.port
                        ));
                    }
                    ProxyProtocol::Https => {
                        proxy.push_str(&format!(
                            "https://{}:{}@{}:{}\n",
                            self.auth.as_ref().unwrap().username,
                            self.auth.as_ref().unwrap().password,
                            self.host,
                            self.port
                        ));
                    }
                    ProxyProtocol::Ftp => {
                        proxy.push_str(&format!(
                            "ftp://{}:{}@{}:{}\n",
                            self.auth.as_ref().unwrap().username,
                            self.auth.as_ref().unwrap().password,
                            self.host,
                            self.port
                        ));
                    }
                    ProxyProtocol::Socks => {
                        proxy.push_str(&format!(
                            "socks://{}:{}@{}:{}\n",
                            self.auth.as_ref().unwrap().username,
                            self.auth.as_ref().unwrap().password,
                            self.host,
                            self.port
                        ));
                    }
                }
            }
        } else {
            for protocol in &self.protocols {
                match protocol {
                    ProxyProtocol::Http => {
                        proxy.push_str(&format!("http://{}:{}\n", self.host, self.port));
                    }
                    ProxyProtocol::Https => {
                        proxy.push_str(&format!("https://{}:{}\n", self.host, self.port));
                    }
                    ProxyProtocol::Ftp => {
                        proxy.push_str(&format!("ftp://{}:{}\n", self.host, self.port));
                    }
                    ProxyProtocol::Socks => {
                        proxy.push_str(&format!("socks://{}:{}\n", self.host, self.port));
                    }
                }
            }
        }
        proxy.push_str(&format!("no_proxy={}", self.no_proxy.join(",")));
        write!(f, "{}", proxy)
    }
}

pub trait Proxy {
    fn new(host: String, port: String, auth: Option<ProxyAuth>) -> Self
    where
        Self: Sized;
    fn protocol(&self) -> ProxyProtocol;
    fn host(&self) -> &str;
    fn port(&self) -> &str;
    fn auth(&self) -> Option<&ProxyAuth>;
    fn to_string(&self) -> String {
        match self.auth() {
            Some(auth) => self.to_string_with_auth(auth),
            None => self.to_string_without_auth(),
        }
    }
    fn to_string_with_auth(&self, auth: &ProxyAuth) -> String;
    fn to_string_without_auth(&self) -> String;
    fn from_string(proxy: &str) -> Result<Box<dyn Proxy>, Box<dyn Error>>
    where
        Self: Sized;
}

impl fmt::Display for dyn Proxy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub struct GeneralProxy {
    pub host: String,
    pub port: String,
    pub auth: Option<ProxyAuth>,
    pub protocol: ProxyProtocol,
}

impl Proxy for GeneralProxy {
    fn new(host: String, port: String, auth: Option<ProxyAuth>) -> Self
    where
        Self: Sized,
    {
        Self {
            host,
            port,
            auth,
            protocol: ProxyProtocol::Http,
        }
    }

    fn protocol(&self) -> ProxyProtocol {
        self.protocol
    }

    fn host(&self) -> &str {
        &self.host
    }

    fn port(&self) -> &str {
        &self.port
    }

    fn auth(&self) -> Option<&ProxyAuth> {
        self.auth.as_ref()
    }

    fn to_string_with_auth(&self, auth: &ProxyAuth) -> String {
        format!(
            "{}://{}:{}@{}:{}",
            self.protocol, auth.username, auth.password, self.host, self.port
        )
    }

    fn to_string_without_auth(&self) -> String {
        format!("{}://{}:{}", self.protocol, self.host, self.port)
    }

    fn from_string(proxy: &str) -> Result<Box<dyn Proxy>, Box<dyn Error>>
    where
        Self: Sized,
    {
        let url = url::Url::parse(proxy)?;
        let host = url.host_str().unwrap().to_string();
        let port = url.port().unwrap().to_string();
        let auth = match (url.username(), url.password()) {
            (username, Some(password)) => {
                Some(ProxyAuth::new(username.to_string(), password.to_string()))
            }
            _ => None,
        };

        Ok(Box::new(GeneralProxy {
            host,
            port,
            auth,
            protocol: ProxyProtocol::Http,
        }))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HttpProxy {
    pub host: String,
    pub port: String,
    pub auth: Option<ProxyAuth>,
}

impl Proxy for HttpProxy {
    fn new(host: String, port: String, auth: Option<ProxyAuth>) -> Self
    where
        Self: Sized,
    {
        Self { host, port, auth }
    }

    fn protocol(&self) -> ProxyProtocol {
        ProxyProtocol::Http
    }

    fn host(&self) -> &str {
        &self.host
    }

    fn port(&self) -> &str {
        &self.port
    }

    fn auth(&self) -> Option<&ProxyAuth> {
        self.auth.as_ref()
    }

    fn to_string_with_auth(&self, auth: &ProxyAuth) -> String {
        format!(
            "http://{}:{}@{}:{}",
            auth.username, auth.password, self.host, self.port
        )
    }

    fn to_string_without_auth(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }

    fn from_string(proxy: &str) -> Result<Box<dyn Proxy>, Box<dyn Error>>
    where
        Self: Sized,
    {
        let url = url::Url::parse(proxy)?;
        let host = url.host_str().unwrap().to_string();
        let port = url.port().unwrap().to_string();
        let auth = match (url.username(), url.password()) {
            (username, Some(password)) => {
                Some(ProxyAuth::new(username.to_string(), password.to_string()))
            }
            _ => None,
        };

        Ok(Box::new(HttpProxy { host, port, auth }))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HttpsProxy {
    pub host: String,
    pub port: String,
    pub auth: Option<ProxyAuth>,
}

impl Proxy for HttpsProxy {
    fn new(host: String, port: String, auth: Option<ProxyAuth>) -> Self
    where
        Self: Sized,
    {
        Self { host, port, auth }
    }

    fn protocol(&self) -> ProxyProtocol {
        ProxyProtocol::Https
    }

    fn host(&self) -> &str {
        &self.host
    }

    fn port(&self) -> &str {
        &self.port
    }

    fn auth(&self) -> Option<&ProxyAuth> {
        self.auth.as_ref()
    }

    fn to_string_with_auth(&self, auth: &ProxyAuth) -> String {
        format!(
            "https://{}:{}@{}:{}",
            auth.username, auth.password, self.host, self.port
        )
    }

    fn to_string_without_auth(&self) -> String {
        format!("https://{}:{}", self.host, self.port)
    }

    fn from_string(proxy: &str) -> Result<Box<dyn Proxy>, Box<dyn Error>>
    where
        Self: Sized,
    {
        let url = url::Url::parse(proxy)?;
        let host = url.host_str().unwrap().to_string();
        let port = url.port().unwrap().to_string();
        let auth = match (url.username(), url.password()) {
            (username, Some(password)) => {
                Some(ProxyAuth::new(username.to_string(), password.to_string()))
            }
            _ => None,
        };

        Ok(Box::new(HttpsProxy { host, port, auth }))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FtpProxy {
    pub host: String,
    pub port: String,
    pub auth: Option<ProxyAuth>,
}

impl Proxy for FtpProxy {
    fn new(host: String, port: String, auth: Option<ProxyAuth>) -> Self
    where
        Self: Sized,
    {
        Self { host, port, auth }
    }

    fn protocol(&self) -> ProxyProtocol {
        ProxyProtocol::Ftp
    }

    fn host(&self) -> &str {
        &self.host
    }

    fn port(&self) -> &str {
        &self.port
    }

    fn auth(&self) -> Option<&ProxyAuth> {
        self.auth.as_ref()
    }

    fn to_string_with_auth(&self, auth: &ProxyAuth) -> String {
        format!(
            "ftp://{}:{}@{}:{}",
            auth.username, auth.password, self.host, self.port
        )
    }

    fn to_string_without_auth(&self) -> String {
        format!("ftp://{}:{}", self.host, self.port)
    }

    fn from_string(proxy: &str) -> Result<Box<dyn Proxy>, Box<dyn Error>>
    where
        Self: Sized,
    {
        let url = url::Url::parse(proxy)?;
        let host = url.host_str().unwrap().to_string();
        let port = url.port().unwrap().to_string();
        let auth = match (url.username(), url.password()) {
            (username, Some(password)) => {
                Some(ProxyAuth::new(username.to_string(), password.to_string()))
            }
            _ => None,
        };

        Ok(Box::new(FtpProxy { host, port, auth }))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SocksProxy {
    pub host: String,
    pub port: String,
    pub auth: Option<ProxyAuth>,
}

impl Proxy for SocksProxy {
    fn new(host: String, port: String, auth: Option<ProxyAuth>) -> Self
    where
        Self: Sized,
    {
        Self { host, port, auth }
    }

    fn protocol(&self) -> ProxyProtocol {
        ProxyProtocol::Socks
    }

    fn host(&self) -> &str {
        &self.host
    }

    fn port(&self) -> &str {
        &self.port
    }

    fn auth(&self) -> Option<&ProxyAuth> {
        self.auth.as_ref()
    }

    fn to_string_with_auth(&self, auth: &ProxyAuth) -> String {
        format!(
            "socks://{}:{}@{}:{}",
            auth.username, auth.password, self.host, self.port
        )
    }

    fn to_string_without_auth(&self) -> String {
        format!("socks://{}:{}", self.host, self.port)
    }

    fn from_string(proxy: &str) -> Result<Box<dyn Proxy>, Box<dyn Error>>
    where
        Self: Sized,
    {
        let url = url::Url::parse(proxy)?;
        let host = url.host_str().unwrap().to_string();
        let port = url.port().unwrap().to_string();
        let auth = match (url.username(), url.password()) {
            (username, Some(password)) => {
                Some(ProxyAuth::new(username.to_string(), password.to_string()))
            }
            _ => None,
        };

        Ok(Box::new(SocksProxy { host, port, auth }))
    }
}
