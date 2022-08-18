use crate::enemy::Enemy1;
use crate::entity::{Coord, Entity, Visible};
use crate::palette::COLOR3;
use crate::snake::Snake1;
use crate::wasm4::SCREEN_SIZE;

#[derive(Debug, Copy, Clone)]
pub struct Bomb(Entity);

impl Visible for Bomb {
    fn draw(&self) {
        self.0.draw();
    }
    fn collided_with(&self, other: &Entity) -> bool {
        self.0.collided_with(other)
    }
}

impl Bomb {
    pub fn new(pos: &Coord) -> Self {
        let mut b = Bomb(Entity::new());
        b.0.size = 8.0;
        b.0.position.x = pos.x;
        b.0.position.y = pos.y;
        b.0.speed = 0.0;
        b.0.life = 60 / 2;
        b.0.color = COLOR3;
        b
    }

    // pub fn get_position(&self) -> Coord {
    //     self.0.position
    // }
    // pub fn get_size(&self) -> u32 {
    //     self.0.size as u32
    // }

    pub fn grow(&mut self) {
        let grow_amt = 3.5;
        self.0.size = (self.0.size + grow_amt).clamp(2.0, SCREEN_SIZE as f64);
        self.0.position.x -= (grow_amt as f64) / 2.0;
        self.0.position.y -= (grow_amt as f64) / 2.0;
        self.0.life -= 1;
    }

    pub fn draw(&self) {
        self.0.draw();
    }

    pub fn life(&self) -> i32 {
        self.0.life
    }

    // pub fn id(&self) -> u32 {
    //     self.0.id
    // }
    pub fn collided_with(&self, snake: &Snake1) -> bool {
        let other: Entity = Entity {
            position: Coord {
                x: snake.get_position().x - 2.0,
                y: snake.get_position().y - 2.0,
            },
            direction: Coord { x: 0.0, y: 0.0 }, //don't care
            size: snake.get_size() as f64 + 4.0,
            speed: 0.0, //don't care
            id: 0,      //don't care
            color: 0,   //don't care
            life: 0,    //don't care
        };
        self.0.collided_with(&other)
    }

    pub fn collided_with_enemy(&self, enemy: &Enemy1) -> bool {
        let other: Entity = Entity {
            position: Coord {
                x: enemy.get_position().x - 2.0,
                y: enemy.get_position().y - 2.0,
            },
            direction: Coord { x: 0.0, y: 0.0 }, //don't care
            size: enemy.get_size() as f64 + 4.0,
            speed: 0.0, //don't care
            id: 0,      //don't care
            color: 0,   //don't care
            life: 0,    //don't care
        };
        self.0.collided_with(&other)
    }
}
