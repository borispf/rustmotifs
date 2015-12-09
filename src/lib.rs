#![feature(static_mutex)]

extern crate arrayvec;
extern crate fixedbitset;
extern crate libc;
extern crate petgraph;

pub mod motifs;
pub mod nauty;
#[allow(non_camel_case_types)]
pub mod nauty_bindings;
pub mod network;
