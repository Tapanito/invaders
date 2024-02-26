use crate::bullet::Bullet;

pub struct Invader {
    position: (i32, i32),
}

impl Invader {
    pub fn new(x: i32, y: i32) -> Self {
        Invader { position: (x, y) }
    }

    pub fn hit(&self, bullet: &Bullet) -> bool {
        return self.position == bullet.position;
    }
}
