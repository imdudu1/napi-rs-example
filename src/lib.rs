use napi::bindgen_prelude::*;
use napi_derive::napi;

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
