use super::*;
use globals::*;

//=================================================================
//== StatusModule::init_settings
//=================================================================
#[skyline::hook(replace=StatusModule::init_settings)]
unsafe fn init_settings_hook(boma: &mut BattleObjectModuleAccessor, mut situation: smash::app::SituationKind, kinetic_type: i32, ground_correct_kind: u32, ground_cliff_check_kind: smash::app::GroundCliffCheckKind, jostle: bool, keep_flag: i32, keep_int: i32, keep_float: i32, arg10: i32) -> u64 {
    let id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let fighter_kind = boma.kind();
    let status_kind = StatusModule::status_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let mut cliff_check_kind = ground_cliff_check_kind;
    let mut kinetic_type = kinetic_type.clone();
    let mut ground_correct_kind = ground_correct_kind.clone();
    ground_correct_kind = super::ground::init_settings_edges(boma, situation, kinetic_type, ground_correct_kind, ground_cliff_check_kind, jostle, keep_flag, keep_int, keep_float, arg10);

    if boma.is_fighter() {
        if boma.is_prev_situation(*SITUATION_KIND_AIR) && (
            situation.0 == *SITUATION_KIND_GROUND || (
                boma.is_situation(*SITUATION_KIND_GROUND) 
                && situation.0 == *SITUATION_KIND_NONE
            ) 
        ) {
            if kinetic_type == *FIGHTER_KINETIC_TYPE_MOTION {
                kinetic_type = *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL;
            }
        }

        if boma.is_prev_status_one_of(&[
            *FIGHTER_STATUS_KIND_DEMO,
            *FIGHTER_STATUS_KIND_ENTRY,
            *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
            *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
            *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
            *FIGHTER_STATUS_KIND_THROWN
        ]) && situation.0 == *SITUATION_KIND_AIR {
            WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR);
        }

        JostleModule::set_team(boma, 0);

        if ground_correct_kind == *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK as u32 {
            ground_correct_kind = *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32;
        }
    }

    // Occupy ledge on ledgegrab
    if boma.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_CLIFF_CATCH,
            *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE,
            *FIGHTER_STATUS_KIND_CLIFF_WAIT]
        ) {
        let cliff_id = GroundModule::get_cliff_id_uint32(boma);
            VarModule::set_int(boma.object(), vars::common::instance::OCCUPIED_LEDGE_ID, cliff_id as i32);
            VarModule::set_int(boma.object(), vars::common::instance::OCCUPIED_LEDGE_ID_FOR_TETHERS, cliff_id as i32);
    }
    
    if VarModule::has_var_module(boma.object()) {
        let mut mask = 0;
        if keep_flag != *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG {
            mask += VarModule::RESET_STATUS_FLAG;
        }
        if keep_int != *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT {
            mask += VarModule::RESET_STATUS_INT;
            mask += VarModule::RESET_STATUS_INT64;
        }
        if keep_float != *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT {
            mask += VarModule::RESET_STATUS_FLOAT;
        }
        VarModule::reset(boma.object(), mask);
    }
    original!()(boma, situation, kinetic_type, ground_correct_kind, cliff_check_kind, jostle, keep_flag, keep_int, keep_float, arg10)
}

#[skyline::hook(replace=StatusModule::change_status_request)]
unsafe fn change_status_request_hook(boma: &mut BattleObjectModuleAccessor, status_kind: i32, arg3: bool) -> u64 {
    let mut next_status = status_kind;

    if boma.is_fighter() {
        // Tether trump logic
        if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_AIR_LASSO, *FIGHTER_STATUS_KIND_AIR_LASSO_REACH, *FIGHTER_STATUS_KIND_AIR_LASSO_HANG, *FIGHTER_STATUS_KIND_AIR_LASSO_REWIND])
        && [*FIGHTER_STATUS_KIND_CLIFF_CATCH, *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE, *FIGHTER_STATUS_KIND_CLIFF_WAIT].contains(&next_status) {
            let player_number = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
            let cliff_id = GroundModule::get_cliff_id_uint32(boma);

            for object_id in util::get_all_active_battle_object_ids() {
                let object = ::utils::util::get_battle_object_from_id(object_id);
                if !object.is_null() {
                    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) == WorkModule::get_int(&mut *(*object).module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) {
                        continue;
                    }

                    if VarModule::get_int(object, vars::common::instance::OCCUPIED_LEDGE_ID_FOR_TETHERS) == cliff_id as i32 {
                        next_status = *FIGHTER_STATUS_KIND_CLIFF_ROBBED;
                    }
                }
            }
        }
    }
    original!()(boma, next_status, arg3)
}


pub fn install() {
    skyline::install_hooks!(
        init_settings_hook,
        change_status_request_hook,
        //change_status_request_from_script_hook
    );
}