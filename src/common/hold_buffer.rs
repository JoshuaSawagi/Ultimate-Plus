use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::*;
use smash::lib::{L2CValue, L2CAgent};
use std::{fs, path::Path};
use smash::phx::Vector2f;
use smash::app::sv_system;
use smash::app;

static HOLD_BUFFER_LIMIT : i32 = 20;

unsafe extern "C" fn hold_buffer_killer(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        let buttons_list = [
            *CONTROL_PAD_BUTTON_ATTACK,
            *CONTROL_PAD_BUTTON_JUMP,
            *CONTROL_PAD_BUTTON_CATCH,
            *CONTROL_PAD_BUTTON_GUARD,
            *CONTROL_PAD_BUTTON_SMASH,
            *CONTROL_PAD_BUTTON_SPECIAL,
            *CONTROL_PAD_BUTTON_CSTICK_ON,
            *CONTROL_PAD_BUTTON_JUMP_MINI,
            *CONTROL_PAD_BUTTON_ATTACK_RAW,
            *CONTROL_PAD_BUTTON_SPECIAL_RAW,
            *CONTROL_PAD_BUTTON_SPECIAL_RAW2
        ];
        let mut hold_buffer_lim = HOLD_BUFFER_LIMIT;

        //Multiplies hold buffer duration by 2x during damage states to allow for pressing buttons out of hitstun as per usual
        if (*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind){
            hold_buffer_lim *= 2;
        }
        //If time since you've pressed the button exceeds hold buffer limit, kills the input
        for i in buttons_list {
                if ControlModule::get_trigger_count(fighter.module_accessor, i as u8) > hold_buffer_lim && ControlModule::check_button_on(boma, i) 
                && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) 
                && !(ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) && ItemModule::is_have_item(boma, 0))
                && !(ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) && ItemModule::is_have_item(boma, 0))
                && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) //So taunts dont tpose
                && ![*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_GUARD_OFF, *FIGHTER_STATUS_KIND_JUMP_SQUAT].contains(&status_kind){ //Ignores shield and js
                    ControlModule::reset_trigger(fighter.module_accessor);
                    ControlModule::clear_command(fighter.module_accessor, true);
                    //ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_F);
                }
        }
    };
}

pub fn install() {
    Agent::new("fighter")
	.on_line(Main, hold_buffer_killer)
	.install();
}