pub mod FighterStatusGuard;

mod Catch;
pub mod Guard;
mod GuardOn;
mod GuardDamage;
mod Dash;
mod AttackDash;
mod Edge_Cancel;

mod EscapeAir;
mod Rebirth;
mod Damage;
mod Ground;

pub fn install() {

    FighterStatusGuard::install();

    Catch::install();
    Guard::install();
    GuardOn::install();
    GuardDamage::install();
    Dash::install();
    AttackDash::install();
    Edge_Cancel::install();

    EscapeAir::install();
    Rebirth::install();
    Damage::install();
    Ground::install();
}