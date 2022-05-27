/* Author: Brandon Minter
* Date of release | Q2 '22
* Duos Technologies Inc. 05/10/2022
*
* Description: This program, when compiled to an executable, will be activated via windows task scheduler on a nightly basis
* It will create a pooled connection to our main local database and search a table that stores AEI tags which are new and not currently generating information
*
* If this "unknown" table has any new AEI tags, the program will send an active WSDL webservices request to Umler to get the missing train car info associated with the AEI's
* The program will then update our main local database to add the missing car info.
*
* Our databases for each location may need a program to interface with our main db and sync the newly obtained information 
* since we have unique db's running at each site. This program will also log any issue's should they arise during the process. 
*
* This program in order to function for an extended amount of time without unnecessary calls to umler, or duplicate calls for ID's we already have, or even to avoid
* an ever growing storage issue, The table storing the unknown car Id's encountered, needs to have a counter column, and this needs to be an auto-incremented column...
* This doesn't need to be the primary key, it just needs to be present. My SQL doesn't monitor entry position eithout this, preferably name this column "entryNo" 
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
//mod scrap;
//mod mysql_typecheck; //used for testing for schema !!unsafe atm!!

use std::{env, ptr::null}; 

fn main() { 
    
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