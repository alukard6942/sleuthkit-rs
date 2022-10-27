/**
 * File: error.rs
 * Author: xkoval18 <xkoval18@github>
 * Date: 20.10.2022
 * Last Modified Date: 20.10.2022
 */

#[derive(Debug)]
pub enum Nullptr {
    ImgOpen,
    VsOpen,
    FsOpen,
    DirOpen,
    FileOpen,
}

#[derive(Debug)]
pub enum TskError {
    Nullptr(Nullptr),
    Dynamic(Box<dyn Error>),
    Str(String),
}

impl From<Nullptr> for TskError {
    fn from(t: Nullptr) -> Self {
        TskError::Nullptr(t)
    }
}
impl From<String> for TskError {
    fn from(t: String) -> Self {
        TskError::Str(t)
    }
}
impl From<&str> for TskError {
    fn from(t: &str) -> Self {
        TskError::Str(t.to_string())
    }
}


use std::error::Error;
pub type DResult<_T> = Result<_T, TskError>;

