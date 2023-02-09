pub mod base_crud;
pub mod common_structs;
pub mod errors;
pub mod init_service;

pub use init_service::init_service;
pub use scopes_macro::scopes;


#[macro_use]
extern crate async_trait;

pub mod prelude {
    pub use moscow::entities::prelude::*;
}