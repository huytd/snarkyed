pub struct Color {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32
}

impl Color {
  pub fn as_slice(&self) -> [f32; 4] {
    self.into()
  }
}

impl std::convert::From<&Color> for [f32; 4] {
    fn from(c: &Color) -> Self {
      [c.r, c.g, c.b, c.a]
    }
}

pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> Color {
  Color {
    r: r as f32 / 255.0,
    g: g as f32 / 255.0,
    b: b as f32 / 255.0,
    a: a
  }
}

pub fn rgb(r: u8, g: u8, b: u8) -> Color {
  rgba(r, g, b, 1.0)
}

pub fn hex(h: &str) -> Color {
  let c = parse_hex(h);
  rgb(c[0], c[1], c[2])
}

pub fn hexa(h: &str, a: f32) -> Color {
  let c = parse_hex(h);
  rgba(c[0], c[1], c[2], a)
}

fn parse_hex(hex_asm: &str) -> Vec<u8> {
    let mut hex_bytes = hex_asm.as_bytes().iter().filter_map(|b| {
        match b {
            b'0'...b'9' => Some(b - b'0'),
            b'a'...b'f' => Some(b - b'a' + 10),
            b'A'...b'F' => Some(b - b'A' + 10),
            _ => None,
        }
    }).fuse();

    let mut bytes = Vec::new();
    while let (Some(h), Some(l)) = (hex_bytes.next(), hex_bytes.next()) {
        bytes.push(h << 4 | l)
    }
    bytes
}