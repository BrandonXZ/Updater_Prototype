/* 
* This module defines the codegen functions used to append car AEI's to the wsdl request being sent to umler webservices.
* For ease of coding, the receive function will be defined on a different module as it will likely be async.
*
* Comments with "Debug" indicate variables that when uncommented reveal the description.
*/
#![allow(unused_variables)]


use std::env::var;
use mysql::{PooledConn, TxOpts, prelude::Queryable, Error, from_value_opt, FromValueError, Row, from_row, Value};
use mysql_common::*;
//need to create a build.rs for this to work...A struct will need to be created for AEI tags in dbStructs.rs for this to work.
use crate::settings;



//The function below works but due to the for loop, will return only the first column to avoid excess info during development. This it Type: Vec<Rows>
pub fn dummy_wsdl_send(db_connection:PooledConn, queryStmt: String, db_string:String) -> Result<Vec<Vec<String>>, Error> {
    let mut conn = db_connection.unwrap();
    let mut selection = conn.start_transaction(TxOpts::default())?;
    let res:Vec<Row> = selection.query(queryStmt).unwrap();
    //let mut column_vec:Vec<String>= vec![];                                     // need to uncomment to debug function and change return type above after -> to match   
    let mut vec_of_vecs:Vec<Vec<String>> = vec![];
    let printable = res.clone();
    let newattempt = res.clone();
    for row in res {
        let mut column_vec:Vec<String>= vec![];
            let row_length =  row.len();
            for i in 0..row_length{
                let conversion = row[i].clone();
                //attempt to convert
                let conversion = match from_value_opt::<String>(conversion.clone()){
                    Ok(string) => {
                        //println!("\nColumn value before: {:?}", conversion);     //Debug, shows MySQL value before conversion
                        //println!("Column value after conversion: {}\n", string); //Debug, shows MySQL value converted to string
                        column_vec.push(string);
                    }
                    Err(e) => settings::logthis_dbRelated(e.to_string(), db_string.clone()) /*conversion Error?*/,
                };
                               
            }

            vec_of_vecs.push(column_vec.clone());
            //println!("\nVec of Vec is: {:?}\n", vec_of_vecs.clone());         //Debug, ensures Vec is constructed properly for each iteration
            //println!("Row length is: {}", row_length);                        //Debug, shows all columns in row are captured.
    }
    //println!("\nFinished Vec of Vec:{:?}\n", vec_of_vecs.clone());            //Debug, Shows what will be returned from this function a Vec<rows> that is iterable
                                                                                //which is a Vec<Vec<String>

    // let mut i = 0;                                                           //Debug only, this just counts the iterations
    // for row in printable.clone() {                                           //
    //      println!("Row #: {}", i);                                           //
    //      i = i + 1;                                                          //
    // }                                                                        //


    //println!("\nWhats returned from query(res):\n {:?}\n", printable.clone()); //Debug, displays raw blob returned from MySQL(bytes data-type)
    Ok(vec_of_vecs)
}



pub fn db_statement_formatter(current_IDs:Vec<String>) -> String {
    println!("\n\n*******Database SEARCH statement acting as dummy Umler Webservice call*******\n\n");

    let unknown_IDs = current_IDs.clone();
    let mut search_stmt_vec = vec![];
    let search_stmt = format!("SELECT * FROM dummy_umler_webservice WHERE ");
    
    search_stmt_vec.push(search_stmt);
    
        if current_IDs.len() >=2 {
            let additional_ID = format!("(Equipment_id = \"{}\")", current_IDs[0]);
            search_stmt_vec.push(additional_ID);
            
            for i in unknown_IDs.clone() {
                if i.eq(&unknown_IDs[0]) {continue;}
                let additional_ID2 = format!("OR (Equipment_id = \"{}\")", i);
                search_stmt_vec.push(additional_ID2);
            }
        } else {
            let additional_ID = format!("(Equipment_id = \"{}\")", current_IDs[0]);
            search_stmt_vec.push(additional_ID);
        }


    //println!("Search Statement Vec is: {:?}", search_stmt_vec);      //commented since this is  working, left for debug purposes only
    let joined = search_stmt_vec.join("\n");
    println!("\nComplete Search statement : {}\n\n\n", joined);
 
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

