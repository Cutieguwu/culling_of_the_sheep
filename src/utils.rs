// Copyright (c) 2024 Cutieguwu | Olivia Brooks
//
// -*- coding: utf-8 -*-
// @Title: Culling of the Sheep
// @Author: Cutieguwu | Olivia Brooks
// @Description: Utils for cli argument handling.
//
// @Script: characters.rs
// @Date Created: 03 Dec, 2024
// @Last Modified: 10 Dec, 2024
// @Last Modified by: Cutieguwu | Olivia Brooks
// --------------------------------------------


use std::env::args;
use std::io;
use std::str::Chars;

const DASH: char = 45 as u8 as char;

#[derive(Debug, Clone)]
pub enum ArgType {
    #[allow(dead_code)]
    Binary(String),
    Command(String),
    Flag(FlagType),
}

#[derive(Debug, Clone)]
pub enum FlagType {
    //Currently only handles single flags. eg. "-S", "-y"
    Short(String),
    Long(String),
}

pub fn fmt_args() -> Vec<ArgType> {
    let mut args_vec:Vec<ArgType> = Vec::new();

    for obj in args() {
        args_vec.push(match try_flag(&obj) {
            None => ArgType::Command(obj),
            Some(flag) => ArgType::Flag(flag),
        });
    };

    args_vec[0] = ArgType::Binary(match &args_vec[0] {
        ArgType::Command(command) => command.to_owned(),
        err => panic!(
            "Expected ArgType::Command at args_vec[0], found {:?}",
            err,
        ),
    });

    args_vec
}

fn try_flag<'a>(arg: &'a String) -> Option<FlagType> {
    // Short term var to reduce number of chars() calls.
    let arg_chars: Chars = arg.chars();

    if arg_chars.clone().nth(1).unwrap() == DASH {
        //eg. --my-flag
        Some(FlagType::Long(break_flag_long(arg.clone())))
    } else if arg_chars.clone().nth(0).unwrap() == DASH {
        //eg. -Syu
        Some(FlagType::Short(break_flag_short(arg.clone())))
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
    let mut input_buffer: String = String::new();

    io::stdin()
        .read_line(&mut input_buffer)
        .expect("Failed to read line");

    // Clear special characters and return as String
    input_buffer.trim_end_matches(&['\r', '\n'][..]).to_string()
}