use crate::imports::BuildImports::*;

#[skyline::hook(replace = L2CFighterCommon_FighterStatusGuard__calc_shield_scale)]
pub unsafe fn calc_shield_scale(
    fighter: &mut L2CFighterCommon,
    shield_level: L2CValue,
) -> L2CValue {
    let shield_level = shield_level.get_f32();
    let shield_max = WorkModule::get_float(
        fighter.module_accessor,
        *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MAX,
    );

    let shield_size = WorkModule::get_param_float(
        fighter.module_accessor,
        hash40("common"),
        hash40("shield_size"),
    );
    let shield_scale_min = WorkModule::get_param_float(
        fighter.module_accessor,
        hash40("common"),
        hash40("shield_scale_min"),
    );
    let shield_radius =
        WorkModule::get_param_float(fighter.module_accessor, hash40("shield_radius"), 0);

    // Read analog shield input (0.0 to 1.0 range)
    let shield_pressure = ControlModule::get_trigger(fighter.module_accessor);

    // Apply shield scaling
    let scale = if shield_pressure > 0.0 && shield_pressure < 0.5 { // Light shield threshold
        let light_shield_multiplier = 1.2; // Increase shield size for light shield
        (shield_level / shield_max) * (1.0 - shield_scale_min) + (shield_scale_min * light_shield_multiplier)
    } else {
        // Normal shield
        (shield_level / shield_max) * (1.0 - shield_scale_min) + shield_scale_min
    };

    // Return final shield size
    L2CValue::F32(scale * shield_radius)
}


pub fn install() {
    skyline::install_hooks!(
        calc_shield_scale,
    );
}