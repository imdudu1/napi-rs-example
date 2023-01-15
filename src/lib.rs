#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(js_name = "Person")]
pub struct Person {
  name: String,
  age: u32,
}

#[napi]
impl Person {
  #[napi(factory)]
  pub fn with_name_and_age(name: String, age: u32) -> Person {
    Self {
      name,
      age,
    }
  }

  #[napi(constructor)]
  pub fn new() -> Person {
    Person {
      name: "".to_string(),
      age: 0,
    }
  }

  #[napi]
  pub fn increase_age(&mut self) {
    self.age += 1;
  }

  #[napi]
  pub fn change_name(&mut self, new_name: String) {
    self.name = new_name;
  }

  #[napi(getter)]
  pub fn get_name(&self) -> String {
    self.name.clone()
  }

  #[napi(getter)]
  pub fn get_age(&self) -> u32 {
    self.age
  }
}

#[napi]
pub fn plus_100(input: u32) -> u32 {
  input + 100
}

#[napi]
pub fn minus_100(input: i32) -> i32 {
  input - 100
}

#[napi]
pub async fn async_plus_100(input: Promise<u32>) -> Result<u32> {
  let v = input.await?;

  Ok(v + 100)
}

#[test]
fn test_plus_100() {
  let expect = 110;

  assert_eq!(plus_100(10), expect);
}

#[test]
fn test_minus_100() {
  let expect = -90;

  assert_eq!(minus_100(10), expect);
}
