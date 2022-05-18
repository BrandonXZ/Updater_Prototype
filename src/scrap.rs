//scrap to fix file overwriting, despite this being the best method, its no good for logging. IMO
// #![allow(unused_must_use)]
// #![allow(unused_mut)]
// use std::{fs::OpenOptions, io::{SeekFrom, Seek, Write, Read}};

// pub fn write(){
//     let mut offset;
//     let mut holder = String::new();
//     let logfile = "umler_updater-log.txt";
//     let mut file_handle_thing = OpenOptions::new().read(true).write(true).open(logfile).unwrap();
//     offset = >FILE_VAR>.read_to_string(&mut holder).unwrap();
//     println!("Offset is currently set to--->{:?}", offset);

//     file_handle_thing.seek(SeekFrom::Start(offset.try_into().unwrap()));
//     file_handle_thing.write_all(b"\nThis is what I'm writing after the start of the seek offset...");
// }



