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


/***********************************************************************************************************************************************************************/
//Currently, working on which would be better for MySQL a batch of the same statements or to append one large statement...

// pub fn db_statement_formatter(current_IDs:Vec<String>) {
//     println!("\n SCRAP.RS: Web Service Formatter functionality not added yet...\n");

//     let unknown_IDs = current_IDs.clone();
//     let mut search_stmt_vec = vec![];
    
//     let search_stmt = format!("SELECT \"Equipment_id\" FROM dummy_umler_webservice WHERE ");
//     search_stmt_vec.push(search_stmt);

    
//         if current_IDs.len() >=2 {
//             let additional_ID = format!("(dum_search_key = {})", current_IDs[0]);
//             search_stmt_vec.push(additional_ID);
            
//             for i in unknown_IDs.clone() {
//                 if i.eq(&unknown_IDs[0]) {continue;}
//                 let additional_ID2 = format!(" OR (dum_search_key = {})", i);
//                 search_stmt_vec.push(additional_ID2);
//             }
//         } else {
//             let additional_ID = format!("(dum_search_key = {})", current_IDs[0]);
//             search_stmt_vec.push(additional_ID);
//         }


//     println!("Search Statement Vec is: {:?}", search_stmt_vec);
//         let joined = search_stmt_vec.join("\n");
//     println!("joined and ready is : {}", joined);
 
// }

/***********************************************************************************************************************************************************************/
/* A test to see if  the result from a MySQL will map to a struct and provided default vals correctly if not provided*/
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]
use std::{fs::{OpenOptions, self}, io::{Seek, SeekFrom,  Read, BufReader, BufRead}, ptr::null};
use mysql::{self, Opts, Pool, PooledConn, Error, TxOpts, prelude::Queryable, Row, from_value_opt, FromValueError};
use mysql_common::*;
use crate::dbStructs::{self, DuosCarData};
const DB_REF_FILE: &str = "db_ref.txt";

pub fn run(){
    let current_connection = db_connection().unwrap();
    let query_results = send(current_connection);
}

pub fn db_connection () -> Result<PooledConn, Error> {
    let mut db_reference = OpenOptions::new().read(true).write(true).open(DB_REF_FILE).unwrap();
    let test_message = "can't add to db because ID was blank".to_string();
    let mut holder = String::new();
    let mut reader = BufReader::new(db_reference);
    let db_string = reader.read_line(&mut holder);
    holder.clear();
    let db_string = reader.read_line(&mut holder);

    let db_url = holder;
    let opts = Opts::from_url(&db_url)?;
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
    //println!("{} - attempting connection", db_url);
    Ok(conn)
}


pub fn send(connection:PooledConn, ) {
    let mut conn = connection;
    let stmt = "SELECT * from dummy_umler_webservice WHERE (dum_search_key = \"Rust_tester_CARID\") OR (dum_search_key = \"Rust_tester_carID_2\")".to_string();
    println!("\nstmt is: {}\n", stmt.clone());
    let res = conn.query_map(stmt, |(Equipment_id, ummd, umet, umow)|
        DuosCarData {
            Equipment_id: Equipment_id,
            ummd:ummd,
            umet:umet,
            umow:umow,
            ..Default::default()
        }
).expect("Query failed...");

for r in res {
    println!("Results of query: {}, {}, {}, {}", r.Equipment_id, r.ummd, r.umet, r.umow);
    }
}
