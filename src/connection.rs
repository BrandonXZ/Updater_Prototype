/* This module defines functions for the pooled connections to our MySQL db*/

#![allow(unused_mut)]
#![allow(unused_variables)]


use mysql::*;
use mysql::prelude::*;

use crate::{dbInterface, settings};

// use byteorder::{LittleEndian as LE, ReadBytesExt, WriteBytesExt};
// use bytes::BufMut;



pub fn run(path: String, dbn: &str) -> Result<()> {
    let db_url = path;
    let opts = Opts::from_url(&db_url)?;
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
    println!("{} - attempting connection", db_url);

    //get tables available within the database
    let table_stmt = format!("SHOW TABLES IN {}", dbn);
    println!("{}", table_stmt);
    let mut selection = conn.start_transaction(TxOpts::default())?; // may not need this
    
    let tables_avail = get_avail_tables(selection, table_stmt);
    subscriber_selection(tables_avail); 
    Ok(())
}

 //code following this comment really just checks the database connection and access to specified tables.

fn get_avail_tables(mut selection: Transaction, table_stmt: String) -> Vec<String> {
    let res:Result<Vec<String>> = selection.query(table_stmt);
    for row in res.iter(){
        let mut rowz = row.as_slice();     
        println!(" Tables available are: {:?}, rows in database: {:?}", row, rowz);
    }
    res.unwrap()
}

/*check if desired table selection exists and get other selections if more than one */ 

pub fn subscriber_selection (tables:Vec<String>) {
    let tables_avail = tables;
    println!("Enter Car Id Table name");
    let mut selected_table = String::new();
    let response = std::io::stdin().read_line(&mut selected_table);
    let sel_tables_as_str = str::replace(&selected_table, "\r\n", "");
    //println!("trimmed is {:?}", sel_tables_as_str);

    if tables_avail.contains(&sel_tables_as_str) {
        println!("The table exists in the database");
        dbInterface::add();
        settings::run();
        
    } else {
        println!("That table does not exist in the database...please try again");
    }
}