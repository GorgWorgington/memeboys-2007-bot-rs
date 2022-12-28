use serde::Deserialize;

#[derive(Debug)]
pub enum ConfigError {
  ParseFailure,
}

#[derive(Deserialize, Debug)]
pub struct Config {
  discord_token: String,
}

impl Config {
  pub fn from_json(config_json: String) -> Result<Config, ConfigError> {
    let res = serde_json::from_str(&*config_json);

    match res {
      Ok(cfg) => Ok(cfg),
      Err(_) => Err(ConfigError::ParseFailure),
    }
  }

  pub fn token(&self) -> String {
    self.discord_token.to_owned()
  }
}
