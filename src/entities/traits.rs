// Trait for anything that can move on the screen
pub trait Movable {
    fn update_position(&mut self);
}
// Trait for anything that's visible on screen
pub trait Visible {
    fn draw(&self);
}
