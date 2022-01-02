fn main() {
  let mut s = String::from("hello");
  let ref1 = &s;
  let ref2 = &ref1;
  let ref3 = &ref2;
  s = String::from("goodbye");
  println!("{}", ref3.to_uppercase());
}