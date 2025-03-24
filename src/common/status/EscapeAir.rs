
use smash::phx::Vector3f;
use crate::imports::BuildImports::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_EscapeAir)]
pub unsafe fn status_EscapeAir_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), 
        *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, 
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);

    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, 
        *FIGHTER_TREADED_KIND_DISABLE, false, false, false, 0, 0, 0, 0);

    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_WAVEDASH) 
        && stick_y < -0.3 && stick_x.abs() > 0.3
        && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        let x_vel = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let velocity = Vector3f { x: x_vel, y: 0.0, z: 0.0 };

        // Get current ECB bottom position
        let mut ecb_bottom = *PostureModule::pos(fighter.module_accessor);

        // Push ECB slightly downward to ensure proper landing detection
        ecb_bottom.y -= 0.5;

        // Apply the modified ECB position
        PostureModule::set_pos(fighter.module_accessor, &ecb_bottom);

        // Attach to ground and apply horizontal movement
        GroundModule::attach_ground(fighter.module_accessor, true);
        KineticModule::add_speed(fighter.module_accessor, &velocity);

        // Transition to landing state
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 0.into();
    }
    0.into()
}


fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_EscapeAir_Pre,
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}