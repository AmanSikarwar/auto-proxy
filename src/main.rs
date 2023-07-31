use cli_parser::{cli, prompt_new_config};

mod cli_parser;
mod config;
mod network;
mod proxy;
mod proxy_profile;
mod setup;
mod target;

fn main() {
    let app = cli();
    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("config", config_matches)) => match config_matches.subcommand() {
            Some(("new", new_matches)) => {
                let name = new_matches.get_one::<String>("config-name");
                let config = prompt_new_config(name.cloned());

                let res = match config.create_profile() {
                    Ok(_) => (),
                    Err(err) => panic!("{}", err),
                };
                println!("Creating new proxy configuration: ");
            }
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
