//This section gets user input and preps the path to the database and ensures that user is not putting invalid options.

#![allow(unused_variables)]
#![allow(unused_mut)]
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

    use std::fmt::Debug;

pub fn run() -> (String, String) {
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
    
    let mut dbu = DbUrl::new(dbt.as_str(), &usr.as_str(), &psw, &hst, &prt, &dbn);
    let dburl: String = format!("{}://{}:{}@{}:{}/{}",dbt.trim(), usr.trim(), psw.trim(), hst.trim(), prt.trim(), dbn.trim());
    
    (dburl, dbn)
    } //end run