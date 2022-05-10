/* This module sets the log file, database credentials and Soap version, etc. 
* before the program is called by the task scheduler, this program should be run atleast once and passed "setup" as the menu option
* from there it will walk the user through what it needs to function properly. */
#![allow(unreachable_code)]
use std::{path::Path, fs::File};




pub fn run() {
    let db_ref_file = "db_ref.txt";
    let logfile = "umler_updater-log.txt";

//check if log exists and try to create log file if not

    if Path::new(logfile).exists() {
        println!("{} exists already!!\n", db_ref_file);
        let log = File::open(logfile);

        //checks permissions for the file, and sets it to writable 
        let metadata = log.unwrap().metadata();
        let mut perm = metadata.unwrap().permissions();
        perm.set_readonly(false);
    } else {
        println!("creating new ref file...\n");
        File::create("umler_updater-log.txt").expect(panic!("Issue creating log files..."));
    }


//check if db ref file exists and Try to create file to save db ref info to if not and log if unable. 

    if Path::new(db_ref_file).exists() {
        println!("{} exists already!!\n", db_ref_file);
        println!("recreate of ref file attempted @ {}\n", chrono::Local::now());
        let db_reference = File::open(db_ref_file);
        //checks permissions for the file, and sets it to writable 
        let metadata = db_reference.unwrap().metadata();
        let mut perm = metadata.unwrap().permissions();
        perm.set_readonly(false);
    } else {
        println!("creating new ref file now...\n");
        File::create("db_ref.txt").expect(panic!("Issue creating db ref files..."));
    }


    println!("successfully setup the Umler updater program....\n");
}