/* This module defines the functions that will scrub our unknown AEI table for new/unknown records and then add the newly obtained records received from umler
* to the correct car table*/
#![allow(unused_variables)]
#![allow(unused_must_use)]
use std::{fs::{OpenOptions, self}, io::{Seek, SeekFrom,  Read}};

use crate::settings;
const DB_REF_FILE: &str = "db_ref.txt";

//check schema, check unknown car table, apply any necessary conversions/formatting, send the request via umler webservices(by calling wsdl_send module's function)
pub fn run() {

    let blank_ID = "".to_string();
    println!("run functionality not added yet....");
    webservice_formatter(blank_ID)

}



//add the returned car data(response from webservice) to the car info table or log if errors occur
pub fn add() { 
    let current_ID: String = "".to_string();
    println!("add functionality currently being coded...");
    //arg type almost guaranteed to change and be a vector or array of webservice response type or json to be decoded, formatted then added to the db.
    
    //This reads the db reference file to get the db connection info being used.
    let mut db_reference = OpenOptions::new().read(true).write(true).open(DB_REF_FILE).unwrap();
    let test_message = "can't add to db because ID was blank".to_string();
    let offset:usize;
    let mut holder = String::new();
    offset = db_reference.read_to_string(&mut holder).unwrap();
    
    db_reference.seek(SeekFrom::Start(61));
    let db_string = fs::read_to_string(DB_REF_FILE).unwrap();

    println!("Current offset is: {:?}, the value at this position is: {:?}", offset, db_string);
    settings::logthis_dbRelated(test_message, db_string)
    //db_reference.seek(SeekFrom::Start(offset.try_into().unwrap()));

    //settings::logthis_dbRelated(test_message,)
}



/*performs formatting to equipment Id required for webservice. This needs to be padded to be exactly 10 alphanumeric items long, 
either by adding 0's to beginning of the letter portion of the "String"(Mfr) or the numeric portion(ID)*/

pub fn webservice_formatter(current_ID:String) {

    let webservice_comm_type = "send function".to_string();
    println!("Formatter functionality not added yet...");
    if current_ID.is_empty(){
        let Errornote = "No ID received for formatting".to_string();
        settings::logthis_webService(Errornote, webservice_comm_type);
        
    }
        println!("Current ID is ---->{:?}", current_ID);
}