mod airdodge;
mod ground_movement;
mod hold_buffer;
mod jump_cancel;
mod landing;
mod ledge;
mod physics;
mod shield;
//pub mod momentum_transfer;
mod cliff;

pub fn install() {
    airdodge::install();
    ground_movement::install();    
    hold_buffer::install();
    jump_cancel::install();
    landing::install();
    ledge::install();
    physics::install();
    shield::install();
    cliff::install();
}