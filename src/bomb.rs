use crate::enemy::Enemy;
use crate::entity::{Coord, Entity, Visible};
use crate::palette::COLOR_BOMB;
use crate::player::Player;
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
        b.0.size = 9.0;
        b.0.position.x = pos.x;
        b.0.position.y = pos.y;
        b.0.speed = 0.0;
        b.0.life = 60 / 2;
        b.0.color = COLOR_BOMB;
        b
    }

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

    pub fn id(&self) -> usize {
        self.0.id
    }

    pub fn collided_with_player(&self, player: &Player) -> bool {
        let extra_reach = 1.0;
        let other: Entity = Entity {
            position: Coord {
                x: player.get_position().x - extra_reach,
                y: player.get_position().y - extra_reach,
            },
            direction: Coord { x: 0.0, y: 0.0 }, //don't care
            size: player.get_size() as f64 + extra_reach * 2.0, // Make bombs easy to trigger
            speed: 0.0,                          //don't care
            id: 0,                               //don't care
            color: 0,                            //don't care
            life: 0,                             //don't care
        };
        self.0.collided_with(&other)
    }

    pub fn collided_with_enemy(&self, enemy: &Enemy) -> bool {
        let extra_reach = 2.0;
        let other: Entity = Entity {
            position: Coord {
                x: enemy.get_position().x - extra_reach,
                y: enemy.get_position().y - extra_reach,
            },
            direction: Coord { x: 0.0, y: 0.0 }, //don't care
            size: enemy.get_size() as f64 + extra_reach * 2.0, // Make bombs turn enemies easily
            speed: 0.0,                          //don't care
            id: 0,                               //don't care
            color: 0,                            //don't care
            life: 0,                             //don't care
        };
        self.0.collided_with(&other)
    }
}
