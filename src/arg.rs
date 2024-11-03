#[repr(C)]
pub enum lexopt_arg {
    short(u32),
    long(std::ptr::NonNull<std::os::raw::c_char>),
    value(std::ptr::NonNull<std::os::raw::c_char>),
}

impl From<lexopt::Arg<'_>> for lexopt_arg {
    fn from(value: lexopt::Arg) -> Self {
        match value {
            lexopt::Arg::Short(c) => {
                let c = c as u32;
                lexopt_arg::short(c)
            }
            lexopt::Arg::Long(s) => {
                let s = std::ffi::CString::new(s).unwrap();
                let s = s.into_raw();
                lexopt_arg::long(unsafe { std::ptr::NonNull::new_unchecked(s) })
            }
            lexopt::Arg::Value(s) => {
                let s = s.into_encoded_bytes();
                let s = unsafe { std::ffi::CString::from_vec_unchecked(s) };
                let s = s.into_raw();
                lexopt_arg::value(unsafe { std::ptr::NonNull::new_unchecked(s) })
            }
        }
    }
}

impl From<lexopt_arg> for lexopt::Arg<'_> {
    fn from(arg: lexopt_arg) -> Self {
        match arg {
            lexopt_arg::short(c) => {
                let c = char::from_u32(c).unwrap();
                lexopt::Arg::Short(c)
            }
            lexopt_arg::long(s) => {
                let s = unsafe { std::ffi::CStr::from_ptr(s.as_ptr()) };
                let s = s.to_str().unwrap();
                lexopt::Arg::Long(s)
            }
            lexopt_arg::value(s) => {
                let s = unsafe { std::ffi::CString::from_raw(s.as_ptr()) };
                let s = s.into_bytes();
                let s = unsafe { std::ffi::OsString::from_encoded_bytes_unchecked(s) };
                lexopt::Arg::Value(s)
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn lexopt_arg_unexpected(arg: lexopt_arg) -> crate::lexopt_error {
    let arg: lexopt::Arg = arg.into();
    let err = arg.unexpected();
    err.into()
}
