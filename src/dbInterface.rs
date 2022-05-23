/* 
* This module defines the functions that will scrub our unknown AEI table for new/unknown records and then add the newly obtained records received from umler
* to the correct car table (see process order below)
*
* TO DO: 
* still need to save the returned schema we are using to a vector or array and then convert to a struct (for use later?)
* define function to search db_ref file for last equipment Id passed to umler and check for new ID's in "unknown car ID" table using that
* define function that will generate MySQL Insert command for newly obtained data
* Create Error handling schema both for umler call and for mysql insert.(should be minimal from mysql since we're pulling updated schema each time.)
* ^^but may require converting between datatypes if something changes on umlers end.
*
***********************************************PROCESS ORDER****************************************************************** 
* Create or obtain existing pooled connection.
* Query our Car detail table for columns and data-types to ensure we grab the relevant information (must be dynamic in case table schema changes).
* Query unknown Id tables for the ID's we will be using in the umler webservice request call using last equipment ID used(saved in db_reference file)
* Create and send the umler webservice call (occuring in WSDL_SEND.RS) 
* Obtain and convert response from umler request using relevent schema info pulled from above step(occuring in WSDL_RECEIVE.RS)
* post newly obtained info to our mysql "car details" table.
* save last inserted equipment ID to the db reference save-file and overwriting the old.
* log successful completion timestamp
*/
#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
use std::{fs::{OpenOptions, self}, io::{Seek, SeekFrom,  Read, BufReader, BufRead}, ptr::null};

use mysql::{self, Opts, Pool, PooledConn, Error, TxOpts, prelude::Queryable, Row, from_value_opt, FromValueError};
use mysql_common::*;

use crate::{settings, wsdl_send};
const DB_REF_FILE: &str = "db_ref.txt";

//check schema, check unknown car table, apply any necessary conversions/formatting, send the request via umler webservices(by calling wsdl_send module's function)
pub fn run() {

    let mut current_ID = "".to_string();
    println!("\nRun func: ID --> {:?}", current_ID.clone());
    let mut current_connection = db_connection().unwrap();
    println!("\nRun func: conn --> {:?}", &current_connection);
    let unknown_car_table = get_unknown_ID_table();
    println!("\nRun func: unknown table --> {}", unknown_car_table.clone());
    let car_details_table = get_car_details_table();
    println!("\nRun func: detail table --> {}", car_details_table.clone());
    let unk_stmt = prep_unknown_Id_query(unknown_car_table.trim().to_string());
    println!("\nRun func: stmt --> {}", unk_stmt.clone());
    let last_searched = get_last_unknown();
    println!("\nRun func: last--> {}", last_searched.clone());
    let current_schema = get_table_schema(current_connection).unwrap(); //not returning anything need to tweak return type for this when vec is created
    for i in current_schema.iter() {println!("\nname: {:?}", i);}
    println!("Run func: Current Schema {:?}", current_schema); //see above comment!!
    let current_connection = db_connection().unwrap();
    let current_connection2 = db_connection().unwrap();
    let current_connection3 = db_connection().unwrap();
    let current_connection4 = db_connection().unwrap();
    // println!("updated connection is: \n {:?}", &current_connection);
    let unknown_car_IDs = scrub_unknowns(current_connection, unk_stmt);
    println!("\nRun func: car ID's{:?}", unknown_car_IDs.clone());
    if unknown_car_IDs.len() >= 2 {
        for i in unknown_car_IDs.iter() {webservice_formatter(i.clone()); println!("i is currently: {}", i); } 
        let wsdl_response = wsdl_send::dummysend_multiple(current_connection2, unknown_car_IDs.clone());
        add(current_connection4, wsdl_response.unwrap(), car_details_table);
    } else {
        let wsdl_response = wsdl_send::dummysend_single(current_connection3, unknown_car_IDs[0].clone());
        let mut wsdl_vec:Vec<String> = vec![];
        wsdl_vec.push(wsdl_response.unwrap());
        add(current_connection4, wsdl_vec, car_details_table);
    }
    println!("Run func: Unknown Cars {:?}", unknown_car_IDs.clone());
    
}


//add the returned car data(response from webservice) to the car info table or log if errors occur
pub fn add(current_connection: PooledConn, current_ID:Vec<String>, car_details_table: String ) { 
    let mut conn = current_connection;
    let default_ID: String = "".to_string();
    let insert_stmt = format!("INSERT INTO {} VALUES", car_details_table.trim());
    println!("The Schema statement---> {}\n", insert_stmt);
    let selection = conn.exec_drop(insert_stmt, ());
    if current_ID.len() > 1 {

    } else {
        let currentID = current_ID[0].clone(); 
    }
    println!("add func: add functionality currently being coded...");
    let holder = get_db_url();
    let test_message = "can't add to db because ID was blank".to_string();
    settings::logthis_dbRelated(test_message, holder);
    //db_reference.seek(SeekFrom::Start(offset.try_into().unwrap())); //may need later to move cursor but probably not with above read line method
}


/* May be added to wsdl_send module for cleaner code an better org. 
* performs formatting to equipment Id required for webservice. This needs to be padded to be exactly 10 alphanumeric items long, 
* either by adding 0's to beginning of the letter portion of the "String"(Mfr) or the numeric portion(ID)
* This is temporarily set up for testing but will need to read value of sql query response when sql process is finished. (currently coding)*/

pub fn webservice_formatter(current_ID:String) {
    let webservice_comm_type = "send function".to_string();
    println!("Web Service Formatter functionality not added yet...");
    if current_ID.is_empty(){
        let Errornote = "No ID received for formatting".to_string();
        settings::logthis_webService(Errornote, webservice_comm_type.clone());
        
    } else {
        let lognote = format!("Current unknown car ID is-----> {}", current_ID);
        println!("{}", lognote);
        settings::logthis_nonError(lognote);
    }

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

pub fn get_table_schema (current_connection: PooledConn) -> Result<Vec<String>, Error> {
    let mut conn = current_connection;
    let car_details_table_pass = get_car_details_table();
    let get_schema_stmt = format!("SHOW COLUMNS IN {}", car_details_table_pass.trim());
    println!("The Schema statement---> {}\n", get_schema_stmt);
    let mut return_vec:Vec<String>= vec![];
    let mut selection = conn.start_transaction(TxOpts::default())?;
    let res:Vec<Row> = selection.query(get_schema_stmt).unwrap();
    // println!("Row data returned: \n"); //raw example of what is returned from query...
    // println!("{:?}", res);


    /*Working but very very clunky way of extracting the column name and data types needed without using metadata */
    let mut i = 0;
    for row in res{  //3 options are generated in loop due to the 3 column names we iterate through 
        // let mut i = 0;
        //println!("\ninside for loop...\n");
        let row1 = row.columns().to_vec();
        let row2 = row.columns_ref();

        println!("\nThis is counter: {}", &i);

        //successfully pulling column names from query and converting from mysql value type below(odd bytes type)
        let conversion = row[0].clone();
        let conversion = match from_value_opt::<String>(conversion){
            Ok(string) => {
                println!("Column: {}", string);
                return_vec.push(string);
                // return Ok(()); //may remove, this was used in an example but may be a weird format thing
            }
            Err(FromValueError(conversion)) => () /*conversion*/,
        };

        let conversion2 = row[1].clone();
        let conversion2 = match from_value_opt::<String>(conversion2){
            Ok(string) => {
                println!("Data-Type: {}", string);
                return_vec.push(string);
                // return Ok(());
            }
            Err(FromValueError(conversion2)) => () /*conversion2*/,
        };

        /*
        Code below shows "subfields" for each item. a database can have tables, tables can have columns, rows, and schema info about the table
        "subfields" in this context refers to info about, say, a column itself...
        */

        // for columns in row1.iter() {
        //     println!("->{:?}", columns.name_str());
            
        // }
        i = i+1;
    }
    Ok(return_vec)
 }

 //This reads the db reference file to get the db connection info being used.
 pub fn get_db_url()-> String {
    let mut db_reference = OpenOptions::new().read(true).write(true).open(DB_REF_FILE).unwrap();
    let mut holder = String::new();
    let mut reader = BufReader::new(db_reference);
    let db_string = reader.read_line(&mut holder);
    holder.clear();
    let db_string = reader.read_line(&mut holder);
    let holder = str::replace(&holder, "\n", "");
    holder
 }

//This preps the MySQL statement for a select transaction
 pub fn prep_unknown_Id_query (unknown_Id_table:String) -> String {
    let get_schema_stmt = format!("SELECT {}.Equipment_id from db_centraco3.{}", unknown_Id_table.trim(), unknown_Id_table.trim());
    println!("The Schema statement---> {}\n", get_schema_stmt);
    
    return get_schema_stmt;
 }

 pub fn scrub_unknowns (current_connection:PooledConn, stmt:String) -> Vec<String> {
    let mut conn = current_connection;
    println!("Query stmt is: {:?}", &stmt);
    let mut res:Vec<String> =  conn.query(stmt).unwrap();
    let tester = res.clone();
    //let return_vec:Vec<String>= vec![]; //just incase I have to append each iteration by pushing onto a vec...
        for r in tester.iter() {
        println!("\nResult from query is: {}", r);
        }
        println!("\nThis is tester in scrub unknowns Func: {:?}", tester);

        if tester.len() < 2 {
            println!("# of unknown cars returned is: {}", tester.len());
        } else if tester.len() >= 2 {
            println! ("# of unknown cars returned is: {}", tester.len());
        }
    res
 }


 pub fn get_last_unknown () -> String {
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
    holder.clear();
    let db_string = reader.read_line(&mut holder);
    let mut last_unknown = str::replace(&holder, "-last ID searched", "");

    let current_ID = last_unknown.clone();
    if current_ID.is_empty(){
        last_unknown = "null, no ID present".to_string();
        let Errornote = format!("last unknown ID searched was not found in db reference file...{}", last_unknown);
        settings::logthis(Errornote);
        
    } else {
        
        let lognote = format!("Current unknown car ID is-----> {}", current_ID);
        println!("{}", lognote);
        //settings::logthis(lognote); no need to log this atm
    }
    println!("last ID sent to umler was: {}", last_unknown.clone());
    last_unknown 
 }