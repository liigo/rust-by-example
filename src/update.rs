#![deny(warnings)]
#![feature(phase)]

extern crate regex;
#[phase(plugin)]
extern crate regex_macros;
extern crate serialize;

use std::os;
use example::Example;

mod example;
mod file;
mod markdown;
mod playpen;

fn main() {
    let args = os::args();
    let mut postfix = String::new();
    if args.len() > 1 && !args[1].as_slice().is_empty() {
        postfix.push('_');
        postfix.push_str(args[1].as_slice()); // language code such as "zh-CN"
    }

    let examples = Example::get_list(postfix.as_slice());
    let (tx, rx) = channel();

    let mut nexamples = 0;
    for (i, example) in examples.into_iter().enumerate() {
        let tx = tx.clone();
        let count = example.count();
        let postfix = postfix.clone();

        spawn(proc() {
            example.process(vec!(i + 1), tx, 0, String::new(), postfix);
        });

        nexamples += count;
    }

    let mut entries = range(0, nexamples).map(|_| {
        rx.recv()
    }).collect::<Vec<(Vec<uint>, String)>>();

    entries.sort_by(|&(ref i, _), &(ref j, _)| i.cmp(j));

    let summary = entries.into_iter()
                         .map(|(_, s)| s)
                         .collect::<Vec<String>>()
                         .connect("\n");

    match file::write(&Path::new("stage/SUMMARY.md"), summary.as_slice()) {
        Err(why) => panic!("{}", why),
        Ok(_) => {},
    }
}
