/* 
* This module defines the functions that will scrub our unknown AEI table for new/unknown records and then add the newly obtained records received from umler
* to the correct car table (see process order below)
*
* TO DO: 
* Possibly rewrite function to check for last Id searched using MySQL flags instead of an auto-incremented column **Spoke to chili and using flags
* above is pending a meeting later today @1pm to confirm this is agreed upon by those who would need to make some db changes(Amin, Jeff P)
* above TODO will require additional function to send MySQL Statement that update the "processed" (tinyint column) after response from Umler.
* 
***********************************************PROCESS ORDER****************************************************************** 
* Create or obtain existing pooled connection.
* Query our Car detail table for columns and data-types to ensure we grab the relevant information (must be dynamic in case table schema changes).
* Get last car ID sent to Umler using what is saved in db_reference file, and only do the rest if there is a new unknown car, If not exists: log, Stop process.
* If Exists: Query unknown Id tables for the ID's we will be using in the umler webservice request call 
* Create and send the umler webservice call (occuring in WSDL_SEND.RS) 
* Obtain and convert response from umler request using relevent schema info pulled from above step(occuring in WSDL_RECEIVE.RS)
* post newly obtained info to our mysql "car details" table.
* save last inserted equipment ID to the db reference save-file and overwriting the old.
* log successful completion timestamp, End process.
*/

#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![allow(unused_mut)]

use core::time;
use std::{fs::{OpenOptions, self}, io::{Seek, SeekFrom,  Read, BufReader, BufRead}, ptr::null, thread};

use mysql::{self, Opts, Pool, PooledConn, Error, TxOpts, prelude::Queryable, Row, from_value_opt, FromValueError};
use mysql_common::*;
use crate::{settings, wsdl_send, wsdl_receive, dbStructs::{self, DuosCarData}, xml_formatter};

const DB_REF_FILE: &str = "db_ref.txt";

//check schema, check unknown car table, apply any necessary conversions/formatting, send the request via umler webservices(by calling wsdl_send module's function)
pub fn run() {
    let db_url = get_db_url();
    let mut current_ID = "".to_string();
    println!("\nStarting sequence...\n");
    thread::sleep(time::Duration::from_secs(1));
    let mut current_connection = db_connection().unwrap();
    println!("\nConnection established...\n");
    thread::sleep(time::Duration::from_secs(1));
    let unknown_car_table = get_unknown_ID_table();
    println!("\nRun func: unknown table --> {}\n", unknown_car_table.clone());
    thread::sleep(time::Duration::from_secs(1));
    let car_details_table = get_car_details_table();
    println!("\nRun func: detail table --> {}\n", car_details_table.clone()); 
    thread::sleep(time::Duration::from_secs(1));
    let last_searched = get_last_unknown();
    println!("\nRun func: last ID sent to Umler--> {}\n", last_searched.clone());
    thread::sleep(time::Duration::from_secs(1));
    let (current_schema, columns_only) = get_table_schema(current_connection).unwrap(); 
    //for i in current_schema.iter() {println!("\nname: {:?}\n", i);}                    //Debug, same iteration as get_schema, shown from main running process...
    println!("\nRun func: Current Schema ---> {:?}\n", current_schema.clone());                //Debug, same info as line above, just as blob and not iterated...
    thread::sleep(time::Duration::from_secs(1));

    let current_connection = db_connection().unwrap();                       //redundant, I know but its a weird mysql thing
    let last_row_query = prep_lastItem_query(unknown_car_table.clone());
    let check_result = checktest(current_connection, last_searched.clone(), last_row_query);
    println!("\nRun func: Check result is ---> {}\n", check_result.clone());
    thread::sleep(time::Duration::from_secs(1));

    if check_result {
        let current_connection = db_connection().unwrap();  
        let current_connection2 = db_connection().unwrap();
        let current_connection3 = db_connection().unwrap();

        let unk_stmt = prep_unknown_Id_query(unknown_car_table.trim().to_string());
        println!("\nRun func: stmt --> {}\n", unk_stmt.clone());
        thread::sleep(time::Duration::from_secs(2));
        let unknown_car_IDs = scrub_unknowns(current_connection, unk_stmt);
        println!("\nRun func: car ID's ---> {:?}\n", unknown_car_IDs.clone());
        println!("\nThis should be display the last item from the above car ID's list ---> {:?}\n", unknown_car_IDs.last().unwrap().to_string()); 
        thread::sleep(time::Duration::from_secs(2));
        let wsdl_search_IDs = webservice_formatter(unknown_car_IDs.clone(), current_schema.clone());
        settings::saveLastSearch(unknown_car_IDs.last().unwrap().to_string());
        

            //below wsdl calls are demo dummy functions 
        let wsdl_stmt = wsdl_send::db_statement_formatter(unknown_car_IDs.clone());
        let wsdl_response = wsdl_send::dummy_wsdl_send(current_connection2, wsdl_stmt, db_url).unwrap();

        for i in wsdl_response.clone() {
            println!("\nIteration of WSDL response vectors: {:?}\n", i.clone());
            thread::sleep(time::Duration::from_secs(1));
        }
        let insert_stmt = MySQL_Insert_Formatter(wsdl_response, car_details_table.clone());
        println!("\nstmt going to mysql: \n{}\n", insert_stmt.clone());
        add(current_connection3, insert_stmt); //This will need to be moved towards the bottom after the webservice formatter, wsdl_send, wsdl_received 
        println!("\n\n\nThe code below shows what the actual wsdl soap request going to umler will look like\n\n\n");
        thread::sleep(time::Duration::from_secs(2));
        xml_formatter::run(columns_only.clone(), unknown_car_IDs);
        
    } else {
        println!("\nEnd of run function...\n");

    }
    


    println!("\nEnd of run function...\n");                     //remove during production
}




//add the returned car data(response from webservice) to the car info table or log if errors occur
pub fn add(current_connection: PooledConn, insert_stmt: String) -> Result<(), Error> { 
    let mut conn = current_connection;
    let holder = get_db_url();
    let success = "Successfully added to database".to_string();
    let e = "Could not write new car data to database".to_string();
    let success:Result<(), Error> = conn.query_drop(insert_stmt);                    //query drop works but doesn't return if successful or not..
    let success = match success {                                                      //**need to fix this before production 
        Ok(bool) => println!("Successfully added info to database"),  
        Err(e)  => settings::logthis_dbRelated(e.to_string(), holder.clone()),
    };


                                                        //**Comment Below: inaccurate atm and not in use as it would require storing recreation, this is done on the fly.
    //dbStructs::printStruct();                         //<----Above refers to this |Debug, shows value of Umler Car Struct (that should be) mirroring MySQL schema**
    Ok(())
}


/* May be added to wsdl_send module for cleaner code an better org. 
* performs formatting to equipment Id required for webservice. This needs to be padded to be exactly 12-14 characters long, 
* by adding 0's to beginning of the numeric portion(ID) (currently coding)*/

pub fn webservice_formatter(current_IDs:Vec<String>, schema: Vec<String>) -> String {
    let webservice_comm_type = "send function".to_string();
    println!("************Web Service Formatter*************\n");
    //let column_vec_xml = vec![];

    if current_IDs.is_empty() {
        let Errornote = "No ID received for formatting".to_string();
        settings::logthis_webService(Errornote, webservice_comm_type.clone());

    } else {
        for current in current_IDs {
            let lognote = format!("Current unknown car ID is-----> {}", current);
            println!("{}", lognote);
            settings::logthis_nonError(lognote);

        }
    }

    "Placeholder String".to_string()
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
    let mut holder = String::new();
    let mut reader = BufReader::new(db_reference);

    let db_string = reader.read_line(&mut holder);
    holder.clear();
    let db_string = reader.read_line(&mut holder);
    holder.clear();
    let db_string = reader.read_line(&mut holder);
    holder.clear();
    let db_string = reader.read_line(&mut holder);

    let sel_tables_as_str = str::replace(&holder, "-Car Detail Storage Table", "");
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

    let sel_tables_as_str = str::replace(&holder, "-Unknown Car ID Table", "");
    sel_tables_as_str
}

pub fn get_table_schema (current_connection: PooledConn) -> Result<(Vec<String>, Vec<String>), Error> {
    println!("\n\nThe following lines are the Schema Pulled from our car details table....so we know the needed format for info received back from umler\n\n");
    let mut conn = current_connection;
    let car_details_table_pass = get_car_details_table();
    let get_schema_stmt = format!("SHOW COLUMNS IN {}", car_details_table_pass.trim());
    //println!("The Schema statement---> {}\n", get_schema_stmt);
    let mut return_vec:Vec<String>= vec![];
    let mut return_vec2:Vec<String>= vec![];
    let mut selection = conn.start_transaction(TxOpts::default())?;
    let res:Vec<Row> = selection.query(get_schema_stmt).unwrap();
    // println!("Row data returned: \n");                                        //Debug, raw example of what is returned from query...
    // println!("{:?}", res);

    let mut i = 0;
    for row in res{  //3 options are generated in loop due to the 3 column names we iterate through 
        // let mut i = 0;
        //println!("\ninside for loop...\n");
        let row1 = row.columns().to_vec();
        let row2 = row.columns_ref();

        //println!("\nThis is counter: {}", &i);                                   //Debug, counts current loop iteration

        //successfully pulling column names from query and converting from mysql value type below(odd bytes type)
        let conversion = row[0].clone();
        let conversion = match from_value_opt::<String>(conversion){
            Ok(string) => {
                println!("Column: {}", string); 
                return_vec.push(string.clone());                                  //Debug, shows name of column in our database
                return_vec2.push(string);
            }
            Err(FromValueError(conversion)) => () /*conversion Error?*/,
        };

        let conversion2 = row[1].clone();
        let conversion2 = match from_value_opt::<String>(conversion2){
            Ok(string) => {
                println!("Data-Type: {}\n", string);                        //Debug, shows data type in MySQL for the above Column name
                return_vec.push(string);
                // return Ok(());
            }
            Err(FromValueError(conversion2)) => () /*conversion2*/,
        };

        /*
        Code below shows "subfields" for each item. a database can have tables, tables can have columns, rows, and schema info about the table
        "subfields" in this context refers to info about, say, a column itself...not for production...
        */

        // for columns in row1.iter() {
        //     println!("->{:?}", columns.name_str());
        // }

        i = i+1;
    }
    Ok((return_vec, return_vec2))
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

 pub fn prep_lastItem_query (unknown_Id_table: String) -> String {
    let get_last_stmt = format!("SELECT Equipment_id FROM {} ORDER BY entryNo DESC LIMIT 1;", unknown_Id_table );
    get_last_stmt
 }

//This preps the MySQL statement for a select transaction                                    //needs to be modified to reflect what comes after last item 
 pub fn prep_unknown_Id_query (unknown_Id_table:String) -> String {
    let get_rows_stmt = format!("SELECT {}.Equipment_id from db_centraco3.{}", unknown_Id_table.trim(), unknown_Id_table.trim());
    // println!("The Schema statement---> {}\n", get_schema_stmt);                                 //Debug, shows statement to get unknown car ID's from table
    
    return get_rows_stmt;
 }

 // COMMENT NEEDED!!!
 pub fn scrub_unknowns (current_connection:PooledConn, stmt:String) -> Vec<String> {
    let mut conn = current_connection;
    //println!("Query stmt is: {:?}", &stmt);
    let mut res:Vec<String> =  conn.query(stmt).unwrap();
    let tester = res.clone();
    //let return_vec:Vec<String>= vec![]; //just incase I have to append each iteration by pushing onto a vec...
        for r in tester.iter() {
        //println!("\nResult from query is a vector of {} Rows", r);                  //passed to run func fine, so this is not needed.
        }
        //println!("\nThis is a test var in scrub unknowns Func: {:?}", tester);      //passed to run func fine, so this is not needed.

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
    //println!("last ID sent to umler was: {}", last_unknown.clone());
    last_unknown 
 }

//**Need to fix the last comma at the end of this statement  */
pub fn MySQL_Insert_Formatter(new_car_data: Vec<Vec<String>>, car_details_table: String) -> String {
    println!("\n\n*******Database INSERT statement to save newly received Car data*******\n\n");

    let mut holder:Vec<String> = vec![];
    let mut new_car_data = new_car_data.clone();
    
    let insert_stmt = format!("INSERT INTO {} VALUES", car_details_table.trim());

    holder.push(insert_stmt.clone());

    for row in new_car_data.clone() {
        let mut insert_stmt_vec = vec![];
        let open = "(".to_string();
        let close = "),".to_string();
        let nextline = "\n".to_string();
        insert_stmt_vec.push(open);

        for items in row {
            let mut new_insert = String::new();
            new_insert = format!("\"{}\", ", items.clone());
            //println!("\"{}\"", items.clone());
            insert_stmt_vec.push(new_insert);  
        }
        // if new_car_data.last() {}
        insert_stmt_vec.push(close);
        let joined = insert_stmt_vec.join("");
        holder.push(joined);

    }
    //removing last comma added for each row of info being inserted and replace with end of statment ';'
    let formed = holder.join("\n");
    let mut form = str::replace(formed.as_str(), ", )", ")");            //Debug, add ~ to ")" to test logging          
    //let end_of_line = form.len(); println!("line count is: {}\n", end_of_line);               //Debug, shows statement length in the rare event MySQL limit is reached.
    form.pop();                                 
    form.push(';');
    //println!("\nFinal Form is: \n {}\n", form);                                               //Debug, shows the insert statement created, use for syntax errors. 

    form    
}

pub fn checktest (current_connection: PooledConn, last_searched: String, get_last_stmt: String) -> bool {
    let option_1 = "null, no ID present".to_string();
    let mut conn = current_connection;
    let res:Vec<String> = conn.query(get_last_stmt).unwrap();
    let this = res.join(" ").trim().to_string();
    println!("\nThis is the result from last record query: {}\n", this.clone());
    let modif = last_searched.trim().clone();

    if last_searched.eq(&option_1) {
        //println!("in the if...");                                //Debug, just to see what is being returned from test
        return true
    } else if modif.eq(&this.to_string()){
        //println!("in the else if...");                           //Debug, just to see what is being returned from test
        return false 
    } else {
        //println!("in the else...");                              //Debug, just to see what is being returned from test
        return true
    }
 
}

//update the unknown id table to reflect that the equipment id was sent over to umler for updated details to avoid duplicate calls 
pub fn update_column_status () {

}