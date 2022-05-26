/* 
* This module defines the codegen functions used to append car AEI's to the wsdl request being sent to umler webservices.
* For ease of coding, the receive function will be defined on a different module as it will likely be async.
* 
*/
#![allow(unused_variables)]


use std::env::var;
use mysql::{PooledConn, TxOpts, prelude::Queryable, Error, from_value_opt, FromValueError, Row, from_row, Value};
use mysql_common::*;
//need to create a build.rs for this to work...A struct will need to be created for AEI tags in dbStructs.rs for this to work.
use savon;
use crate::settings;

/* New idea 05/25 lets try pushing the returned values (Row) from the SQL query onto a newly created vector so then oving each element into its own vector
then iterating over that, its convoluted and clunky but i think it will allow for addressing individual items in a row since MySQL Row can not implement iter()
Its not ideal to pass desired order to mysql as I can't trust the order the queries are answered. would rather get an ordered vector or row returned so I can ensure 
the right values coorespond to the right columns by iterating over each rows columns, problem is, MySQL doesn't implement Row as an iterator since-____-' */


//The function below works but due to the for loop, will return only the first column to avoid excess info during development. This it Type: Vec<Rows>
pub fn dummy_wsdl_send(db_connection:PooledConn, queryStmt: String, db_string:String, columnCount:i32) -> Result<Vec<String>, Error> {
    let mut conn = db_connection.unwrap();
    let mut selection = conn.start_transaction(TxOpts::default())?;
    let res:Vec<Row> = selection.query(queryStmt).unwrap();
    let mut return_vec:Vec<String>= vec![];
    let printable = res.clone();
    let newattempt = res.clone();
    for row in res {
            let row_length =  row.len();
            for i in 0..row_length{
                let conversion = row[i].clone();
                //println!("data iterated per row?...{:?}", row[i]);
                //attempt to convert
                let conversion = match from_value_opt::<String>(conversion.clone()){
                    Ok(string) => {
                        println!("\nColumn value before: {:?}", conversion);
                        println!("Column value after conversion: {}\n", string); //wrong order information, search term doesn't match equipment ID
                        return_vec.push(string);
                        // return Ok(()); //may remove, this was used in an example but may be a weird format thing
                    }
                    Err(e) => settings::logthis_dbRelated(e.to_string(), db_string.clone()) /*conversion*/,
                };

            }


            println!("Row length is: {}", row_length);
         
    }
    let mut i = 0;
    for row in printable.clone() {
    
        i = i + 1;
    }


    println!("\nWhats returned from query(res):\n {:?}\n", printable.clone()); //right info, wrong format(bytes) and blob so needs iterator 
    //println!("\nWhats returned from query(returned_vec): {:?}\n", return_vec.clone()); //Not what I'm looking for.... this is the equipment ID's being searched
    let joined = return_vec.clone().join(", ");
    println!("\nJoined is: {}\n", joined);
    Ok(return_vec)
}



pub fn db_statement_formatter(current_IDs:Vec<String>) -> String {
    println!("\n\n\n*******Database SEARCH statement acting as dummy Umler Webservice call*******\n\n\n\n");

    let unknown_IDs = current_IDs.clone();
    let mut search_stmt_vec = vec![];
    let search_stmt = format!("SELECT * FROM dummy_umler_webservice WHERE ");
    
    search_stmt_vec.push(search_stmt);
    
        if current_IDs.len() >=2 {
            let additional_ID = format!("(dum_search_key = \"{}\")", current_IDs[0]);
            search_stmt_vec.push(additional_ID);
            
            for i in unknown_IDs.clone() {
                if i.eq(&unknown_IDs[0]) {continue;}
                let additional_ID2 = format!("OR (dum_search_key = \"{}\")", i);
                search_stmt_vec.push(additional_ID2);
            }
        } else {
            let additional_ID = format!("(dum_search_key = \"{}\")", current_IDs[0]);
            search_stmt_vec.push(additional_ID);
        }


    //println!("Search Statement Vec is: {:?}", search_stmt_vec);      //commented since this is  working, left for debug purposes only
    let joined = search_stmt_vec.join("\n");
    println!("\nComplete Search statement : {}\n\n\n\n\n\n\n\n", joined);
 
    joined
}





//See email from Tracy regarding the appropriate webservice activity to call upon**

// pub fn codegen() {

//     mod soap {
//         include!(concat!(env!(OUT_DIR), "holding directory/example.rs"));
//     }
// }

// pub fn Car_update_request() {
//     let client = soap::EMISQueryServiceWeb::new("https://services.railinc.com/EMISQueryServiceWeb/services/UmlerEquipmentQueryService.wsdl".to_string());
//     let res = client.get_updated_car_info(soap::umlerEquipmentSelectElemetsQueryRequest()); //comment pointing here regarding dbStructs.rs
// }

