use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::Instant;

use age::secrecy::Secret;
use age::{Decryptor, Encryptor};
use anyhow::bail;
use directories::BaseDirs;
use druid::im::Vector;
use druid::{Data, Lens};
use keyring::Entry;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

static CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let mut pargs = pico_args::Arguments::from_env();
    match pargs
        .opt_value_from_str("--config-path")
        .expect("Failed to parse command line arguments")
    {
        Some(config_dir) => config_dir,
        None => BaseDirs::new()
            .expect("Could find the settings path")
            .preference_dir()
            .join("lol_account_manager.yml")
    }
});

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Data, Serialize, Deserialize)]
pub enum Theme {
    #[default]
    Light,
    Dark
}

#[derive(Debug, Clone, PartialEq, Data, Lens, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub close_on_login: bool,
    pub force_focus: bool,
    pub theme: Theme,
    pub last_database: Option<String>
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            close_on_login: true,
            force_focus: true,
            theme: Theme::Light,
            last_database: None
        }
    }
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

#[derive(Debug, Clone, Default, PartialEq, Data, Lens, Serialize, Deserialize)]
pub struct Account {
    pub name: String,
    pub username: String,
    pub password: String,
    #[serde(with = "crate::util::string_list")]
    pub notes: String
}

#[derive(Debug, Clone, Data, Lens, PartialEq)]
pub struct Database {
    pub accounts: Vector<Account>,
    pub password: String,
    pub path: String
}

impl Database {
    pub fn new(path: &str, password: &str) -> anyhow::Result<Self> {
        let db = Self {
            accounts: Default::default(),
            password: password.to_owned(),
            path: path.to_owned()
        };
        db.save()?;
        Ok(db)
    }

    pub fn import(input: &str, output: &str, password: &str) -> anyhow::Result<Self> {
        let accounts: Vector<Account> = serde_yaml::from_reader(File::open(input)?)?;
        let db = Self {
            accounts,
            password: password.to_owned(),
            path: output.to_owned()
        };
        db.save()?;
        Ok(db)
    }

    pub fn load(path: &str, password: &str) -> anyhow::Result<Self> {
        let time = Instant::now();
        let file = File::open(path)?;
        let decryptor = match Decryptor::new(file)? {
            Decryptor::Passphrase(d) => d,
            _ => bail!("Only password encrypted files are supported!")
        };
        let reader = decryptor.decrypt(&Secret::new(password.to_owned()), None)?;
        let accounts: Vector<Account> = serde_yaml::from_reader(reader)?;
        println!("loading time: {}ms", time.elapsed().as_secs_f64() * 1000.0);
        Ok(Self {
            accounts,
            password: password.to_owned(),
            path: path.to_owned()
        })
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = Path::new(&self.path);
        if let Some(path) = path.parent() {
            std::fs::create_dir_all(path)?;
        }
        let time = Instant::now();
        let encryptor = Encryptor::with_user_passphrase(Secret::new(self.password.clone()));
        let file = File::create(path)?;
        let mut writer = encryptor.wrap_output(file)?;
        serde_yaml::to_writer(&mut writer, &self.accounts)?;
        writer.finish()?;
        println!("writing time: {}ms", time.elapsed().as_secs_f64() * 1000.0);
        Ok(())
    }
}

pub struct Password;

impl Password {
    fn entry(path: &str) -> keyring::Result<Entry> {
        Entry::new(path, "local")
    }

    pub fn store(path: &str, password: &str) -> keyring::Result<()> {
        Self::entry(path)?.set_password(password)
    }

    pub fn get(path: &str) -> keyring::Result<String> {
        Self::entry(path)?.get_password()
    }
}
