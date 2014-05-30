#![crate_id(name="factor", vers="1.0.0", author="T. Jameson Little")]
#![feature(macro_rules)]

/*
* This file is part of the uutils coreutils package.
*
* (c) T. Jameson Little <t.jameson.little@gmail.com>
*
* For the full copyright and license information, please view the LICENSE file
* that was distributed with this source code.
*/

extern crate getopts;
extern crate libc;
extern crate num;

use std::os;
use std::vec::{Vec};
use std::io::{stdin};
use num::bigint::BigUint;

#[path="../common/util.rs"]
mod util;

static VERSION: &'static str = "1.0.0";
static NAME: &'static str = "factor";

fn approx_sqrt(number: BigUint) -> BigUint {
    let mut approx = number.clone();
    for _ in range(0, 4) {
        approx = (approx + (number / approx)) / BigUint::new(vec!(2));
    }
    approx
}

fn factor(mut num: BigUint) -> Vec<BigUint> {
    let zero = BigUint::new(vec!(0));
    let one = BigUint::new(vec!(1));
    let two = BigUint::new(vec!(2));

    let mut ret = Vec::new();

    let sqrt = approx_sqrt(num.clone());
    for i in std::iter::range_inclusive(two, sqrt) {
        while num % i == zero {
            num = num / i;
            ret.push(i.clone());
        }
        if i > num {
            break;
        }
    }
    if num > one {
        ret.push(num);
    }
    ret
}

fn print_factors(num: BigUint) {
    print!("{}:", num);
    for fac in factor(num).iter() {
        print!(" {}", fac);
    }
    println!("");
}

#[allow(dead_code)]
fn main() { uumain(os::args()); }

pub fn uumain(args: Vec<String>) {
    let program = args.get(0).as_slice();
    let opts = [
        getopts::optflag("h", "help", "show this help message"),
        getopts::optflag("v", "version", "print the version and exit"),
    ];

    let matches = match getopts::getopts(args.tail(), opts) {
        Ok(m) => m,
        Err(f) => crash!(1, "Invalid options\n{}", f.to_err_msg())
    };

    if matches.opt_present("help") {
        println!("{} {}", program, VERSION);
        println!("");
        println!("Usage:");
        println!("  {0:s} [NUMBER]...", program);
        println!("  {0:s} [OPTION]", program);
        println!("");
        print!("{}", getopts::usage("Print the prime factors of the given number(s). If none are specified, read from standard input.", opts).as_slice());
        return;
    }
    if matches.opt_present("version") {
        println!("{} {}", program, VERSION);
        return;
    }

    if matches.free.is_empty() {
        for line in stdin().lines() {
            let num_str = line.unwrap();
            let num = match BigUint::parse_bytes(num_str.as_slice().trim().as_bytes(), 10) {
                Some(x) => x,
                None => { crash!(1, "{} not a number", num_str); }
            };

            print_factors(num);
        }
    } else {
        for num_str in matches.free.iter() {
            let num = match BigUint::parse_bytes(num_str.as_bytes(), 10) {
                Some(x) => x,
                None => { crash!(1, "{} not a number", num_str); }
            };

            print_factors(num);
        }
    }
}
