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

// pub fn convert_using_slices(string: &mut String) -> Option<String> {
//   if string == "" { return None; }
//   let first_letter = &string[0..1];
//   match first_letter {
//     "a" | "e" | "i" | "o" | "u" => {
//       string.push_str("-hay");
//       Some(string.clone())
//     },
//     _ => {
//       let from_second = &string[1..];
//       let pig_latin = format!("{}-{}ay", from_second, first_letter);
//       Some(pig_latin)
//     }
//   }
// }

pub fn convert_using_slices_and_split_at(string: &str) -> Option<String> {
  if string == "" { return None; }
  let (head, tail) = string.split_at(1);
  match head {
    "a" | "e" | "i" | "o" | "u" => {
      let res = format!("{}-hay", string);
      Some(res)
    },
    _ => {
      let res = format!("{}-{}ay", tail, head);
      Some(res)
    }
  }
}