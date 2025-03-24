use crate::imports::BuildImports::*;


#[skyline::hook(replace = L2CFighterCommon_status_JumpSquat_Main)]
unsafe extern "C" fn fighter_frame(fighter: &mut L2CFighterCommon) {
    let status = StatusModule::status_kind(fighter.module_accessor);
        if [
        *FIGHTER_STATUS_KIND_JUMP_SQUAT  
        ].contains(&status) {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH){
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH.into(), false.into());
            }
   }             
}


pub fn install() {
    Agent::new("fighter")
        .on_line(Main, fighter_frame) // Global opff
        .install();
}
