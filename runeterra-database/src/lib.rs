#[macro_use]
extern crate rust_embed;

mod database;

pub mod db {
    pub use crate::database::*;
}
