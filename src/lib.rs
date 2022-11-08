/**
 * File: lib.rs
 * Author: xkoval18 <xkoval18@github>
 * Date: 20.10.2022
 * Last Modified Date: 20.10.2022
 */

mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]

    include!("./bindings.rs");
}

pub mod error;
mod tchar;
pub mod img_info;
pub mod vs_info;
pub mod fs_info;


pub mod entry;
