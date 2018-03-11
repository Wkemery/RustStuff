

struct Point
{
  x: f64,
  y: f64
}

struct Line
{
  start: Point,
  end: Point
}

pub fn structures()
{
  /*Strucutres are pretty much the same*/
  let p = Point { x : 3.0, y: 4.0};
  println!("point p is at ({}, {})", p.x, p.y);
  
  let p2 = Point {x: 5.0, y:10.0};
  let myline = Line {start: p, end: p2 };
  
}
/**********************************************************************************************************************/
enum Color
{
  Red,
  Green,
  Blue,
  RgbColor(u8, u8, u8), //Tuple like. RgbColor consists of 3 u8 values cannot name them when declared this way
  CmykColor{cyan:u8, magenta:u8, yellow:u8, black:u8}, /*Struct like. Curly brace constuct allows you to name them. Notice everywhere
  this is used later it also uses {}*/
}

/*Enums work great with match statements. They just define some names that you can set to variables and test later */
pub fn enums()
{
//   let c:Color = Color::Red;
//   let c:Color = Color::RgbColor(10,0,0);
  let c:Color = Color::CmykColor{cyan: 0, magenta: 128, yellow: 0, black: 0};
  
  match c 
  {
    /*Pattern matching against elements of Enums*/
    Color::Red => println!("r"),
    Color::Green => println!("g"),
    Color::Blue => println!("b"),
    Color::RgbColor(0,0,0) //We're allowed to do an or symbol | for the same funciton for same case
    | Color::CmykColor{cyan: _, magenta:_, yellow:_, black:255} => println!("Black"),
    Color::RgbColor(r,g,b) => println!("rgb {}, {}, {}", r, g, b), // abstracting it a bit
    _ => ()
  }
}

/**********************************************************************************************************************/
/*Unions*/

/*Placeholder for either a i32 or an f32.*/
/*Need to do a lot of unsafe stuff with these. but can also use match on them*/
union IntOrFloat
{
  i: i32,
  f: f32,
}


fn proccess_value(iof: IntOrFloat)
{
  unsafe
  {
    match iof{
      IntOrFloat { i: 42} => {println!("Maining of life");}
      IntOrFloat {f } => {println!("f32 = {}", f);}
    }
  }
}

pub fn unions()
{
  let mut iof = IntOrFloat {i: 123}; //can only assign one of the values never more than that
  
  unsafe {iof.i = 42;}
  
  let value = unsafe {iof.i};
  
  proccess_value(iof);
}

/**********************************************************************************************************************/
/*Option<T> */
//Allow you to not return a result, like Haskell
//Option can either contain a Some(z) or None

//if let and while let

pub fn option()
{
  //Option<T>
  
  let x = 3.0;
  let y = 2.0;
  
  let result:Option<f64> = 
    if( y != 0.0) {Some(x/y)} else {None};
    
  println!("{:?}", result);
  
  //Get the option as the real value
  match result 
  {
    Some(z) => println!("{}/{} = {}", x, y, z),
    None => println!("Cannot divide {} by {}", x, y),
  }
  
  //if let. This only comes true if the destructuring works. think of let Some(z) = result as being the condition to 
  // be evauluated.
  if let Some(z) = result {println!("z = {}", z);}
  
}

/**********************************************************************************************************************/
/*Arrays*/
/* there is a .len() function
 * really only syntax is different
 */

pub fn array()
{
  let mut a:[i32; 5] = [1,2,3,4,5];
  // or let mut a:[5] = [1,2,3,4,5];
  
  let b = [1; 10]; //10 elements all initialized to 1
  //can also let b = [1u16; 10];
  for i in 0..b.len()
  {
    println!("{}", b[i]);
    
  }
  
  // Multidimensional arrays - just an array of arrays
  let mtx:[[f32;3]; 2] = 
  [
    [1.0, 0.0, 0.0],
    [0.0, 2.0, 0.0]
  ];
  println!("{:?}", mtx);
  
  
  //iterating over rows and columns
  for i in 0..mtx.len()
  {
    for j in 0..mtx[i].len()
    {
      if i == j
      {
        println!("mtx[{}][{}] = {}", i, j, mtx[i][j])
      }
    }
  }
}

/**********************************************************************************************************************/
/*Vectors*/
/* push() instead of push_back()
 * can use [], but the get() function will return an Option type
 * there is a pop() function. also returns Option
 */

pub fn vectors()
{
  let mut a = Vec::new();
  a.push(1);
  a.push(2);
  a.push(3);
  
  println!("a = {:?}", a);
  
  let idx:usize = 0; //This must be a usize! to be used as access var below
  println!("a[0] = {}", a[idx]); //Unsafe Code!
  
  //SAFE code!
  match a.get(6)
  {
    Some(x) => println!("a[6] = {}", x),
    None => println!("Error, no such element")
  }
  
  //iterating over vector
  for x in &a {println!("{}", x);}
  
  let last_elem = a.pop();
  println!("Last element is {:?}, a = {:?}", last_elem, a);
  
  while let Some(x) = a.pop()
  {
    println!("{}", x);
  }
  
}


/**********************************************************************************************************************/
/*Slices*/
/* parts of arrays, take a slice of an arrray and do something with it
 * 
 */


fn use_slice(slice: &mut [i32]) //we're borrowing a part of an arrary of i32s, the mut here is optional 
{
  println!("First elem = {}, len = {}", slice[0], slice.len());
}

pub fn slices()
{
  let mut data = [1,2,3,4,5];
  
  use_slice(&mut data[1..4]); //the mut here is optional, but must match
}

/**********************************************************************************************************************/
/*Strings*/
/* chars() function gives you sequence of characters
 */

pub fn strings()
{
  let s:&'static str = "hello there!"; //&str is a string slice. static because its hard coded
  
  //Not much you can do with s
  if let Some(first_char) = s.chars().nth(0)
  {
    println!("first letter is {}", first_char);
  }
  
  //Heap style
  let mut letters = String::new();
  let mut a = 'a' as u8;
  while a <= ('z' as u8)
  {
    letters.push(a as char); // for chars only
    letters.push_str(","); //for appending strings
    a+=1;
  }
  
  println!("{}", letters);
  
  //converting between string slice and String
  let u:&str = &letters;
  
}

/**********************************************************************************************************************/
/*Tuples*/
/* are variables taken together whenever you want. doesn't have to be defined somewhere like a struct. 
 * accessing with dot operator or destructuring
 * destructuring is a lot like Haskell and what we did with Option<T>. Can destructure as far down as you need
 * Tuples of Tuples is allowed but accessing is a bit different need ()
 */


fn sum_and_product(x:i32, y:i32) -> (i32, i32)
{
  (x+y, x*y) 
}

pub fn tuples()
{
//   let x = 3;
//   let y = 4;
//   let sp = sum_and_product(x,y);
//   println!("sp = {:?}", sp);
//   
//   println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);
//   
//   /*destructuring*/
//   let (a,b) = sp;
//   println!("a = {}. b = {}", a, b);
//   
//   /*Can have tuples of tuples*/
//   let combined = (sp, sp2);
//   println!("{:?}", combined);
//   println!("last elem = {}", (combined.1).1);
//   
//   let ((c,d),(e,f)) = combined;
//   
//   /*Tuple of single element*/
//   let meaning = (42,);
  
}

