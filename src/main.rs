/* Author: Brandon Minter
* Date of release | Q2 '22
* Duos Technologies Inc. 05/10/2022
*
* Description: This program, when compiled to an executable, will be activated via windows task scheduler on a nightly basis
* It will create a pooled connection to our main local database and search a table that stores AEI tags which are new and not currently generating information
*
* If this table has any new AEI tags, the program will send an active WSDL webservices request to Umler to get the missing train car info associated with the AEI's
* The program will then update our main local database to add the missing car info.
*
* Our databases for each location may need a program to interface with our main db and sync the newly obtained information 
* since we have unique db's running at each site. This program will also log any issue's should they arise during the process. 
*/
#![allow(non_snake_case)]
#![allow(unused_imports)]
mod connection;
mod pathprep;
mod dbStructs;
mod settings;
mod menu;
mod dbInterface;
mod splash;
mod wsdl_send;
//mod mysql_typecheck; //used for testing for schema !!unsafe atm!!

use std::{env, ptr::null}; 

fn main() { 
    /* Rust oddly enough doesn't allow you to check if a non-existent array/vector element is null, I technnically could've
    worked with passing arguments during call by using an array and checking the length of the array to deduce if an arg was passed but
    arrays aren't really growable like vectors making additions later on a tiny bit more tedious.*/
    let mut args:Vec<String> = env::args().collect();
    let auto_command = String::from("automated_run");
    args.push("blank".to_string());     


    //check if running automated or not
    if args[1].eq_ignore_ascii_case("blank") { 
        splash::showWelcome();
        menu::run();
    }
    else if args[1].eq_ignore_ascii_case(&auto_command){ 
        automated_run();
    } else {
        let Errornote = "Incorrect argument passed when starting, if attempting an automated run, or reconfig see documentation...".to_string();
        settings::logthis(Errornote);
    }
}

// For Automatic use with Windows task scheduler
fn automated_run() {
    dbInterface::run();
}