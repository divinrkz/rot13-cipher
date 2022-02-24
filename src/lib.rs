/**
 * ROT13 Algorithm Implementation
 */
pub fn rot13(str: String) -> String {
  let mut converted: String = String::with_capacity(str.len());

  for chr  in str.bytes() {
    let mut adjusted = chr;
    if chr >= b'a' && chr <= b'z' {
      if chr > b'm' {
        adjusted -= 13;
      } else {
        adjusted += 13;
      }
    } else if chr >= b'A' && chr <= b'Z' {
      if chr > b'M' {
        adjusted -= 13;
      } else {
        adjusted += 13;
      }
    }
    converted.push(adjusted as char);
  }
  converted
}