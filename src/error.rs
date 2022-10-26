/**
 * File: error.rs
 * Author: xkoval18 <xkoval18@github>
 * Date: 20.10.2022
 * Last Modified Date: 20.10.2022
 */

use std::error::Error;
pub type DResult<_T> = Result<_T, Box<dyn Error>>;
