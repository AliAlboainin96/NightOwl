// proj.rs: Manages the inputted data; Contains project struct to define 
use std::fs;
use std::io::prelude::*;
use std::fs::File;
use std::{collections::BTreeMap};
use serde::{Deserialize, Serialize};
use toml;

#[derive(Debug, Default, Serialize)] 
pub struct Projects<'param> {
    pub projects: BTreeMap<&'param str, Project<'param>>,
}

#[derive(Debug, Serialize)]
pub struct Project<'param> {
    #[serde(rename = "Project")]
    pub project: &'param str,

    #[serde(rename = "Author")]
    pub author:     &'param str,

    #[serde(rename = "Description")]
    pub description:  &'param str,

    #[serde(rename = "Language")]
    pub language:    &'param str,

    #[serde(rename = "Genre")]
    pub genre:       &'param str,

    #[serde(rename = "ID")]
    pub id:          u32
}

impl Project<'_> {
    
    // Takes the defined structure and pass it into the toml_buffer

    pub fn WriteToFile(&self) -> std::io::Result<()> {
        let mut buffer = Projects::default();
        let mut File = fs::OpenOptions::new()
                        .read(true)
                        .append(true)
                        .create(true)
                        .open("data.toml")
                        .unwrap();
        
    
        // "buffer" will be initialized and the defined project struct will be passed to it.

        buffer.projects.insert ( 
            
            // self.project is the main header of the toml object. eg "[project.NightOwl]"
            self.project,
            
            Project {
                project:        self.project, 
                author:         self.author,
                description:    self.description,
                language:       self.language,
                genre:          self.genre,
                id:             self.id
            },
        );

        // Convert the buffer data into toml based string
        let toml_string = toml::to_string(&buffer)
                        .expect("Could not encode TOML value");
        
        // Write the toml_string into File variable which contain the defined flags for appending, etc...
        writeln!(File, "{}", toml_string)
                        .expect("Could not write to file");

       Ok(()) 
    }
}