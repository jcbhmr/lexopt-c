#[no_mangle]
pub extern "C" fn lexopt_parser_next(parser: *mut lexopt::Parser, out_arg: *mut lexopt::Arg) -> *const lexopt::Error {
    let parser = unsafe {parser.as_mut()}.expect("parser is null");
    let out_arg = unsafe{out_arg.as_mut()}.expect("out_arg is null");
    match parser.next() {
        Ok(Some(arg)) => {
            *out_arg = arg;
            std::ptr::null()
        }
        Ok(None) => std::ptr::null(),
        Err(err) => Box::into_raw(Box::new(err)),
    }
}