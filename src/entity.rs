use crate::{
    common::{Coord, Movable, Visible},
    palette,
    wasm4::{self, SCREEN_SIZE},
};

#[derive(Debug, Clone)]
pub struct Entity {
    pub position: Coord,
    pub direction: Coord,
    pub size: f64,
    pub speed: f64,
    // pub id: usize, // unused
    pub color: u16,
    pub life: u32,
}

impl Entity {
    /// Distance between the center of two visible entities (assumed circular)
    pub fn distance(&self, other: &Entity) -> f64 {
        let radius1 = other.size / 2.0 - 0.5;
        let radius2 = self.size / 2.0 - 0.5;

        let center1 = other.position
            + Coord {
                x: radius1,
                y: radius1,
            };
        let center2 = self.position
            + Coord {
                x: radius2,
                y: radius2,
            };

        center1.distance_to(&center2)
    }

    pub fn collided_with(&self, other: &Entity, extra_reach: f64) -> bool {
        //Circular bounding box collision
        let radius1 = other.size / 2.0 - 0.5;
        let radius2 = self.size / 2.0 - 0.5;

        self.distance(other) < radius1 + radius2 + extra_reach
    }
}

/// All entities by default will move in the direction they are facing
impl Movable for Entity {
    fn update_position(&mut self) {
        let norm = self.direction.norm();
        if norm <= f64::EPSILON {
            return;
        }
        self.position += self.direction.scale(self.speed / norm);
        self.position = self.position.clamp(
            0.0,
            SCREEN_SIZE as f64 - self.size,
            0.0,
            SCREEN_SIZE as f64 - self.size,
        );
    }
}

/// All entities by default will be drawn as a circle
impl Visible for Entity {
    fn draw(&self) {
        palette::set_draw_color(self.color);
        wasm4::oval(
            self.position.x as i32,
            self.position.y as i32,
            self.size as u32,
            self.size as u32,
        );
    }
}
