use log::*;

use std::{
    error::Error,
    path::PathBuf,
    fs::{
        File,
        self
    }
};

use simplelog::{
    LevelFilter,
    WriteLogger,
    ConfigBuilder
};

use config::Config;

use app_dirs::{
    app_dir,
    AppDataType
};

use crate::constants::APP_INFO;


pub fn init(config: &Config) -> Result<(), Box<dyn Error>> {
    let log_level = match config.get("logging.level").unwrap_or("debug") {
        "off" => LevelFilter::Off,
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info
    };
    let max_stored = config.get::<u32>("logging.max_stored").unwrap_or(10);
    let log_folder = app_dir(AppDataType::UserData, &APP_INFO, "logs")?;

    prepare_folder(log_folder.to_owned(), max_stored)?;

    //Order: Off, Error, Warn, Info, Debug, Trace
    let log_config = ConfigBuilder::new()
        .set_thread_level(LevelFilter::Info)
        .set_target_level(LevelFilter::Off)
        .set_location_level(LevelFilter::Info)
        .set_time_to_local(true)
        .build();

    WriteLogger::init(log_level, log_config, File::create(log_folder.join("latest.log")).unwrap())?;

    info!("Logger initialized :)");
    Ok(())
}

fn prepare_folder(folder: PathBuf, max_files: u32) -> Result<(), Box<dyn Error>> {
    let folder = folder.as_path();

    //Keep only .log files on the list
    let mut entries: Vec<PathBuf> = fs::read_dir(folder)?
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap().path())
        .filter(|entry| entry.is_file())
        .filter(|entry| entry.extension().unwrap().to_str() == Some("log"))
        .collect();

    if entries.len() == 0 {
        //This means we don't need to manage any files, so we can just skip this part
        return Ok(());
    }

    //Make sure the files are sorted, and then reverse it
    entries.sort();
    //It's important to reverse it, so we don't have filename conflicts, the sequence should be:
    //1.log <- 2.log <- 3.log...
    entries.reverse();

    for file in entries {
        if file.file_name().unwrap() == "latest.log" {
            continue;
        }

        let file_number: u32 = file.file_stem().unwrap().to_str().unwrap().parse().unwrap();

        //If the file exceeds the file limit, remove it
        if file_number >= max_files {
            fs::remove_file(file)?;
        //For every log file different from the latest, increase it's number by 1
        } else {
            let file = file.as_path();

            fs::rename(file, folder.join(format!("{}.log", file_number+1)))?;
        }
    }

    //Rename latest to be the first file
    fs::rename(folder.join("latest.log"), folder.join("1.log"))?;

    Ok(())
}
