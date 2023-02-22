use crate::{
    common::types::Coord,
    entities::traits::{Movable, Visible},
    graphics::palette,
    wasm4::{self, SCREEN_SIZE},
};

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

#[cfg(test)]
mod tests {
    use crate::{common::types::Coord, entities::traits::Movable};
    use approx::assert_abs_diff_eq;

    use super::Entity;

    #[test]
    fn distance_same() {
        let e1 = Entity {
            position: Coord { x: 0.0, y: 0.0 },
            direction: Coord { x: 0.0, y: 0.0 },
            size: 1.0,
            speed: 0.0,
            color: 0,
            life: 0,
        };
        let e2 = Entity {
            position: Coord { x: 0.0, y: 0.0 },
            direction: Coord { x: 0.0, y: 0.0 },
            size: 1.0,
            speed: 0.0,
            color: 0,
            life: 0,
        };
        assert_abs_diff_eq!(e1.distance(&e2), 0.0);
    }
    #[test]
    fn distance_same_size() {
        let e1 = Entity {
            position: Coord { x: 0.0, y: 0.0 },
            direction: Coord { x: 0.0, y: 0.0 },
            size: 1.0,
            speed: 0.0,
            color: 0,
            life: 0,
        };
        let e2 = Entity {
            position: Coord { x: 1.0, y: 0.0 },
            direction: Coord { x: 0.0, y: 0.0 },
            size: 1.0,
            speed: 0.0,
            color: 0,
            life: 0,
        };
        assert_abs_diff_eq!(e1.distance(&e2), 1.0);
    }
    #[test]
    fn distance_all_different() {
        let e1 = Entity {
            position: Coord { x: 0.0, y: 0.0 },
            direction: Coord { x: 0.0, y: 0.0 },
            size: 10.0,
            speed: 0.0,
            color: 0,
            life: 0,
        };
        let e2 = Entity {
            position: Coord { x: 10.0, y: 10.0 },
            direction: Coord { x: 0.0, y: 0.0 },
            size: 10.0,
            speed: 0.0,
            color: 0,
            life: 0,
        };
        assert_abs_diff_eq!(e1.distance(&e2), 14.142135623730951);
    }

    #[test]
    fn collided_with_distance() {
        let e1 = Entity {
            position: Coord { x: 0.0, y: 0.0 },
            direction: Coord { x: 0.0, y: 0.0 },
            size: 20.0,
            speed: 0.0,
            color: 0,
            life: 0,
        };
        let e2 = Entity {
            position: Coord { x: 10.0, y: 10.0 },
            direction: Coord { x: 0.0, y: 0.0 },
            size: 20.0,
            speed: 0.0,
            color: 0,
            life: 0,
        };
        assert!(e1.collided_with(&e2, 0.0));
        assert!(!e1.collided_with(&e2, -10.0));
    }

    #[test]
    fn collided_with_size() {
        let mut e1 = Entity {
            position: Coord { x: 0.0, y: 0.0 },
            direction: Coord { x: 0.0, y: 0.0 },
            size: 10.0,
            speed: 0.0,
            color: 0,
            life: 0,
        };
        let e2 = Entity {
            position: Coord { x: 100.0, y: 100.0 },
            direction: Coord { x: 0.0, y: 0.0 },
            size: 10.0,
            speed: 0.0,
            color: 0,
            life: 0,
        };
        assert!(!e1.collided_with(&e2, 0.0));
        e1.size = 150.0;
        assert!(e1.collided_with(&e2, 0.0));
    }

    #[test]
    fn update_position_diagonal() {
        let mut e1 = Entity {
            position: Coord { x: 0.0, y: 0.0 },
            direction: Coord { x: 1.0, y: 1.0 },
            size: 10.0,
            speed: 10.0,
            color: 0,
            life: 0,
        };
        e1.update_position();
        assert_abs_diff_eq!(e1.position.x, 7.071067811865475);
        assert_abs_diff_eq!(e1.position.y, 7.071067811865475);
    }

    #[test]
    fn update_position_horizontal() {
        let mut e1 = Entity {
            position: Coord { x: 0.0, y: 0.0 },
            direction: Coord { x: 1.0, y: 0.0 },
            size: 10.0,
            speed: 10.0,
            color: 0,
            life: 0,
        };
        e1.update_position();
        assert_abs_diff_eq!(e1.position.x, 10.0);
        assert_abs_diff_eq!(e1.position.y, 0.0);
    }
}
