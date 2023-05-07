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
use std::fs;

use strong_xml::XmlRead;

use crate::model::Konfigurator;

pub mod cli;
pub mod model;

pub const IN_FILE: &'static str = "Konfigurator.xml";
pub const OUT_FILE: &'static str = "Konfigurator.rs";

pub fn construct_path(work_dir: Option<&String>, file: &str) -> Result<String, Box<dyn Error>> {
    let default_dir = "./".to_string();
    let work_dir = work_dir.unwrap_or(&default_dir);
    std::fs::create_dir_all(work_dir).or_else(
        |err| { Err(format!("Error occurred while creating path `{}`: {}", work_dir, err.kind())) }
    )?;
    let path = fs::canonicalize(work_dir)?
        .join(file.to_string())
        .to_string_lossy()
        .to_string();
    Ok(path)
}

pub fn bake(work_dir: Option<&String>, out_dir: Option<&String>) -> Result<String, Box<dyn Error>> {
    let in_file = construct_path(work_dir, IN_FILE)?;
    let out_file = construct_path(out_dir, OUT_FILE)?;

    let xml = std::fs::read_to_string(in_file.as_str()).or_else(
        |err| Err(format!("Error occurred while opening `{}`: {}", in_file, err.kind()))
    )?;
    let konfigurator = Konfigurator::from_str(&xml.as_str()).or_else(
        |err| Err(format!("Error occurred while parsing `{}`: {}", in_file, err))
    )?;
    konfigurator.dump_to_file(out_file.as_str()).or_else(
        |err| Err(format!("Error occurred while writing to `{}`: {}", out_file, err.kind()))
    )?;

    Ok(out_file)
}
