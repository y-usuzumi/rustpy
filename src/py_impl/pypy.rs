use libc;
use std;

#[link(name=":libpypy-c.so")]
extern {
    fn rpython_startup_code();
    fn pypy_setup_home(home: *const libc::c_char, verbose: libc::c_int) -> libc::c_int;
    fn pypy_execute_source(source: *const libc::c_char) -> libc::c_int;
}

pub fn execute(code: &String) -> i8 {
    unsafe {
        rpython_startup_code();
        // let res = pypy_setup_home(std::ptr::null(), 1);
        let home = std::ffi::CString::new("/opt/pypy").unwrap();
        let res = pypy_setup_home(home.as_ptr(), 1);
        if res != 0 {
            println!("Error setting pypy home!");
            return 1;
        }
        let c_code = std::ffi::CString::new(code.clone()).unwrap();
        let res = pypy_execute_source(c_code.as_ptr());
        if res != 0 {
            println!("Error calling pypy_execute_source!");
            return 2;
        }
        return 0;
    }
}
