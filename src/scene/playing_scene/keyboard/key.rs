use crate::{
    config::ColorSchema,
    utils::{Point, Size},
};
use neothesia_pipelines::quad::QuadInstance;
use wgpu_jumpstart::Color;

pub struct Key {
    pub(super) pos: Point<f32>,
    pub(super) size: Size<f32>,
    pub(super) is_black: bool,
    pub(super) note_id: u8,

    pressed_by_file: Option<Color>,
    pressed_by_user: bool,
}

impl Key {
    pub fn new(is_black: bool) -> Self {
        Self {
            pos: Default::default(),
            size: Default::default(),
            is_black,
            note_id: 0,

            pressed_by_file: None,
            pressed_by_user: false,
        }
    }

    pub fn set_pressed_by_user(&mut self, is: bool) {
        self.pressed_by_user = is;
    }

    pub fn pressed_by_file_on(&mut self, schem: &ColorSchema) {
        let (r, g, b) = if self.is_black {
            schem.dark
        } else {
            schem.base
        };

        self.pressed_by_file = Some(Color::from_rgba8(r, g, b, 1.0));
    }

    pub fn pressed_by_file_off(&mut self) {
        self.pressed_by_file = None;
    }

    pub fn x_position(&self) -> f32 {
        self.pos.x
    }

    pub fn width(&self) -> f32 {
        self.size.w
    }

    pub fn is_black(&self) -> bool {
        self.is_black
    }
}

impl From<&Key> for QuadInstance {
    fn from(key: &Key) -> QuadInstance {
        let kind_multiplier = if key.is_black() { 1.0 } else { 3.5 };

        let radius = key.size.w * 0.08;
        let radius = radius * kind_multiplier;

        let color = if key.pressed_by_user {
            let v = if key.is_black() { 0.3 } else { 0.5 };
            Color::new(v, v, v, 1.0)
        } else if let Some(color) = key.pressed_by_file {
            color
        } else if key.is_black {
            Color::new(0.0, 0.0, 0.0, 1.0)
        } else {
            Color::new(1.0, 1.0, 1.0, 1.0)
        };

        QuadInstance {
            position: key.pos.into(),
            size: key.size.into(),
            color: color.into_linear_rgba(),
            border_radius: [0.0, 0.0, radius, radius],
        }
    }
}
