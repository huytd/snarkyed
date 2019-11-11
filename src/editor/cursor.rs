pub struct Cursor {
  pub row: i32,
  pub col: i32
}

impl Cursor {
  pub fn new() -> Cursor {
    Cursor {
      row: 0,
      col: 0
    }
  }
}