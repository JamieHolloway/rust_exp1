#![allow(dead_code)]     // directive is just a convenience when testing
mod funcs;
mod stack_and_heap;

fn main() 
{
    //funcs::data_types();  // function names need to be snake case per compiler
    //stack_and_heap::memory();
    if_statement();
}

fn if_statement()
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
}