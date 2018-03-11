/*Pattern Matching*/
/* 
 * 
 */

fn how_many(x:i32) -> &'static str
{
  match x
  {
    0 => "no",
    _ => "yes"
  }
}


pub fn pattern_matching()
{
  for x in 0..13
  {
    println!("{}: I have {} oranges", x, how_many(x));
  }
}