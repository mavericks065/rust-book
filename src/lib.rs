#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;
extern crate dotenv;

pub mod domain;
pub mod infrastructure;
