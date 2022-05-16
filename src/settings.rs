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
        let printable_Perm = perm.clone();
        perm.set_readonly(false);
        fs::set_permissions(logfile, perm);
        println!("This is 1st if metadata: --->{:?}", metadata);
        println!("This is 1st if permission: ---->{:?}", printable_Perm);

    } else {
        println!("creating new log file...\n");
        let log = File::create("umler_updater-log.txt").expect("Issue creating log files..");
        let metadata = log.metadata().unwrap();
        let mut perm = metadata.permissions();
        let printable_Perm = perm.clone();
        perm.set_readonly(false);
        let printable = fs::set_permissions(logfile, perm);
        println!("This is 1st else metadata: --->{:?}", metadata);
        println!("This is 1st else permission: ---->{:?}", printable);
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
        let printable_Perm = perm.clone();
        perm.set_readonly(false);
        fs::set_permissions(logfile, perm);
        log.write_all(dupl_message.as_bytes()).expect("could not write duplication notice");
        println!("This is 2nd if metadata: --->{:?}", metadata);
        println!("This is 2nd if permission: ---->{:?}", printable_Perm);
    } else {
        println!("creating new ref file now...\n");
        let db_reference = File::create(db_ref_file).expect("Issue creating db ref files...");
        let metadata = db_reference.metadata().unwrap();
        let mut perm = metadata.permissions();
        let printable_Perm = perm.clone();
        perm.set_readonly(false);
        let printable = fs::set_permissions(db_ref_file, perm);
        println!("This is 2nd else metadata: --->{:?}", metadata);
        println!("This is 2nd else permission: ---->{:?}", printable);
    }

    let mut saving = File::open(db_ref_file).expect("Could not open the db ref file...");
    let mut perm = saving.metadata().unwrap().permissions();
    let printable_Perm = perm.clone();
    perm.set_readonly(false);
    saving.set_permissions(perm);
    saving.write_all(current_db).expect("Could not save db credentials to file...");
    println!("successfully setup the Umler updater program....\n");
    println!("This is final metadata: --->{:?}", saving.metadata().unwrap());
    println!("This is final permission: ---->{:?}", printable_Perm);
}