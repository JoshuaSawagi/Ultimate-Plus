mod movement;
mod wavedash;
mod edge_cancel;
mod remove_quake;
mod shine_cancel;
mod jump_cancel;
mod run;
mod dacus;
mod dash;
mod melee;
mod aerial;
mod cliff;

pub fn install() {
    movement::install();
    wavedash::install();
    edge_cancel::install();
    remove_quake::install();
    shine_cancel::install();
    jump_cancel::install();
    run::install();
    dacus::install();
    dash::install();
    melee::install();
    aerial::install();
    cliff::install();
}