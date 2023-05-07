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
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum IOError {
    PathResolutionError(String, String),
    ReadError(String, String),
    WriteError(String, String),
}

impl Display for IOError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PathResolutionError(err_kind, cause) => {
                write!(f, "Could not resolve path: {}: {}", err_kind, cause)
            }
            Self::ReadError(err_kind, cause) => {
                write!(f, "Error while reading: {}: {}", err_kind, cause)
            }
            Self::WriteError(err_kind, cause) => {
                write!(f, "Error while writing: {}: {}", err_kind, cause)
            }
        }
    }
}

impl Error for IOError {}

#[derive(Debug)]
pub struct FileFormatError(pub String, pub String);

impl Display for FileFormatError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "File format error: {}: {}", self.0, self.1)
    }
}

impl Error for FileFormatError {}
