use super::*;
use globals::*;
use interpolation::Lerp;

pub static VECTOR_ZERO : Vector3f = Vector3f { x: 0.0, y: 0.0, z: 0.0 };

macro_rules! interrupt {
    () => { return L2CValue::I32(1); };
    ($fighter:ident, $status:expr, $repeat:expr) => {{ $fighter.change_status($status.into(), $repeat.into()); interrupt!(); }}
}
//mod airdodge;
mod dash;
mod guard;
//mod guarddamage;
//mod guardon;
mod jumpsquat;
mod run;
mod turn;
mod walk;

// I honestly don't know why this function was needed in vanilla in the first place
// Forces situation kind changes during ledge actions, even though situation kind automatically changes based on character position
// Also forces ECB shape changes, while stubbing this doesn't affect ECB shape whatsoever
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_cliff_uniq_process_exec_fix_pos)]
pub unsafe fn sub_cliff_uniq_process_exec_fix_pos(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_CLIFF_WAIT
    && fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_CLIFF_CATCH
    && fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_CLIFF_ATTACK
    && fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_CLIFF_CLIMB
    && fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_CLIFF_ESCAPE
    && fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_CLIFF_JUMP1
    {
        return;
    }
    if fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_CLIFF_WAIT {
        if !GroundModule::is_status_cliff(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
                if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_CLIFF {
                    return;
                }
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), false);
                let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
                fighter.global_table[PREV_SITUATION_KIND].assign(&L2CValue::I32(situation_kind));
                fighter.global_table[SITUATION_KIND].assign(&L2CValue::I32(*SITUATION_KIND_AIR));
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                return;
            }
            let correct = GroundModule::get_correct(fighter.module_accessor);
            if correct != *GROUND_CORRECT_KIND_CLIFF {
                return;
            }
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
            app::sv_kinetic_energy::set_ground_trans(fighter.lua_state_agent);
            GroundModule::set_shape_flag(fighter.module_accessor, *GROUND_CORRECT_SHAPE_RHOMBUS_MODIFY_FLAG_FRONT_FIX as u16, false);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CLIFF_FLAG_TO_GROUND);
        }
        else {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
            let is_cliff_ground_trans = app::sv_kinetic_energy::is_cliff_ground_trans(fighter.lua_state_agent);
            if !is_cliff_ground_trans {
                return;
            }
            let mut tra_out = Vector3f::zero();
            MotionModule::trans_tra(fighter.module_accessor, &mut tra_out as *mut Vector3f, true, true);
            if tra_out.z < 0.4  // 0.3 in vanilla
            || tra_out.y < -0.02  // -0.03 in vanilla
            {
                return;
            }
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
            app::sv_kinetic_energy::set_ground_trans(fighter.lua_state_agent);
            GroundModule::set_shape_flag(fighter.module_accessor, *GROUND_CORRECT_SHAPE_RHOMBUS_MODIFY_FLAG_FRONT_FIX as u16, false);
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), false);
            let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
            fighter.global_table[PREV_SITUATION_KIND].assign(&L2CValue::I32(situation_kind));
            fighter.global_table[SITUATION_KIND].assign(&L2CValue::I32(*SITUATION_KIND_GROUND));
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            GroundModule::set_status_ground(fighter.module_accessor);
            GroundModule::leave_cliff(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CLIFF_FLAG_TO_GROUND);
        }
    }
    else {
        let is_touch_down = GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        if !is_touch_down {
            return;
        }
        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), false);
        let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
        fighter.global_table[PREV_SITUATION_KIND].assign(&L2CValue::I32(situation_kind));
        fighter.global_table[SITUATION_KIND].assign(&L2CValue::I32(*SITUATION_KIND_GROUND));
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    return;
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_cliff_uniq_process_exec_fix_pos
        );
    }
}

pub fn install() {
    //airdodge::install();
    dash::install();
    guard::install();
    //guarddamage::install();
    //guardon::install();
    jumpsquat::install();
    run::install();
    turn::install();
    walk::install();
    //cliff::install();

    skyline::nro::add_hook(nro_hook);
}