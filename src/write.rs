use chrono::{ DateTime, Local };
use std::error::Error;
use std::fs::{ self, OpenOptions };
use std::io::{ BufWriter, Write };
use std::path::PathBuf;

use crate::model::blj::Kind;

pub fn write(data: String, group: Kind, filename: &str) -> Result<(), Box<dyn Error>> {
    let dt: DateTime<Local> = Local::now();

    let filename = format!("{}_{}", filename, dt.format("%H_%M_%S"));

    let group = match group {
        Kind::Intranet => "内网",
        Kind::Internet => "外网",
    };

    let path = PathBuf::from(
        format!(r#".\output\{}\{}\{}.txt"#, dt.format("%Y_%m_%d"), group, filename)
    );

    let folder = path.parent().ok_or("目录错误").unwrap();
    fs::create_dir_all(folder)?;

    let file = OpenOptions::new().create(true).write(true).truncate(true).open(path)?;

    let mut buffer = BufWriter::new(file);

    buffer.write_all(data.as_bytes())?;

    buffer.flush()?;

    Ok(())
}
