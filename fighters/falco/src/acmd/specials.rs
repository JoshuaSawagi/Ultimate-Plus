use super::*;


unsafe extern "C" fn game_specialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, false, 0);
        if ArticleModule::is_exist(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER){
            ArticleModule::change_motion(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("open"), false, 0.0);
        }
    }
}

unsafe extern "C" fn sound_specialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_falco_special_n02"));
    }
}

unsafe extern "C" fn game_specialnloop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        ArticleModule::generate_article(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET, false, 0);
        if ArticleModule::is_exist(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER){
            ArticleModule::change_motion(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("loop"), false, 0.0);
        }
    }
    frame(lua_state, 7.0);
    frame(lua_state, 17.0);
    if is_excute(agent) {
    WorkModule::off_flag(boma, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
    }
}

unsafe extern "C" fn game_specialairnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, false, 0);
        if ArticleModule::is_exist(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER){
            ArticleModule::change_motion(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("open"), false, 0.0);
        }
    }
}

unsafe extern "C" fn game_specialairnloop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        //DamageModule::add_damage(boma, 1.0, 0);
        ArticleModule::generate_article(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET, false, 0);
        if ArticleModule::is_exist(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER){
            ArticleModule::change_motion(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("loop"), false, 0.0);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
        if WorkModule::is_flag(boma,*FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP){
            //laser loop motion rate(changing this allows for faster or slower sequential lasers from the first)
            FT_MOTION_RATE(agent,0.10);
        }
    }
}


pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnstart", game_specialnstart, Priority::Low);
    agent.acmd("game_specialairnstart", game_specialairnstart, Priority::Low);
    agent.acmd("game_specialnloop", game_specialnloop, Priority::Low);
    agent.acmd("game_specialairnloop", game_specialairnloop, Priority::Low);;
    
}
