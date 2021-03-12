// proj.rs: Manages the inputted data; Contains project struct to define and create new "projects"

use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

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
}
