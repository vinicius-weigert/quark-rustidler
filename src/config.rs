use std::error::Error;

use config::{
    File,
    Config,
    Environment
};


pub fn init_cfg() -> Result<Config, Box<dyn Error>> {
    let mut settings = Config::default();

    settings.merge(File::with_name("cfg/default"))?;
    settings.merge(File::with_name("cfg/settings"))?;
    settings.merge(Environment::with_prefix("QUARK"))?;

    Ok(settings)
}
