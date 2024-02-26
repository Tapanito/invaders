use crate::{frame::Drawable, BULLET_CELL};

pub struct Bullet {
    position: (i32, i32),
}

impl Bullet {
    pub fn new(x: i32, y: i32) -> Self {
        Bullet { position: (x, y) }
    }

    pub fn try_move(&self, _: i32) -> Vec<(i32, i32)> {
        vec![(self.position.0, self.position.1 - 1)]
    }

    pub fn apply_move(&mut self, _: i32) {
        if self.position.1 > 0 {
            self.position.1 -= 1;
        }
    }
}

impl Drawable for Bullet {
    fn values(&self) -> Vec<(usize, usize, &'static str)> {
        vec![(
            self.position.0 as usize,
            self.position.1 as usize,
            BULLET_CELL,
        )]
    }
}
