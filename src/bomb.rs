use crate::entity::{Coord, Entity, Visible};
use crate::palette::COLOR_BOMB;
use crate::wasm4::SCREEN_SIZE;

#[derive(Debug)]
pub struct Bomb(Entity);

impl Bomb {
    const EXPANSION_RATE: f64 = 3.5;
    const DEFAULT_SIZE: f64 = 9.0;
    const DEFAULT_LIFE_SPAN: u32 = 60 / 2;

    pub fn new(pos: &Coord) -> Self {
        let mut b = Bomb(Entity::new());
        b.0.size = Self::DEFAULT_SIZE;
        b.0.position.x = pos.x;
        b.0.position.y = pos.y;
        b.0.speed = 0.0;
        b.0.life = Self::DEFAULT_LIFE_SPAN;
        b.0.color = COLOR_BOMB;
        b
    }

    pub fn grow(&mut self) {
        let grow_amt = Self::EXPANSION_RATE;
        self.0.size = (self.0.size + grow_amt).clamp(2.0, SCREEN_SIZE as f64);
        self.0.position.x -= (grow_amt as f64) / 2.0;
        self.0.position.y -= (grow_amt as f64) / 2.0;
        self.0.life -= 1;
    }

    pub fn get_life(&self) -> u32 {
        self.0.life
    }

    #[allow(dead_code)]
    pub fn get_id(&self) -> usize {
        self.0.id
    }
    pub fn get_super(&self) -> &Entity {
        &self.0
    }
}

impl Visible for Bomb {
    fn draw(&self) {
        self.0.draw();
    }
    fn get_size(&self) -> f64 {
        self.0.size
    }
    fn get_position(&self) -> Coord {
        self.0.position
    }
    // fn collided_with(&self, other: &impl Visible, extra_reach: f64) -> bool {
    //     self.0.collided_with(other, extra_reach)
    // }
}
