
/// check if the param is an error (== 0)
/// if it is, get the error code from bass,
/// and return from the function with said error
#[macro_export]
macro_rules! check_bass_err {
    ($check:expr) => {
        check_bass_err_val!($check, 0)
    };
}

/// check if the param is equal to an error value
/// if it is, get the error code from bass,
/// and return from the function with said error
#[macro_export]
macro_rules! check_bass_err_val {
    ($check:expr, $err_val:expr) => {
        if $check == $err_val {
            return Err(BassError::from_code(BASS_ErrorGetCode()));
        }
    };
}

/// check if the param is true
/// if it is, get the error code from bass,
/// and return from the function with said error
#[macro_export]
macro_rules! check_bass_err_bool {
    ($check:expr) => {
        if $check {
            return Err(BassError::from_code(BASS_ErrorGetCode()));
        }
    };
}