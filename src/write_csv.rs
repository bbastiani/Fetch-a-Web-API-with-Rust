use crate::utils::Measure;
use crate::file::check_file;
use crate::file::CheckFileError;
use std::fs::File;
use csv;
use log::info;
use serde::de::DeserializeOwned;
use std::{
    fs::{OpenOptions},
    io::Read,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CsvError {
    #[error("File open error")]
    FileOpenError(#[from] std::io::Error),
    #[error("Parse CSV Error")]
    ParseCSVError(#[from] csv::Error),
    #[error("Not found last line")]
    ReadLastLineError,
    #[error("Parse line error")]
    ParseLineError,
    #[error("Check File Error")]
    CreateFileError(#[from] CheckFileError),
}

pub fn save_data_csv(filename: &str, data: &Vec<Measure>) -> Result<(), CsvError> {
    // check if file exist or is empty
    if check_file(filename)? {
        save_csv(filename, data)?;
        info!("Save {} records", data.len());
        return Ok(());
    }    
    let last_line = read_last_line(filename)?;
    let last_line_parsed: Measure = parse_csv_line(last_line)?;
    // if first timestamp is greater than last line timestamp, we save all data
    if data[0].instante > last_line_parsed.instante {
        save_csv(filename, data)?;
        info!("Save {} records", data.len());
        return Ok(());
    }
    // save data where timestamp is greater than last line timestamp
    for (id, measure) in data.iter().rev().enumerate() {
        // check if measure is already saved
        if measure.instante < last_line_parsed.instante {
            // we iterate in reverse order, then if instant is less than last line, we save the rest of the data
            save_csv(filename, &data[data.len() - id + 1..].to_vec())?;
            info!("Save {} records", id - 1);
            break;
        }
    }
    Ok(())
}

pub fn load_data_csv<T>(filename: &str) -> Result<Vec<T>, CsvError>
where
    T: DeserializeOwned,
{
    let mut rdr = csv::Reader::from_path(filename)?;
    let mut measures: Vec<T> = Vec::new();

    for result in rdr.deserialize() {
        let record: T = result?;
        measures.push(record);
    }
    Ok(measures)
}

fn save_csv(filename: &str, measures: &Vec<Measure>) -> Result<(), CsvError> {
    let file = OpenOptions::new().write(true).append(true).open(filename)?;
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);

    for measure in measures {
        wtr.serialize(measure)?;
    }
    wtr.flush()?;
    Ok(())
}

fn read_last_line(filename: &str) -> Result<String, CsvError> {
    let mut file = File::open(filename)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    if let Some(last_line) = buffer.lines().last() {
        Ok(last_line.to_string())
    } else {
        Err(CsvError::ReadLastLineError)
    }
}

pub fn parse_csv_line(line: String) -> Result<Measure, CsvError> {
    if let Some(result) = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(line.as_bytes())
        .deserialize()
        .next()
    {
        let record: Measure = result?;
        Ok(record)
    } else {
        Err(CsvError::ParseLineError)
    }
}
