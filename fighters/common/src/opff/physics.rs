use utils::{
    *,
    ext::*,
    consts::*
};
use smash::app::BattleObjectModuleAccessor;
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::{Vector2f, Vector3f, Hash40};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::hash40;
use smash::app::utility::*;
use vars::*;
use super::*;
use globals::*;

static mut ECB_Y_OFFSETS: [f32; 9] = [0.0; 9];
pub unsafe fn get_player_number(boma: &mut smash::app::BattleObjectModuleAccessor) -> usize {
    return WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
}

/// Shifts fighter's ECB (Environment Collision Box) rhombus up to around their knees when they are in the air for over
/// a certain amount of frames *and* they are in the proper status
unsafe fn ecb_shifts(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32) {
    let status = StatusModule::status_kind(boma);
    let prev_status = StatusModule::prev_status_kind(boma, 0);
    let situation = StatusModule::situation_kind(boma);
    let kind = app::utility::get_kind(&mut *boma);

    // Abort if game isn't active
    if !sv_information::is_ready_go() {
        GroundModule::set_rhombus_offset(boma, &Vector2f{x:0.0, y:0.0});
        return;
    }

    // Vanilla state filter
    let vanilla_ecb = [
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
        *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
        *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
        *FIGHTER_STATUS_KIND_CAPTURE_CUT,
        *FIGHTER_STATUS_KIND_THROWN,
        *FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL,
        *FIGHTER_STATUS_KIND_TREAD_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_BURY,
        *FIGHTER_STATUS_KIND_BURY_WAIT,
    ].contains(&status);

    let previous_states = [
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
        *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
        *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
        *FIGHTER_STATUS_KIND_CAPTURE_CUT,
        *FIGHTER_STATUS_KIND_THROWN,
    ].contains(&prev_status);

    let fighter_low_offset = [
        *FIGHTER_KIND_KIRBY,
        *FIGHTER_KIND_PIKACHU,
        *FIGHTER_KIND_NESS,
        *FIGHTER_KIND_PURIN,
        *FIGHTER_KIND_GAMEWATCH,
        *FIGHTER_KIND_POPO,
        *FIGHTER_KIND_NANA,
        *FIGHTER_KIND_PICHU,
        *FIGHTER_KIND_METAKNIGHT,
        *FIGHTER_KIND_WARIO,
        *FIGHTER_KIND_PZENIGAME,
        *FIGHTER_KIND_PFUSHIGISOU,
        *FIGHTER_KIND_LUCAS,
        *FIGHTER_KIND_PIKMIN,
        *FIGHTER_KIND_TOONLINK,
        *FIGHTER_KIND_DUCKHUNT,
        *FIGHTER_KIND_MURABITO,
        *FIGHTER_KIND_INKLING,
        *FIGHTER_KIND_SHIZUE
    ];

    let fighter_mid_offset = [        
        *FIGHTER_KIND_MARIO,
        *FIGHTER_KIND_YOSHI,
        *FIGHTER_KIND_LUIGI,
        *FIGHTER_KIND_MARIOD,
        *FIGHTER_KIND_YOUNGLINK,
        *FIGHTER_KIND_PLIZARDON,
        *FIGHTER_KIND_DIDDY,
        *FIGHTER_KIND_DEDEDE,
        *FIGHTER_KIND_ROCKMAN,
        *FIGHTER_KIND_GEKKOUGA,
        *FIGHTER_KIND_PACMAN,
        *FIGHTER_KIND_KOOPAJR,
        *FIGHTER_KIND_PACKUN,
        *FIGHTER_KIND_MIIFIGHTER,
        *FIGHTER_KIND_MIISWORDSMAN,
        *FIGHTER_KIND_MIIGUNNER,
        *FIGHTER_KIND_PACKUN,
        *FIGHTER_KIND_BUDDY,
        *FIGHTER_KIND_PICKEL
    ];

    let fighter_high_offset = [
        *FIGHTER_KIND_FOX,
        *FIGHTER_KIND_FALCO,
        *FIGHTER_KIND_DAISY,
        *FIGHTER_KIND_MEWTWO,
        *FIGHTER_KIND_PIT,
        *FIGHTER_KIND_PITB,
        *FIGHTER_KIND_SONIC,
        *FIGHTER_KIND_LUCARIO,
        *FIGHTER_KIND_ROBOT,
        *FIGHTER_KIND_WOLF,
        *FIGHTER_KIND_LITTLEMAC,
        *FIGHTER_KIND_KROOL,
        *FIGHTER_KIND_GAOGAEN
    ];

    let fighter_max_offset = [
        *FIGHTER_KIND_DONKEY,
        *FIGHTER_KIND_LINK,
        *FIGHTER_KIND_CAPTAIN,
        *FIGHTER_KIND_PEACH,
        *FIGHTER_KIND_KOOPA,
        *FIGHTER_KIND_SHEIK,
        *FIGHTER_KIND_ZELDA,
        *FIGHTER_KIND_MARTH,
        *FIGHTER_KIND_LUCINA,
        *FIGHTER_KIND_GANON,
        *FIGHTER_KIND_ROY,
        *FIGHTER_KIND_CHROM,
        *FIGHTER_KIND_SZEROSUIT,
        *FIGHTER_KIND_SNAKE,
        *FIGHTER_KIND_IKE,
        *FIGHTER_KIND_WIIFIT,
        *FIGHTER_KIND_ROSETTA,
        *FIGHTER_KIND_PALUTENA,
        *FIGHTER_KIND_REFLET,
        *FIGHTER_KIND_SHULK,
        *FIGHTER_KIND_RYU,
        *FIGHTER_KIND_KEN,
        *FIGHTER_KIND_CLOUD,
        *FIGHTER_KIND_KAMUI,
        *FIGHTER_KIND_BAYONETTA,
        *FIGHTER_KIND_RIDLEY,
        *FIGHTER_KIND_SIMON,
        *FIGHTER_KIND_RICHTER,
        *FIGHTER_KIND_JACK,
        *FIGHTER_KIND_BRAVE,
        *FIGHTER_KIND_DOLLY,
        *FIGHTER_KIND_MASTER,
        *FIGHTER_KIND_TANTAN,
        *FIGHTER_KIND_EFLAME,
        *FIGHTER_KIND_ELIGHT,
        *FIGHTER_KIND_DEMON,
        *FIGHTER_KIND_TRAIL
    ];

    let fighter_damn_offset = [
        *FIGHTER_KIND_SAMUS,
        *FIGHTER_KIND_SAMUSD,
        *FIGHTER_KIND_EDGE
    ];

    let offset_y = if fighter_low_offset.contains(&kind) {
        2.0
    } else if fighter_mid_offset.contains(&kind) {
        3.5
    } else if fighter_high_offset.contains(&kind) {
        4.0
    } else if fighter_max_offset.contains(&kind) {
        5.0
    } else if fighter_damn_offset.contains(&kind) {
        6.0
    } else {
        3.0 // fallback
    };
    // Skip states that should use vanilla ECB
    if vanilla_ecb || previous_states {
        return;
    }
    // Reset offset during ENTRY or early PASS (e.g. platform drop or training reset)
    let prev_status = StatusModule::prev_status_kind(boma, 0);
    let motion_frame = MotionModule::frame(boma);

    if status == *FIGHTER_STATUS_KIND_ENTRY
        || (prev_status == *FIGHTER_STATUS_KIND_PASS && motion_frame < 3.0)
    {
        GroundModule::set_offset_y(boma, 0.0);
        GroundModule::set_rhombus_offset(boma, &Vector2f { x: 0.0, y: 0.0 });
        return;
    }

    // Check if fighter just entered the air
    let air_trans = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) < 10;

    // Only apply ECB logic for valid airborne movement states
    let valid_air_states = [
        *FIGHTER_STATUS_KIND_JUMP,
        *FIGHTER_STATUS_KIND_JUMP_AERIAL,
        *FIGHTER_STATUS_KIND_FALL,
        *FIGHTER_STATUS_KIND_FALL_AERIAL,
        *FIGHTER_STATUS_KIND_FALL_SPECIAL,
    ];

    if situation == *SITUATION_KIND_AIR
        && valid_air_states.contains(&status)
        && !(vanilla_ecb || previous_states)
        && motion_frame > 2.0 // avoid applying offset too early during transition
    {
        GroundModule::set_offset_y(boma, offset_y);

        if air_trans {
            GroundModule::set_rhombus_offset(boma, &Vector2f { x: 0.0, y: 0.0 });
        }
    }
    // Reset offset on ground... I think
    else if situation == *SITUATION_KIND_GROUND {
        GroundModule::set_offset_y(boma, 0.0);

        if vanilla_ecb {
            GroundModule::set_rhombus_offset(boma, &Vector2f { x: 0.0, y: 0.0 });
        }
    }
}

//=================================================================
//== EXTRA TRACTION
//=================================================================
/// Sets the extra traction flag depending on current speed and current status in order to prevent
/// the game feeling too slippery
unsafe fn extra_traction(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
    let max_walk = WorkModule::get_param_float(boma, hash40("walk_speed_max"), 0);
    let ground_brake = WorkModule::get_param_float(boma, hash40("ground_brake"), 0);
    let added_traction: smash::phx::Vector3f = smash::phx::Vector3f {x: -1.0 * PostureModule::lr(boma) * ground_brake * speed_x.signum(), y: 0.0, z: 0.0};
    let double_traction_statuses = [
        *FIGHTER_STATUS_KIND_WAIT,
        *FIGHTER_STATUS_KIND_JUMP_SQUAT,
        *FIGHTER_STATUS_KIND_SQUAT,
        *FIGHTER_STATUS_KIND_SQUAT_RV,
        *FIGHTER_STATUS_KIND_SQUAT_WAIT,
        *FIGHTER_STATUS_KIND_LANDING_LIGHT,
        *FIGHTER_STATUS_KIND_LANDING,
        *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL,
        *FIGHTER_STATUS_KIND_ATTACK,
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_S4_START,
        *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD,
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
        *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
        *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_STATUS_KIND_CATCH,
        *FIGHTER_STATUS_KIND_CATCH_WAIT,
        *FIGHTER_STATUS_KIND_CATCH_ATTACK,
        *FIGHTER_STATUS_KIND_CATCH_PULL,
        *FIGHTER_STATUS_KIND_ITEM_THROW
    ];

    if boma.is_status_one_of(&double_traction_statuses) {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
        let motion_accel = smash::app::sv_kinetic_energy::get_accel(fighter.lua_state_agent);

        // reset flag at beginning of any status
        if fighter.global_table[CURRENT_FRAME].get_i32() == 0 {
            VarModule::off_flag(boma.object(), vars::common::instance::IS_MOTION_BASED_ATTACK);
        }
        // if we detect that the current animation is trans-motion-based (shifts your character's position), disable traction for the entire attack 
        if motion_accel.x != 0.0 && !VarModule::is_flag(boma.object(), vars::common::instance::IS_MOTION_BASED_ATTACK) {
            VarModule::on_flag(boma.object(), vars::common::instance::IS_MOTION_BASED_ATTACK);
        }
        if speed_x.abs() > max_walk
        && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
        && !VarModule::is_flag(boma.object(), vars::common::instance::IS_MOTION_BASED_ATTACK) {
            if boma.is_prev_status_one_of(&double_traction_statuses) {
                KineticModule::add_speed(boma, &added_traction);
            }
            else if fighter.global_table[CURRENT_FRAME].get_i32() > 0 {
                KineticModule::add_speed(boma, &added_traction);
            }
        }
    }
}

pub unsafe fn run(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    extra_traction(fighter, boma); 
    ecb_shifts(boma, status_kind, situation_kind, fighter_kind);
}