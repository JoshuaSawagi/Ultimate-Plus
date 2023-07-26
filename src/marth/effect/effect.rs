use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;

#[acmd_script( agent = "marth", script = "effect_attackairlw", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_marth_attackairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_marth_sword1"), Hash40::new("tex_marth_sword2"), 8, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 14.5, true, Hash40::new("marth_sword_blue"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 6);
    }
}

#[acmd_script( agent = "marth", script = "effect_specials3lw", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_marth_specials3lw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("none"), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("marth_mc_3lw"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("marth_sword_green"), false, true);
    }
}


pub fn install() {
    smashline::install_acmd_scripts!(
        effect_marth_attackairlw,
        effect_marth_specials3lw,
    );
}