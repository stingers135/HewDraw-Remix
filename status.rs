use super::*;
use globals::*;
utils::import!(common::djc::attack_air_main_status);

#[status_script(agent = "jack", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn init_attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
		let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
		
		if motion_kind == smash::hash40("jump_aerial_f") || motion_kind == smash::hash40("jump_aerial_b") {
			if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
				KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND);
			} else {
				KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
			}
		}
		let _ = fighter.sub_attack_air_uniq_process_init();
		0.into()
	} else {
		0.into()
	}
}


#[status_script(agent = "jack", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
pub unsafe fn exec_attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
	if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
		KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
		0.into()
	} else {
		0.into()
	}
}


pub fn install() {
    install_status_scripts!(
        init_attack_air,
        exec_attack_air,
    );
}