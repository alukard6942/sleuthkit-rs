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

use std::str::Utf8Error;

#[derive(Debug)]
pub enum TskError {
    Nullptr(Nullptr),
    Dynamic(Box<dyn Error>),
    Msg(String),
    Cstr(Utf8Error),
}

impl From<Nullptr> for TskError {
    fn from(t: Nullptr) -> Self {
        TskError::Nullptr(t)
    }
}

impl From<String> for TskError {
    fn from(t: String) -> Self {
        TskError::Msg(t)
    }
}

impl From<&str> for TskError {
    fn from(t: &str) -> Self {
        TskError::Msg(t.to_string())
    }
}

impl From<Utf8Error> for TskError {
    fn from(t: Utf8Error) -> Self {
        TskError::Cstr(t)
    }
}


use std::error::Error;
pub type DResult<_T> = Result<_T, TskError>;

