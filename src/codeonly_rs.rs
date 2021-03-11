use std::{self, fs::DirEntry};

use crate::codeonly_c;

pub fn remove(file: &DirEntry) { codeonly_c::remove(file); }
