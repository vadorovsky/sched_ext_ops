use std::{
    ffi::{c_char, CStr},
    mem,
};

use sched_ext_ops::sched_ext_ops;

#[test]
fn test_sched_ext_ops_minimal() {
    let minimal = sched_ext_ops!(c"minimal");

    let inner_name: &[c_char] = &minimal.name;
    let inner_name: &[u8] = unsafe { mem::transmute::<&[c_char], &[u8]>(inner_name) };
    let inner_name = CStr::from_bytes_until_nul(inner_name).unwrap();

    assert_eq!(inner_name, c"minimal");
}
