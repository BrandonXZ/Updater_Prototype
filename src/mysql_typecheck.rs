/*Credit for this goes to the great Anatoly** This code simply checks the value of a mysql return in the event the source schema is unknown. currently on MIT
but credit is given anyways...
*/
#![allow(unreachable_code)]

use mysql::*;
use mysql::prelude::*;
use mysql_common::time03::PrimitiveDateTime;




pub fn run () {

    let via_test_protocol: u32 = from_value(Value::Bytes(b"65536".to_vec()));
    let via_bin_protocol: u32 = from_value(Value::UInt(65536));
    println!("Via test protocol is: {}...Via bin protocol is: {}", via_test_protocol, via_bin_protocol);

    let unknown_val:Value; // This will have an error below until code that passes a mysql value is provided to this func...

    // Maybe it is a float?
    let unknown_val = match from_value_opt::<f64>(unknown_val) {
        Ok(float) => {
            println!("A float value: {}", float);
            //return Ok(()).unwrap();
            return
        }
        Err(FromValueError(unknown_val)) => unknown_val,
    };

    // Or a string?
    let unknown_val = match from_value_opt::<String>(unknown_val) {
        Ok(string) => {
            println!("A string value: {}", string);
            //return Ok(()).unwrap();
            return
        }
        Err(FromValueError(unknown_val)) => unknown_val,
    };

    // Screw this, I'll simply match on it
    match unknown_val {
        val @ Value::NULL => {
            println!("An empty value: {:?}", from_value::<Option<u8>>(val))
        },
        val @ Value::Bytes(..) => {
            // It's non-utf8 bytes, since we already tried to convert it to String
            println!("Bytes: {:?}", from_value::<Vec<u8>>(val))
        }
        val @ Value::Int(..) => {
            println!("A signed integer: {}", from_value::<i64>(val))
        }
        val @ Value::UInt(..) => {
            println!("An unsigned integer: {}", from_value::<u64>(val))
        }
        Value::Float(..) => unreachable!("already tried"),
        val @ Value::Double(..) => {
            println!("A double precision float value: {}", from_value::<f64>(val))
        }
        val @ Value::Date(..) => {
            // use time::PrimitiveDateTime;
            println!("A date value: {}", from_value::<PrimitiveDateTime>(val))
        }
        val @ Value::Time(..) => {
            use std::time::Duration;
            println!("A time value: {:?}", from_value::<Duration>(val))
        }
    }
}