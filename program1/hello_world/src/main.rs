use std::mem ;

mod sh;
mod data_structures;
mod pm;

const MEANING_OF_LIFE:u8 = 42;//constant, must give it a type. no fixed address. like a #define. actual val is subsituted

static Z:i32 = 123; //kind of like a constant. but it does have a fixed address.

static mut X:i32 = 123; //anytime you use this, you must use an unsafe block

fn main()
{
//   sh::stack_and_heap(); //calling a public function from another file
//   control_flow();
//   data_structures::tuples();
  arc_demo();
}

fn data_types()
{
  /*Data Types*/
  let a:u8 = 123; //declare variable a as an unsigned 8 byte integer

  println!("a = {}", a);

  //a = 456; cannot do, a is immutable

  //using the Mutable keyword
  let mut b:i8 = 0; //b is a signed 8 byte mutable integer
  b = 42;

  let mut c = 123456789; // 32 bit value default
  println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
  c = -1;

  //i is signed, u is unsigned. 8 16 32 64

  let z:isize = 123; //isize or usize, just the pointer size
  let size_of_z = mem::size_of_val(&z);
  println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, 8 * size_of_z);


  let d:char = 'x'; //size of a char is 4 bytes!
  println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));


  //non whole number types
  let e = 2.5; //default is f64
  println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

  //Booleans!
  let g = false;
  println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
}

fn operators()
{
  /*operators*/

  let mut a = 2+3*4;
  println!("{}", a);
  a = a + 1; //there is no ++ and --!!, however, -=, += /= etc... still there
  println!("can have expressions in {}", (a%3));

  let b = 2.5;
  let b_cubed = f64::powi(b, 3);
  let b_to_pi = f64::powf(b, std::f64::consts::PI);
  println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

  //bitwise  or | , and &, XOR ^, NOR !
  //logical is the same
}

fn scope_and_shadowing()
{
  //pretty much the same
  {
    //can have scope brackets like this out of nowhere
  }
}

fn constants()
{
  unsafe
  {
    println!("{}", X); //this line won't work without unsafe because it is static and mutable
  }
}

fn control_flow()
{
  /*if statement*/

  let temp = 35;
  if temp > 30
  {
    println!("really hot outside!");
  }
  else if temp < 10
  {
    println!("Really Cold!");
  }
  else
  {
    println!("Temp is okay");
  }

  /*if statements are expressions allowing:*/
  let day = if temp > 20 {"sunny"} else {"cloudy"};
  println!("today is {}", day);


  /*Can even be in the println!*/
  println!("it is {}",
    if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"OK"}
  );


  /*Ifs inside ifs*/
  println!("it is {}",
    if temp > 20 {
      if temp > 30 {"very hot"} else {"hot"}
    } else if temp < 10 {"cold"} else {"OK"}
  );


  /********************************************************************************************************************/
  /*Looping*/

  let mut x = 1;
  while x < 1000
  {
    x*=2;

    /*continue keyword. jump back up to testing condition*/
    if x == 64 {continue;}

    println!("x = {}", x);
  }

  let mut y = 1;
  loop //while true
  {
    y *= 2;
    println!("y = {}", y);

    /* << is a left shift on the bits*/
    if y == 1<<10 {break;}

  }

  /*For Loops*/
  for x in 0..10
  {
    println!("x = {}", x);
  }

  /*pos goes from 0 to 10. y goes from 30 to 40. Basically counting 2 things at once*/
  for (pos,y) in (30..41).enumerate()
  {
    println!("{}: {}", pos, y);
  }


/**********************************************************************************************************************/
  /*match statement*/
  let country_code = 6;


  /*NOTE: 1..999 is everything from 1 to 999 not including 999 but 1...999 includes 999*/

  /*This will match the first one it can*/
  /*NOTE: 999 is matched*/
  let country = match country_code
  {
    44 => "UK",
    46 => "Sweden",
    7 => "Russia",
    1...999 => "some country",// if its anything between 1 and 999 thats not listed above,
    _ => "Invalid" //_ is default
  };

  println!("The country with code {} is {}", country_code, country);

}


fn is_even(x: i32) -> bool
{
  x % 2 == 0
}

fn hof()
{
  /*Closures : passing functions as parameters. functional programming*/

  let limit = 500;
  let mut sum = 0;
  for i in 0..
  {
    let isq = i*i;

    if isq > limit {break;}
    else if is_even(isq) {sum += isq;}
  }


  /*map takes a function as a parameter. here we are mapping x to x*x and then doing some other stuff. notice the funxitons
   are similar to Haskell */
  let sum2 =
    (0..).map(|x| x*x)
      .take_while(|&x| x <= limit)
      .filter(|x| is_even(*x))
      .fold(0, |sum,x| sum + x);
}

/**********************************************************************************************************************/

/*Dispatch, two types. static and dynamic*/

trait Printable
{
  fn format(&self) -> String;
}

impl Printable for i32
{
  fn format(&self) -> String
  {
    format!("i32: {}", *self)
  }
}

impl Printable for String
{
  fn format(&self) -> String
  {
    format!("String: {}", *self)
  }
}

/*When this is compiled you get concrete types put into this. the function is basically written for you with the types
 * This is static dispatch.
 * Less expensive. inherently faster
 */
fn print_it<T: Printable>(z : T)
{
  println!("{}", z.format());
  //monomorphization
}

/*Dynamic Dispatch.
 * Whenever this function is called, at runtime the type is determined and this function is called. We lose all information
 * regarding the type. We only know its a Printable "Trait Object" and it can do all stuff defined for Printable.
 * More expensive. inherently slower.
 * The type may not be known at compile time.
 */
fn print_it_too(z : &Printable)
{
  //Type erasure
  println!("{}", z.format());
}

fn traits()
{
  let a = 123;
  let b = "hello".to_string();

    // println!("{}", a.format());
    // println!("{}", b.format());
    // print_it(a);
    // print_it(b);

  print_it_too(&a);
  print_it_too(&b);


}

/**********************************************************************************************************************/

/*Ownership
* Only a single variable can own a piece of memory at a time
* Non primitive data types are really "pointers" and can't be copied by default. can implement a copy trait
* functions can return values they take ownership of and transfer ownership back
*/
fn ownership()
{
    let v = vec![1,2,3]; //v owns this vector

    // let v2 = v; //v2 invalidates v and takes ownership of the vector.

    // println!("{}", v); // cannot do this line!

    let foo = |v:Vec<i32>| ();
    foo(v); //this closure takes ownership of v and you can't use it after

    let u = 1;
    let u2 = u;//however, for primitives, full copies are made. both u and u2 are valid after this line

    let u3 = Box::new(1);
    let u4 = u3; //Box::new is always a pointer so u3 is invalidated at this point.

    let print_vector = |x:Vec<i32>| -> Vec<i32>
    {
        println!("{:?}", x);
        x //however this returns control back
    };

}

/*
* Borrowing
* lets you borrow a value and not take Ownership of a value
* keep in mind you can only have one mutable reference to a variable at a time in a given scope!
* You can have more than 1 reference to a resource, but only 1 mutable reference.
* YOu cannnot mix immutable and mutable references.
* You cannot borrow an immutable value as mutable!!
*/
fn borrow()
{
    let print_vector = |x:&Vec<i32>|
    {
        println!("x[0] = {}", x[0]);
    };
    let v = vec![3,2,1];
    print_vector(&v); //this just borrows the vector and you get it back, when the function ends

    let mut a = 40;
    {
        let b = &mut a;
        *b += 2 //the * operator allows you to access what the reference is pointing to
    }
    println!("{}", a); //you can only do this line because b went out of scope.

    let mut z = vec![3,2,1];
    for i in &z //for i in reference to z. You are iterating here. and you cannot add values when iterating
    {
        println!("i = {}", i);
    }
}

/* Lifetimes
* When you store references to variables in other places, like a struct you open up the possibility of
* having the data that the reference is to, going out of scope or being destroyed for some reason.
* For example, below, what if the Person object referenced in Company is destroyed before Company,
* you will have a dangling pointer. Lifetimes allow guaruntees that that won't happen
*/

struct Person
{
    name: String
}

impl Person
{
    fn get_ref_name<'a>(&'a self) -> &'a String //these lifetimes are not necessary because in this case, the compiler
    // can infer the lifetimes. but it normally cannot.
    {
        &self.name
    }
}

struct Company<'z> //this specifies a lifetime. We're saying that the Company object is valid only as long as the Person
//object is has a reference to is valid.
{
    name: String,
    ceo: &'z Person
}

fn lifetime()
{
    let boss = Person {name: String::from("Elon Musk")};
    let tesla = Company {name: String::from("Tesla"), ceo: &boss};

    let mut z: &String;
    {
        // let p = Person {name: String::from("John")};
        // z = p.get_ref_name()
    }
}

/*Reference Counted variables
* A controlled way to share variables around, without the standard rust ownership stuff.
* Not Thread Safe
*/
use std::rc::Rc;

struct Person2
{
    name : Rc<String> //Its an Rc<string>
}

impl Person2{
    fn new(name: Rc<String>) -> Person2
    {
        Person2 {name : name}
    }
    fn greet(&self)
    {
        println!("HI my name is {}", self.name);
    }
}

fn rc_demo()
{
    let name = Rc::new("John".to_string());
    println!("Name = {} has {} strong pointers", name, Rc::strong_count(&name));
    {
    let person = Person2::new(name.clone()); //Have to call clone here to increment reference count
    println!("Name = {} has {} strong pointers", name, Rc::strong_count(&name));
    person.greet();
    }//Notice person goes out of scope here, so one of the pointers to name is destroyed
    println!("Name = {} has {} strong pointers", name, Rc::strong_count(&name));
}

/*Atomic Reference Counted variables
* A way to do the same thing but thread safe.
*/
use std::thread;
use std::sync::Arc;

struct Person3
{
    name : Arc<String> //Its an Arc<string>
}

impl Person3{
    fn new(name: Arc<String>) -> Person3 // An Arc string
    {
        Person3 {name : name}
    }
    fn greet(&self)
    {
        println!("HI my name is {}", self.name);
    }
}

fn arc_demo()
{
    let name = Arc::new("JOhn".to_string());
    let person = Person3::new(name.clone()); //Have to call clone here to increment reference count
    let t = thread::spawn(move || {
        person.greet();
        });
    println!("Name = {}", name);
    t.join().unwrap();
}
