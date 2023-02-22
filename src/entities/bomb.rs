use super::{entity::Entity, player::PlayerN, traits::Visible};
use crate::{common::types::Coord, graphics::palette::DRAW_COLOR_BOMB, wasm4::SCREEN_SIZE};

pub struct Bomb {
    pub entity: Entity,
    pub exploded: bool,
    pub who_exploded: Option<PlayerN>,
    growth_rate: f64,
}

impl Bomb {
    pub fn new(pos: &Coord) -> Self {
        let mut bomb = Self::default();
        bomb.entity.position = *pos;
        bomb
    }

    pub fn update(&mut self) {
        if self.exploded {
            self.grow();
        }
    }

    fn grow(&mut self) {
        let grow_amt = self.growth_rate;
        self.entity.size = (self.entity.size + grow_amt).clamp(2.0, SCREEN_SIZE as f64 - 1.0);
        self.entity.position.x -= grow_amt / 2.0;
        self.entity.position.y -= grow_amt / 2.0;
        self.entity.life -= 1;
    }
}
impl Default for Bomb {
    fn default() -> Self {
        const DEFAULT_GROWTH_RATE: f64 = 3.5;
        const DEFAULT_SIZE: f64 = 9.0;
        const DEFAULT_LIFE_SPAN: u32 = 60 / 2;
        Self {
            entity: Entity {
                position: Coord::default(),
                direction: Coord::default(),
                size: DEFAULT_SIZE,
                speed: 0.0,
                // id: 0,
                color: DRAW_COLOR_BOMB,
                life: DEFAULT_LIFE_SPAN,
            },
            growth_rate: DEFAULT_GROWTH_RATE,
            exploded: false,
            who_exploded: None,
        }
    }
}

impl Visible for Bomb {
    fn draw(&self) {
        self.entity.draw();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grow() {
        let mut bomb = Bomb::default();
        bomb.exploded = true;
        bomb.entity.size = 10.0;
        bomb.entity.position = Coord { x: 10.0, y: 10.0 };
        bomb.entity.life = 1;
        bomb.grow();
        assert_eq!(bomb.entity.size, 13.5);
        assert_eq!(bomb.entity.position, Coord { x: 8.25, y: 8.25 });
        assert_eq!(bomb.entity.life, 0);
    }

    #[test]
    fn grow_max() {
        let mut bomb = Bomb::default();
        bomb.exploded = true;
        bomb.entity.size = SCREEN_SIZE as f64 - 1.0;
        bomb.entity.position = Coord { x: 10.0, y: 10.0 };
        bomb.entity.life = 1;
        bomb.grow();
        assert_eq!(bomb.entity.size, SCREEN_SIZE as f64 - 1.0);
        assert_eq!(bomb.entity.position, Coord { x: 8.25, y: 8.25 });
        assert_eq!(bomb.entity.life, 0);
    }
}
