#![allow(unused_imports)] // shotgun
use std::mem;
use std::fmt;
//use ste::fmt::Debug;

#[derive(Debug)]
struct Point
{
    x: f64,
    y: f64
}git 

fn origin() -> Point
{
    Point{x: 0.0, y:0.0}
}

pub fn memory()
{
    let p1 = origin();
    let p2 = Box::new(origin());
    println!("p1 uses {} bytes of stack and p2 uses {} bytes of heap",mem::size_of_val(&p1),mem::size_of_val(&p2)); // returns 16 and 8... 16 for the value on the stack, 8 for the pointer to the heap on a x64 system
    let p3 = *p2;
    println!("x = {}",p3.x);
    println!("formatting p3 = \n{:#?}",p3);
}
