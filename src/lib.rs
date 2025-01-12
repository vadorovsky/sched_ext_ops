#![no_std]

use core::{
    ffi::{c_char, CStr},
    slice,
};

use thiserror::Error;

pub mod bindings;
use bindings::sched_ext_ops;

#[derive(Debug, Error)]
pub enum SchedExtOpsError {
    #[error("name can have at most 127 characters, got {0}")]
    NameTooLong(usize),
}

#[repr(transparent)]
pub struct SchedExtOps {
    _inner: sched_ext_ops,
}

impl SchedExtOps {
    pub fn new(name: &CStr) -> Result<Self, SchedExtOpsError> {
        let name_len = name.count_bytes();
        if name_len > 127 {
            return Err(SchedExtOpsError::NameTooLong(name_len));
        }

        let mut c_name_buf: [c_char; 128] = [0; 128];

        // `CStr` has methods like `to_bytes` and `to_bytes_with_nul`, both of
        // them return `&[u8]`. We are interested in `&[c_char]` (which can be
        // `&[i8]` on many architectures).
        //
        // Even though `CStr` uses `[c_char]` as an internal field, the only
        // public method exposing `c_char` is `as_ptr`. So, unfortunately, the
        // only option for us is to use it and build a slice from it.
        //
        // SAFETY: We are sure about the length of the string and we add one
        // more byte for the NUL character.
        let name_bytes: &[c_char] =
            unsafe { slice::from_raw_parts(name.as_ptr(), name_len + 1) };
        c_name_buf[..name_bytes.len()].copy_from_slice(name_bytes);

        Ok(Self {
            _inner: sched_ext_ops {
                // TODO: Wrap functions.
                select_cpu: None,
                enqueue: None,
                dequeue: None,
                dispatch: None,
                tick: None,
                runnable: None,
                running: None,
                stopping: None,
                quiescent: None,
                yield_: None,
                core_sched_before: None,
                set_weight: None,
                set_cpumask: None,
                update_idle: None,
                cpu_acquire: None,
                cpu_release: None,
                init_task: None,
                exit_task: None,
                enable: None,
                disable: None,
                dump: None,
                dump_cpu: None,
                dump_task: None,
                cgroup_init: None,
                cgroup_exit: None,
                cgroup_prep_move: None,
                cgroup_move: None,
                cgroup_cancel_move: None,
                cgroup_set_weight: None,
                cpu_online: None,
                cpu_offline: None,
                init: None,
                exit: None,

                dispatch_max_batch: 0,
                flags: 0,
                timeout_ms: 0,
                exit_dump_len: 0,
                hotplug_seq: 0,

                name: c_name_buf,
            },
        })
    }
}

#[cfg(test)]
mod test {
    use core::mem;

    use super::*;

    #[test]
    fn test_sched_ext_ops_minimal() {
        let minimal = SchedExtOps::new(c"minimal").unwrap();

        let inner_name: &[c_char] = &minimal._inner.name;
        let inner_name: &[u8] = unsafe { mem::transmute::<&[c_char], &[u8]>(inner_name) };
        let inner_name = CStr::from_bytes_until_nul(inner_name).unwrap();

        assert_eq!(inner_name, c"minimal");
    }

    #[test]
    fn test_sched_ext_ops_name_too_long() {
        let res = SchedExtOps::new(c"hgsdlfgdsfgfdsgsdfagsdfgfdsgfdsgdfgdsfghfdshsdfgfdgfdgsdfgfdsgsdfgsdfhdgfhsfdgafdsgfdgsdfgfdgsdfgfdsgsdfdsgdfgsgdsfggdfsggfhgfdjhfgjgfhfdsrdgragsfdhgsdrgdthdfhgfdgfdgrdgsrgsdrtgdrgdfg");

        assert!(matches!(res, Err(SchedExtOpsError::NameTooLong(_))));
    }
}
