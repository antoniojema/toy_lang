#[macro_export]
macro_rules! unwrap_result_or_return {
    ( $e:expr, $r:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return $r,
        }
    }
}
pub(crate) use unwrap_result_or_return;

#[macro_export]
macro_rules! unwrap_option_or_return {
    ( $e:expr, $r:expr ) => {
        match $e {
            Some(x) => x,
            None => return $r,
        }
    }
}
pub(crate) use unwrap_option_or_return;

#[macro_export]
macro_rules! unwrap_or_return {
    ($e:expr) => {
        match $e {
            Ok(x) => x,
            Err(err) => {return Err(err);},
        }
    }
}
pub(crate) use unwrap_or_return;