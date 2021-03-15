#![allow(unused_must_use)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use std::fs;
use std::io::prelude::*;
use std::fs::File;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::*;
use tui::layout::*;
mod proj;


fn main() { 
  
   // Now it is possible to create a new project struct and call the .WriteToFile function which will create new file
   
   let nightowl_project = proj::Project {
            project: "MouseEditor",
            author:  "Ali",
            description:   "best text editor containing vim's speed and the power of emacs!",
            language:   "Cpp",
            genre:   "Text Editor",
            id:   6
      };

      nightowl_project.WriteToFile();
}


