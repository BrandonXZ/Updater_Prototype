use crate::settings;

/* 
* This module defines the Async function that will listen for a response from the umler webservice and a function to format this information before it 
* is passed back to the dbInterface module and added to our car details table.
*
*********The add function in dbInterface.rs  is the mock up of what is to be done in this module*********
*/
#[tokio::main]
pub async fn send_to_umler(wsdl_stmt: String, current: String) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let WSDL_response = client
    .post("https://api.umler.tst.railinc.com:443/umler-query-service/services/ws/UmlerEquipmentQueryService")
    .body(wsdl_stmt)
    .send()
    .await?
    .text()
    .await?;
    let cloner = WSDL_response.clone();

    /*Reqwest allows for dealing with errors received from webservice but this will be left out pending a message from Tracy regarding accurate column identifiers that
    are not reflected in our database possibly due to us updating something, will call if no email received by 3pm today, for now this is left out to avoid dozens of 
    errors filling up the error logs during demo.  */

    if cloner.is_empty() {
        let Tx_type = "WebService Reply".to_string();
        let lognote = format!("Error no response received----> {}", current);
        println!("{}", lognote);
        settings::logthis_webService(lognote, Tx_type);
    } else {
        let lognote = format!("Response received from Umler for car ID-----> {}", current);
            println!("{}", lognote);
            settings::logthis_nonError(lognote);
    }


    println!("Response from Umler ---> \n{:?}", WSDL_response);
    
    Ok(())
}