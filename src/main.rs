#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::io::prelude::*;
use std::fs::File;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::*;
use tui::layout::*;
mod proj;

fn main() { 
   let nightowl_json = proj::project {
            project: "NightOwl".to_string(),
            author: "Ali".to_string(),
            description: "Project managment written in rust!".to_string(),
            language: "Rust".to_string(),
            genre: "TUI Application".to_string(),
            id: 5
        }; 
    
   nightowl_json.WriteToJSON();
}


