pub mod bullet;
pub mod frame;
pub mod invader;
pub mod player;
pub mod render;

pub const NUM_ROWS: usize = 20;
pub const NUM_COLS: usize = 40;

pub const EMPTY_CELL: &str = " ";
pub const PLAYER_CELL: &str = "X";
pub const BULLET_CELL: &str = "^";
