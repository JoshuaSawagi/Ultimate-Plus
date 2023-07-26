use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;

#[acmd_script( agent = "roy", script = "sound_attacks3hi", category = ACMD_SOUND, low_priority )]
unsafe fn sound_roy_attacks3hi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_roy_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_roy_attackl_s01"));
    }
}

#[acmd_script( agent = "roy", script = "sound_attacks3", category = ACMD_SOUND, low_priority )]
unsafe fn sound_roy_attacks3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_roy_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_roy_attackl_s01"));
    }
}

#[acmd_script( agent = "roy", script = "sound_attacks3lw", category = ACMD_SOUND, low_priority )]
unsafe fn sound_roy_attacks3lw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_roy_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_roy_attackl_s01"));
    }
}


pub fn install() {
    smashline::install_acmd_scripts!(
        sound_roy_attacks3hi,
        sound_roy_attacks3,
        sound_roy_attacks3lw
    );
}