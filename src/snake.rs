use crate::entity::{Coord, Entity, Movable, Visible};
use crate::palette::{COLOR1, COLOR2, HEART};
use crate::wasm4::{blit, BLIT_1BPP, SCREEN_SIZE};

#[derive(Debug, Copy, Clone)]
pub struct Snake1(Entity);

impl Movable for Snake1 {
    fn update_position(&mut self) {
        self.0.update_position();
    }
}

impl Visible for Snake1 {
    // fn draw(&self) {
    //     set_draw_color(0x22);
    //     wasm4::oval(
    //         self.0.position.x as i32,
    //         self.0.position.y as i32,
    //         self.0.size as u32,
    //         self.0.size as u32,
    //     );
    //     // wasm4::oval(10, 10, 10, 10);
    // }

    // fn collided_with(&self, other: &Entity) -> bool {
    //     // Check if we're outside the square bounding box,
    //     !(((self.0.position.x > other.position.x + other.size as f64)
    //         || ((self.0.position.x + self.0.size as f64) < other.position.x))
    //         && ((self.0.position.y > other.position.y + other.size as f64)
    //             || ((self.0.position.y + self.0.size as f64) < other.position.y)))
    // }
    fn draw(&self) {
        self.0.draw();
    }
    fn collided_with(&self, other: &Entity) -> bool {
        self.0.collided_with(other)
    }
}

impl Snake1 {
    pub fn new() -> Self {
        let mut s = Snake1(Entity::new());
        s.0.size = 6.0;
        s.0.position.x = (SCREEN_SIZE as f64 - s.0.size) / 2.0;
        s.0.position.y = (SCREEN_SIZE as f64 - s.0.size) / 2.0;
        s.0.direction.x = 1.0;
        s.0.speed = 1.45;
        s.0.color = COLOR2;
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

    pub fn get_position(&self) -> Coord {
        self.0.position
    }
    pub fn get_size(&self) -> u32 {
        self.0.size as u32
    }
    pub fn get_life(&self) -> i32 {
        self.0.life
    }
    pub fn set_life(&mut self, life: i32) {
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
    // pub fn get_direction(&self) -> Coord {
    //     self.0.direction
    // }

    // pub fn grow(&mut self) {
    //     let grow_amt = 0.8;
    //     self.0.size = (self.0.size + grow_amt).clamp(6.0, SCREEN_SIZE as f64);
    //     self.0.position.x = (self.0.position.x - (grow_amt as f64) / 2.0)
    //         .clamp(0.0, (SCREEN_SIZE - self.get_size()) as f64);
    //     self.0.position.y = (self.0.position.y - (grow_amt as f64) / 2.0)
    //         .clamp(0.0, (SCREEN_SIZE - self.get_size()) as f64);
    // }
    // pub fn shrink(&mut self) {
    //     let shrink_amt = 3.0;
    //     let shrinkage = self.0.size - (self.0.size - shrink_amt).clamp(6.0, SCREEN_SIZE as f64);
    //     self.0.size -= shrinkage;
    //     self.0.position.x = (self.0.position.x + (shrinkage as f64) / 2.0)
    //         .clamp(0.0, (SCREEN_SIZE - self.get_size()) as f64);
    //     self.0.position.y = (self.0.position.y + (shrinkage as f64) / 2.0)
    //         .clamp(0.0, (SCREEN_SIZE - self.get_size()) as f64);
    // }
    pub fn switch_color(&mut self) {
        if self.0.color == COLOR1 {
            self.0.color = COLOR2;
        } else {
            self.0.color = COLOR1;
        }
    }
    // pub fn set_c1(&mut self) {
    //     self.0.color = COLOR1;
    // }
    // pub fn set_c2(&mut self) {
    //     self.0.color = COLOR2;
    // }

    pub fn get_color(&self) -> u16 {
        self.0.color
    }
}
