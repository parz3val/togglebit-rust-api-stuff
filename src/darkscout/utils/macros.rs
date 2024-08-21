use crate::darkscout::web::json_error;
#[macro_export]
macro_rules! json_err {
    () => {
        json_error(None, None)
    };
    ($x: expr, $y: expr) => {
        json_error::<_>(Option::from($x), Option::from($y)) // do not give type here
        // this will make the type concrete at compile time and break
        // the generics
    }
}
#[macro_export]
macro_rules! unwrap_or_else_string {
    ($x:expr, $y:expr) => {
        match $x {
            Some(val) => Some(val),
            None => match $y {
                Some(val) => Some(val),
                None => Some(String::from("")),
            },
        }
    };
}