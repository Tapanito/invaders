use std::error::Error;

use crate::{bullet::Bullet, frame::Drawable, PLAYER_CELL};

pub struct Player {
    // the player is made of three pieces
    positions: [(i32, i32); 3],
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Player {
            positions: [(x - 1, y), (x, y - 1), (x + 1, y)],
        }
    }

    pub fn try_move(&self, dir: i32) -> Vec<(i32, i32)> {
        self.positions.iter().map(|f| (f.0 + dir, f.1)).collect()
    }

    pub fn apply_move(&mut self, dir: i32) {
        self.positions[0].0 += dir;
        self.positions[1].0 += dir;
        self.positions[2].0 += dir;
    }

    pub fn fire(&self) -> Bullet {
        Bullet::new(self.positions[1].0, self.positions[1].1)
    }
}

impl Drawable for Player {
    fn values(&self) -> Vec<(usize, usize, &'static str)> {
        self.positions
            .iter()
            .map(|f| (f.0 as usize, f.1 as usize, PLAYER_CELL))
            .collect()
    }
}
