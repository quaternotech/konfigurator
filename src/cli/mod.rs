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

use clap::Arg;
use clap::Command;

pub mod actions;

pub fn interface() -> Command {
    let crate_name = env!("CARGO_PKG_NAME");
    let crate_version = env!("CARGO_PKG_NAME");
    let crate_description = env!("CARGO_PKG_DESCRIPTION");

    Command::new(crate_name)
        .version(crate_version)
        .about(crate_description)
        .subcommand(
            Command::new(actions::BAKE).args(
                &[
                    Arg::new("out_dir")
                        .short('o')
                        .long("out-dir")
                        .value_name("OUT_DIR")
                        .help("Specify the directory where the `Konfigurator.rs` file will be placed"),
                    Arg::new("work_dir")
                        .short('w')
                        .long("work-dir")
                        .value_name("WORK_DIR")
                        .help("Specify the directory containing the `Konfigurator.xml` file"),
                ]
            ).about("Bake the configuration")
        )
}
