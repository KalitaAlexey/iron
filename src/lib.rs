#![crate_id = "persistent"]
#![license = "MIT"]
//#![deny(missing_doc)]
#![deny(unused_result, unused_result, unnecessary_qualification,
        non_camel_case_types, unused_variable, unnecessary_typecast)]

extern crate iron;

pub use persistent::Persistent;
pub use shared::Shared;

pub mod persistent;
pub mod shared;
pub mod mixin;

