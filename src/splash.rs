

pub fn run() {
    
    println!("\n ");
    showInstructions();
    println!("\n ");

}

pub fn showWelcome() {
    let message = "This is the Duos Car AEI information updater.
    \nThis Program checks our local database table for AEI tags 
    \nthat are not recognized during train passage at a RIP.
    \nIt then communicates with Umler webservices to obtain that information and update our local database tables.
    \nBy typing \"setup\", the user can insert information for the table that holds records of unrecognized train cars and the table for storing the updated information.
    \nThis program is intended to run automatically after setup is complete but can also be run manually by typing \"automated-run\" at the menu.
    \nThis program will need to be configured in windows task scheduler during installation using the attached batch file.";
    println!("{}", message);
}

pub fn showInstructions () {
    let message = "This program is currently only built to work with mysql, SQL server version will be released should free time permit.
    \nThe host should be \"localhost\", or an ip address that the database is running on with the appropriate port #.
    \nThis program will pass any MySQL errors directly to the log file if encountered after setup.
    \nTables can be updated freely, but for security purposes: if a new database is needed, the local database reference file(db_ref.txt) will need to be deleted.
    \nAfter the file is deleted, setup will need to be run again...";
    println!("{}", message);
}