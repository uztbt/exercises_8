pub fn convert(string: &mut String) -> Option<String> {
  let mut string_iter = string.chars();
  if let Some(first) = string_iter.next() {
    match first {
      'a' | 'e' | 'i' | 'o' | 'u' => {
        string.push_str("-hay");
        Some(string.to_string())
      },
      _ => {
        let mut from_second = string_iter.collect::<String>();
        from_second.push('-');
        from_second.push(first);
        from_second.push_str("ay");
        Some(from_second)
      }
    }
  } else {
    None
  }
}