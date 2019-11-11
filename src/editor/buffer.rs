extern crate ropey;
use ropey::Rope;

pub struct Buffer {
  content: Rope,
}

impl Buffer {
  pub fn new(file: &str) -> Buffer {
    let content = Rope::from_reader(std::fs::File::open(file).unwrap()).unwrap();
    Buffer {
      content: content
    }
  }

  pub fn get_lines_count(&self) -> usize {
    self.content.len_lines()
  }

  pub fn get_lines(&self, from: usize, to: usize) -> String {
    let start = self.content.line_to_char(from);
    let end = self.content.line_to_char(to);
    String::from(self.content.slice(start..end))
  }
}