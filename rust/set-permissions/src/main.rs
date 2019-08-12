// Copyright 2019 Joseph Lorimer <joseph@lorimer.me>
//
// Permission to use, copy, modify, and distribute this software for any purpose
// with or without fee is hereby granted, provided that the above copyright
// notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
// REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
// AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
// INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
// LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
// OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
// PERFORMANCE OF THIS SOFTWARE.

use std::fs;
use std::os::unix::fs::PermissionsExt;

extern crate clap;
use clap::{App, Arg, ArgMatches};

extern crate magic;
use magic::{Cookie, CookieFlags};

extern crate walkdir;
use walkdir::WalkDir;

fn get_mode(matches: &ArgMatches, key: &str) -> u32 {
    return u32::from_str_radix(matches.value_of(key).unwrap(), 8).unwrap();
}

fn main() {
    let matches = App::new("app")
        .arg(
            Arg::with_name("file-mode")
                .short("f")
                .long("file-mode")
                .value_name("MODE")
                .required(true),
        )
        .arg(
            Arg::with_name("dir-mode")
                .short("d")
                .long("dir-mode")
                .value_name("MODE")
                .required(true),
        )
        .arg(
            Arg::with_name("exec")
                .short("x")
                .long("exec")
                .help("Apply user execute bit to executable files"),
        )
        .arg(Arg::with_name("PATH").required(true))
        .get_matches();

    let cookie = Cookie::open(CookieFlags::default()).unwrap();
    let databases = vec!["/usr/share/misc/magic"];
    assert!(cookie.load(&databases).is_ok());
    let file_mode = get_mode(&matches, "file-mode");
    let dir_mode = get_mode(&matches, "dir-mode");
    let start = matches.value_of("PATH").unwrap();
    for entry in WalkDir::new(start) {
        let file = match entry {
            Ok(f) => f,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        let meta = match file.metadata() {
            Ok(m) => m,
            Err(_e) => continue,
        };
        let mut perms = meta.permissions();
        let cur_mode = perms.mode() & 0o777;
        let mut new_mode;
        if meta.is_file() && cur_mode != file_mode {
            new_mode = file_mode;
            if matches.is_present("exec") {
                let filetype = cookie.file(file.path()).unwrap();
                if filetype.find("executable") != None {
                    new_mode = new_mode | 0o700;
                }
            }
        } else if meta.is_dir() && cur_mode != dir_mode {
            new_mode = dir_mode;
        } else {
            continue;
        }
        println!(
            "{:o} => {:o}\t{}",
            cur_mode,
            new_mode,
            file.path().display()
        );
        perms.set_mode(new_mode);
        assert!(fs::set_permissions(file.path(), perms).is_ok());
    }
}
