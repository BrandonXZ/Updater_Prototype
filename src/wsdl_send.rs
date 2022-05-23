/* 
* This module defines the codegen functions used to append car AEI's to the wsdl request being sent to umler webservices.
* For ease of coding, the receive function will be defined on a differen module as it will likely be async.
* 
*/
#![allow(unused_variables)]


use mysql::{PooledConn, TxOpts, prelude::Queryable, Error, from_value_opt, FromValueError, Row};
//need to create a build.rs for this to work...A struct will need to be created for AEI tags in dbStructs.rs for this to work.
use savon;

pub fn dummysend_single(db_connection:PooledConn, unknown_ID:String) -> Result<String, Error> {
    println!("\ndebug value of unknown_IDs: {:?}\n", unknown_ID.clone());

    let mut conn = db_connection.unwrap();
    let search_stmt = format!("SELECT \"Equipment_id\" FROM dummy_umler_webservice WHERE dum_search_key = {}", unknown_ID).to_string();
    println!("\ndebug value of search stmt: {}\n", search_stmt.clone());
    let mut selection = conn.start_transaction(TxOpts::default())?;
    let res:Vec<Row> = selection.query(search_stmt).unwrap();
    let mut return_vec:Vec<String>= vec![];
    for row in res {
    let conversion = row[0].clone();
        let conversion = match from_value_opt::<String>(conversion){
            Ok(string) => {
                println!("Column: {}", string);
                return_vec.push(string);
                // return Ok(()); //may remove, this was used in an example but may be a weird format thing
            }
            Err(FromValueError(conversion)) => () /*conversion*/,
        };
    }
    Ok(return_vec[0].clone())
}

pub fn dummysend_multiple(db_connection:PooledConn, unknown_IDs: Vec<String>) -> Result<Vec<String>, Error> {
    let mut conn = db_connection.unwrap();
    let search_stmt = format!("SELECT \"Equipment_id\" from dummy_umler_webservice where dum_search_key = {:?}", unknown_IDs).to_string();
    println!("\ndebug value of search stmt: {}\n", search_stmt.clone());
    let mut selection = conn.start_transaction(TxOpts::default())?;
    let res:Vec<Row> = selection.query(search_stmt).unwrap();
    let mut return_vec:Vec<String>= vec![];
    for row in res {
    let conversion = row[0].clone();
        let conversion = match from_value_opt::<String>(conversion){
            Ok(string) => {
                println!("Column: {}", string);
                return_vec.push(string);
                // return Ok(()); //may remove, this was used in an example but may be a weird format thing
            }
            Err(FromValueError(conversion)) => () /*conversion*/,
        };
    }
    Ok(return_vec)

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

