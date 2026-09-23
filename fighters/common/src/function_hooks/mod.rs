use super::*;
use crate::globals::*;
use crate::opff::physics::get_player_number;

pub mod controls;
//pub mod collision;
pub mod energy;
pub mod jumps;
pub mod kinetic;
pub mod momentum_transfer;
pub mod transition;
mod lua_bind_hook;

#[repr(C)]
pub struct TempModule {
    vtable: *const u64,
    // ...
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ModuleAccessor {
    vtable: *const *const u64,
    battle_object_id: u32,
    padding: u32,
    event_manager: [u8; 0x28],
    posture_module: &'static TempModule,
    status_module: &'static TempModule,
    control_module: &'static TempModule,
    work_module: &'static TempModule,
    ground_module: &'static TempModule,
    camera_module: &'static TempModule,
    kinetic_module: &'static TempModule,
    color_blend_module: &'static TempModule,
    model_module: &'static TempModule,
    physics_module: &'static TempModule,
    motion_module: &'static TempModule,
    stop_module: &'static TempModule,
    article_module: &'static TempModule,
    attack_module: &'static TempModule,
    damage_module: &'static TempModule,
    hit_module: &'static TempModule,
    combo_module: &'static TempModule,
    area_module: &'static TempModule,
    item_module: &'static TempModule,
    link_module: &'static TempModule,
    team_module: &'static TempModule,
    search_module: &'static TempModule,
    unk1_module: &'static TempModule,
    turn_module: &'static TempModule,
    reflect_module: &'static TempModule,
    shield_module: &'static TempModule,
    reflector_module: &'static TempModule,
    absorber_module: &'static TempModule,
    jostle_module: &'static TempModule,
    catch_module: &'static TempModule,
    cancel_module: &'static TempModule,
    unk2_module: &'static TempModule,
    capture_module: &'static TempModule,
    effect_module: &'static TempModule,
    sound_module: &'static TempModule,
    visibility_module: &'static TempModule,
    grab_module: &'static TempModule,
    slope_module: &'static TempModule,
    shake_module: &'static TempModule,
    slow_module: &'static TempModule,
    unk3_module: &'static TempModule,
    shadow_module: &'static TempModule,
    motion_animcmd_module: &'static TempModule,
    lua_module: &'static TempModule,
    ink_paint_module: &'static TempModule
}


pub fn install() {
    controls::install();
    //collision::install();
    energy::install();
    //jumps::install();
    kinetic::install();
    momentum_transfer::install();
    transition::install();
    lua_bind_hook::install();
    let _ = skyline::patching::Patch::in_text(0x633de0).nop(); //Removes the vanilla kill zoom in favor of the updated function. This one handles normal hits
    let _ = skyline::patching::Patch::in_text(0x6373a4).data(0xD503201Fu32); //Removes the vanilla kill zoom in favor of the updated function. This one handles throws
    unsafe {
        // Allows airtime counter (FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR)
        // to increment when on ledge (SITUATION_KIND_CLIFF)
        // Matches Melee behavior
        skyline::patching::Patch::in_text(0x614fb4).data(0x54000060);
        skyline::patching::Patch::in_text(0x3e6d08).data(0x14000012u32); //Removes phantoms

    }

}