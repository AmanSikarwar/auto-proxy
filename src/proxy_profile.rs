use crate::proxy::ProxySettings;
use config::{Config, File};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File as FsFile;
use std::io::Write;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProxyProfile {
    pub name: String,
    pub proxy_settings: ProxySettings,
    pub auto_apply_networks: Vec<String>,
}

impl ProxyProfile {
    pub fn new(
        name: String,
        proxy_settings: ProxySettings,
        auto_apply_networks: Vec<String>,
    ) -> Self {
        Self {
            name,
            proxy_settings,
            auto_apply_networks,
        }
    }

    pub fn create_profile(self) -> Result<(), Box<dyn Error>> {
        let profile_dir = dirs::home_dir().unwrap().join(".auto-proxy/profiles");

        let profile_file = profile_dir.join(format!("{}.yaml", self.name));

        let mut file = FsFile::create(profile_file)?;

        let profile_content = serde_yaml::to_string(&self)?;

        file.write_all(profile_content.as_bytes())?;
        print!("Done!");

        Ok(())
    }

    pub fn from_file(profile_file: &str) -> Result<Self, Box<dyn Error>> {
        let config = Config::builder()
            .add_source(File::with_name(profile_file))
            .build()?;

        let name = config.get_string("name")?;
        let proxy_settings = config.get::<ProxySettings>("proxy_settings")?;
        let auto_apply_networks = config.get::<Vec<String>>("auto_apply_networks")?;

        Ok(Self::new(name, proxy_settings, auto_apply_networks))
    }

    pub fn from_string(profile_content: &str) -> Result<Self, Box<dyn Error>> {
        let profile: Self = serde_yaml::from_str(profile_content)?;
        Ok(profile)
    }

    pub fn to_string(&self) -> Result<String, Box<dyn Error>> {
        let profile_content = serde_yaml::to_string(&self)?;
        Ok(profile_content)
    }

    pub fn to_file(&self, profile_file: &str) -> Result<(), Box<dyn Error>> {
        let mut file = FsFile::create(profile_file)?;

        let profile_content = serde_yaml::to_string(&self)?;

        file.write_all(profile_content.as_bytes())?;

        Ok(())
    }

    pub fn update_profile(&self) -> Result<(), Box<dyn Error>> {
        let profile_dir = dirs::home_dir().unwrap().join(".auto-proxy/profiles");

        let profile_file = profile_dir.join(format!("{}.yaml", self.name));

        let mut file = FsFile::create(profile_file)?;

        let profile_content = serde_yaml::to_string(&self)?;

        file.write_all(profile_content.as_bytes())?;

        Ok(())
    }

    pub fn delete_profile(&self) -> Result<(), Box<dyn Error>> {
        let profile_dir = dirs::home_dir().unwrap().join(".auto-proxy/profiles");

        let profile_file = profile_dir.join(format!("{}.yaml", self.name));

        std::fs::remove_file(profile_file)?;

        Ok(())
    }

    pub fn list_profiles() -> Result<(), Box<dyn Error>> {
        let profile_dir = dirs::home_dir().unwrap().join(".auto-proxy/profiles");

        let profile_files = std::fs::read_dir(profile_dir)?;

        for profile_file in profile_files {
            let profile_file = profile_file?;
            let profile_file_name = profile_file.file_name();
            let profile_file_name = profile_file_name.to_str().unwrap();
            let profile = Self::from_file(profile_file_name)?;
            println!("{}", profile.name);
        }

        Ok(())
    }

    pub fn get_profile(profile_name: &str) -> Result<Self, Box<dyn Error>> {
        let profile_dir = dirs::home_dir().unwrap().join(".auto-proxy/profiles");

        let profile_file = profile_dir.join(format!("{}.yaml", profile_name));

        let profile = Self::from_file(profile_file.to_str().unwrap())?;

        Ok(profile)
    }
}

#[cfg(test)]
mod tests {
    use crate::proxy::{ProxyAuth, ProxyProtocol, ProxySettings};

    use super::*;
    use std::fs::File;

    #[test]
    fn test_proxy_profile() {
        let profile = ProxyProfile::new(
            "test".to_string(),
            ProxySettings::new(
                "proxy.example.com".to_string(),
                "8080".to_string(),
                Some(ProxyAuth::new("user".to_string(), "pass".to_string())),
                vec![ProxyProtocol::Http, ProxyProtocol::Https],
                vec![
                    "localhost.com".to_string(),
                    "127.0.0.1".to_string(),
                    "192.168.1.1".to_string(),
                ],
            ),
            vec!["SSID1".to_string(), "SSID2".to_string()],
        );

        let profile_content = profile.to_string().unwrap();

        let mut file = File::create("test.yaml").unwrap();

        file.write_all(profile_content.as_bytes()).unwrap();

        let profile = ProxyProfile::from_file("test.yaml").unwrap();

        assert_eq!(profile.name, "test");
        assert_eq!(profile.auto_apply_networks.len(), 2);
        assert_eq!(profile.auto_apply_networks[0], "SSID1");
        assert_eq!(profile.auto_apply_networks[1], "SSID2");
        assert_eq!(profile.proxy_settings.host, "proxy.example.com");
        assert_eq!(profile.proxy_settings.port, "8080");
        if let Some(auth) = profile.proxy_settings.auth {
            assert_eq!(auth.username, "user");
            assert_eq!(auth.password, "pass");
        } else {
            panic!("auth is None");
        }
        assert_eq!(profile.proxy_settings.protocols.len(), 2);
        assert_eq!(profile.proxy_settings.protocols[0], ProxyProtocol::Http);
        assert_eq!(profile.proxy_settings.protocols[1], ProxyProtocol::Https);
        assert_eq!(profile.proxy_settings.no_proxy.len(), 3);
        assert_eq!(profile.proxy_settings.no_proxy[0], "localhost.com");
        assert_eq!(profile.proxy_settings.no_proxy[1], "127.0.0.1");
        assert_eq!(profile.proxy_settings.no_proxy[2], "192.168.1.1");

        std::fs::remove_file("test.yaml").unwrap();

        let profile = ProxyProfile::from_string(&profile_content).unwrap();

        assert_eq!(profile.name, "test");
        assert_eq!(profile.auto_apply_networks.len(), 2);
        assert_eq!(profile.auto_apply_networks[0], "SSID1");
        assert_eq!(profile.auto_apply_networks[1], "SSID2");
        assert_eq!(profile.proxy_settings.host, "proxy.example.com");
        assert_eq!(profile.proxy_settings.port, "8080");
        if let Some(auth) = profile.proxy_settings.auth {
            assert_eq!(auth.username, "user");
            assert_eq!(auth.password, "pass");
        } else {
            panic!("auth is None");
        }
        assert_eq!(profile.proxy_settings.protocols.len(), 2);
        assert_eq!(profile.proxy_settings.protocols[0], ProxyProtocol::Http);
        assert_eq!(profile.proxy_settings.protocols[1], ProxyProtocol::Https);
        assert_eq!(profile.proxy_settings.no_proxy.len(), 3);
        assert_eq!(profile.proxy_settings.no_proxy[0], "localhost.com");
        assert_eq!(profile.proxy_settings.no_proxy[1], "127.0.0.1");
        assert_eq!(profile.proxy_settings.no_proxy[2], "192.168.1.1");

        let profile = ProxyProfile::new(
            "test".to_string(),
            ProxySettings::new(
                "proxy.example.com".to_string(),
                "8080".to_string(),
                Some(ProxyAuth::new("user".to_string(), "pass".to_string())),
                vec![ProxyProtocol::Http, ProxyProtocol::Https],
                vec![
                    "localhost.com".to_string(),
                    "127.0.0.1".to_string(),
                    "192.168.1.1".to_string(),
                ],
            ),
            vec!["SSID1".to_string(), "SSID2".to_string()],
        );

        let profile_content = profile.to_string().unwrap();

        let mut file = File::create("test.yaml").unwrap();

        file.write_all(profile_content.as_bytes()).unwrap();

        let profile = ProxyProfile::from_file("test.yaml").unwrap();

        assert_eq!(profile.name, "test");
        assert_eq!(profile.auto_apply_networks.len(), 2);
        assert_eq!(profile.auto_apply_networks[0], "SSID1");
        assert_eq!(profile.auto_apply_networks[1], "SSID2");
        assert_eq!(profile.proxy_settings.host, "proxy.example.com");
        assert_eq!(profile.proxy_settings.port, "8080");
        if let Some(auth) = profile.proxy_settings.auth {
            assert_eq!(auth.username, "user");
            assert_eq!(auth.password, "pass");
        } else {
            panic!("auth is None");
        }
        assert_eq!(profile.proxy_settings.protocols.len(), 2);
        assert_eq!(profile.proxy_settings.protocols[0], ProxyProtocol::Http);
        assert_eq!(profile.proxy_settings.protocols[1], ProxyProtocol::Https);
        assert_eq!(profile.proxy_settings.no_proxy.len(), 3);
        assert_eq!(profile.proxy_settings.no_proxy[0], "localhost.com");
        assert_eq!(profile.proxy_settings.no_proxy[1], "127.0.0.1");
        assert_eq!(profile.proxy_settings.no_proxy[2], "192.168.1.1");
    }
}

// impl ProxyProfile {
//     fn new(
//         name: String,
//         http_proxy: Option<HttpProxy>,
//         https_proxy: Option<HttpsProxy>,
//         ftp_proxy: Option<FtpProxy>,
//         socks_proxy: Option<SocksProxy>,
//         no_proxy: String,
//         auto_apply_networks: Vec<String>,
//     ) -> Self {
//         Self {
//             name,
//             http_proxy,
//             https_proxy,
//             ftp_proxy,
//             socks_proxy,
//             no_proxy,
//             auto_apply_networks,
//         }
//     }

//     fn create_profile(self) -> Result<(), Box<dyn Error>> {
//         let profile_dir = dirs::home_dir().unwrap().join(".auto-proxy/profiles");

//         let profile_file = profile_dir.join(format!("{}.yaml", self.name));

//         let mut file = FsFile::create(profile_file)?;

//         let profile_content = serde_yaml::to_string(&self)?;

//         file.write_all(profile_content.as_bytes())?;

//         Ok(())
//     }

//     fn from_file(profile_file: &str) -> Result<Self, Box<dyn Error>> {
//         let config = Config::builder()
//             .add_source(File::with_name(profile_file))
//             .build()?;

//         let name = config.get_string("name")?;
//         let http_proxy = config.get::<Option<HttpProxy>>("http_proxy")?;
//         let https_proxy = config.get::<Option<HttpsProxy>>("https_proxy")?;
//         let ftp_proxy = config.get::<Option<FtpProxy>>("ftp_proxy")?;
//         let socks_proxy = config.get::<Option<SocksProxy>>("socks_proxy")?;
//         let no_proxy = config.get_string("no_proxy")?;
//         let auto_apply_networks = config.get::<Vec<String>>("auto_apply_networks")?;

//         Ok(Self::new(
//             name,
//             http_proxy,
//             https_proxy,
//             ftp_proxy,
//             socks_proxy,
//             no_proxy,
//             auto_apply_networks,
//         ))
//     }

//     fn from_string(profile_content: &str) -> Result<Self, Box<dyn Error>> {
//         let profile: Self = serde_yaml::from_str(profile_content)?;
//         Ok(profile)
//     }

//     fn to_string(&self) -> Result<String, Box<dyn Error>> {
//         let profile_content = serde_yaml::to_string(&self)?;
//         Ok(profile_content)
//     }

//     fn to_file(&self, profile_file: &str) -> Result<(), Box<dyn Error>> {
//         let mut file = FsFile::create(profile_file)?;

//         let profile_content = serde_yaml::to_string(&self)?;

//         file.write_all(profile_content.as_bytes())?;

//         Ok(())
//     }

//     fn update_profile(&self) -> Result<(), Box<dyn Error>> {
//         let profile_dir = dirs::home_dir().unwrap().join(".auto-proxy/profiles");

//         let profile_file = profile_dir.join(format!("{}.yaml", self.name));

//         let mut file = FsFile::create(profile_file)?;

//         let profile_content = serde_yaml::to_string(&self)?;

//         file.write_all(profile_content.as_bytes())?;

//         Ok(())
//     }

//     fn delete_profile(&self) -> Result<(), Box<dyn Error>> {
//         let profile_dir = dirs::home_dir().unwrap().join(".auto-proxy/profiles");

//         let profile_file = profile_dir.join(format!("{}.yaml", self.name));

//         std::fs::remove_file(profile_file)?;

//         Ok(())
//     }

//     fn list_profiles() -> Result<(), Box<dyn Error>> {
//         let profile_dir = dirs::home_dir().unwrap().join(".auto-proxy/profiles");

//         let profile_files = std::fs::read_dir(profile_dir)?;

//         for profile_file in profile_files {
//             let profile_file = profile_file?;
//             let profile_file_name = profile_file.file_name();
//             let profile_file_name = profile_file_name.to_str().unwrap();
//             let profile = Self::from_file(profile_file_name)?;
//             println!("{}", profile.name);
//         }

//         Ok(())
//     }

//     fn get_profile(profile_name: &str) -> Result<Self, Box<dyn Error>> {
//         let profile_dir = dirs::home_dir().unwrap().join(".auto-proxy/profiles");

//         let profile_file = profile_dir.join(format!("{}.yaml", profile_name));

//         let profile = Self::from_file(profile_file.to_str().unwrap())?;

//         Ok(profile)
//     }
// }

// // code to test the above snippet
// #[cfg(test)]
// mod tests {
//     use crate::proxy::{Proxy, ProxyAuth};

//     use super::*;
//     use std::fs::File;

//     #[test]
//     fn test_proxy_profile() {
//         let profile = ProxyProfile::new(
//             "test".to_string(),
//             Some(HttpProxy::new(
//                 "proxy.example.com".to_string(),
//                 "8080".to_string(),
//                 Some(ProxyAuth::new("user".to_string(), "pass".to_string())),
//             )),
//             Some(HttpsProxy::new(
//                 "proxy.example.com".to_string(),
//                 "8080".to_string(),
//                 Some(ProxyAuth::new("user".to_string(), "pass".to_string())),
//             )),
//             Some(FtpProxy::new(
//                 "proxy.example.com".to_string(),
//                 "8080".to_string(),
//                 Some(ProxyAuth::new("user".to_string(), "pass".to_string())),
//             )),
//             Some(SocksProxy::new(
//                 "proxy.example.com".to_string(),
//                 "8080".to_string(),
//                 Some(ProxyAuth::new("user".to_string(), "pass".to_string())),
//             )),
//             "localhost.com".to_string(),
//             vec!["SSID1".to_string(), "SSID2".to_string()],
//         );

//         let profile_content = profile.to_string().unwrap();

//         let mut file = File::create("test.yaml").unwrap();

//         file.write_all(profile_content.as_bytes()).unwrap();

//         let profile = ProxyProfile::from_file("test.yaml").unwrap();

//         assert_eq!(profile.name, "test");
//         assert_eq!(profile.no_proxy, "localhost.com");
//         assert_eq!(profile.auto_apply_networks.len(), 2);
//         assert_eq!(profile.auto_apply_networks[0], "SSID1");
//         assert_eq!(profile.auto_apply_networks[1], "SSID2");
//         if let Some(http_proxy) = profile.http_proxy {
//             assert_eq!(http_proxy.host, "proxy.example.com");
//             assert_eq!(http_proxy.port, "8080");
//             if let Some(auth) = http_proxy.auth {
//                 assert_eq!(auth.username, "user");
//                 assert_eq!(auth.password, "pass");
//             } else {
//                 panic!("http_proxy.auth is None");
//             }
//         } else {
//             panic!("http_proxy is None");
//         }
//         if let Some(https_proxy) = profile.https_proxy {
//             assert_eq!(https_proxy.host, "proxy.example.com");
//             assert_eq!(https_proxy.port, "8080");
//             if let Some(auth) = https_proxy.auth {
//                 assert_eq!(auth.username, "user");
//                 assert_eq!(auth.password, "pass");
//             } else {
//                 panic!("https_proxy.auth is None");
//             }
//         } else {
//             panic!("https_proxy is None");
//         }
//         if let Some(ftp_proxy) = profile.ftp_proxy {
//             assert_eq!(ftp_proxy.host, "proxy.example.com");
//             assert_eq!(ftp_proxy.port, "8080");
//             if let Some(auth) = ftp_proxy.auth {
//                 assert_eq!(auth.username, "user");
//                 assert_eq!(auth.password, "pass");
//             } else {
//                 panic!("ftp_proxy.auth is None");
//             }
//         } else {
//             panic!("ftp_proxy is None");
//         }
//         if let Some(socks_proxy) = profile.socks_proxy {
//             assert_eq!(socks_proxy.host, "proxy.example.com");
//             assert_eq!(socks_proxy.port, "8080");
//             if let Some(auth) = socks_proxy.auth {
//                 assert_eq!(auth.username, "user");
//                 assert_eq!(auth.password, "pass");
//             } else {
//                 panic!("socks_proxy.auth is None");
//             }
//         } else {
//             panic!("socks_proxy is None");
//         }

//         std::fs::remove_file("test.yaml").unwrap();

//         let profile = ProxyProfile::from_string(&profile_content).unwrap();

//         assert_eq!(profile.name, "test");
//         assert_eq!(profile.no_proxy, "localhost.com");
//         assert_eq!(profile.auto_apply_networks.len(), 2);
//         assert_eq!(profile.auto_apply_networks[0], "SSID1");
//         assert_eq!(profile.auto_apply_networks[1], "SSID2");
//         if let Some(http_proxy) = profile.http_proxy {
//             assert_eq!(http_proxy.host, "proxy.example.com");
//             assert_eq!(http_proxy.port, "8080");
//             if let Some(auth) = http_proxy.auth {
//                 assert_eq!(auth.username, "user");
//                 assert_eq!(auth.password, "pass");
//             } else {
//                 panic!("http_proxy.auth is None");
//             }
//         } else {
//             panic!("http_proxy is None");
//         }
//         if let Some(https_proxy) = profile.https_proxy {
//             assert_eq!(https_proxy.host, "proxy.example.com");
//             assert_eq!(https_proxy.port, "8080");
//             if let Some(auth) = https_proxy.auth {
//                 assert_eq!(auth.username, "user");
//                 assert_eq!(auth.password, "pass");
//             } else {
//                 panic!("https_proxy.auth is None");
//             }
//         } else {
//             panic!("https_proxy is None");
//         }
//         if let Some(ftp_proxy) = profile.ftp_proxy {
//             assert_eq!(ftp_proxy.host, "proxy.example.com");
//             assert_eq!(ftp_proxy.port, "8080");
//             if let Some(auth) = ftp_proxy.auth {
//                 assert_eq!(auth.username, "user");
//                 assert_eq!(auth.password, "pass");
//             } else {
//                 panic!("ftp_proxy.auth is None");
//             }
//         } else {
//             panic!("ftp_proxy is None");
//         }
//         if let Some(socks_proxy) = profile.socks_proxy {
//             assert_eq!(socks_proxy.host, "proxy.example.com");
//             assert_eq!(socks_proxy.port, "8080");
//             if let Some(auth) = socks_proxy.auth {
//                 assert_eq!(auth.username, "user");
//                 assert_eq!(auth.password, "pass");
//             } else {
//                 panic!("socks_proxy.auth is None");
//             }
//         } else {
//             panic!("socks_proxy is None");
//         }
//     }
// }
