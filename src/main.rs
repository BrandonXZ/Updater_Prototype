#![allow(non_snake_case)]
/* Author: Brandon Minter
* Date of release | Q2 '22
* Duos Technologies Inc. 05/10/2022
*
* Description: This program, when compiled to an executable, will be activated via windows task scheduler on a nightly basis
* It will create a pooled connection to our main local database and search a table that stores AEI tags which are new and not currently generating information
* If this table has any new AEI tags, the program will send an active WSDL webservices request to Umler to get the missing train car info associated with the AEI's
* The program will then update our main local database to add the missing car info.
* Our databases for each location may need a program to interface with our main db and sync the newly obtained information 
* since we have unique db's running at each site. This program will also log any issue's should they arise during the process. 
*/
mod connection;
mod pathprep;
mod dbStructs;
mod settings;
mod menu;
mod dbInterface;


fn main() {
    menu::run();
    
}
