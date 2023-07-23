use clap::{Arg, Command};

fn cli() -> Command {
    Command::new("auto-proxy")
        .version("0.1.0")
        .author("Aman Sikarwar <amansikarwaar@gmail.com>")
        .about("Set system-wide proxy automatically")
        .arg_required_else_help(true)
        .subcommands([
            Command::new("config")
                .about("Configure proxy")
                .arg_required_else_help(true)
                .subcommands([
                    Command::new("new")
                        .about("Create a new proxy configuration")
                        .arg(
                            Arg::new("config-name")
                                .help("Name of the proxy configuration")
                                .required(false),
                        ),
                    Command::new("use").about("Use a proxy configuration").arg(
                        Arg::new("config-name")
                            .help("Name of the proxy configuration")
                            .required(true),
                    ),
                    Command::new("delete")
                        .about("Delete a proxy configuration")
                        .arg(
                            Arg::new("config-name")
                                .help("Name of the proxy configuration")
                                .required(true),
                        ),
                    Command::new("list").about("List all proxy configurations"),
                    Command::new("show").about("Show current proxy configuration"),
                ]),
            Command::new("set").about("Set proxy").args([
                Arg::new("http-host")
                    .help("HTTP host")
                    .long("http-host")
                    .required(false),
                Arg::new("http-port")
                    .default_value("8080")
                    .help("HTTP port")
                    .long("http-port")
                    .required(false),
                Arg::new("https-host")
                    .default_value("http-host")
                    .help("HTTPS host")
                    .long("https-host")
                    .required(false),
                Arg::new("https-port")
                    .default_value("8080")
                    .help("HTTPS port")
                    .long("https-port")
                    .required(false),
                Arg::new("ftp-host")
                    .default_value("http-host")
                    .help("FTP host")
                    .long("ftp-host")
                    .required(false),
                Arg::new("ftp-port")
                    .default_value("8080")
                    .help("FTP port")
                    .long("ftp-port")
                    .required(false),
                Arg::new("socks-host")
                    .default_value("http-host")
                    .help("SOCKS host")
                    .long("socks-host")
                    .required(false),
                Arg::new("socks-port")
                    .default_value("8080")
                    .help("SOCKS port")
                    .long("socks-port")
                    .required(false),
                Arg::new("no-proxy")
                    .default_value("localhost, 127.0.0.1, 192.168.1.1, ::1, *.local")
                    .help("No proxy")
                    .long("no-proxy")
                    .required(false),
                Arg::new("username")
                    .help("Username for proxy authentication")
                    .long("username")
                    .required(false),
                Arg::new("password")
                    .help("Password for proxy authentication")
                    .long("password")
                    .required(false),
            ]),
            Command::new("unset").about("Unset proxy"),
            Command::new("show").about("Show current proxy"),
            Command::new("auto-apply").about("Apply proxy automatically based on network"),
        ])
}

fn main() {
    let app = cli();
    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("config", config_matches)) => match config_matches.subcommand() {
            Some(("new", new_matches)) => match new_matches.get_one::<String>("config-name") {
                Some(name) => println!("Creating new proxy configuration: {}", name),
                None => println!("Creating new proxy configuration"),
            },
            Some(("use", use_matches)) => {
                let name: &String = use_matches.get_one("name").unwrap();
                println!("Using proxy configuration: {}", name);
            }
            Some(("delete", delete_matches)) => {
                let name: &String = delete_matches.get_one("name").unwrap();
                println!("Deleting proxy configuration: {}", name);
            }
            Some(("list", _list_matches)) => {
                println!("Listing all proxy configurations");
            }
            Some(("show", _show_matches)) => {
                println!("Showing current proxy configuration");
            }
            _ => unreachable!(),
        },
        Some(("set", set_matches)) => {
            let default = String::from("");
            let http_proxy: &String = set_matches.get_one("http-proxy").unwrap_or(&default);
            let https_proxy: &String = set_matches.get_one("https-proxy").unwrap_or(&default);
            let ftp_proxy: &String = set_matches.get_one("ftp-proxy").unwrap_or(&default);
            let socks_proxy: &String = set_matches.get_one("socks-proxy").unwrap_or(&default);
            let no_proxy: &String = set_matches.get_one("no-proxy").unwrap_or(&default);
            let username: &String = set_matches.get_one("username").unwrap_or(&default);
            let password: &String = set_matches.get_one("password").unwrap_or(&default);
            println!(
                "Setting proxy: http_proxy: {}, https_proxy: {}, ftp_proxy: {}, socks_proxy: {}, no_proxy: {}, username: {}, password: {}",
                http_proxy, https_proxy, ftp_proxy, socks_proxy, no_proxy, username, password
            );
        }
        Some(("unset", _unset_matches)) => {
            println!("Unsetting proxy");
        }
        Some(("show", _show_matches)) => {
            println!("Showing current proxy");
        }
        Some(("auto-apply", _auto_apply_matches)) => {
            println!("Applying proxy automatically based on network");
        }
        _ => unreachable!(),
    }
}
