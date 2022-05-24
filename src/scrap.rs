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