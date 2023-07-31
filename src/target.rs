use std::error::Error;

use crate::proxy::ProxySettings;

mod bash;

trait ProxyTarget {
    fn get(&self) -> Option<Vec<ProxySettings>>;
    fn set(&self, ProxySettings: Vec<&ProxySettings>) -> Result<(), Box<dyn Error>>;
    fn unset(&self) -> Result<(), Box<dyn Error>>;
}
