
use super::*;

#[acmd_script( agent = "simon", script = "game_attackairn" , category = ACMD_GAME , low_priority)]
unsafe fn simon_attack_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 2.5, 4.5);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 8.0);
    for _ in 0..6 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 1.9, 367, 10, 0, 55, 10.0, 0.0, 10.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 1.9, 93, 10, 0, 45, 10.0, 0.0, 10.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 1.0);
    }
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 50, 105, 0, 60, 11.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "simon", script = "game_attackairfhi" , category = ACMD_GAME , low_priority)]
unsafe fn simon_attack_air_f_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.8);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::on_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 40, 67, 0, 70, 4.5, 0.0, 23.0, 38.0, Some(0.0), Some(22.37097), Some(36.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 361, 67, 0, 66, 4.5, 0.0, 10.0, 7.0, Some(0.0), Some(23.0), Some(38.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 361, 55, 0, 74, 5.5, 0.0, 10.0, 2.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::off_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "simon_whip", script = "game_attackairfhi" , category = ACMD_GAME , low_priority)]
unsafe fn simon_whip_attack_air_f_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.8);
    if is_excute(fighter) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_COLLIDE);
    }
    
}

#[acmd_script( agent = "simon", script = "game_attackairf" , category = ACMD_GAME , low_priority)]
unsafe fn simon_attack_air_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.8);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::on_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 40, 67, 0, 70, 4.5, 0.0, 8.0, 40.0, Some(0.0), Some(8.0), Some(38.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 361, 67, 0, 66, 4.5, 0.0, 8.0, 7.0, Some(0.0), Some(8.0), Some(40.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 361, 55, 0, 74, 5.5, 0.0, 8.0, 2.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::off_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "simon_whip", script = "game_attackairf" , category = ACMD_GAME , low_priority)]
unsafe fn simon_whip_attack_air_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.8);
    if is_excute(fighter) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_COLLIDE);
    }
    
}

#[acmd_script( agent = "simon", script = "game_attackairflw" , category = ACMD_GAME , low_priority)]
unsafe fn simon_attack_air_f_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.8);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::on_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 40, 67, 0, 70, 4.5, 0.0, -7.0, 38.0, Some(0.0), Some(-6.37097), Some(36.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 40, 67, 0, 66, 4.5, 0.0, 6.0, 7.0, Some(0.0), Some(-7.0), Some(38.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 40, 55, 0, 74, 5.5, 0.0, 6.0, 2.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::off_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "simon_whip", script = "game_attackairflw" , category = ACMD_GAME , low_priority)]
unsafe fn simon_whip_attack_air_f_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.8);
    if is_excute(fighter) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_COLLIDE);
    }
    
}

#[acmd_script( agent = "simon", script = "game_attackairbhi" , category = ACMD_GAME , low_priority)]
unsafe fn simon_attack_air_b_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.5);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 4.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::on_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::off_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 45, 76, 0, 70, 4.5, 0.0, 23.0, -38.0, Some(0.0), Some(22.37097), Some(-36.5), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.5, 45, 78, 0, 65, 4.5, 0.0, 10.0, -7.0, Some(0.0), Some(23.0), Some(-38.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 45, 70, 0, 70, 5.5, 0.0, 10.0, -2.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "simon_whip", script = "game_attackairbhi" , category = ACMD_GAME , low_priority)]
unsafe fn simon_whip_attack_air_b_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.5);
    if is_excute(fighter) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }
    frame(lua_state, 4.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_COLLIDE);
    }
    
}

#[acmd_script( agent = "simon", script = "game_attackairb" , category = ACMD_GAME , low_priority)]
unsafe fn simon_attack_air_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.5);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 4.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::on_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::off_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 361, 76, 0, 70, 4.5, 0.0, 8.0, -40.0, Some(0.0), Some(8.0), Some(-38.5), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.5, 361, 78, 0, 65, 4.5, 0.0, 8.0, -7.0, Some(0.0), Some(8.0), Some(-40.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 361, 70, 0, 70, 5.5, 0.0, 8.0, -2.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "simon_whip", script = "game_attackairb" , category = ACMD_GAME , low_priority)]
unsafe fn simon_whip_attack_air_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.5);
    if is_excute(fighter) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }
    frame(lua_state, 4.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_COLLIDE);
    }
    
}

#[acmd_script( agent = "simon", script = "game_attackairblw" , category = ACMD_GAME , low_priority)]
unsafe fn simon_attack_air_b_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.5);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 4.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::on_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::off_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 40, 76, 0, 70, 4.5, 0.0, -7.0, -38.0, Some(0.0), Some(-6.37097), Some(-36.5), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.5, 40, 78, 0, 65, 4.5, 0.0, 6.0, -7.0, Some(0.0), Some(-7.0), Some(-38.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 40, 70, 0, 70, 5.5, 0.0, 6.0, -2.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "simon_whip", script = "game_attackairblw" , category = ACMD_GAME , low_priority)]
unsafe fn simon_whip_attack_air_b_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.5);
    if is_excute(fighter) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }
    frame(lua_state, 4.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_COLLIDE);
    }
    
}

#[acmd_script( agent = "simon", script = "game_attackairhi" , category = ACMD_GAME , low_priority)]
unsafe fn simon_attack_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.5);
    frame(lua_state, 8.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        WorkModule::on_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::on_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::off_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.5, 90, 83, 0, 60, 4.5, 0.0, 53.0, 2.5, Some(0.0), Some(51.0), Some(2.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 90, 75, 0, 60, 4.5, 0.0, 17.0, 2.5, Some(0.0), Some(53.0), Some(2.5), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 90, 75, 0, 60, 6.0, 0.0, 19.0, 2.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "simon_whip", script = "game_attackairhi" , category = ACMD_GAME , low_priority)]
unsafe fn simon_whip_attack_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.5);
    if is_excute(fighter) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_COLLIDE);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        simon_attack_air_n_game,
        simon_attack_air_f_hi_game,
        simon_whip_attack_air_f_hi_game,
        simon_attack_air_f_game,
        simon_whip_attack_air_f_game,
        simon_attack_air_f_lw_game,
        simon_whip_attack_air_f_lw_game,
        simon_attack_air_b_hi_game,
        simon_whip_attack_air_b_hi_game,
        simon_attack_air_b_game,
        simon_whip_attack_air_b_game,
        simon_attack_air_b_lw_game,
        simon_whip_attack_air_b_lw_game,
        simon_attack_air_hi_game,
        simon_whip_attack_air_hi_game,
    );
}

