
fn main() {
  let s = drip_drop();
}

fn drip_drop() -> &String {
  let s = String::from("hello world!");
  return &s;
}