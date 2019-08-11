fn main() {
    use std::mem;
    //println!("Hello rusty");
    let a:u8 = 123; // unsigned integer 8bits immutable
    let mut b:i8 = -123; // integer 8bits mutable
    println!("a={} b={}", a, b);
    b = 42;
    println!("a={} b={}", a, b);
    let c:u8 = a;
    println!("a={} b={} c={}", a, b, c);
    let d = 123456789;
    println!("b={} size = {} bytes", d, mem::size_of_val(&b));    
}
