//pub extern crate libspdk_sys as raw;
extern crate libspdk_sys as raw;
#[macro_use]
extern crate failure;
extern crate libc;

mod event;
mod bdev;

pub use event::{AppOpts, AppContext, app_stop};
pub use bdev::{Bdev};


//fn main() {
//    let mut opts :raw::spdk_app_opts;
//}
