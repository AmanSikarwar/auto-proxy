use crate::{proxy::ProxySettings, target::ProxyTarget};

use shellexpand;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};

pub struct Gradle;

impl Gradle {
    const GRADLE_PROPERTIES_FILE: &'static str = "~/.gradle/gradle.properties";

    fn generate_gradle_proxy_content(settings: &[&ProxySettings]) -> String {
        let mut content = String::new();
        for setting in settings {
            if !setting.host.is_empty() && !setting.port.is_empty() {
                if let Some(auth) = &setting.auth {
                    content.push_str(&format!(
                        "systemProp.http.proxyHost={}\n\
                         systemProp.http.proxyPort={}\n\
                         systemProp.https.proxyHost={}\n\
                         systemProp.https.proxyPort={}\n",
                        setting.host, setting.port, setting.host, setting.port
                    ));

                    if !auth.username.is_empty() && !auth.password.is_empty() {
                        content.push_str(&format!(
                            "systemProp.http.proxyUser={}\n\
                             systemProp.http.proxyPassword={}\n\
                             systemProp.https.proxyUser={}\n\
                             systemProp.https.proxyPassword={}\n",
                            auth.username, auth.password, auth.username, auth.password
                        ));
                    }
                }
            }
        }
        content
    }
}

impl ProxyTarget for Gradle {
    fn get(&self) -> Option<Vec<ProxySettings>> {
        let gradle_properties_file =
            shellexpand::tilde(Gradle::GRADLE_PROPERTIES_FILE).into_owned();
        let file = match std::fs::File::open(gradle_properties_file) {
            Ok(file) => file,
            Err(_) => return None,
        };

        let reader = BufReader::new(file);
        let mut proxy_settings = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();
            let trimmed_line = line.trim();
            if trimmed_line.starts_with("systemProp.http.proxyHost=") {
                let host = trimmed_line
                    .split('=')
                    .nth(1)
                    .map(|s| s.to_string())
                    .unwrap_or_default();

                let port_line = reader
                    .lines()
                    .next()
                    .unwrap_or_else(|| Ok("systemProp.http.proxyPort=".to_string()))
                    .unwrap_or_default();
                let port = port_line
                    .trim()
                    .split('=')
                    .nth(1)
                    .map(|s| s.to_string())
                    .unwrap_or_default();

                let mut auth = None;
                let username_line = reader
                    .lines()
                    .next()
                    .unwrap_or_else(|| Ok("systemProp.http.proxyUser=".to_string()))
                    .unwrap_or_default();
                let username = username_line
                    .trim()
                    .split('=')
                    .nth(1)
                    .map(|s| s.to_string())
                    .unwrap_or_default();

                let password_line = reader
                    .lines()
                    .next()
                    .unwrap_or_else(|| Ok("systemProp.http.proxyPassword=".to_string()))
                    .unwrap_or_default();
                let password = password_line
                    .trim()
                    .split('=')
                    .nth(1)
                    .map(|s| s.to_string())
                    .unwrap_or_default();

                if !host.is_empty() && !port.is_empty() {
                    let setting = ProxySettings {
                        host,
                        port,
                        auth: if !username.is_empty() && !password.is_empty() {
                            auth = Some(ProxyAuth { username, password });
                            Some(auth.as_ref().unwrap().clone())
                        } else {
                            None
                        },
                        protocols: Vec::new(),
                        no_proxy: Vec::new(),
                    };
                    proxy_settings.push(setting);
                }
            }
        }

        if !proxy_settings.is_empty() {
            Some(proxy_settings)
        } else {
            None
        }
    }

    fn set(&self, settings: Vec<&ProxySettings>) -> Result<(), Box<dyn Error>> {
        let content = Gradle::generate_gradle_proxy_content(&settings);
        let gradle_properties_file =
            shellexpand::tilde(Gradle::GRADLE_PROPERTIES_FILE).into_owned();
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(gradle_properties_file)?;

        file.write_all(content.as_bytes())?;
        Ok(())
    }

    fn unset(&self) -> Result<(), Box<dyn Error>> {
        let gradle_properties_file =
            shellexpand::tilde(Gradle::GRADLE_PROPERTIES_FILE).into_owned();
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(gradle_properties_file)?;

        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let updated_content = content.replace(
            "systemProp.http.proxyHost=",
            "systemProp.http.proxyHost=unspecified",
        );
        file.set_len(0)?;
        file.seek(std::io::SeekFrom::Start(0))?;
        file.write_all(updated_content.as_bytes())?;

        Ok(())
    }
}
