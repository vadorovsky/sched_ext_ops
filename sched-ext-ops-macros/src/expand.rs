use std::{ffi::c_char, mem};

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    Error, LitCStr, Result,
};

pub(crate) struct SchedExtOpsArgs {
    name: LitCStr,
}

impl Parse for SchedExtOpsArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: LitCStr = input.parse()?;
        Ok(Self { name })
    }
}

pub(crate) fn sched_ext_ops(args: SchedExtOpsArgs) -> Result<TokenStream> {
    let SchedExtOpsArgs { name } = args;

    if name.value().count_bytes() > 127 {
        return Err(Error::new_spanned(
            name,
            "name is too long, the limit is 127",
        ));
    }

    let mut name_buf: [c_char; 128] = [0; 128];

    let name = name.value();
    let name_bytes: &[c_char] = unsafe { mem::transmute::<&[u8], &[c_char]>(name.to_bytes()) };
    name_buf[..name_bytes.len()].copy_from_slice(name_bytes);

    Ok(quote! {
        ::sched_ext_ops::bindings::sched_ext_ops {
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

            name: [ #(#name_buf),* ],
        }
    })
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;

    use super::*;

    // TODO: Fix the name handling. For some reason the first byte is eaten...
    #[test]
    fn test_sched_ext_ops() {
        let args = SchedExtOpsArgs {
            name: LitCStr::new(c"minimal", Span::call_site()),
        };
        let expanded = sched_ext_ops(args).unwrap();
        let expected = quote! {
            ::sched_ext_ops::bindings::sched_ext_ops {
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

                name: [
                    109i8 , 105i8 , 110i8 , 105i8 , 109i8 , 97i8 , 108i8 , 0i8 , 0i8 , 0i8 , 0i8 ,
                    0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 ,
                    0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 ,
                    0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 ,
                    0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 ,
                    0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 ,
                    0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 ,
                    0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 ,
                    0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 ,
                    0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8 , 0i8
                ],
            }
        };
        assert_eq!(expected.to_string(), expanded.to_string());
    }

    #[test]
    fn test_sched_ext_ops_name_too_long() {
        let args = SchedExtOpsArgs {
            name: LitCStr::new(c"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxyyyyyyyyyy", Span::call_site())
        };
        let res = sched_ext_ops(args);
        assert!(matches!(res, Err(_)));
    }
}
