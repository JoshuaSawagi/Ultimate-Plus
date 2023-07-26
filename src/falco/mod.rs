mod game;
mod effect;
mod opff;

pub fn install() {
    game::install();
    effect::install();
    opff::install();
}