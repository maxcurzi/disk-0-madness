use crate::{
    common::{Coord, Movable, Visible},
    draws::pixel,
    entity::Entity,
    palette::{self, COLOR1, COLOR2},
    wasm4::SCREEN_SIZE,
};

#[derive(Clone, Copy, Debug)]
pub enum PlayerN {
    P1 = 0,
    P2 = 1,
    P3 = 2,
    P4 = 3,
}
#[derive(Debug)]
pub struct Player {
    pub entity: Entity,
    pub player_number: PlayerN,
}

impl Player {
    pub fn new(player_number: PlayerN) -> Self {
        let mut p = Self::default();
        p.player_number = player_number;
        p
    }

    pub fn left(&mut self) {
        self.entity.direction = Coord {
            x: -1.0,
            y: self.entity.direction.y,
        };
    }

    pub fn right(&mut self) {
        self.entity.direction = Coord {
            x: 1.0,
            y: self.entity.direction.y,
        };
    }

    pub fn up(&mut self) {
        self.entity.direction = Coord {
            x: self.entity.direction.x,
            y: -1.0,
        };
    }

    pub fn down(&mut self) {
        self.entity.direction = Coord {
            x: self.entity.direction.x,
            y: 1.0,
        };
    }
    pub fn stop(&mut self) {
        self.entity.direction = Coord { x: 0.0, y: 0.0 };
    }

    pub fn toggle_color(&mut self) {
        if self.entity.color == COLOR1 {
            self.entity.color = COLOR2;
        } else {
            self.entity.color = COLOR1;
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        const DEFAULT_SIZE: f64 = 7.0;
        const DEFAULT_SPEED: f64 = 1.40;

        Self {
            entity: Entity {
                position: Coord {
                    x: (SCREEN_SIZE as f64 - DEFAULT_SIZE) / 2.0,
                    y: (SCREEN_SIZE as f64 - DEFAULT_SIZE) / 2.0,
                },
                direction: Coord { x: 0.0, y: 0.0 },
                size: DEFAULT_SIZE,
                speed: DEFAULT_SPEED,
                color: COLOR2,
                life: 1,
                id: 0, // TODO: generate id
            },
            player_number: PlayerN::P1,
        }
    }
}

impl Movable for Player {
    fn update_position(&mut self) {
        self.entity.update_position();
    }
}

impl Visible for Player {
    fn draw(&self) {
        self.entity.draw();

        let radius = self.entity.size / 2.0 - 0.5;
        let center_coord = self.entity.position
            + Coord {
                x: radius,
                y: radius,
            };

        let mut dots: Vec<Coord> = vec![];

        // Add dots in the center of the disk to distinguish player number
        match self.player_number {
            PlayerN::P1 => dots.push(center_coord),
            PlayerN::P2 => {
                dots.push(Coord {
                    x: center_coord.x - 1.0,
                    y: center_coord.y,
                });
                dots.push(Coord {
                    x: center_coord.x + 1.0,
                    y: center_coord.y,
                });
            }
            PlayerN::P3 => {
                dots.push(Coord {
                    x: center_coord.x - 1.0,
                    y: center_coord.y,
                });
                dots.push(Coord {
                    x: center_coord.x,
                    y: center_coord.y - 1.0,
                });
                dots.push(Coord {
                    x: center_coord.x + 1.0,
                    y: center_coord.y,
                });
            }
            PlayerN::P4 => {
                dots.push(Coord {
                    x: center_coord.x - 1.0,
                    y: center_coord.y,
                });
                dots.push(Coord {
                    x: center_coord.x,
                    y: center_coord.y + 1.0,
                });
                dots.push(Coord {
                    x: center_coord.x,
                    y: center_coord.y - 1.0,
                });
                dots.push(Coord {
                    x: center_coord.x + 1.0,
                    y: center_coord.y,
                });
            }
        }
        palette::set_draw_color(0x33);
        for center_coord in dots {
            pixel(center_coord.x as i32, center_coord.y as i32);
        }
    }
}
