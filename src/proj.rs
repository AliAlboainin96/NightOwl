// proj.rs: Manages the inputted data; Contains project struct to define and create new "projects"

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;


#[derive(Debug, Deserialize, Serialize)]

pub struct project {
    pub project:     String,
    pub author:      String,
    pub description: String,
    pub language:    String,
    pub genre:       String,
    pub id:          u32
}

impl project {
    pub fn DisplayData(&self) { 
        println!("Project : {}", self.project);
        println!("Autho: {}", self.author);
        println!("Description: {}", self.description);
        println!("Language: {}", self.language);
        println!("Genre: {}", self.genre);
        println!("ID: {}", self.id);

    }

    pub fn WriteToJSON(&self) -> std::io::Result<()> {
        //let proj_obj = serde_json::from_str(raw_obj).unwrap();
        serde_json::to_writer_pretty(&File::create("data.json")?, &self);
        Ok(())
    }

}
