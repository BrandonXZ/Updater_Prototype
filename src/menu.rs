/* This module really only defines a menu for a single usable option for ease of development. 
* Running the program once will allow for the user to set up, after that task scheule will call this program and pass an argument for automation
*/ 

use std::io;
use std::collections::{HashSet, HashMap};

use crate::{dbInterface, pathprep};

pub fn run(){
    let options = vec!["automated-run", "setup"]; 
    println!("\nUmler Car info updater \n");
    let mut input = "blank".to_string();
    
    while input.trim().ne("quit") {
        println!("Enter a command: ");
        input = String::new();
        let choice = io::stdin().read_line(&mut input);
        let selection = input.to_ascii_lowercase();

        if options.contains(&selection.trim()) {
        println!("The user selection is  {}", input);
        
        match &selection.trim() {
            &"automated-run" => dbInterface::run(),
            &"setup"=> pathprep::run(),
        }
        
        } else if input.trim().eq("quit") {
            println!("goodbye");
            break;
        } else if !options.contains(&input.trim()){
            println!("Not a valid entry...Try \"setup\", or \"quit\"");
        }
    }
}
