use smashline::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::L2CValue;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smash::phx::Hash40;
use crate::globals::*;
use smash::hash40;
use smash::app::SituationKind;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_IS_WAVEDASH: i32 = 506;

//Changes your situation kind if you're wavedashing
static mut IS_WAVEDASH: [bool; 8] = [false; 8];
static mut FORCE_WAVEDASH: [bool; 8] = [false; 8];
static mut WAVEDASH_DONE: [bool; 8] = [false; 8];
const TRACTION_MAX: f32 = 0.086;
const TRACTION_HIGH: f32 = 0.075;
const TRACTION_MID: f32 = 0.06;
const TRACTION_LOW: f32 = 0.054;
const TRACTION_MIN: f32 = 0.045;

pub(crate) fn get_wd_length(fighter_kind : i32) -> f32 {
	let max = [
		*FIGHTER_KIND_LITTLEMAC, *FIGHTER_KIND_CHROM, *FIGHTER_KIND_DEMON,
		*FIGHTER_KIND_PEACH, *FIGHTER_KIND_DAISY, *FIGHTER_KIND_SZEROSUIT,
		*FIGHTER_KIND_ROCKMAN, *FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_ROBOT,
		*FIGHTER_KIND_KEN, *FIGHTER_KIND_RYU, *FIGHTER_KIND_ELIGHT
	];
	let high = [
		*FIGHTER_KIND_MURABITO, *FIGHTER_KIND_ZELDA, *FIGHTER_KIND_GANON,
		*FIGHTER_KIND_ROY, *FIGHTER_KIND_MIIFIGHTER, *FIGHTER_KIND_LUCAS,
		*FIGHTER_KIND_KROOL, *FIGHTER_KIND_MARIOD, *FIGHTER_KIND_WOLF,
		*FIGHTER_KIND_PIKACHU, *FIGHTER_KIND_PICKEL, *FIGHTER_KIND_YOUNGLINK,
		*FIGHTER_KIND_DONKEY, *FIGHTER_KIND_WIIFIT, *FIGHTER_KIND_PITB,
		*FIGHTER_KIND_SHEIK, *FIGHTER_KIND_GAOGAEN, *FIGHTER_KIND_DOLLY,
		*FIGHTER_KIND_DIDDY, *FIGHTER_KIND_PLIZARDON
	];
	let low = [
		*FIGHTER_KIND_PURIN, *FIGHTER_KIND_TANTAN, *FIGHTER_KIND_SONIC,
		*FIGHTER_KIND_GEKKOUGA, *FIGHTER_KIND_PALUTENA, *FIGHTER_KIND_MARTH,
		*FIGHTER_KIND_IKE, *FIGHTER_KIND_TOONLINK, *FIGHTER_KIND_LUCARIO,
		*FIGHTER_KIND_KAMUI, *FIGHTER_KIND_GAMEWATCH, *FIGHTER_KIND_JACK,
		*FIGHTER_KIND_ROSETTA, *FIGHTER_KIND_RIDLEY, *FIGHTER_KIND_TRAIL,
		*FIGHTER_KIND_MASTER, *FIGHTER_KIND_BRAVE, *FIGHTER_KIND_PACMAN,
		*FIGHTER_KIND_SNAKE, *FIGHTER_KIND_METAKNIGHT
	];
	let min = [
		*FIGHTER_KIND_INKLING, *FIGHTER_KIND_LUIGI, *FIGHTER_KIND_SHIZUE,
		*FIGHTER_KIND_BUDDY, *FIGHTER_KIND_NANA, *FIGHTER_KIND_POPO,
		*FIGHTER_KIND_PIKMIN, *FIGHTER_KIND_KOOPAJR, *FIGHTER_KIND_PACKUN, 
		*FIGHTER_KIND_SAMUSD, *FIGHTER_KIND_PZENIGAME, *FIGHTER_KIND_EFLAME
	];
	if max.contains(&fighter_kind) {
		return TRACTION_MAX;
	};
	if high.contains(&fighter_kind) {
		return TRACTION_HIGH;
	};
	if low.contains(&fighter_kind) {
		return TRACTION_LOW;
	};
	if min.contains(&fighter_kind) {
		return TRACTION_MIN;
	};
	return TRACTION_MID;
}

#[skyline::hook(replace = StatusModule::change_status_request_from_script)]
pub unsafe fn change_status_hook(module_accessor: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, unk: bool) -> u64 {
    let next_status = status_kind;
    let get_kind = smash::app::utility::get_kind(&mut *module_accessor);
    if get_kind == *FIGHTER_KIND_ALL {
        if [*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B].contains(&next_status) {
            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP_MINI) {
                original!()(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, false);
            } 
            else {
                original!()(module_accessor, status_kind, unk);
            }
        }
        else if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&next_status) {
            if WorkModule::is_flag(module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_WAVEDASH) {
                StatusModule::set_situation_kind(module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            }
            original!()(module_accessor, status_kind, unk);
        }
        else {
            original!()(module_accessor, status_kind, unk);
        }
    }
    return original!()(module_accessor, status_kind, unk);
}

#[skyline::hook(replace = smash::app::lua_bind::StatusModule::change_status_request)]
pub unsafe fn change_status_request_hook(module_accessor: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, arg3: bool) -> u64 {
    let next_status = status_kind;
    if smash::app::utility::get_category(module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if [*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B].contains(&next_status) {
            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP_MINI) {
                original!()(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, false)
            } 
            else {
                original!()(module_accessor, status_kind, arg3)
            }
        } 
        else if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&next_status) {
            if WorkModule::is_flag(module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_WAVEDASH) {
                StatusModule::set_situation_kind(module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            }
            original!()(module_accessor, status_kind, arg3)
        }
        else {
            original!()(module_accessor, status_kind, arg3)
        }
    } 
    else {
        original!()(module_accessor, status_kind, arg3)
    }
}

#[common_status_script(status = FIGHTER_STATUS_KIND_ESCAPE_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE, symbol = "_ZN7lua2cpp16L2CFighterCommon20status_pre_EscapeAirEv")]
pub unsafe fn status_pre_escape_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Automatically forces you to the ground if you're buffering Wavedashes during the startup of Airdodge
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_WAVEDASH) && stick_y < 0.5 && 
    ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP_MINI) {
        GroundModule::attach_ground(fighter.module_accessor, true);
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 0.into();
    }
    call_original!(fighter)
}
//Escape Air
#[common_status_script(status = FIGHTER_STATUS_KIND_ESCAPE_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN, symbol = "_ZN7lua2cpp16L2CFighterCommon16status_EscapeAirEv")]
unsafe fn status_escapeair(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_escape_air_common();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air_slide"), 0.0, 1.0, false, 0.0, false, false);
    } 
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(status_escapeair_main as *const () as _))
}

unsafe extern "C" fn status_escapeair_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let escape_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_FRAME);
    let escape_throw_item_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("escape_throw_item_frame"));
    let agt_window = {escape_frame <= escape_throw_item_frame};
    let throwable = !fighter.pop_lua_stack(1).get_bool();
    let lasso_type = WorkModule::get_param_int(fighter.module_accessor, hash40("air_lasso_type"), 0);
    if !fighter.sub_escape_air_common_main().get_bool() {
        fighter.sub_escape_check_rumble();
    }
    /* START OF NEW ADDITION */
    //Rivals of Aether Momentum
    if (18.0..=34.0).contains(&frame) {
        KineticModule::unable_energy_all(boma);
        KineticModule::clear_speed_all(boma);
    }
    if frame > 34.0 {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
        fighter.sub_transition_group_check_air_cliff();
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    //Aerial Glide Tossing
    if ![hash40("special_n"), hash40("special_air_n")].contains(&motion_kind) {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW) 
        && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
        && ItemModule::is_have_item(fighter.module_accessor, 0)
        && agt_window {
            item!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW);
            if throwable {
                let stick_x = fighter.global_table[STICK_X].get_f32();
                let stick_y = fighter.global_table[STICK_Y].get_f32();
                sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 5.0*stick_x.abs());
                sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 5.0*stick_y.abs());
                fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
                return 1.into();
            }
        }
    }
    //Airdodge Canceled Zair
    if agt_window {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO);
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO)
        && lasso_type != *FIGHTER_AIR_LASSO_TYPE_NONE
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
        && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
        && !LinkModule::is_link(fighter.module_accessor, *FIGHTER_LINK_NO_CONSTRAINT) {
            fighter.change_status(FIGHTER_STATUS_KIND_AIR_LASSO.into(), false.into());
            return 1.into();
        }
    }
    else {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO);
    }
    /* END OF NEW ADDITION */
    0.into()
}

pub fn install() {
    skyline::install_hooks!(
        change_status_hook,
        change_status_request_hook
    );
    smashline::install_status_scripts!(
        status_pre_escape_air,
        status_escapeair
    );
}
//Credit to PhazoGanon for sharing this wavedash