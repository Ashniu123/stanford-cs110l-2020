fn main() {
  let s1 = String::from("hello");
  let mut v = Vec::new();
  v.push(s1);
  let s2: String = v[0];
  println!("{}", s2);
}