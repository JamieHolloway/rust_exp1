#![allow(dead_code)]     // directive is just a convenience when testing
mod funcs;
mod stack_and_heap;

fn main() 
{
    //funcs::data_types();  // function names need to be snake case per compiler
    stack_and_heap::memory();
}