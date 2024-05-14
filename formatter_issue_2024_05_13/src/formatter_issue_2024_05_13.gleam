pub fn main() {
  wibble("Hello")
}

fn wibble(value: String) {
  case [], [], [] {
    [day_string, time_string], _, _
    | _, [day_string, time_string], _
    | _, _, [day_string, time_string]
    -> Ok(#(day_string, time_string))
    [_], [_], [_] -> Ok(#(value, "00"))
    _, _, _ -> Error(Nil)
  }
}
