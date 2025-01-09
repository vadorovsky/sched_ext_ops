#![no_std]
#![allow(non_camel_case_types)]

use aya_ebpf::{
    bindings::{cgroup, task_struct},
    cty::{c_char, c_ulong},
};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cpumask {
    _unused: [u8; 0],
}

/// Argument container for [`cgroup_init`](sched_ext_ops::cgroup_init)
/// operation.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct scx_cgroup_init_args {
    pub weight: u32,
}

/// Argument container for [`cgroup_init`](sched_ext_ops::cpu_acquire)
/// operation. It's currently empty, introduced as a placeholder.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct scx_cpu_acquire_args {}

/// Reason for SCX being preempted.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum scx_cpu_preempt_reason {
    /// New task is being scheduled by `sched_class_rt`.
    SCX_CPU_PREEMPT_RT = 0,
    /// New task is being scheduled by `sched_class_dl`.
    SCX_CPU_PREEMPT_DL = 1,
    /// New task is being scheduled by `sched_class_stop`.
    SCX_CPU_PREEMPT_STOP = 2,
    /// Unknown reason for SCX being preempted.
    SCX_CPU_PREEMPT_UNKNOWN = 3,
}

/// Argument container for [`cpu_release`](sched_ext_ops::cpu_release)
/// operation.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct scx_cpu_release_args {
    pub reason: scx_cpu_preempt_reason,
    pub task: *mut task_struct,
}

/// Context provided to [`dump`](sched_ext_ops::dump),
/// [`dump_cpu`](sched_ext_ops::dump_cpu) and
/// [`dump_task`](sched_ext_ops::dump_task) operations.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct scx_dump_ctx {
    pub kind: scx_exit_kind,
    pub exit_code: i64,
    pub reason: *const c_char,
    pub at_ns: u64,
    pub at_jiffies: u64,
}

/// Argument container for [`init_task`](sched_ext_ops::init_task) operation.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct scx_init_task_args {
    pub fork: bool,
    pub cgroup: *mut cgroup,
}

/// Reason for exiting SCX.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum scx_exit_kind {
    SCX_EXIT_NONE = 0,
    /// Task is done.
    SCX_EXIT_DONE = 1,
    /// Unregistration initiated by the user-space.
    SCX_EXIT_UNREG = 64,
    /// Unregistration initiated by an eBPF program.
    SCX_EXIT_UNREG_BPF = 65,
    /// Unregistration initiated by the kernel.
    SCX_EXIT_UNREG_KERN = 66,
    /// Unregistration initiated by sysrq.
    SCX_EXIT_SYSRQ = 67,
    /// Runtime error.
    SCX_EXIT_ERROR = 1024,
    /// Error triggered by an eBPF program through `scx_bpf_error`.
    SCX_EXIT_ERROR_BPF = 1025,
    /// Watchdog detected stalled runnable tasks.
    SCX_EXIT_ERROR_STALL = 1026,
}

/// Information passed to [`exit`](sched_ext_ops::exit) to describe why the
/// eBPF scheduler is being disabled.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct scx_exit_info {
    /// Category of the exit reason.
    pub kind: scx_exit_kind,
    // Exit code.
    pub exit_code: i64,
    /// Textual representation of [`kind`](Self::kind) and
    /// [`exit_code`](Self::exit_code).
    pub reason: *const c_char,
    /// Backtrace, if exiting due to an error.
    pub bt: *mut c_ulong,
    pub bt_len: u32,
    /// Informational message.
    pub msg: *mut c_char,
    /// Debug dump.
    pub dump: *mut c_char,
}

/// Argument container for [`exit_task`](sched_ext_ops::exit_task) operation.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct scx_exit_task_args {
    cancelled: bool,
}

/// Operation table for eBPF scheduler implementation.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sched_ext_ops {
    /// Picks the target CPU for a task which is being woken up.
    pub select_cpu:
        Option<unsafe extern "C" fn(arg1: *mut task_struct, arg2: i32, arg3: u64) -> i32>,
    /// Enqueues a task on the eBPF scheduler.
    pub enqueue: Option<unsafe extern "C" fn(arg1: *mut task_struct, arg2: u64)>,
    /// Removes a task from the eBPF scheduler.
    pub dequeue: Option<unsafe extern "C" fn(arg1: *mut task_struct, arg2: u64)>,
    /// Dispatches tasks from the eBPF scheduler and/or consumes DSQs.
    pub dispatch: Option<unsafe extern "C" fn(arg1: i32, arg2: *mut task_struct)>,
    /// Periodic tick.
    pub tick: Option<unsafe extern "C" fn(arg1: *mut task_struct)>,
    pub runnable: Option<unsafe extern "C" fn(arg1: *mut task_struct, arg2: u64)>,
    pub running: Option<unsafe extern "C" fn(arg1: *mut task_struct)>,
    pub stopping: Option<unsafe extern "C" fn(arg1: *mut task_struct, arg2: bool)>,
    pub quiescent: Option<unsafe extern "C" fn(arg1: *mut task_struct, arg2: u64)>,
    pub yield_:
        Option<unsafe extern "C" fn(arg1: *mut task_struct, arg2: *mut task_struct) -> bool>,
    pub core_sched_before:
        Option<unsafe extern "C" fn(arg1: *mut task_struct, arg2: *mut task_struct) -> bool>,
    pub set_weight: Option<unsafe extern "C" fn(arg1: *mut task_struct, arg2: u32)>,
    pub set_cpumask: Option<unsafe extern "C" fn(arg1: *mut task_struct, arg2: *const cpumask)>,
    pub update_idle: Option<unsafe extern "C" fn(arg1: i32, arg2: bool)>,
    pub cpu_acquire: Option<unsafe extern "C" fn(arg1: i32, arg2: *mut scx_cpu_acquire_args)>,
    pub cpu_release: Option<unsafe extern "C" fn(arg1: i32, arg2: *mut scx_cpu_release_args)>,
    pub init_task:
        Option<unsafe extern "C" fn(arg1: *mut task_struct, arg2: *mut scx_init_task_args) -> i32>,
    pub exit_task:
        Option<unsafe extern "C" fn(arg1: *mut task_struct, arg2: *mut scx_exit_task_args)>,
    pub enable: Option<unsafe extern "C" fn(arg1: *mut task_struct)>,
    pub disable: Option<unsafe extern "C" fn(arg1: *mut task_struct)>,
    pub dump: Option<unsafe extern "C" fn(arg1: *mut scx_dump_ctx)>,
    pub dump_cpu: Option<unsafe extern "C" fn(arg1: *mut scx_dump_ctx, arg2: i32, arg3: bool)>,
    pub dump_task: Option<unsafe extern "C" fn(arg1: *mut scx_dump_ctx, arg2: *mut task_struct)>,
    pub cgroup_init:
        Option<unsafe extern "C" fn(arg1: *mut cgroup, arg2: *mut scx_cgroup_init_args) -> i32>,
    pub cgroup_exit: Option<unsafe extern "C" fn(arg1: *mut cgroup)>,
    pub cgroup_prep_move: Option<
        unsafe extern "C" fn(arg1: *mut task_struct, arg2: *mut cgroup, arg3: *mut cgroup) -> i32,
    >,
    pub cgroup_move:
        Option<unsafe extern "C" fn(arg1: *mut task_struct, arg2: *mut cgroup, arg3: *mut cgroup)>,
    pub cgroup_cancel_move:
        Option<unsafe extern "C" fn(arg1: *mut task_struct, arg2: *mut cgroup, arg3: *mut cgroup)>,
    pub cgroup_set_weight: Option<unsafe extern "C" fn(arg1: *mut cgroup, arg2: u32)>,
    pub cpu_online: Option<unsafe extern "C" fn(arg1: i32)>,
    pub cpu_offline: Option<unsafe extern "C" fn(arg1: i32)>,
    /// Initialized the eBPF scheduler.
    pub init: Option<unsafe extern "C" fn() -> i32>,
    /// Cleans up after the eBPF scheduler.
    pub exit: Option<unsafe extern "C" fn(arg1: *mut scx_exit_info)>,
    pub dispatch_max_batch: u32,
    pub flags: u64,
    /// The maximum amount of time, in milliseconds, that a runnable task
    /// should be able to wait before being scheduled. The maximum timeout
    /// cannot exceed the default timeout of 30 seconds.
    pub timeout_ms: u32,
    pub exit_dump_len: u32,
    pub hotplug_seq: u64,
    /// eBPF scheduler's name.
    pub name: [c_char; 128usize],
}
