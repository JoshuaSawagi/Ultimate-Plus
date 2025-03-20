mod JumpAerial;
pub mod FighterStatusGuard;
mod Float;
mod Fly;
pub mod GlideStart;
pub mod Glide;
pub mod GlideAttack;
pub mod GlideEnd;
pub mod GlideLanding;
mod Sub_Glide_Checks;
mod Catch;
pub mod Guard;
mod GuardOn;
mod GuardDamage;
mod Dash;
mod AttackDash;
mod Edge_Cancel;
mod SpecialHi;
mod EscapeAir;
mod Rebirth;
mod Damage;
mod Ground;

pub fn install() {
    JumpAerial::install();
    FighterStatusGuard::install();
    Float::install();
    Fly::install();
    GlideStart::install();
    Glide::install();
    GlideAttack::install();
    GlideEnd::install();
    GlideLanding::install();
    Sub_Glide_Checks::install();
    Catch::install();
    Guard::install();
    GuardOn::install();
    GuardDamage::install();
    Dash::install();
    AttackDash::install();
    Edge_Cancel::install();
    SpecialHi::install();
    EscapeAir::install();
    Rebirth::install();
    Damage::install();
    Ground::install();

}