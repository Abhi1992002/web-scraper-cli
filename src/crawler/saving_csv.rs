use std::{fs::OpenOptions, path::Path};
use csv::{Error, Writer};

pub fn write_header(file_path:&str, header : &Vec<&str>)->Result<(),Error>{
   // we are using external package csv to handle writnig inside the csv file
   let mut wtr = Writer::from_path(&Path::new(file_path))?; // It overwrites all the content in the file
   
   // Writing header od csv
   wtr.write_record(header)?; 
   wtr.flush()?;
   Ok(())
}

pub fn write_content(file_path:&str, content : &Vec<&str>)->Result<(),Error>{
    let file = OpenOptions::new().append(true).write(true).open(&Path::new(file_path))?;
    let mut wtr = Writer::from_writer(file); // we created writer this way , so that we can add our custom file handler
    
    // Writing header od csv
    wtr.write_record(content)?;
    wtr.flush()?;
    Ok(())
}
