//! app configs file

use std::path::PathBuf;
use anyhow::Error;
use dirs;
use reqwest::Url;

use crate::app_cfg::{parse_toml, AppCfg, APP_HOME, CONFIG_FILE};
use crate::key_manager;

static ACCOUNTS_DB_FILE: &str = "accounts.json";

// get the config path for files
pub fn default_config_path() -> PathBuf {
  dirs::home_dir()
    .unwrap()
    .join(APP_HOME)
    .join(CONFIG_FILE)
}

/// Get all the app configs. For tx sending and upstream nodes
pub fn get_cfg() -> Result<AppCfg, Error> {
  parse_toml(None) // gets default toml path.
}

pub fn default_accounts_db_path() -> PathBuf {
  dirs::home_dir().unwrap().join(APP_HOME).join(ACCOUNTS_DB_FILE)
}

/// All the parameters needed for a client transaction.
#[derive(Debug)]
pub struct TxParams {
  /// User's authkey
  pub auth_key: String,
  /// Address of the signer of transaction, e.g. owner's operator
  pub signer_address: String,
  /// Address of the sender.
  // TODO: refactor so that this is not par of the TxParams type
  pub owner_address: String,
  /// Url
  pub url: Url,
  /// waypoint
  pub waypoint: String,
  /// KeyPair
  pub keypair: String, // NOTE: user actual keypair type
}

/// get transaction parameters from config file
pub fn get_tx_params() -> Result<TxParams, anyhow::Error> {
  // TODO: Should the Error type be a WalletError?
  let config = get_cfg()?;

  // Requires user input to get OS keyring
  let _keypair = key_manager::get_keypair(&config.profile.account.to_string())?;
  Ok(TxParams {
    auth_key: "demo".to_string(),
    signer_address: "demo".to_string(),
    owner_address: "demo".to_string(),
    url: "http://0.0.0.0".parse()?,
    waypoint: "demo".to_string(),
    keypair: "demo".to_string(),
  })
}

pub fn is_initialized() -> bool {
  default_config_path().exists()
}
