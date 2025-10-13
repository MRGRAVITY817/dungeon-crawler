use crate::prelude::*;

/// A camera acts as your game's viewport into the world
pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    /// Create a new camera centered around the player
    pub fn new(player_pos: Point) -> Self {
        Self {
            left_x: player_pos.x - DISPLAY_WIDTH / 2,
            right_x: player_pos.x + DISPLAY_WIDTH / 2,
            top_y: player_pos.y - DISPLAY_HEIGHT / 2,
            bottom_y: player_pos.y + DISPLAY_HEIGHT / 2,
        }
    }

    /// Update the camera's position based on the player's position
    pub fn on_player_move(&mut self, player_pos: Point) {
        self.left_x = player_pos.x - DISPLAY_WIDTH / 2;
        self.right_x = player_pos.x + DISPLAY_WIDTH / 2;
        self.top_y = player_pos.y - DISPLAY_HEIGHT / 2;
        self.bottom_y = player_pos.y + DISPLAY_HEIGHT / 2;
    }
}
