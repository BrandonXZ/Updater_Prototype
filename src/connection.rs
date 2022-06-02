/* This module defines functions for the pooled connections to our MySQL db*/

#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

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
    println!("connection run func: {} - attempting connection", db_url);

    //get tables available within the database
    let table_stmt = format!("SHOW TABLES IN {}", dbn);
    println!("\n{}  -MySQL Statement", table_stmt);
    let mut selection = conn.start_transaction(TxOpts::default())?; // may not need this
    
    let tables_avail = get_avail_tables(selection, table_stmt);
    subscriber_selection(db_url, tables_avail); 
    Ok(())
}

 //code following this comment really just checks the database connection and access to specified tables.
 //Need to add something to loop through the availabel tables for ease of reading and cleaner look.
fn get_avail_tables(mut selection: Transaction, table_stmt: String) -> Vec<String> {
    let res:Result<Vec<String>> = selection.query(table_stmt);
    for row in res.iter(){
        let mut rowz = row.as_slice();     
        //println!("Get_avail_tables func: Tables available are: {:?}\n", row);            //Debug, Shows available tables before iterator...
        println!("getting available tables\n");
        for items in row.iter() {
            println!("-------> {}", items);
        }
    }
    res.unwrap()
}


/*check if desired table selection exists and get other selections if more than one */ 
pub fn subscriber_selection (db_url: String, tables:Vec<String>) {
    let tables_avail = tables;
    let mut test = 0;
    let mut unknown_car_table = String::new();
    let mut car_details_table= String::new();
    while test == 0 {

        println!("Enter Name of Table holding unknown car ID's: ");
        
        let response = std::io::stdin().read_line(&mut unknown_car_table);
        let sel_tables_as_str = str::replace(&unknown_car_table, "\r\n", "");

        println!("Enter Name of Table that will be storing obtained car details(**not the unknownID's table**): ");
        
        let car_tab = std::io::stdin().read_line(&mut car_details_table);
        let formed_det_tab = str::replace(&car_details_table, "\r\n", "");

        if tables_avail.contains(&sel_tables_as_str) {
            println!("The unknown ID's table exists in the database!!");
            test = 1;
        } else {
            println!("That table does not exist in the database...please try again");
            test = 0;
        }

        if tables_avail.contains(&formed_det_tab) {
            println!("The details table exists in the database");
            test = 1;
        } else {
            println!("That table does not exist in the database...please try again");
            test = 0;
        }
        
    }
    //only God knows why I had to reformat outside the while loop...-____-'
    let sel_tables_as_str = str::replace(&unknown_car_table, "\r\n", "");
    let formed_det_tab = str::replace(&car_details_table, "\r\n", "");
    println!("connection>subscriber_selection func: car details table name: {:?}, and unknown table name: {:?}",car_details_table, unknown_car_table);
    settings::run(db_url, formed_det_tab, sel_tables_as_str);
}