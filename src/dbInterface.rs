/* This module defines the functions that will scrub our unknown AEI table for new/unknown records and then add the newly obtained records received from umler
* to the correct car table*/
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
use std::{fs::{OpenOptions, self}, io::{Seek, SeekFrom,  Read, BufReader, BufRead}};

use mysql::{self, Opts, Pool, PooledConn, Error, TxOpts, prelude::Queryable, Row};
use mysql_common::uuid::Bytes;

use crate::settings;
const DB_REF_FILE: &str = "db_ref.txt";

//check schema, check unknown car table, apply any necessary conversions/formatting, send the request via umler webservices(by calling wsdl_send module's function)
pub fn run() {

    let blank_ID = "".to_string();
    webservice_formatter(blank_ID);
    add();
    let unknown_table_temp_remove_me_before_production = get_unknown_ID_table();
    println!("\nRun func: Name of unknown table pulled from file  ---> {}\n", unknown_table_temp_remove_me_before_production.trim());

}


//add the returned car data(response from webservice) to the car info table or log if errors occur
pub fn add() { 
    let current_ID: String = "".to_string();
    println!("add func: add functionality currently being coded...");
    //arg type almost guaranteed to change and be a vector or array of webservice response type or json to be decoded, formatted then added to the db.
    //function contents are just to test something and will change

    //This reads the db reference file to get the db connection info being used.
    let mut db_reference = OpenOptions::new().read(true).write(true).open(DB_REF_FILE).unwrap();
    let test_message = "can't add to db because ID was blank".to_string();
    let mut holder = String::new();
    let mut reader = BufReader::new(db_reference);
    let db_string = reader.read_line(&mut holder);
    holder.clear();
    let db_string = reader.read_line(&mut holder);
    let holder = str::replace(&holder, "\n", "");
    // println!("add func:Current length in bytes is: {:?}, the value in db-string is: {:?}", db_string.unwrap(), holder);
    settings::logthis_dbRelated(test_message, holder);
    let current_connection = db_connection().unwrap();
    get_table_schema(current_connection);

   


    //db_reference.seek(SeekFrom::Start(offset.try_into().unwrap())); //may need later to move cursor but probably not with above read line method
}



/*performs formatting to equipment Id required for webservice. This needs to be padded to be exactly 10 alphanumeric items long, 
either by adding 0's to beginning of the letter portion of the "String"(Mfr) or the numeric portion(ID)
This is temporarily set up for testing but will need to read value of sql query response when sql process is finished. (currently coding)*/

pub fn webservice_formatter(current_ID:String) {

    let webservice_comm_type = "send function".to_string();
    println!("Web Service Formatter functionality not added yet...");
    if current_ID.is_empty(){
        let Errornote = "No ID received for formatting".to_string();
        settings::logthis_webService(Errornote, webservice_comm_type);
        
    }
        // println!("webservice_comm_func: Current ID is ---->{:?}", current_ID);
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
    println!("{} - attempting connection \n", db_url);
    Ok(conn)
}

pub fn get_car_details_table () -> String {
    let mut db_reference = OpenOptions::new().read(true).write(true).open(DB_REF_FILE).unwrap();
    let test_message = "can't add to db because ID was blank".to_string();
    let mut holder = String::new();
    let mut reader = BufReader::new(db_reference);
    let db_string = reader.read_line(&mut holder);
    holder.clear();
    let db_string = reader.read_line(&mut holder);
    holder.clear();
    let db_string = reader.read_line(&mut holder);
    holder.clear();
    let db_string = reader.read_line(&mut holder);
    // println!("get_car_details_table func: Current length in bytes is: {:?}, the value in db-string is: {:?}", db_string.unwrap(), holder);
    let sel_tables_as_str = str::replace(&holder, "-Car Detail Storage Table", "");
    println!("get_car_details_table func: formatted is: {:?}\n", sel_tables_as_str.trim());
    let return_val = sel_tables_as_str.to_owned();
    return_val.to_string()
}

pub fn get_unknown_ID_table () -> String {
    let mut db_reference = OpenOptions::new().read(true).write(true).open(DB_REF_FILE).unwrap();
    let mut holder = String::new();
    let mut reader = BufReader::new(db_reference);
    let db_string = reader.read_line(&mut holder);
    holder.clear();
    let db_string = reader.read_line(&mut holder);
    holder.clear();
    let db_string = reader.read_line(&mut holder);
    // println!("get_unknown_ID_table func: Current length in bytes is: {:?}, the value in db-string is: {:?}", db_string.unwrap(), holder.trim());
    let sel_tables_as_str = str::replace(&holder, "-Unknown Car ID Table", "");
    println!("\nget_unknown_ID_table func: formatted is: {}\n", sel_tables_as_str.trim());
    sel_tables_as_str
}

pub fn get_table_schema (current_connection: PooledConn) -> Result<(), Error> {
    let mut conn = current_connection;
    let car_details_table_pass = get_car_details_table();
    let get_schema_stmt = format!("SHOW COLUMNS IN {}", car_details_table_pass.trim());
    println!("The Schema statement---> {}\n", get_schema_stmt);
    let mut selection = conn.start_transaction(TxOpts::default())?;
    let res:Vec<Row> = selection.query(get_schema_stmt).unwrap();
    println!("Row data returned: \n");
    println!("{:?}", res);


    /*loop below unstable for now. Need to get what is returned after "row data returned:" to return just the column names or the formatter will have
    to be extra greppy and string manipulative.....-_____-' */
    

    for row in res{  //3 options are generated in loop due to the 3 column names we iterate through 
        println!("\ninside for loop...\n");
        let row1 = row.columns().to_vec();
        let row2 = row.columns_ref();
        
        // let newattempt:String = mysql::from_value(row[1]);
        println!("Column name value: {:?}\n", row[0]); //getting closer
        for columns in row1.iter() {
            println!("->{:?}", columns.name_str());
            
        }
        
    }
   
    Ok(())
 }