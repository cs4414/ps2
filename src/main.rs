//
// gash.rs
//
// Starting code for PS2
// Running on Rust 0.9
//
// University of Virginia - cs4414 Spring 2014
// Weilin Xu, David Evans
// Version 0.4
//

extern crate getopts;

use getopts::{optopt, getopts};
use std::io::BufferedReader;
use std::io::Command;
use std::io::stdin;
use std::{io, os};
use std::str;

struct Shell<'a> {
    cmd_prompt: &'a str,
}

impl <'a>Shell<'a> {
    fn new(prompt_str: &'a str) -> Shell<'a> {
        Shell { cmd_prompt: prompt_str }
    }

    fn run(&self) {
        let mut stdin = BufferedReader::new(stdin());

        loop {
            io::stdio::print(self.cmd_prompt.as_slice());
            io::stdio::flush();

            let line = stdin.read_line().unwrap();
            let cmd_line = line.trim();
            let program = cmd_line.splitn(1, ' ').nth(0).expect("no program");

            match program {
                ""      =>  { continue; }
                "exit"  =>  { return; }
                _       =>  { self.run_cmdline(cmd_line); }
            }
        }
    }

    fn run_cmdline(&self, cmd_line: &str) {
        let argv: Vec<&str> = cmd_line.split(' ').filter_map(|x| {
            if x == "" {
                None
            } else {
                Some(x)
            }
        }).collect();

        match argv.head() {
            Some(&program) => self.run_cmd(program, argv.tail()),
            None => (),
        };
    }

    fn run_cmd(&self, program: &str, argv: &[&str]) {
        if self.cmd_exists(program) {
            io::stdio::print(str::from_utf8(Command::new(program).args(argv).output().unwrap().output.as_slice()).unwrap());
        } else {
            println!("{}: command not found", program);
        }
    }

    fn cmd_exists(&self, cmd_path: &str) -> bool {
        Command::new("which").arg(cmd_path).status().unwrap().success()
    }
}

fn get_cmdline_from_args() -> Option<String> {
    /* Begin processing program arguments and initiate the parameters. */
    let args = os::args();

    let opts = &[
        getopts::optopt("c", "", "", "")
    ];

    getopts::getopts(args.tail(), opts).unwrap().opt_str("c")
}

fn main() {
    let opt_cmd_line = get_cmdline_from_args();

    match opt_cmd_line {
        Some(cmd_line) => Shell::new("").run_cmdline(cmd_line.as_slice()),
        None           => Shell::new("gash > ").run(),
    }
}
