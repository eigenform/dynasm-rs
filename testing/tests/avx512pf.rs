#![feature(plugin)]
#![plugin(dynasm)]

#![allow(unused_imports)]

#[macro_use]
extern crate dynasmrt;

use dynasmrt::DynasmApi;

include!("gen/avx512pf.rs.gen");
