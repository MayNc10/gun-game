use crate::gun::Gun;
use crate::movement::Position;

pub struct Player {
    health: f64,
    guns_equipped: Vec<Gun>,
    position: Position,
}
