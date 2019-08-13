#![allow(dead_code)]     // directive is just a convenience when testing
mod funcs;
mod stack_and_heap;

// git hotfix practice

fn main() 
{
    //funcs::data_types();  // function names need to be snake case per compiler
    //stack_and_heap::memory();
    control_flow();
    data_structures();
}

fn data_structures()
{
    let x = 3.0;
    let y = 0.0;
    let result:Option<f64> = if y != 0.0 { Some(x/y)} else { None };
    println!("{:?}", result);

    let a:[i32;5] = [1,2,3,4,5];
    println!("a has {} elemetns, first is {}", a.len(), a[0]);
    println!("{:?}",a);

    let b = [1; 10]; 
    println!("{:?}",b)
    for i in 0..b.len()]
}

fn control_flow()
{
    let temp = 40;

    if temp > 30
    {
        println!("temp is gt 30")
    }
    else 
    {
        println!("temp is le 30")
    }

    let day = if temp > 30 {"sunny"} else {"cooler"};
    println!("{}",day);
    println!("the weather is {}",if temp > 30 {"hot"} else if temp < 30 {"cold"} else {"neither"});

    loop
    {
        println!("short loop");
        break;
    }

    let mut running = true;
    while running
    {
        println!("short while");
        running = false;
    }

    // match statemen does similar to case in ohter langs
}