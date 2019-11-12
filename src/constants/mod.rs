use glium::glutin::ModifiersState;

pub const BASE_FONT_SIZE: f32 = 16.0;

pub const NO_MODIFIERS: ModifiersState = ModifiersState {
  alt: false, ctrl: false, logo: false, shift: false
};

pub const CTRL_HOLD: ModifiersState = ModifiersState {
  alt: false, ctrl: true, logo: false, shift: false
};

pub const SHIFT_HOLD: ModifiersState = ModifiersState {
  alt: false, ctrl: false, logo: false, shift: true
};

pub const CMD_SHIFT_HOLD: ModifiersState = ModifiersState {
  alt: false, ctrl: false, logo: true, shift: true
};