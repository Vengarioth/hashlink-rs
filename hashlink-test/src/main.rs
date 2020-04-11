extern crate hashlink_sys;

use hashlink_sys::*;

fn main() {
    println!("Hello, world!");

    unsafe {
        hl_global_init();
    }
}
