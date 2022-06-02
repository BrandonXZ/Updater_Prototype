/* This is the actual XML formatter for sending wsdl requests for car info to Rail Inc. Umler systems. Please disregard the webservice formatter in the 
dbInterface module, it's only there for demonstration purposes to show whats happening behing the scenes.
Rust doesn't support variadic function arguments, therefore every necessary operation is performed using a vector of <T> or called multiple times 
by an iterator of the vec.
*/
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use chrono::Utc;
use crate::settings;


pub fn run (schema:Vec<String>, unknown_IDs:Vec<String>) -> String {
    let schema = schema.clone();
    let xml_column_vec = elements_vec(schema);
    let (car_initials, car_numbers) = equipment_initials_check(unknown_IDs);
    let car_numbers = len_check(car_numbers);
    let car_initials = initial_len_check(car_initials);
    xml_stmt_caller(car_initials, car_numbers, xml_column_vec);
    "\nComplete WSDL run\n".to_string()
}

pub fn elements_vec (schema_columns:Vec<String>) -> Vec<String> {
    let mut xml_column_vec = vec![];

    for columns in schema_columns.iter(){
        let mut xml_column = format!("<dyn:ElementId>{}</dyn:ElementId>", columns); 
        xml_column_vec.push(xml_column);
    }   

    //println!("current column vec format --->{:?}", xml_column_vec.clone());          Debug//shows columns being requested from Umler in Xml wsdl format
    xml_column_vec
}


pub fn equipment_initials_check (equipment_identifiers:Vec<String>) -> (Vec<String>, Vec<String>) {
    let initials_vec = equipment_identifiers;
    let mut car_initials = vec![];
    let mut car_numbers = vec![];
    let mut joined_initials = " ".to_string();
    let mut joined_numbers = " ".to_string();

    for items in initials_vec {
        let mut char_counter = 0;
        let mut num_count = 0;
        let mut eq_initial:Vec<char> = vec![];
        let mut eq_number:Vec<char> = vec![];

        for i in items.chars() {
                if i.is_alphabetic() {
                    char_counter = char_counter + 1;
                    eq_initial.push(i);
                } else if i.is_numeric() {
                    num_count = num_count + 1;
                    eq_number.push(i);
                }


                //need a better way to check if string includes atleast 2 letters and any # of digits
                // if char_counter == 0 || num_count == 0 { println!("Either no letters or no numbers!!")} 

        }

        //println!("Eq_initial ->{:?}", eq_initial.clone());                          //Debug, used for Equipment Id troubleshooting
        //println!("Eq_number ->{:?}", eq_number.clone());
        //println!("letters = {}, numbers = {}\n", char_counter, num_count);
        joined_initials = eq_initial.into_iter().collect();
        joined_numbers = eq_number.into_iter().collect();
        //println!("Joined is...{}{}",joined_initials, joined_numbers);               //Debug, shows invalid Equipment Id if a number proceeds equip initial letters
        car_initials.push(joined_initials);
        car_numbers.push(joined_numbers);
    }

    (car_initials, car_numbers)
}

pub fn len_check (current_IDs: Vec<String>) -> Vec<String> {
    let webservice_comm_type = "send function".to_string();
    println!("ID Formatter\n");
    let mut new_ID_vec = vec![];
    

    if current_IDs.is_empty() {
        let Errornote = "No ID received for formatting".to_string();
        settings::logthis_webService(Errornote, webservice_comm_type.clone());
    } else {
        for current in current_IDs {
            let lognote = format!("Current unknown equipment number formatting is-----> {}", current);
            println!("{}", lognote);
            settings::logthis_nonError(lognote);
           if current.len() != 10 {
            //println!("not 10 characters...\n");                         //Remove during production
            let zeros_to_add = 10 - current.len();
            let zeros = "0".repeat(zeros_to_add);
            let formed = format!("{}{}",zeros, current);
            new_ID_vec.push(formed);
           } else {
               //println!("{} is 10 characters long...", current);         //Remove during production
               new_ID_vec.push(current);
           }
            
        }
        println!("formatted vec is ---->{:?}", new_ID_vec);
    }

    new_ID_vec
}

pub fn initial_len_check(initials: Vec<String>) -> Vec<String> {
    let cloner = initials.clone();
    for item in initials{
        if item.len() < 2 {
            println!("letter quantity doesn't meet spec, atleast 2 needed for each initial\n");
        }
    }
    cloner
}



pub fn xml_stmt_caller (initials: Vec<String>, numbers: Vec<String>, columns:Vec<String> ) {
    let initials = initials.clone();
    let mut numbers = numbers.clone();
    let columns = columns.clone();
    for (it, it2) in initials.iter().zip(numbers.iter_mut()) {
        xml_stmt_maker(it.to_string(), it2.to_string(), columns.clone())
    }
}

pub fn xml_stmt_maker (Equip_Initial: String, Equip_Number: String, Columns: Vec<String>) -> ()/*change to String */ {
    let xml_stmt_vec: Vec<String> = vec![];
    let USERID = "DUOSUSER".to_string();
    let PASSWORD = "Centraco2020".to_string();
    let TIMESTAMP = Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let INSTANCE_ID = 1234;
    let SCAC = "D795".to_string();
    let TRACE_ID = 111234;
    let ENV_TYPE = "TEST".to_string();
    
/* Correct format for the WSDL SOAP message to Umler*/
    // let Original = 
    // "<soapenv:Envelope xmlns:soapenv=\"http://schemas.xmlsoap.org/soap/envelope/\" 
    // xmlns:equ=\"http://schemas.railinc.com/eq/umler/service/dynamic/equipmentservice\"
    // xmlns:head=\"http://schemas.railinc.com/eq/umler/service/header\"
    // xmlns:dyn=\"http://schemas.railinc.com/eq/umler/equipment/dynamic\">
    //     <soapenv:Header>
    //     <wsse:Security soapenv:mustUnderstand=\"1\" xmlns:wsse=\"http://docs.oasis-
    //     open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd\">
    //     <wsse:UsernameToken>
    //     <wsse:Username>DUOSUSER</wsse:Username>
    //     <wsse:Password>Centraco2020</wsse:Password>
    //     </wsse:UsernameToken>
    //     </wsse:Security>
    //     </soapenv:Header>
    //     <soapenv:Body>
    //     <equ:EquipmentSelectElementsQueryRequest>
    //     <equ:DocumentHeader>
    //     <head:Sender>
    //     <head:OrganizationContact>
    //     <head:OrganizationIdentification>
    //     <head:OrganizationName>D795</head:OrganizationName>
    //     <head:OrganizationIdentifier>D795</head:OrganizationIdentifier>
    //     </head:OrganizationIdentification>                </head:OrganizationContact>
    //     </head:Sender>
    //     <head:Receiver>
    //     <head:OrganizationContact>
    //     <head:OrganizationIdentification>
    //     <head:OrganizationName>D795</head:OrganizationName>
    //     <head:OrganizationIdentifier>D795</head:OrganizationIdentifier>
    //     </head:OrganizationIdentification>
    //     </head:OrganizationContact>
    //     </head:Receiver>
    //     <head:DocumentDetail>
    //     <head:CreateDateAndTime>2022-05-31T20:22:33</head:CreateDateAndTime>
    //     <head:InstanceIdentifier>1234</head:InstanceIdentifier>
    //     </head:DocumentDetail>
    //     </equ:DocumentHeader>
    //     <equ:ServiceContext>
    //     <head:CustomerIdentity>
    //     <head:Customer>
    //     <head:SCAC>D795</head:SCAC>
    //     </head:Customer>
    //     </head:CustomerIdentity>
    //     <head:ClientContext>
    //     <head:TraceID>111234</head:TraceID>
    //     <head:EnvironmentType>TEST</head:EnvironmentType>
    //     </head:ClientContext>
    //     </equ:ServiceContext>
    //     <equ:EquipmentQueryRequestDetail>
    //     <dyn:EquipmentQuerySearchCriteria>
    //     <dyn:EquipmentID>
    //     <dyn:EquipmentInitial>CN </dyn:EquipmentInitial>
    //     <dyn:EquipmentNumber>136403</dyn:EquipmentNumber>
    //     </dyn:EquipmentID>             </dyn:EquipmentQuerySearchCriteria>
    //     <dyn:EquipmentQueryElementSelection>
    //     <dyn:ElementId>A252</dyn:ElementId>
    //     <dyn:ElementId>RBDT</dyn:ElementId>
    //     </dyn:EquipmentQueryElementSelection>
    //     <dyn:EquipmentQueryInspectionSelection>
    //     <dyn:InspectionType>ARI</dyn:InspectionType>
    //     <dyn:InspectionType>ABT</dyn:InspectionType>
    //     <dyn:InspectionType>FRA</dyn:InspectionType>
    //     </dyn:EquipmentQueryInspectionSelection>
    //     <dyn:IncludeConflictElements>true</dyn:IncludeConflictElements>
    //     </equ:EquipmentQueryRequestDetail>
    //     </equ:EquipmentSelectElementsQueryRequest>
    //     </soapenv:Body>
    //     </soapenv:Envelope>".to_string();


    let xml_part1 = format!("<soapenv:Envelope xmlns:soapenv=\"http://schemas.xmlsoap.org/soap/envelope/\" 
    xmlns:equ=\"http://schemas.railinc.com/eq/umler/service/dynamic/equipmentservice\"
    xmlns:head=\"http://schemas.railinc.com/eq/umler/service/header\"
    xmlns:dyn=\"http://schemas.railinc.com/eq/umler/equipment/dynamic\">
        <soapenv:Header>
        <wsse:Security soapenv:mustUnderstand=\"1\" xmlns:wsse=\"http://docs.oasis-
        open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd\">
        <wsse:UsernameToken>
        <wsse:Username>{}</wsse:Username>
        <wsse:Password>{}</wsse:Password>
        </wsse:UsernameToken>
        </wsse:Security>
        </soapenv:Header>
        <soapenv:Body>
        <equ:EquipmentSelectElementsQueryRequest>
        <equ:DocumentHeader>
        <head:Sender>
        <head:OrganizationContact>
        <head:OrganizationIdentification>
        <head:OrganizationName>{}</head:OrganizationName>
        <head:OrganizationIdentifier>{}</head:OrganizationIdentifier>
        </head:OrganizationIdentification>                </head:OrganizationContact>
        </head:Sender>
        <head:Receiver>
        <head:OrganizationContact>
        <head:OrganizationIdentification>
        <head:OrganizationName>{}</head:OrganizationName>
        <head:OrganizationIdentifier>{}</head:OrganizationIdentifier>
        </head:OrganizationIdentification>
        </head:OrganizationContact>
        </head:Receiver>
        <head:DocumentDetail>
        <head:CreateDateAndTime>{}</head:CreateDateAndTime>
        <head:InstanceIdentifier>{}</head:InstanceIdentifier>
        </head:DocumentDetail>
        </equ:DocumentHeader>
        <equ:ServiceContext>
        <head:CustomerIdentity>
        <head:Customer>
        <head:SCAC>{}</head:SCAC>
        </head:Customer>
        </head:CustomerIdentity>
        <head:ClientContext>
        <head:TraceID>{}</head:TraceID>
        <head:EnvironmentType>TEST</head:EnvironmentType>
        </head:ClientContext>
        </equ:ServiceContext>
        <equ:EquipmentQueryRequestDetail>
        <dyn:EquipmentQuerySearchCriteria>
        <dyn:EquipmentID>
        <dyn:EquipmentInitial>{}</dyn:EquipmentInitial>
        <dyn:EquipmentNumber>{}</dyn:EquipmentNumber>
        </dyn:EquipmentID>             </dyn:EquipmentQuerySearchCriteria> 
        <dyn:EquipmentQueryElementSelection>",
        USERID.clone(), 
        PASSWORD.clone(), 
        SCAC.clone(), SCAC.clone(), SCAC.clone(), SCAC.clone(),
        TIMESTAMP,
        INSTANCE_ID,
        SCAC.clone(),
        TRACE_ID,
        Equip_Initial,
        Equip_Number);

    let column_part = Columns.join("\n");

    let last_part = "</dyn:EquipmentQueryElementSelection>
    <dyn:EquipmentQueryInspectionSelection>
    <dyn:InspectionType>ARI</dyn:InspectionType>
    <dyn:InspectionType>ABT</dyn:InspectionType>
    <dyn:InspectionType>FRA</dyn:InspectionType>
    </dyn:EquipmentQueryInspectionSelection>
    <dyn:IncludeConflictElements>true</dyn:IncludeConflictElements>
    </equ:EquipmentQueryRequestDetail>
    </equ:EquipmentSelectElementsQueryRequest>
    </soapenv:Body>
    </soapenv:Envelope>".to_string();

    let final_stmt = format!(
        "
             {}
             {}
             {}", xml_part1, column_part, last_part);
        

    println!("{}", final_stmt);
}