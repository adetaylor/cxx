use cxx_test_suite::ffi;
use std::cell::Cell;
use std::ffi::CStr;

thread_local! {
    static CORRECT: Cell<bool> = Cell::new(false);
}

#[no_mangle]
extern "C" fn cxx_test_suite_set_correct() {
    CORRECT.with(|correct| correct.set(true));
}

macro_rules! check {
    ($run:expr) => {{
        CORRECT.with(|correct| correct.set(false));
        $run;
        assert!(CORRECT.with(|correct| correct.get()), stringify!($run));
    }};
}

#[test]
fn test_c_return() {
    let shared = ffi::Shared { z: 2020 };

    assert_eq!(2020, ffi::c_return_primitive());
    assert_eq!(2020, ffi::c_return_shared().z);
    assert_eq!(2020, *ffi::c_return_box());
    ffi::c_return_unique_ptr();
    assert_eq!(2020, *ffi::c_return_ref(&shared));
    assert_eq!("2020", ffi::c_return_str(&shared));
    assert_eq!("2020", ffi::c_return_rust_string());
    assert_eq!(
        "2020",
        ffi::c_return_unique_ptr_string()
            .as_ref()
            .unwrap()
            .to_str()
            .unwrap()
    );
    assert_eq!(
        4,
        ffi::c_return_unique_ptr_vector_u8()
            .as_ref()
            .unwrap()
            .size()
    );
    assert_eq!(
        200_u8,
        ffi::c_return_unique_ptr_vector_u8()
            .as_ref()
            .unwrap()
            .into_iter()
            .sum()
    );
    assert_eq!(
        200.5_f64,
        ffi::c_return_unique_ptr_vector_f64()
            .as_ref()
            .unwrap()
            .into_iter()
            .sum()
    );
    assert_eq!(
        2,
        ffi::c_return_unique_ptr_vector_shared()
            .as_ref()
            .unwrap()
            .size()
    );
    assert_eq!(
        2021_usize,
        ffi::c_return_unique_ptr_vector_shared()
            .as_ref()
            .unwrap()
            .into_iter()
            .map(|o| o.z)
            .sum()
    );
}

#[test]
fn test_c_try_return() {
    assert_eq!((), ffi::c_try_return_void().unwrap());
    assert_eq!(2020, ffi::c_try_return_primitive().unwrap());
    assert_eq!(
        "logic error",
        ffi::c_fail_return_primitive().unwrap_err().what(),
    );
    assert_eq!(
        "ok",
        ffi::c_try_return_string()
            .unwrap()
            .as_ref()
            .unwrap()
            .to_string()
    );
    assert_eq!(
        "logic error getting string",
        ffi::c_fail_return_string().unwrap_err().what(),
    );
    assert_eq!(2020, *ffi::c_try_return_box().unwrap());
    assert_eq!("2020", *ffi::c_try_return_ref(&"2020".to_owned()).unwrap());
    assert_eq!("2020", ffi::c_try_return_str("2020").unwrap());
    assert_eq!("2020", ffi::c_try_return_rust_string().unwrap());
    assert_eq!(
        "2020",
        ffi::c_try_return_unique_ptr_string()
            .unwrap()
            .as_ref()
            .unwrap()
    );
}

#[test]
fn test_c_take() {
    let unique_ptr = ffi::c_return_unique_ptr();

    check!(ffi::c_take_primitive(2020));
    check!(ffi::c_take_shared(ffi::Shared { z: 2020 }));
    check!(ffi::c_take_box(Box::new(2020)));
    check!(ffi::c_take_ref_c(unique_ptr.as_ref().unwrap()));
    check!(ffi::c_take_unique_ptr(unique_ptr));
    check!(ffi::c_take_str("2020"));
    check!(ffi::c_take_rust_string("2020".to_owned()));
    check!(ffi::c_take_unique_ptr_string(
        ffi::c_return_unique_ptr_string()
    ));
    check!(ffi::c_take_unique_ptr_vector_u8(
        ffi::c_return_unique_ptr_vector_u8()
    ));
    check!(ffi::c_take_unique_ptr_vector_f64(
        ffi::c_return_unique_ptr_vector_f64()
    ));
    check!(ffi::c_take_unique_ptr_vector_shared(
        ffi::c_return_unique_ptr_vector_shared()
    ));

    check!(ffi::c_take_vec_u8(&[86_u8, 75_u8, 30_u8, 9_u8].to_vec()));

    check!(ffi::c_take_vec_shared(&vec![
        ffi::Shared { z: 1010 },
        ffi::Shared { z: 1011 }
    ]));
}

#[test]
fn test_c_call_r() {
    fn cxx_run_test() {
        extern "C" {
            fn cxx_run_test() -> *const i8;
        }
        let failure = unsafe { cxx_run_test() };
        if !failure.is_null() {
            let msg = unsafe { CStr::from_ptr(failure) };
            eprintln!("{}", msg.to_string_lossy());
        }
    }
    check!(cxx_run_test());
}

#[no_mangle]
extern "C" fn cxx_test_suite_get_box() -> *mut cxx_test_suite::R {
    Box::into_raw(Box::new(2020usize))
}

#[no_mangle]
unsafe extern "C" fn cxx_test_suite_r_is_correct(r: *const cxx_test_suite::R) -> bool {
    *r == 2020
}
