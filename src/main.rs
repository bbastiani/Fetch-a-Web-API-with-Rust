use log::{error, info, warn};

// my modules
mod config_yaml;
mod logging;
mod request;
mod utils;
mod file;
mod write_csv;
// use
use config_yaml::{load_config_file, LoadConfigError};
use logging::setup_log;
use request::{get_auth_token, get_measures};
use utils::Measure;
use write_csv::save_data_csv;

#[tokio::main]
async fn main() -> Result<(), String> {
    // log start program
    info!("Start program");
    // setup log file
    if let Err(e) = setup_log() {
        return Err(e.to_string());
    }
    // load config file
    let config = match load_config_file() {
        Ok(config) => config,
        Err(e) => match e {
            LoadConfigError::FileNotFound(_) => {
                error!("Config file not found");
                return Err(e.to_string());
            }
            LoadConfigError::FileNotValid(_) => {
                error!("Config file not valid");
                return Err(e.to_string());
            }
        },
    };
    // request auth token
    let auth_token =
        match get_auth_token(&config.token_url, &config.username, &config.password).await {
            Ok(auth_token) => auth_token,
            Err(e) => {
                warn!("{:#?}", e);
                return Err(e.to_string());
            }
        };
    // get data from api
    let measures: Vec<Measure> = match get_measures(&config.get_url, &auth_token).await {
        Ok(measures) => measures,
        Err(e) => {
            warn!("{:#?}", e);
            return Err(e.to_string());
        }
    };
    // save data to csv
    let _ = match save_data_csv(&config.filename, &measures) {
        Ok(_) => (),
        Err(e) => {
            warn!("{:#?}", e);
            return Err(e.to_string());
        }
    };
    // log end program
    info!("End program");
    Ok(())
}
