#![feature(target_feature)]
#![feature(iter_arith)]
#![feature(fnbox)]
#![feature(static_in_const)]
#![feature(as_unsafe_cell)]
#![feature(heap_api)]
#![feature(unboxed_closures)]
#![feature(oom)]
#![feature(alloc)]
#![feature(box_syntax)]
#![feature(optin_builtin_traits)]
#![feature(const_fn)]
#![feature(step_by)]
#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(custom_derive)]
#![feature(libc)]

//#![allow(unused_imports)]
#![allow(improper_ctypes)]
#![allow(unused_must_use)]
#![allow(deprecated)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_unsafe)]
#![allow(unused_variables)]
#![allow(unused_features)]
#![allow(unused_mut)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![feature(libc)]
#![feature(core_intrinsics)]
#![feature(thread_id)]
#![feature(custom_attribute)]
#![feature(fused)]
#![feature(fn_traits)]

extern crate net2;
extern crate nix;
extern crate alloc;
extern crate time;

#[macro_use]
pub mod session_types;
#[macro_use]
pub mod reactors;
pub mod io;
pub mod intercore;
pub mod commands;
pub mod args;
pub mod streams;
pub mod queues;
pub mod handle;
pub mod sys;

#[macro_use]
extern crate libc;
#[macro_use]
extern crate log;
#[macro_use]
extern crate core;
#[macro_use]
extern crate bitflags;
extern crate http_muncher;
extern crate sha1;
extern crate rustc_serialize;