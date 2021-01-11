#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use pgx::*;

use std::ffi::{CStr, CString};
use std::str::Utf8Error;
use std::{env, ptr};

use std::sync::{Mutex, Arc};

#[macro_use]
extern crate lazy_static;

pg_module_magic!();

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

static failed: &str = "failed";

pub struct GraalVM {
    pub thread: i64,
}

unsafe fn isolate() -> Arc<Mutex<GraalVM>> {
    let mut isolate: *mut graal_isolate_t = ptr::null_mut();
    let mut thread: *mut graal_isolatethread_t = ptr::null_mut();
    graal_create_isolate(ptr::null_mut(), &mut isolate, &mut thread);

    let ptr = thread as i64;
    return Arc::new(Mutex::new(GraalVM {
        thread: ptr,
    }));
}

lazy_static! {
    static ref GRAALVM: Arc<Mutex<GraalVM>> = unsafe { isolate() };
}

fn eval(expr: String) -> Result<String, Utf8Error> {
    unsafe {
        let c_string = CString::new(expr).expect(failed);
        let ptr = c_string.as_ptr();

        let graalvm = GRAALVM.lock().unwrap();

        let result = eval_string(
            graalvm.thread,
            ptr,
        );

        let cstr = CStr::from_ptr(result).to_str()?;
        let ret = String::from(cstr);

        drop(graalvm);

        return Ok(ret);
    }
}

#[pg_extern]
fn plsci(expr: String) -> String {
    return eval(expr).unwrap();
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_hello_plsci() {
        assert_eq!("Hello, plsci", crate::plsci());
    }

}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
