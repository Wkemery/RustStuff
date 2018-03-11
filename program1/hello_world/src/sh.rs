#![allow(dead_code)]

use std::mem;


struct Pointa
{
  x: f64,
  y: f64
}

fn origin() -> Pointa
{
  Pointa{x: 0.0, y: 0.0}
}

pub fn stack_and_heap()
{
  /*Let bindings are stack operations*/
  /*There are pointers and new operations*/
  
  let p1 = origin(); //stack allocated
  let p2 = Box::new(origin());//heap allocated. p2 is a pointer
  
  println!("p1 takes up {} bytes", mem::size_of_val(&p1));
  println!("p2 takes up {} bytes", mem::size_of_val(&p2));
  
  let p3 = *p2; //dereferencing the pointer
  println!("{}", p3.x);
  
}