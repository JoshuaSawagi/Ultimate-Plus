use super::*;


unsafe extern "C" fn sound_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let dash_sfx_handle = SoundModule::play_se(boma, Hash40::new("se_elight_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 20.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_elight_step_left_l"));
    }
}

unsafe extern "C" fn game_escapen(agent: &mut L2CAgentBase) { }

unsafe extern "C" fn game_escapeair(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    frame(lua_state, 29.0);
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }

}

unsafe extern "C" fn game_escapeairslide(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    frame(lua_state, 29.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }

}

unsafe extern "C" fn effect_cliffattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 7, -6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_open"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_beam_s"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_sword_hdr"), Hash40::new("tex_elight_sword2"), 6, Hash40::new("sword1"), 0, 0, -0.08, Hash40::new("sword1"), 14.5, 0, -0.08, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.7, 0, 0, 4, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_light"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.85, true);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_light"), false, true);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_beam_s"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_sword_close_s"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
}

unsafe extern "C" fn game_jumpaerialfront(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    if is_excute(agent) {
        GroundModule::select_cliff_hangdata(boma, 1);
    }
}

unsafe extern "C" fn game_jumpaerialback(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    if is_excute(agent) {
        GroundModule::select_cliff_hangdata(boma, 1);
    }
}

pub fn install(agent: &mut Agent) {

    agent.acmd("sound_dash", sound_dash, Priority::Low);

    agent.acmd("game_escapen", game_escapen, Priority::Low);

    agent.acmd("game_escapeair", game_escapeair, Priority::Low);
    agent.acmd("game_escapeairslide", game_escapeairslide, Priority::Low);

    agent.acmd("effect_cliffattack", effect_cliffattack, Priority::Low);

    agent.acmd("game_jumpaerialfront", game_jumpaerialfront, Priority::Low);
    agent.acmd("game_jumpaerialback", game_jumpaerialback, Priority::Low);
}