/* This module sets the log file, database credentials and Soap version, etc. 
* before the program is called by the task scheduler, this program should be run atleast once and passed "setup" as the menu option
* from there it will walk the user through what it needs to function properly. */
#![allow(unreachable_code)]
#![allow(unused_must_use)]
use std::{path::Path, fs::{File, OpenOptions}, io::{Write, Read, Seek, SeekFrom}};



pub fn run(db_url:String) {
    let current_db = db_url.as_bytes();
    let db_ref_file = "db_ref.txt";
    let logfile = "umler_updater-log.txt";
    let mut offset:usize;
    let mut holder = String::new();
//check if logfile exists and try to create logfile if not

    if Path::new(logfile).exists() {
        println!("{} exists already!!\n", logfile);
        let current_time = chrono::Local::now();
        let dupl_message = format!("\nrecreate of log file attempted @ {}\n", current_time); 

        //Open File
        let mut log = OpenOptions::new().read(true).write(true).open(logfile).unwrap();
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
        let creation_message = format!("{} was created on {:?}", logfile, current_time);

        let mut log = File::create("umler_updater-log.txt").expect("Issue creating log files..");
        let metadata = log.metadata().unwrap();
        let mut perm = metadata.permissions();
        perm.set_readonly(false);
        log.write(creation_message.as_bytes());
    }


//check if db ref file exists and Try to create if not and log if unable. 

    if Path::new(db_ref_file).exists() {
        let current_time = chrono::Local::now();
        let dupl_message = format!("\nrecreate of ref file attempted @ {}\n", current_time); 
        println!("{} exists already!!\n", db_ref_file);
        println!("{:?}", dupl_message);

        //Open File
        let mut log = OpenOptions::new().read(true).write(true).open(logfile).unwrap();
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
        let creation_message = format!("{} was created on {:?} \n", db_ref_file, current_time);
        let mut db_reference = File::create(db_ref_file).expect("Issue creating db ref files...");
        let metadata = db_reference.metadata().unwrap();
        let mut perm = metadata.permissions();
        perm.set_readonly(false);
        db_reference.write(creation_message.as_bytes());
        db_reference.write(current_db);
    }

    println!("successfully setup the Umler updater program....\n");
}