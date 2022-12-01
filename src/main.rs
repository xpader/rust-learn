#![allow(dead_code)]

mod structs;
mod learn_trait_objects;
mod learn_debug_display;
mod learn_hashmap;

use std::{fmt::Display};
use crate::structs::Point;
use crate::learn_hashmap::learn_hashmap;
use crate::learn_trait_objects::learn_trait_object;
use learn_debug_display::learn_debug_display;

#[allow(dead_code)]
fn read() -> ! {
    unimplemented!()
}

fn first_word(s: &String) -> &str {
    &s[..1]
}

fn learn_struct() {
    let bp = Box::new(Point{x: 32, y:64});

    // let ubp = *bp;

    println!("{:?}", &*bp);

    let mut s = String::from("hello world");
    s.push_str(" Holla");

    let c = first_word(&s);


    //此处仍然保持原有不可变引用，下面 clear 会使用可变引用会报错
    // let f = c.clone();

    //创建一个放在堆上的字符串则不再有引用，不会报错
    let f = String::from(c);

    s.clear(); // error!

    println!("the first word is: {}", f);

}

fn learn_array() {
    let a: [i8; 5] = [3; 5];
    println!("{:?}", a);
    println!("{}", a[1]);

    let str = "Rust is good";
    let str_arr: [String; 5] = core::array::from_fn(|_| String::from(str));
    println!("{:?}", str_arr);
}

enum IpAddr {
    V4(String),
    V6(String)
}

impl Display for IpAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            IpAddr::V4(ip) => write!(f, "v4 -> {}", ip),
            IpAddr::V6(ip) => write!(f, "v6 -> {}", ip)
        }
    }
}

fn learn_enum() {
    let ipv4 = IpAddr::V4("192.168.1.1".to_string());
    let ipv6 = IpAddr::V6("::1".to_string());

    if let IpAddr::V4(v) = &ipv4 {
        println!("This is a ipv4: {}", v);
    }

    match &ipv6 {
        IpAddr::V6(v) => println!("This is a ipv6: {}", v),
        _ => ()
    }

    let ip_list = vec![ipv4, ipv6];

    for ip in &ip_list {
        println!("{}", ip);
    }
}

fn learn_vec() {
    let mut v: Vec<u8> = Vec::new();
    v.push(1);
    v.push(128);
    println!("{:?}", &v);

    let mut v2 = Vec::with_capacity(1);
    v2.push(512);
    println!("{:?}", v2);

    let mut v3 = vec![111, 222, 333];
    v3.push(666);

    match v3.get(2) {
        Some(val) => println!("{}", val),
        None => println!("None val")
    }

    for i in &mut v3 {
        *i += 1;
    }

    println!("{:?}", v3);
}

fn learn_loop() {
    //for
    for i in 0..9 {
        if i == 2 {
            continue;
        } else if i == 9 {
            break;
        }

        println!("for at {}", i);
    }

    for i in 0..=9 {
        println!("for with ..= {}", i);
    }

    //while
    let mut i = 0;

    while i < 10 {
        if i == 6 {
            break;
        }
        println!("while at {}", i);
        i += 1;
    }

    //loop
    let mut i = 0;
    loop {
        if i == 7 {
            break;
        }
        println!("loop at {}", i);
        i += 1;
    };
}

fn learn_match() {
    let cc: Option<u8> = Option::Some(8);

    match cc {
        Some(n) => println!("match is {}", n),
        None => println!("match is none")
    }

    if let Some(n) = cc {
        println!("cc in if let is {}", n);
    } else {
        println!("cc in if let is none");
    }
}

impl Point {
    fn dump(&self) {
        println!("dump point: {:?}", &self);
    }

    fn move_up(&mut self) {
        self.y -= 1;
    }

    fn move_right(&mut self) {
        self.x += 1;
    }
}

fn learn_method() {
    let mut p = Point {
        x: 123,
        y: 456
    };
    p.dump();

    p.move_up();
    println!("after move up: {:?}", p);

    p.move_right();
    println!("after move right: {:?}", p);
}

fn learn_clone_copy() {
    let a = Point {
        x: 123,
        y: 456
    };

    let clone_a = a.clone();
    let copy_a = a;
    let ref_a = &a;
    let ref_a2 = &a;

    println!("a: {:?}", a);
    println!("clone_a: {:?}", clone_a);
    println!("copy_a: {:?}", copy_a);
    println!("ref_a: {:?}", ref_a);
    println!("ref_a2: {:?}", ref_a2);
}

fn learn_type_as() {
    let a: i32 = 128;
    let b: u8 = 255;

    if a < b as i32 {
        println!("a < b");
    }

    println!("8: {}..{}, 0..{}", i8::MIN, i8::MAX, u8::MAX);
    println!("16: {}..{}, 0..{}", i16::MIN, i16::MAX, u16::MAX);
    println!("32: {}..{}, 0..{}", i32::MIN, i32::MAX, u32::MAX);
    println!("64: {}..{}, 0..{}", i64::MIN, i64::MAX, u64::MAX);
    println!("128: {}..{}, 0..{}", i128::MIN, i128::MAX, u128::MAX);

    // as_ptr() 是获取它的内存地址
    let v = vec![a, b as i32];
    let p1 = v.as_ptr() as usize;
    println!("{}", p1);
}

fn main() {
    learn_struct();
    learn_array();
    learn_vec();
    learn_enum();
    learn_loop();
    learn_match();
    learn_method();
    learn_clone_copy();
    learn_debug_display();
    learn_trait_object();
    learn_hashmap();
    learn_type_as();
}
