pub fn data_types()
{
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
    let e:isize = 123;
    let size_of_e = mem::size_of_val(&e);
    println!("e = {}, takes up {} bytes, {}-bit OS",e, size_of_e, size_of_e * 8);
    let f:char = 'x'; // can use let f = 'x' an compiler will derive type
    println!("f = {} size of char f is {} bytes",f, mem::size_of_val(&f));
    let g = 2.5;  // double preceision f64...or it could be explicitly ste at f32 bits
    println!("g = {} size of float g is {} bytes", g, mem::size_of_val(&g));
    let h = false;
    println!("h = {} size of float h is {} bytes", h, mem::size_of_val(&h));  
    let mut i = 2+3*4;
    println!("i = {}",i);  
    i = i + 1;
    i += 1;
    println!("i = {}", i);
    println!("reaminder of {}/{} = {}",i, 3, (i%3));
    let i_cubed = i32::pow(i,3);
    println!("i cubed is {}", i_cubed);
    println!("PI is {}",(std::f64::consts::PI));
}