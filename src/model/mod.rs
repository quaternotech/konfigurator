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

use std::{fs, io};
use std::borrow::Cow;
use std::fmt::Display;

use strong_xml::XmlRead;
use titlecase::titlecase;

macro_rules! banner {
    ($($line:expr),*$(,)?) => {
        format!("//\n{}//\n", vec![$(format!("// {}\n", $line)),*].join(""))
    };
}

pub trait Dump {
    fn dump(&self, buffer: &mut String, depth: Option<&mut Vec<String>>);
}

#[derive(Debug, XmlRead)]
#[xml(tag = "Konfigurator")]
pub struct Konfigurator<'a> {
    #[xml(attr = "name")]
    name: Cow<'a, str>,
    #[xml(attr = "arch")]
    arch: Cow<'a, str>,
    #[xml(attr = "profile")]
    profile: Cow<'a, str>,
    #[xml(child = "Section")]
    sections: Vec<SectionType<'a>>,
}

impl<'a> Konfigurator<'a> {
    pub fn dump_to_file(&self, path_to_file: &str) -> io::Result<()> {
        let mut buffer = String::new();
        self.dump(&mut buffer, None);
        fs::write(path_to_file, buffer)?;

        Ok(())
    }
}

impl<'a> Dump for Konfigurator<'a> {
    fn dump(&self, buffer: &mut String, _depth: Option<&mut Vec<String>>) {
        let crate_name = env!("CARGO_PKG_NAME");
        let crate_version = env!("CARGO_PKG_VERSION");

        let data = banner!(
            "This is an auto-generated file; Do not edit.",
            format!("{} v{} | {}/{}/{}",
                titlecase(crate_name.replace("_", " ").as_str()),
                crate_version,
                self.name, self.arch, self.profile),
        );

        buffer.push_str(data.as_str());

        let mut depth = Vec::new();
        for section in self.sections.iter() {
            buffer.push_str("\n");
            depth.push(section.name.to_string());
            section.dump(buffer, Some(&mut depth));
            depth.pop();
        }
    }
}

#[derive(Debug, XmlRead)]
#[xml(tag = "Section")]
pub struct SectionType<'a> {
    #[xml(attr = "name")]
    name: Cow<'a, str>,
    #[xml(child = "Section")]
    sections: Vec<SectionType<'a>>,
    #[xml(child = "Config")]
    configs: Vec<ConfigType<'a>>,
}

impl<'a> Dump for SectionType<'a> {
    fn dump(&self, buffer: &mut String, depth: Option<&mut Vec<String>>) {
        let depth = depth.unwrap();

        let data = banner!(
            format!("{}", depth.join("::")),
        );

        buffer.push_str(data.as_str());

        for section in self.sections.iter() {
            buffer.push_str("\n");
            depth.push(section.name.to_string());
            section.dump(buffer, Some(depth));
            depth.pop();
        }

        for config in self.configs.iter() {
            config.dump(buffer, Some(depth));
        }
    }
}

pub enum ConfigDataType {
    Boolean(bool),
    Integer(i32),
    Double(f64),
    String(String),
    Enum(String),
}

#[derive(Debug, XmlRead)]
#[xml(tag = "Config")]
pub struct ConfigType<'a> {
    #[xml(attr = "key")]
    key: Cow<'a, str>,
    #[xml(child = "Boolean")]
    boolean: Option<BooleanType>,
    #[xml(child = "Integer")]
    integer: Option<IntegerType>,
    #[xml(child = "Double")]
    double: Option<DoubleType>,
    #[xml(child = "String")]
    string: Option<StringType<'a>>,
    #[xml(child = "Enum")]
    enumeration: Option<EnumType<'a>>,
}

impl<'a> ConfigType<'a> {
    pub fn value(&self) -> ConfigDataType {
        match self {
            Self { boolean: Some(inner), .. } => {
                ConfigDataType::Boolean(
                    inner.value.unwrap_or(inner.default)
                )
            }
            Self { integer: Some(inner), .. } => {
                ConfigDataType::Integer(
                    inner.value.unwrap_or(inner.default)
                )
            }
            Self { double: Some(inner), .. } => {
                ConfigDataType::Double(
                    inner.value.unwrap_or(inner.default)
                )
            }
            Self { string: Some(inner), .. } => {
                ConfigDataType::String(
                    inner.value
                         .clone()
                         .unwrap_or(inner.default.clone())
                         .to_string()
                )
            }
            Self { enumeration: Some(inner), .. } => {
                ConfigDataType::Enum(
                    inner.members
                         .get(inner.value.unwrap_or(inner.default) as usize)
                         .unwrap().content.to_string()
                )
            }
            _ => { unreachable!() }
        }
    }
}

macro_rules! quote {
    ($s:expr) => { format!("\"{}\"", $s)};
}

impl<'a> Dump for ConfigType<'a> {
    fn dump(&self, buffer: &mut String, _depth: Option<&mut Vec<String>>) {
        fn generic_impl<T: Display>(outer: &ConfigType, dtype: &str, buffer: &mut String, value: T) {
            let config = format!("#[no_mangle]\npub static {}: {} = {};\n", outer.key.to_uppercase(), dtype, value);
            buffer.push_str(config.as_str());
        }

        match self.value() {
            ConfigDataType::Boolean(value) => generic_impl(self, "bool", buffer, value),
            ConfigDataType::Integer(value) => generic_impl(self, "i32", buffer, value),
            ConfigDataType::Double(value) => generic_impl(self, "f64", buffer, value),
            ConfigDataType::String(value) => generic_impl(self, "&'static str", buffer, quote!(value)),
            ConfigDataType::Enum(value) => generic_impl(self, "&'static str", buffer, quote!(value)),
        }
    }
}

#[derive(Debug, XmlRead)]
#[xml(tag = "Boolean")]
pub struct BooleanType {
    #[xml(attr = "default")]
    default: bool,
    #[xml(attr = "value")]
    value: Option<bool>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "Integer")]
pub struct IntegerType {
    #[xml(attr = "default")]
    default: i32,
    #[xml(attr = "value")]
    value: Option<i32>,
    #[allow(dead_code)]
    #[xml(attr = "min")]
    min: Option<i32>,
    #[allow(dead_code)]
    #[xml(attr = "max")]
    max: Option<i32>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "Double")]
pub struct DoubleType {
    #[xml(attr = "default")]
    default: f64,
    #[xml(attr = "value")]
    value: Option<f64>,
    #[allow(dead_code)]
    #[xml(attr = "min")]
    min: Option<f64>,
    #[allow(dead_code)]
    #[xml(attr = "max")]
    max: Option<f64>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "String")]
pub struct StringType<'a> {
    #[xml(attr = "default")]
    default: Cow<'a, str>,
    #[xml(attr = "value")]
    value: Option<Cow<'a, str>>,
    #[allow(dead_code)]
    #[xml(attr = "min-length")]
    min_length: Option<u32>,
    #[allow(dead_code)]
    #[xml(attr = "max-length")]
    max_length: Option<u32>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "Enum")]
pub struct EnumType<'a> {
    #[xml(attr = "default")]
    default: u32,
    #[xml(attr = "value")]
    value: Option<u32>,
    #[xml(child = "Member")]
    members: Vec<MemberType<'a>>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "Member")]
pub struct MemberType<'a> {
    #[xml(text)]
    content: Cow<'a, str>,
}
