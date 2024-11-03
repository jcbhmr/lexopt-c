#[repr(C)]
pub enum lexopt_error {
    missing_value {
        option: *mut std::os::raw::c_char,
    },
    unexpected_option(std::ptr::NonNull<std::os::raw::c_char>),
    unexpected_argument(std::ptr::NonNull<std::os::raw::c_char>),
    unexpected_value {
        option: std::ptr::NonNull<std::os::raw::c_char>,
        value: std::ptr::NonNull<std::os::raw::c_char>,
    },
    parsing_failed {
        value: std::ptr::NonNull<std::os::raw::c_char>,
        error: std::ptr::NonNull<std::os::raw::c_char>,
    },
    non_unicode_value(std::ptr::NonNull<std::os::raw::c_char>),
    custom(std::ptr::NonNull<std::os::raw::c_char>),
}

impl From<lexopt::Error> for lexopt_error {
    fn from(value: lexopt::Error) -> Self {
        match value {
            lexopt::Error::MissingValue { option } => {
                let option = if let Some(option) = option {
                    let option = std::ffi::CString::new(option).unwrap();
                    let option = option.into_raw();
                    option
                } else {
                    std::ptr::null_mut()
                };
                lexopt_error::missing_value { option }
            },
            lexopt::Error::UnexpectedOption(option) => {
                let option = std::ffi::CString::new(option).unwrap();
                let option = option.into_raw();
                lexopt_error::unexpected_option(unsafe { std::ptr::NonNull::new_unchecked(option) })
            },
            lexopt::Error::UnexpectedArgument(arg) => {
                let arg = arg.into_encoded_bytes();
                let arg = std::ffi::CString::new(arg).unwrap();
                let arg = arg.into_raw();
                lexopt_error::unexpected_argument(unsafe { std::ptr::NonNull::new_unchecked(arg) })
            },
            lexopt::Error::UnexpectedValue { option, value } => {
                let option = std::ffi::CString::new(option).unwrap();
                let option = option.into_raw();
                let value = value.into_encoded_bytes();
                let value = std::ffi::CString::new(value).unwrap();
                let value = value.into_raw();
                lexopt_error::unexpected_value {
                    option: unsafe { std::ptr::NonNull::new_unchecked(option) },
                    value: unsafe { std::ptr::NonNull::new_unchecked(value) },
                }
            },
            lexopt::Error::ParsingFailed { value, error } => {
                let value = std::ffi::CString::new(value).unwrap();
                let value = value.into_raw();
                let error = error.to_string();
                let error = std::ffi::CString::new(error).unwrap();
                let error = error.into_raw();
                lexopt_error::parsing_failed {
                    value: unsafe { std::ptr::NonNull::new_unchecked(value) },
                    error: unsafe { std::ptr::NonNull::new_unchecked(error) },
                }
            },
            lexopt::Error::NonUnicodeValue(value) => {
                let value = value.into_encoded_bytes();
                let value = std::ffi::CString::new(value).unwrap();
                let value = value.into_raw();
                lexopt_error::non_unicode_value(unsafe { std::ptr::NonNull::new_unchecked(value) })
            },
            lexopt::Error::Custom(error) => {
                let error = error.to_string();
                let error = std::ffi::CString::new(error).unwrap();
                let error = error.into_raw();
                lexopt_error::custom(unsafe { std::ptr::NonNull::new_unchecked(error) })
            },
        }
    }
}

impl From<&lexopt_error> for &lexopt::Error {
    fn from(value: &lexopt_error) -> Self {
        match value {
            lexopt_error::missing_value { option } => {
                let option = if option.is_null() {
                    None
                } else {
                    let option = unsafe { std::ffi::CString::from_raw(option.clone()) };
                    Some(option.into_string().unwrap())
                };
                &lexopt::Error::MissingValue { option: option }
            },
            lexopt_error::unexpected_option(option) => {
                let option = unsafe { std::ffi::CString::from_raw(option.as_ptr()) };
                let option = option.into_string().unwrap();
                lexopt::Error::UnexpectedOption(option)
            },
            lexopt_error::unexpected_argument(arg) => {
                let arg = unsafe { std::ffi::CString::from_raw(arg.as_ptr()) };
                let arg = arg.into_bytes();
                let arg = unsafe { std::ffi::OsString::from_encoded_bytes_unchecked(arg) };
                lexopt::Error::UnexpectedArgument(arg)
            },
            lexopt_error::unexpected_value { option, value } => {
                let option = unsafe { std::ffi::CString::from_raw(option.as_ptr()) };
                let option = option.into_string().unwrap();
                let value = unsafe { std::ffi::CString::from_raw(value.as_ptr()) };
                let value = value.into_bytes();
                let value = unsafe { std::ffi::OsString::from_encoded_bytes_unchecked(value) };
                lexopt::Error::UnexpectedValue { option, value }
            },
            lexopt_error::parsing_failed { value, error } => {
                let value = unsafe { std::ffi::CString::from_raw(value.as_ptr()) };
                let value = value.into_string().unwrap();
                let error = unsafe { std::ffi::CString::from_raw(error.as_ptr()) };
                let error = error.into_string().unwrap();
                lexopt::Error::ParsingFailed { value, error: error.into() }
            },
            lexopt_error::non_unicode_value(value) => {
                let value = unsafe { std::ffi::CString::from_raw(value.as_ptr()) };
                let value = value.into_bytes();
                let value = unsafe { std::ffi::OsString::from_encoded_bytes_unchecked(value) };
                lexopt::Error::NonUnicodeValue(value)
            },
            lexopt_error::custom(error) => {
                let error = unsafe { std::ffi::CString::from_raw(error.as_ptr()) };
                let error = error.into_string().unwrap();
                lexopt::Error::Custom(error.into())
            },
        }
    }
}

#[no_mangle]
pub extern "C" fn lexopt_error_to_string(error: std::ptr::NonNull<lexopt_error>) -> std::ptr::NonNull<std::os::raw::c_char> {
    let error = unsafe { error.read() };
    let error: lexopt::Error = error.into();
    let string = error.to_string();
    let string = std::ffi::CString::new(string).unwrap();
    let string = string.into_raw();
    unsafe { std::ptr::NonNull::new_unchecked(string) }
}

#[no_mangle]
pub extern "C" fn lexopt_error_debug_to_string(error: std::ptr::NonNull<lexopt_error>) -> std::ptr::NonNull<std::os::raw::c_char> {
    let error = unsafe { error.copy() };
    let error: lexopt::Error = error.into();
    let string = format!("{:?}", error);
    let string = std::ffi::CString::new(string).unwrap();
    let string = string.into_raw();
    unsafe { std::ptr::NonNull::new_unchecked(string) }
}