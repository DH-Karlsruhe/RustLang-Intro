// Use `wee_alloc` as the global allocator. (for smaller binaries&rte)
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
// has to be @top ⤴️ (even macros have to be defined afterwards..)

extern crate web_sys;
// A macro to provide `println!(..)`-style syntax for `console.log` logging.

#[warn(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub mod universe;
mod utils;