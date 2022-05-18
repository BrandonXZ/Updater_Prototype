/*This section gets user input and preps the path to the database and ensures that user is not putting invalid options.
* Testing with | Type: mysql ... Username:admin ... Password: admin ... Host: localhost ... port: 3306 ... Database Name: testdb */


#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_must_use)] //This is dangerous and will be removed later, this is because of an unhandled result type that could be an error returned from the 
                          //connection::run function but since this is only a prototype, I'm allowing dead code...
use std::fmt::Debug;
use crate::connection;
use crate::splash;

#[derive(Debug)]
pub struct DbUrl {
    pub db_base: String,
    pub user: String,
    pass: String,
    pub host: String,
    port: String,
    pub db_name: String
    }
    impl DbUrl {
        pub fn new(base: &str, un: &str, pw: &str, hst: &str, prt: &str, dbn: &str) -> DbUrl{
            DbUrl{
                db_base:base.to_string(),
                user:un.to_string(),
                pass:pw.to_string(),
                host:hst.to_string(),
                port:prt.to_string(),
                db_name:dbn.to_string()
            } // end db_url instance-def

        } //end fn new
        pub fn create_path (&self) -> String{
            format!("{}://{}:{}@{}:{}/{}", 
            self.db_base, self.user, self.pass, self.host, self.port, self.db_name)
        }
        pub fn db_name(&self) -> &str {
            &self.db_name
        }
    } //end impl db_url

    

    //String, String is the return type for the function below. It was removed due to error during testing. and the return value removed below...
pub fn run() -> () {
    splash::run();
    let mut dbt = String::new();
    println!("Enter the Database type (eg. mysql): ");
    let dbtn = std::io::stdin().read_line(&mut dbt);

    let mut usr= String::new();
    println!("Enter Username: ");
    let usrn = std::io::stdin().read_line(&mut usr);

    let mut psw = String::new();
    println!("Enter Password: ");
    let pswn = std::io::stdin().read_line(&mut psw);

    println!("Enter Host: ");
    let mut hst = String::new();
    let hstn = std::io::stdin().read_line(&mut hst);

    println!("Enter Port: ");
    let mut prt = String::new();
    let prtn = std::io::stdin().read_line(&mut prt);

    println!("Enter Database Name: ");
    let mut dbn= String::new();
    let dbN = std::io::stdin().read_line(&mut dbn);
    
    // println!("Enter Name of Table holding unknown car info: ");
    // let mut unknown_table= String::new();
    // let unkn_tab = std::io::stdin().read_line(&mut unknown_table);

    // println!("Enter Name of Table that will be storing obtained car details(**not the unknownID's table**): ");
    // let mut car_details_table= String::new();
    // let car_tab = std::io::stdin().read_line(&mut car_details_table);

    let mut dbu = DbUrl::new(dbt.as_str(), &usr.as_str(), &psw, &hst, &prt, &dbn);
    //may use regex to correct this later ---->
    let dburl: String = format!("{}://{}:{}@{}:{}/{}",dbt.trim(), usr.trim(), psw.trim(), hst.trim(), prt.trim(), dbn.trim());
    
    //printing for now...
    println!("{} {}", &dburl, &dbn);
    connection::run(dburl, &dbn);
    //(dburl, dbn)
    } //end run