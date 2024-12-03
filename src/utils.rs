use std::env::args;
use std::io;


const DASH:char = 45 as u8 as char;

#[derive(Debug, Clone)]
pub enum ArgType {
    #[allow(dead_code)]
    Binary(String),
    Command(String),
    Flag(FlagType)
}

#[derive(Debug, Clone)]
pub enum FlagType {
    //Currently only handles single flags. eg. "-S", "-y"
    Short(String),
    Long(String)
}

pub fn fmt_args() -> Vec<ArgType> {
    let mut args_vec:Vec<ArgType> = Vec::new();

    for x in args() {
        match try_flag(x.clone()) {
            None => args_vec.push(ArgType::Command(x)),
            Some(f) => args_vec.push(ArgType::Flag(f))
        }
    }

    args_vec[0] = {
        ArgType::Binary({
            match args_vec[0].clone() {
                ArgType::Command(c) => c,
                err => panic!("Expected ArgType::Command at args_vec[0], found {:?}", err)
            }
        })
    };

    args_vec
}

fn try_flag(arg: String) -> Option<FlagType> {
    if arg.chars().nth(1).unwrap() == DASH {
        //eg. --my-flag
        Some(FlagType::Long(break_flag_long(arg)))
    } else if arg.chars().nth(0).unwrap() == DASH {
        //eg. -Syu
        Some(FlagType::Short(break_flag_short(arg)))
    } else {
        None
    }
}

fn break_flag_short(mut arg: String) -> String {
    arg.remove(0);
    arg
}

fn break_flag_long(mut arg: String) -> String {
    for _ in 1..=2 {
        arg.remove(0);
    };

    arg
}

pub fn input() -> String {
    let mut input_buffer = String::new();

    io::stdin()
        .read_line( &mut input_buffer)
        .expect("Failed to read line");

    input_buffer
}