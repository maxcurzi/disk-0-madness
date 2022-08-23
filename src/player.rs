use crate::draws::pixel;
use crate::entity::{Coord, Entity, Movable, Visible};
use crate::palette::{self, COLOR1, COLOR2};
use crate::wasm4::SCREEN_SIZE;

#[derive(PartialEq, Clone, Copy)]
pub enum PlayerN {
    P1 = 0,
    P2 = 1,
    P3 = 2,
    P4 = 3,
}
#[derive(Debug)]
pub struct Player(Entity);

impl Player {
    const DEFAULT_SIZE: f64 = 7.0;
    const DEFAULT_SPEED: f64 = 1.40;

    pub fn new(player_number: PlayerN) -> Self {
        let mut s = Player(Entity::new());
        s.0.size = Self::DEFAULT_SIZE;
        s.0.position.x = (SCREEN_SIZE as f64 - s.0.size) / 2.0; // Start centered
        s.0.position.y = (SCREEN_SIZE as f64 - s.0.size) / 2.0; // Start centered
        s.0.direction.x = 1.0;
        s.0.speed = Self::DEFAULT_SPEED;
        s.0.color = COLOR2;
        s.0.id = player_number as usize;
        s
    }

    pub fn left(&mut self) {
        self.0.direction = Coord {
            x: -1.0,
            y: self.0.direction.y,
        };
    }

    pub fn right(&mut self) {
        self.0.direction = Coord {
            x: 1.0,
            y: self.0.direction.y,
        };
    }

    pub fn up(&mut self) {
        self.0.direction = Coord {
            x: self.0.direction.x,
            y: -1.0,
        };
    }

    pub fn down(&mut self) {
        self.0.direction = Coord {
            x: self.0.direction.x,
            y: 1.0,
        };
    }
    pub fn stop(&mut self) {
        self.0.direction = Coord { x: 0.0, y: 0.0 };
    }
    pub fn get_life(&self) -> u32 {
        self.0.life
    }
    pub fn set_life(&mut self, life: u32) {
        self.0.life = life;
    }

    pub fn set_direction(&mut self, Coord { x, y }: Coord) {
        self.0.direction.x = x;
        self.0.direction.y = y;
    }

    pub fn set_position(&mut self, Coord { x, y }: Coord) {
        self.0.position.x = x;
        self.0.position.y = y;
    }

    pub fn switch_color(&mut self) {
        if self.0.color == COLOR1 {
            self.0.color = COLOR2;
        } else {
            self.0.color = COLOR1;
        }
    }

    pub fn get_id(&self) -> usize {
        self.0.id
    }

    pub fn get_color(&self) -> u16 {
        self.0.color
    }

    #[allow(dead_code)]
    pub fn get_super(&self) -> &Entity {
        &self.0
    }
}

impl Movable for Player {
    fn update_position(&mut self) {
        self.0.update_position();
    }
}

impl Visible for Player {
    fn draw(&self) {
        self.0.draw();

        let radius = self.get_size() / 2.0 - 0.5;
        let center_coord = self.get_position()
            + Coord {
                x: radius,
                y: radius,
            };

        let mut dots: Vec<Coord> = vec![];
        // Add dots to make the disk more recognisable
        match self.get_id() {
            0 => dots.push(center_coord),
            1 => {
                dots.push(Coord {
                    x: center_coord.x - 1.0,
                    y: center_coord.y,
                });
                dots.push(Coord {
                    x: center_coord.x + 1.0,
                    y: center_coord.y,
                });
            }
            2 => {
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
            _ => {
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
    // fn collided_with(&self, other: &impl Visible, extra_reach: f64) -> bool {
    //     self.0.collided_with(other, extra_reach)
    // }

    fn get_size(&self) -> f64 {
        self.0.size
    }

    fn get_position(&self) -> Coord {
        self.0.position
    }
}
