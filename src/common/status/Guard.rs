/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use crate::imports::BuildImports::*;

#[skyline::hook(replace=smash::app::FighterUtil::is_valid_just_shield_reflector)]
pub unsafe fn is_valid_just_shield_reflector(_module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> bool {
    return true;
}

/* SHIELD STATUSES */
// Sub Guard Cont Pre
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_guard_cont_pre)]
pub unsafe fn status_sub_guard_cont_pre(fighter: &mut L2CFighterCommon) {
    let status = fighter.global_table[STATUS_KIND].get_i32();

    //*Shield Stop: Cancel Movement from Any Action
    if status == *FIGHTER_STATUS_KIND_WALK 
    || status == *FIGHTER_STATUS_KIND_DASH
    || status == *FIGHTER_STATUS_KIND_RUN
    || status == *FIGHTER_STATUS_KIND_TURN_RUN
    || status == *FIGHTER_STATUS_KIND_TURN_DASH {
        fighter.change_status(FIGHTER_STATUS_KIND_GUARD_ON.into(), false.into());

        // **Cancel All Motion Speed**
        KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);


        return; // Stop further status processing
    }

    // Shield Drop Mechanic
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && ControlModule::get_stick_y(fighter.module_accessor) < -0.66
    && GroundModule::is_passable_ground(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_PASS.into(), false.into());
        return;
    }

    // Preserve Catch Turn & Dash During Shield On
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_GUARD_ON
    && fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_RUN {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
        let catch_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("catch_dash_frame"));
        WorkModule::set_int(fighter.module_accessor, catch_dash_frame, *FIGHTER_STATUS_GUARD_ON_WORK_INT_CATCH_FRAME);
    }

    // Enable Shield-Related Transitions
    let transition_terms = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B
    ];
    for term in transition_terms.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *term);
    }
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
}

#[skyline::hook(replace = L2CFighterCommon_status_guard_main_common_air)]
pub unsafe fn status_guard_common_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        ControlModule::clear_command(fighter.module_accessor, true);
        true.into()
    } 
    else {
        false.into()
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_guard_main_common)]
pub unsafe fn status_guard_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    let shield_hp = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
    if shield_hp < 0.0 {
        fighter.change_status(FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY.into(), false.into());
        true.into()
    } 
    else {
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) 
        && WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME) <= 0 
        && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(), true.into());
            true.into()
        }
        else {
            false.into()
        }
    }
}

//Sub Guard Cont
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_guard_cont)]
pub unsafe fn status_sub_guard_cont(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cont_stick_x = fighter.global_table[STICK_X].get_f32();
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x = cont_stick_x * lr;
    let turn_run_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_run_stick_x"));
    let turn_run_stick_x_threshold = stick_x * lr <= turn_run_stick_x;
    let check_guard_hold = fighter.check_guard_hold().get_bool();
    let item_lua_stack_no_throw = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW); sv_module_access::item(fighter.lua_state_agent); !fighter.pop_lua_stack(1).get_bool()};
    let is_shield_stop = fighter.global_table[STATUS_KIND_INTERRUPT] == FIGHTER_STATUS_KIND_GUARD_ON && fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_RUN;
    if fighter.global_table[GUARD_CONT_UNIQ].get_bool() && {let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[GUARD_CONT_UNIQ].get_ptr()); callable(fighter).get_bool()} {
        return true.into();
    }
    if !check_guard_hold {
        if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() {
            return true.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD) && ItemModule::is_have_item(fighter.module_accessor, 0) && item_lua_stack_no_throw {
            if fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 
            || (fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER == 0
            && fighter.global_table[CMD_CAT3].get_i32() & (*FIGHTER_PAD_CMD_CAT3_ITEM_LIGHT_THROW_HI | *FIGHTER_PAD_CMD_CAT3_ITEM_LIGHT_THROW_HI4) != 0) {
                if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                    fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
                    return true.into();
                }
            }
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE)
        && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE != 0
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE.into(), true.into());
            return true.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F)
        && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE_F != 0
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_F.into(), true.into());
            return true.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B)
        && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE_B != 0
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_B.into(), true.into());
            return true.into();
        }
        
        /* END OF NEW ADDITION */
    }
    if is_shield_stop {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN)
        && turn_run_stick_x_threshold
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
        && !ItemModule::is_have_item(fighter.module_accessor, 0) {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_TURN.into(), true.into());
            return true.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH)
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
        && !ItemModule::is_have_item(fighter.module_accessor, 0) {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_DASH.into(), true.into());
            return true.into();
        }
    }
    if !fighter.check_guard_attack_special_hi(check_guard_hold.into()).get_bool() {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CATCH_FRAME) == 0 {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH)
            && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0
            && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
            && !ItemModule::is_have_item(fighter.module_accessor, 0) {
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH.into(), true.into());
                return true.into();
            }
        }
        if !check_guard_hold {
            if fighter.sub_transition_group_check_ground_jump().get_bool() {
                return true.into();
            }
        }
        false.into()
    }
    else {
        true.into()
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_end_Guard)]
pub unsafe fn status_end_Guard(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_GUARD 
    && fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_GUARD_DAMAGE {
        effect!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("sys_shield"), true, true);
        effect!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("sys_shield_smoke"), true, true);
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_sub_guard_cont_pre,
            status_guard_common_air,
            status_guard_main_common,
            status_sub_guard_cont,
            status_end_Guard
        );
    }
}

pub fn install() {
    skyline::install_hook!(
        is_valid_just_shield_reflector
    );
    skyline::nro::add_hook(nro_hook);
}
