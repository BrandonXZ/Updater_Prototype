/* This module defines functions for the pooled connections to our MySQL db*/

#![allow(unused_mut)]
#![allow(unused_variables)]
use std::{ptr::null, alloc::System, convert};

use mysql::*;
use mysql::prelude::*;
use mysql_common::*;
use byteorder::{LittleEndian as LE, ReadBytesExt, WriteBytesExt};
use bytes::BufMut;
use crate::{subscriber::{Subscriber, Table}, pathprep, thread_manager};


pub fn run(path: String, dbn: &str) -> Result<()> {
    let db_url = path;
    let opts = Opts::from_url(&db_url)?;
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
    println!("{} - attempting connection", db_url);

    // Get service name 
    let mut subscribe= String::new();
    println!("Enter Service identifier: ");
    let service = std::io::stdin().read_line(&mut subscribe);

    //get tables available within the database
    let table_stmt = format!("SHOW TABLES IN {}", dbn);
    println!("{}", table_stmt);
    let mut selection = conn.start_transaction(TxOpts::default())?; // may not need this
    
    let tables_avail = get_avail_tables(selection, table_stmt);
    subscriber_selection(tables_avail); 
    Ok(())
}