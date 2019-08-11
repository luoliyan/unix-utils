use std::env;
use std::fs;
use std::process::Command;

extern crate regex;
use regex::Regex;

#[macro_use]
extern crate prettytable;
use prettytable::Table;

fn main() {
    let args: Vec<String> = env::args().collect();
    let primes = fs::read_to_string(&args[1]).unwrap();
    let origins_raw = Command::new("pkg").arg("prime-origins").output().unwrap();
    let origins = String::from_utf8_lossy(&origins_raw.stdout);
    let mut table = Table::new();
    table.add_row(row![ByFdc => "Origin", "Required By"]);
    for o in origins.lines() {
        if primes.find(o) != None {
            continue;
        }
        let deps_raw = Command::new("pkg")
            .arg("info")
            .arg("--required-by")
            .arg(o)
            .output()
            .unwrap();
        let deps = String::from_utf8_lossy(&deps_raw.stdout);
        let dep_re = Regex::new(r"^\t").unwrap();
        let vec: Vec<_> = deps
            .lines()
            .filter(|s| dep_re.is_match(s))
            .map(|s| s.trim())
            .collect();
        table.add_row(row!(o, vec.join("\n")));
    }
    table.printstd();
}
