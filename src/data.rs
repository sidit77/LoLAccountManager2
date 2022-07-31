use std::path::PathBuf;
use std::fs::File;
use druid::{Data, Lens};
use druid::im::Vector;
use directories::BaseDirs;
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};

lazy_static!{
    static ref CONFIG_PATH: PathBuf = {
        let mut pargs = pico_args::Arguments::from_env();
        match pargs.opt_value_from_str("--config-path").unwrap() {
            Some(config_dir) => config_dir,
            None => BaseDirs::new()
                .expect("Could find the settings path")
                .preference_dir()
                .join("lol_account_manager.yml")
        }
    };
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Data, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Light
    }
}


#[derive(Debug, Default, Clone, Data, Lens, Serialize, Deserialize)]
pub struct Settings {
    pub close_on_login: bool,
    pub theme: Theme,
    pub last_database: Option<String>
}

impl Settings {

    pub fn load() -> anyhow::Result<Self> {
        Ok(match CONFIG_PATH.exists() {
            true => serde_yaml::from_reader(File::open(&*CONFIG_PATH)?)?,
            false => {
                let result = Self::default();
                Self::save(&result)?;
                result
            }
        })
    }

    pub fn save(&self) -> anyhow::Result<()> {
        if let Some(path) = CONFIG_PATH.parent() {
            std::fs::create_dir_all(path)?;
        }
        serde_yaml::to_writer(File::create(&*CONFIG_PATH)?, self)?;
        Ok(())
    }

}

#[derive(Debug, Clone, Default, Data, Lens, Serialize, Deserialize)]
pub struct Account {
    pub name: String,
    pub username: String,
    pub password: String,
    #[serde(with = "crate::util::string_list")]
    pub notes: String
}

#[derive(Debug, Default, Clone, Data, Lens)]
pub struct Database {
    pub accounts: Vector<Account>,
    pub password: String,
    pub path: String,
}

impl Database {

    pub fn load(path: &str, password: &str) -> anyhow::Result<Self> {
        let accounts: Vector<Account> = serde_yaml::from_reader(File::open(path)?)?;
        Ok(Self {
            accounts,
            password: password.to_owned(),
            path: path.to_owned()
        })
    }

    pub fn save(&self) -> anyhow::Result<()> {
        Ok(serde_yaml::to_writer(File::create(&self.path)?, &self.accounts)?)
    }

}
