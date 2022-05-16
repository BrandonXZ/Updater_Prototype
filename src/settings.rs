/* This module sets the log file, database credentials and Soap version, etc. 
* before the program is called by the task scheduler, this program should be run atleast once and passed "setup" as the menu option
* from there it will walk the user through what it needs to function properly. */
#![allow(unreachable_code)]
#![allow(unused_must_use)]
use std::{path::Path, fs::{File, self}, io::Write};



pub fn run(db_url:String) {
    let current_db = db_url.as_bytes();
    let db_ref_file = "db_ref.txt";
    let logfile = "umler_updater-log.txt";

//check if logfile exists and try to create logfile if not

    if Path::new(logfile).exists() {
        println!("{} exists already!!\n", logfile);
        let log = File::open(logfile).expect("Could not open the existing log file ~");

        //checks permissions for the file, and sets it to writable 
        let metadata = log.metadata().unwrap();
        let mut perm = metadata.permissions();
        perm.set_readonly(false);
        fs::set_permissions(logfile, perm);

    } else {
        println!("creating new log file...\n");
        let mut log = File::create("umler_updater-log.txt").expect("Issue creating log files..");
        let metadata = log.metadata().unwrap();
        let mut perm = metadata.permissions();
        perm.set_readonly(false);
        log.write(b"This stupid fucking thing!!!!!");
    }


//check if db ref file exists and Try to create if not and log if unable. 

    if Path::new(db_ref_file).exists() {
        let current_time = chrono::Local::now();
        let dupl_message = format!("recreate of ref file attempted @ {}\n", current_time); 
        println!("{} exists already!!\n", db_ref_file);
        println!("{:?}", dupl_message);

        //checks permissions for the file, and sets it to writable
        let mut log = File::open(logfile).expect("could not open the db ref file");
        let metadata = log.metadata().unwrap();
        let mut perm = metadata.permissions();
        perm.set_readonly(false);
        fs::set_permissions(logfile, perm);
        log.write(dupl_message.as_bytes()).expect("could not write duplication notice");
    } else {
        println!("creating new ref file now...\n");
        let mut db_reference = File::create(db_ref_file).expect("Issue creating db ref files...");
        let metadata = db_reference.metadata().unwrap();
        let mut perm = metadata.permissions();
        perm.set_readonly(false);
        db_reference.write(b"This stupid fucking thing!!!!!");
    }

    println!("successfully setup the Umler updater program....\n");
}