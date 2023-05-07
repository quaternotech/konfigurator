// MIT License
//
// Copyright (c) 2023 Mansoor Ahmed Memon
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::error::Error;

use strong_xml::XmlRead;

use crate::error::{FileFormatError, IOError};
use crate::model::Konfigurator;

pub mod interface;
pub mod model;
pub mod utils;
pub mod error;

pub const SRC_FILE: &'static str = "Konfigurator.xml";
pub const OUT_FILE: &'static str = "Konfigurator.rs";

pub fn src_file() -> String { String::from(SRC_FILE) }

pub fn out_file() -> String { String::from(OUT_FILE) }

pub fn bake(work_dir: String, out_dir: String, verbose: bool) -> Result<String, Box<dyn Error>> {
    let in_file = utils::resolve_path(work_dir, src_file())?;
    let out_file = utils::resolve_path(out_dir, out_file())?;

    let xml = std::fs::read_to_string(in_file.as_str()).or_else(
        |err| Err(IOError::ReadError(err.kind().to_string(), in_file.clone()))
    )?;

    if verbose { println!("Read: {}", in_file); }

    let konfigurator = Konfigurator::from_str(&xml.as_str()).or_else(
        |err| Err(FileFormatError(err.to_string(), in_file.clone()))
    )?;

    if verbose { println!("Parsed: {}", in_file); }

    konfigurator.dump_to_file(out_file.as_str()).or_else(
        |err| Err(IOError::WriteError(err.kind().to_string(), out_file.clone()))
    )?;

    if verbose { println!("Dumped: {}", out_file); }

    Ok(out_file)
}
