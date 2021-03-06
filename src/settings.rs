/* This module sets the log file, database credentials and Soap version, etc. 
* before the program is called by the task scheduler, this program should be run atleast once and passed "setup" as the menu option
* from there it will walk the user through what it needs to function properly. */
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(dead_code)]
use std::{path::Path, fs::{File, OpenOptions}, io::{Write, Read, Seek, SeekFrom, BufReader, BufRead}};
const LOGFILE: &str = "umler_updater-log.txt";
const DB_REF_FILE: &str = "db_ref.txt";


pub fn run(db_url:String, car_details_table:String, unknown_car_table:String) {
    let current_db = db_url.as_bytes();
    let mut offset:usize;
    let mut holder = String::new();
//check if logfile exists and try to create logfile if not

    if Path::new(LOGFILE).exists() {
        println!("{} exists already!!\n", LOGFILE);
        let current_time = chrono::Local::now();
        let dupl_message = format!("\nrecreate of log file attempted @ {}\n", current_time); 

        //Open File
        let mut log = OpenOptions::new().read(true).write(true).open(LOGFILE).unwrap();
        offset = log.read_to_string(&mut holder).unwrap();

        //Checks permissions for the file, and sets it to writable 
        let metadata = log.metadata().unwrap();
        let mut perm = metadata.permissions();
        perm.set_readonly(false);

        //Moves cursor to end of file to log this event
        log.seek(SeekFrom::Start(offset.try_into().unwrap()));
        log.write(dupl_message.as_bytes());
        log.write(current_db);
        println!("finished duplicate log stuff...");
    } else {
        let current_time = chrono::Local::now();
        println!("creating new log file...\n");
        let creation_message = format!("{} was created on {:?}", LOGFILE, current_time);

        let mut log = File::create("umler_updater-log.txt").expect("Issue creating log files..");
        let metadata = log.metadata().unwrap();
        let mut perm = metadata.permissions();
        perm.set_readonly(false);
        log.write(creation_message.as_bytes());
    }


//check if db ref file exists and Try to create if not and log if unable. 

    if Path::new(DB_REF_FILE).exists() {
        let current_time = chrono::Local::now();
        let dupl_message = format!("\nrecreate of ref file attempted @ {}\n", current_time); 
        println!("{} exists already!!\n", DB_REF_FILE);
        println!("{:?}", dupl_message);

        //Open File
        let mut log = OpenOptions::new().read(true).write(true).open(LOGFILE).unwrap();
        offset = log.read_to_string(&mut holder).unwrap();

        //checks permissions for the file, and sets it to writable
        let metadata = log.metadata().unwrap();
        let mut perm = metadata.permissions();
        perm.set_readonly(false);

        //Moves cursor to end of file to log this event
        log.seek(SeekFrom::Start(offset.try_into().unwrap()));
        log.write(dupl_message.as_bytes());
        log.write(current_db);
        println!("finished duplicate ref file stuff...");
    } else {
        let current_time = chrono::Local::now();
        println!("creating new ref file now...\n");
        let creation_message = format!("{} was created on {:?} \n", DB_REF_FILE, current_time);
        let mut db_reference = File::create(DB_REF_FILE).expect("Issue creating db ref files...");
        let metadata = db_reference.metadata().unwrap();
        let mut perm = metadata.permissions();
        perm.set_readonly(false);
        db_reference.write(creation_message.as_bytes());
        db_reference.write(current_db);
    }
    add_table_of_unknowns(unknown_car_table);
    add_car_details_table(car_details_table);
    println!("successfully setup the Umler updater program....\n");
}

pub fn logthis_nonError(Errornote: String) {
    let current_time = chrono::Local::now();
    let mut log = OpenOptions::new().read(true).write(true).open(LOGFILE).unwrap();
    let mut holder = String::new();
    let offset:usize;

    offset = log.read_to_string(&mut holder).unwrap();
    log.seek(SeekFrom::Start(offset.try_into().unwrap()));

    let message = format!("\nLogging: {:?}, Action occured @: {:?}\n", Errornote, current_time);
    println!("{}", message);
    log.write(message.as_bytes());
}

pub fn logthis(Errornote: String) {
    let current_time = chrono::Local::now();
    let mut log = OpenOptions::new().read(true).write(true).open(LOGFILE).unwrap();
    let mut holder = String::new();
    let offset:usize;

    offset = log.read_to_string(&mut holder).unwrap();
    log.seek(SeekFrom::Start(offset.try_into().unwrap()));

    let message = format!("\nError - This occurred due to internal code exception: {:?}, this occured @: {:?}\n", Errornote, current_time);
    println!("{}", message);
    log.write(message.as_bytes());
}

pub fn logthis_dbRelated(Errornote:String, dburl:String) {
    let current_time = chrono::Local::now();
    let current_db = dburl.as_bytes();
    let mut log = OpenOptions::new().read(true).write(true).open(LOGFILE).unwrap();
    let mut holder = String::new();
    let offset:usize;

    offset = log.read_to_string(&mut holder).unwrap();
    log.seek(SeekFrom::Start(offset.try_into().unwrap()));

    let message = format!("\nError - This occurred due to Database issue: {:?}, this occured @: {:?}\n", Errornote, current_time);
    println!("{}", message);
    log.write(message.as_bytes());
    log.write(current_db);
}

pub fn logthis_webService(Errornote:String, webservice_type:String) {
    let current_time = chrono::Local::now();
    let mut log = OpenOptions::new().read(true).write(true).open(LOGFILE).unwrap();
    let mut holder = String::new();
    let offset:usize;

    offset = log.read_to_string(&mut holder).unwrap();
    log.seek(SeekFrom::Start(offset.try_into().unwrap()));

    let message = format!("\nError - This occurred due to Webservices issue: {:?}, this occured @: {:?}\n", Errornote, current_time);
    println!("{}", message);

    log.write(message.as_bytes());
    
    //this should be either the request or response, message(Errornote) will tell the actual error/exception that occured...
    log.write(webservice_type.as_bytes());  

}

pub fn add_table_of_unknowns (car_details_table: String) {
    let table_to_insert = format!("\n{} -Unknown Car ID Table", car_details_table);
    let mut holder = String::new();
    let mut db_reference = OpenOptions::new().read(true).write(true).open(DB_REF_FILE).unwrap();
    let offset = db_reference.read_to_string(&mut holder).unwrap();
    db_reference.seek(SeekFrom::Start(offset.try_into().unwrap()));
    db_reference.write(table_to_insert.as_bytes());
    println!("successfully saved unknown car table");
}

pub fn add_car_details_table (unknown_car_table:String) {
    let table_to_insert = format!("\n{} -Car Detail Storage Table", unknown_car_table);
    let mut holder = String::new();
    let mut db_reference = OpenOptions::new().read(true).write(true).open(DB_REF_FILE).unwrap();
    let offset = db_reference.read_to_string(&mut holder).unwrap();
    db_reference.seek(SeekFrom::Start(offset.try_into().unwrap()));
    db_reference.write(table_to_insert.as_bytes());
    println!("successfully saved car details table");
}

pub fn saveLastSearch (unknown_Car_ID: String ) {
    let ID_to_insert = format!("\n{} -last ID searched", unknown_Car_ID);
    let mut holder = String::new();
    let mut db_reference = OpenOptions::new().read(true).write(true).open(DB_REF_FILE).unwrap();
    let offset = db_reference.read_to_string(&mut holder).unwrap();
    db_reference.seek(SeekFrom::Start(offset.try_into().unwrap()));
    db_reference.write(ID_to_insert.as_bytes());
    println!("successfully saved last ID to the ref file");
}

pub fn saveLastSearch_v2 (unknown_Car_ID: String) { //current in progress!!
    let mut db_reference = OpenOptions::new().read(true).write(true).open(DB_REF_FILE).unwrap();
    let mut holder = String::new();
    let mut reader = BufReader::new(db_reference);

    let db_string = reader.read_line(&mut holder);
    holder.clear();
    let db_string = reader.read_line(&mut holder);
    holder.clear();
    let db_string = reader.read_line(&mut holder);
    holder.clear();
    let db_string = reader.read_line(&mut holder);
    holder.clear();
    let db_string = reader.read_line(&mut holder);
    println!("saveLast v2  is currently: {}", holder.clone());
    let sel_tables_as_str = str::replace(&holder, "-last ID searched", "");
    let return_val = sel_tables_as_str.to_owned();
    println!("\nreturn_val is: {}", return_val);
    // return_val.to_string();

}