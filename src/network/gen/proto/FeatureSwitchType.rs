// This file is generated by rust-protobuf 3.7.1. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `FeatureSwitchType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:FeatureSwitchType)
pub enum FeatureSwitchType {
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_NONE)
    FEATURE_SWITCH_NONE = 0,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_SHOP)
    FEATURE_SWITCH_SHOP = 1,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_LINEUP_NAME)
    FEATURE_SWITCH_LINEUP_NAME = 2,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_RECHARGE_SHOP)
    FEATURE_SWITCH_RECHARGE_SHOP = 3,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_NICKNAME)
    FEATURE_SWITCH_NICKNAME = 4,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_SIGNATURE)
    FEATURE_SWITCH_SIGNATURE = 5,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_BATTLEPASS)
    FEATURE_SWITCH_BATTLEPASS = 6,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_PUNK_LORD)
    FEATURE_SWITCH_PUNK_LORD = 7,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_MONTHCARD_DAILY)
    FEATURE_SWITCH_MONTHCARD_DAILY = 8,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_PICTURE_SHARE)
    FEATURE_SWITCH_PICTURE_SHARE = 9,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_ROGUE)
    FEATURE_SWITCH_ROGUE = 10,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_CHALLENGE)
    FEATURE_SWITCH_CHALLENGE = 11,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_COCOON)
    FEATURE_SWITCH_COCOON = 12,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_RAID)
    FEATURE_SWITCH_RAID = 13,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_MAZE_PLANE_EVENT)
    FEATURE_SWITCH_MAZE_PLANE_EVENT = 14,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_ACTIVITY_PANEL)
    FEATURE_SWITCH_ACTIVITY_PANEL = 15,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_MAILBOX)
    FEATURE_SWITCH_MAILBOX = 16,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_QUEST)
    FEATURE_SWITCH_QUEST = 17,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_GACHA)
    FEATURE_SWITCH_GACHA = 18,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_CHAT)
    FEATURE_SWITCH_CHAT = 19,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_MODIFY_FRIEND_ALIAS)
    FEATURE_SWITCH_MODIFY_FRIEND_ALIAS = 20,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_USE_ITEM)
    FEATURE_SWITCH_USE_ITEM = 21,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_ACTIVITY_SCHEDULE)
    FEATURE_SWITCH_ACTIVITY_SCHEDULE = 22,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_FARM_ELEMENT)
    FEATURE_SWITCH_FARM_ELEMENT = 23,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_ACHIEVEMENT_LEVEL)
    FEATURE_SWITCH_ACHIEVEMENT_LEVEL = 24,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_DAILY_ACTIVE_LEVEL)
    FEATURE_SWITCH_DAILY_ACTIVE_LEVEL = 25,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_PLAYER_RETURN)
    FEATURE_SWITCH_PLAYER_RETURN = 26,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_FIRST_SET_NICKNAME)
    FEATURE_SWITCH_FIRST_SET_NICKNAME = 27,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_MAIN_MISSION_REWARD)
    FEATURE_SWITCH_MAIN_MISSION_REWARD = 28,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_SUB_MISSION_REWARD)
    FEATURE_SWITCH_SUB_MISSION_REWARD = 29,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_PAM_MISSION)
    FEATURE_SWITCH_PAM_MISSION = 30,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_DESTROY_ITEM)
    FEATURE_SWITCH_DESTROY_ITEM = 32,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_CONSUME_ITEM_TURN)
    FEATURE_SWITCH_CONSUME_ITEM_TURN = 33,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_ROGUE_MODIFIER)
    FEATURE_SWITCH_ROGUE_MODIFIER = 34,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_CHESS_ROGUE)
    FEATURE_SWITCH_CHESS_ROGUE = 35,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_CHESS_ROGUE_BOARD)
    FEATURE_SWITCH_CHESS_ROGUE_BOARD = 36,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_ROLL_SHOP)
    FEATURE_SWITCH_ROLL_SHOP = 37,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_H5_RETURN)
    FEATURE_SWITCH_H5_RETURN = 38,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_OFFERING)
    FEATURE_SWITCH_OFFERING = 39,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_SERVER_RED_POINT)
    FEATURE_SWITCH_SERVER_RED_POINT = 40,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_MONOPOLY_OPTION_RATIO)
    FEATURE_SWITCH_MONOPOLY_OPTION_RATIO = 41,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_MONOPOLY_GET_RAFFLE_TICKET)
    FEATURE_SWITCH_MONOPOLY_GET_RAFFLE_TICKET = 42,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_MONOPOLY_TAKE_RAFFLE_REWARD)
    FEATURE_SWITCH_MONOPOLY_TAKE_RAFFLE_REWARD = 43,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_CHALLENGE_RECOMMEND_LINEUP)
    FEATURE_SWITCH_CHALLENGE_RECOMMEND_LINEUP = 44,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_PSN_MEMBER_SHIP_CHECK)
    FEATURE_SWITCH_PSN_MEMBER_SHIP_CHECK = 45,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_PLAYER_BOARD_DEVELOPMENT)
    FEATURE_SWITCH_PLAYER_BOARD_DEVELOPMENT = 46,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_PVP)
    FEATURE_SWITCH_PVP = 47,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_ROGUE_MODE)
    FEATURE_SWITCH_ROGUE_MODE = 48,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_ROGUE_TOURN_UGC)
    FEATURE_SWITCH_ROGUE_TOURN_UGC = 49,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_RELIC_FILTER_PLAN_NAME)
    FEATURE_SWITCH_RELIC_FILTER_PLAN_NAME = 50,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_MAZE_ITEM_USE_BUFF_DROP)
    FEATURE_SWITCH_MAZE_ITEM_USE_BUFF_DROP = 51,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_RED_DOT)
    FEATURE_SWITCH_RED_DOT = 52,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_GAME_STATE_SERVICE)
    FEATURE_SWITCH_GAME_STATE_SERVICE = 53,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_BENEFIT_INDEX)
    FEATURE_SWITCH_BENEFIT_INDEX = 54,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_ROGUE_TOURN_BUILD_REF)
    FEATURE_SWITCH_ROGUE_TOURN_BUILD_REF = 55,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_PRE_AVATAR_SET_GROWTH_TARGET)
    FEATURE_SWITCH_PRE_AVATAR_SET_GROWTH_TARGET = 56,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_IMPORT_RELIC_FILTER_PLAN)
    FEATURE_SWITCH_IMPORT_RELIC_FILTER_PLAN = 58,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_GACHA_DECIDE_ITEM)
    FEATURE_SWITCH_GACHA_DECIDE_ITEM = 59,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_ITEM_SYNC)
    FEATURE_SWITCH_ITEM_SYNC = 60,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_RECHARGE_BENEFIT)
    FEATURE_SWITCH_RECHARGE_BENEFIT = 61,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_RECHARGE_GIFT)
    FEATURE_SWITCH_RECHARGE_GIFT = 62,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_GACHA_AVATAR_TOAST)
    FEATURE_SWITCH_GACHA_AVATAR_TOAST = 64,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_RELIC_SMART_DISCARD)
    FEATURE_SWITCH_RELIC_SMART_DISCARD = 66,
    // @@protoc_insertion_point(enum_value:FeatureSwitchType.FEATURE_SWITCH_PLANETFES_SOCIAL)
    FEATURE_SWITCH_PLANETFES_SOCIAL = 67,
}

impl ::protobuf::Enum for FeatureSwitchType {
    const NAME: &'static str = "FeatureSwitchType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<FeatureSwitchType> {
        match value {
            0 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_NONE),
            1 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_SHOP),
            2 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_LINEUP_NAME),
            3 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_RECHARGE_SHOP),
            4 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_NICKNAME),
            5 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_SIGNATURE),
            6 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_BATTLEPASS),
            7 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_PUNK_LORD),
            8 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_MONTHCARD_DAILY),
            9 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_PICTURE_SHARE),
            10 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_ROGUE),
            11 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_CHALLENGE),
            12 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_COCOON),
            13 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_RAID),
            14 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_MAZE_PLANE_EVENT),
            15 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_ACTIVITY_PANEL),
            16 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_MAILBOX),
            17 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_QUEST),
            18 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_GACHA),
            19 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_CHAT),
            20 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_MODIFY_FRIEND_ALIAS),
            21 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_USE_ITEM),
            22 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_ACTIVITY_SCHEDULE),
            23 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_FARM_ELEMENT),
            24 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_ACHIEVEMENT_LEVEL),
            25 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_DAILY_ACTIVE_LEVEL),
            26 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_PLAYER_RETURN),
            27 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_FIRST_SET_NICKNAME),
            28 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_MAIN_MISSION_REWARD),
            29 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_SUB_MISSION_REWARD),
            30 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_PAM_MISSION),
            32 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_DESTROY_ITEM),
            33 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_CONSUME_ITEM_TURN),
            34 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_ROGUE_MODIFIER),
            35 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_CHESS_ROGUE),
            36 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_CHESS_ROGUE_BOARD),
            37 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_ROLL_SHOP),
            38 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_H5_RETURN),
            39 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_OFFERING),
            40 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_SERVER_RED_POINT),
            41 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_MONOPOLY_OPTION_RATIO),
            42 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_MONOPOLY_GET_RAFFLE_TICKET),
            43 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_MONOPOLY_TAKE_RAFFLE_REWARD),
            44 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_CHALLENGE_RECOMMEND_LINEUP),
            45 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_PSN_MEMBER_SHIP_CHECK),
            46 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_PLAYER_BOARD_DEVELOPMENT),
            47 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_PVP),
            48 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_ROGUE_MODE),
            49 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_ROGUE_TOURN_UGC),
            50 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_RELIC_FILTER_PLAN_NAME),
            51 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_MAZE_ITEM_USE_BUFF_DROP),
            52 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_RED_DOT),
            53 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_GAME_STATE_SERVICE),
            54 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_BENEFIT_INDEX),
            55 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_ROGUE_TOURN_BUILD_REF),
            56 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_PRE_AVATAR_SET_GROWTH_TARGET),
            58 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_IMPORT_RELIC_FILTER_PLAN),
            59 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_GACHA_DECIDE_ITEM),
            60 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_ITEM_SYNC),
            61 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_RECHARGE_BENEFIT),
            62 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_RECHARGE_GIFT),
            64 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_GACHA_AVATAR_TOAST),
            66 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_RELIC_SMART_DISCARD),
            67 => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_PLANETFES_SOCIAL),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<FeatureSwitchType> {
        match str {
            "FEATURE_SWITCH_NONE" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_NONE),
            "FEATURE_SWITCH_SHOP" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_SHOP),
            "FEATURE_SWITCH_LINEUP_NAME" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_LINEUP_NAME),
            "FEATURE_SWITCH_RECHARGE_SHOP" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_RECHARGE_SHOP),
            "FEATURE_SWITCH_NICKNAME" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_NICKNAME),
            "FEATURE_SWITCH_SIGNATURE" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_SIGNATURE),
            "FEATURE_SWITCH_BATTLEPASS" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_BATTLEPASS),
            "FEATURE_SWITCH_PUNK_LORD" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_PUNK_LORD),
            "FEATURE_SWITCH_MONTHCARD_DAILY" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_MONTHCARD_DAILY),
            "FEATURE_SWITCH_PICTURE_SHARE" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_PICTURE_SHARE),
            "FEATURE_SWITCH_ROGUE" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_ROGUE),
            "FEATURE_SWITCH_CHALLENGE" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_CHALLENGE),
            "FEATURE_SWITCH_COCOON" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_COCOON),
            "FEATURE_SWITCH_RAID" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_RAID),
            "FEATURE_SWITCH_MAZE_PLANE_EVENT" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_MAZE_PLANE_EVENT),
            "FEATURE_SWITCH_ACTIVITY_PANEL" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_ACTIVITY_PANEL),
            "FEATURE_SWITCH_MAILBOX" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_MAILBOX),
            "FEATURE_SWITCH_QUEST" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_QUEST),
            "FEATURE_SWITCH_GACHA" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_GACHA),
            "FEATURE_SWITCH_CHAT" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_CHAT),
            "FEATURE_SWITCH_MODIFY_FRIEND_ALIAS" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_MODIFY_FRIEND_ALIAS),
            "FEATURE_SWITCH_USE_ITEM" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_USE_ITEM),
            "FEATURE_SWITCH_ACTIVITY_SCHEDULE" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_ACTIVITY_SCHEDULE),
            "FEATURE_SWITCH_FARM_ELEMENT" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_FARM_ELEMENT),
            "FEATURE_SWITCH_ACHIEVEMENT_LEVEL" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_ACHIEVEMENT_LEVEL),
            "FEATURE_SWITCH_DAILY_ACTIVE_LEVEL" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_DAILY_ACTIVE_LEVEL),
            "FEATURE_SWITCH_PLAYER_RETURN" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_PLAYER_RETURN),
            "FEATURE_SWITCH_FIRST_SET_NICKNAME" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_FIRST_SET_NICKNAME),
            "FEATURE_SWITCH_MAIN_MISSION_REWARD" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_MAIN_MISSION_REWARD),
            "FEATURE_SWITCH_SUB_MISSION_REWARD" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_SUB_MISSION_REWARD),
            "FEATURE_SWITCH_PAM_MISSION" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_PAM_MISSION),
            "FEATURE_SWITCH_DESTROY_ITEM" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_DESTROY_ITEM),
            "FEATURE_SWITCH_CONSUME_ITEM_TURN" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_CONSUME_ITEM_TURN),
            "FEATURE_SWITCH_ROGUE_MODIFIER" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_ROGUE_MODIFIER),
            "FEATURE_SWITCH_CHESS_ROGUE" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_CHESS_ROGUE),
            "FEATURE_SWITCH_CHESS_ROGUE_BOARD" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_CHESS_ROGUE_BOARD),
            "FEATURE_SWITCH_ROLL_SHOP" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_ROLL_SHOP),
            "FEATURE_SWITCH_H5_RETURN" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_H5_RETURN),
            "FEATURE_SWITCH_OFFERING" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_OFFERING),
            "FEATURE_SWITCH_SERVER_RED_POINT" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_SERVER_RED_POINT),
            "FEATURE_SWITCH_MONOPOLY_OPTION_RATIO" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_MONOPOLY_OPTION_RATIO),
            "FEATURE_SWITCH_MONOPOLY_GET_RAFFLE_TICKET" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_MONOPOLY_GET_RAFFLE_TICKET),
            "FEATURE_SWITCH_MONOPOLY_TAKE_RAFFLE_REWARD" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_MONOPOLY_TAKE_RAFFLE_REWARD),
            "FEATURE_SWITCH_CHALLENGE_RECOMMEND_LINEUP" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_CHALLENGE_RECOMMEND_LINEUP),
            "FEATURE_SWITCH_PSN_MEMBER_SHIP_CHECK" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_PSN_MEMBER_SHIP_CHECK),
            "FEATURE_SWITCH_PLAYER_BOARD_DEVELOPMENT" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_PLAYER_BOARD_DEVELOPMENT),
            "FEATURE_SWITCH_PVP" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_PVP),
            "FEATURE_SWITCH_ROGUE_MODE" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_ROGUE_MODE),
            "FEATURE_SWITCH_ROGUE_TOURN_UGC" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_ROGUE_TOURN_UGC),
            "FEATURE_SWITCH_RELIC_FILTER_PLAN_NAME" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_RELIC_FILTER_PLAN_NAME),
            "FEATURE_SWITCH_MAZE_ITEM_USE_BUFF_DROP" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_MAZE_ITEM_USE_BUFF_DROP),
            "FEATURE_SWITCH_RED_DOT" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_RED_DOT),
            "FEATURE_SWITCH_GAME_STATE_SERVICE" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_GAME_STATE_SERVICE),
            "FEATURE_SWITCH_BENEFIT_INDEX" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_BENEFIT_INDEX),
            "FEATURE_SWITCH_ROGUE_TOURN_BUILD_REF" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_ROGUE_TOURN_BUILD_REF),
            "FEATURE_SWITCH_PRE_AVATAR_SET_GROWTH_TARGET" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_PRE_AVATAR_SET_GROWTH_TARGET),
            "FEATURE_SWITCH_IMPORT_RELIC_FILTER_PLAN" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_IMPORT_RELIC_FILTER_PLAN),
            "FEATURE_SWITCH_GACHA_DECIDE_ITEM" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_GACHA_DECIDE_ITEM),
            "FEATURE_SWITCH_ITEM_SYNC" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_ITEM_SYNC),
            "FEATURE_SWITCH_RECHARGE_BENEFIT" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_RECHARGE_BENEFIT),
            "FEATURE_SWITCH_RECHARGE_GIFT" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_RECHARGE_GIFT),
            "FEATURE_SWITCH_GACHA_AVATAR_TOAST" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_GACHA_AVATAR_TOAST),
            "FEATURE_SWITCH_RELIC_SMART_DISCARD" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_RELIC_SMART_DISCARD),
            "FEATURE_SWITCH_PLANETFES_SOCIAL" => ::std::option::Option::Some(FeatureSwitchType::FEATURE_SWITCH_PLANETFES_SOCIAL),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [FeatureSwitchType] = &[
        FeatureSwitchType::FEATURE_SWITCH_NONE,
        FeatureSwitchType::FEATURE_SWITCH_SHOP,
        FeatureSwitchType::FEATURE_SWITCH_LINEUP_NAME,
        FeatureSwitchType::FEATURE_SWITCH_RECHARGE_SHOP,
        FeatureSwitchType::FEATURE_SWITCH_NICKNAME,
        FeatureSwitchType::FEATURE_SWITCH_SIGNATURE,
        FeatureSwitchType::FEATURE_SWITCH_BATTLEPASS,
        FeatureSwitchType::FEATURE_SWITCH_PUNK_LORD,
        FeatureSwitchType::FEATURE_SWITCH_MONTHCARD_DAILY,
        FeatureSwitchType::FEATURE_SWITCH_PICTURE_SHARE,
        FeatureSwitchType::FEATURE_SWITCH_ROGUE,
        FeatureSwitchType::FEATURE_SWITCH_CHALLENGE,
        FeatureSwitchType::FEATURE_SWITCH_COCOON,
        FeatureSwitchType::FEATURE_SWITCH_RAID,
        FeatureSwitchType::FEATURE_SWITCH_MAZE_PLANE_EVENT,
        FeatureSwitchType::FEATURE_SWITCH_ACTIVITY_PANEL,
        FeatureSwitchType::FEATURE_SWITCH_MAILBOX,
        FeatureSwitchType::FEATURE_SWITCH_QUEST,
        FeatureSwitchType::FEATURE_SWITCH_GACHA,
        FeatureSwitchType::FEATURE_SWITCH_CHAT,
        FeatureSwitchType::FEATURE_SWITCH_MODIFY_FRIEND_ALIAS,
        FeatureSwitchType::FEATURE_SWITCH_USE_ITEM,
        FeatureSwitchType::FEATURE_SWITCH_ACTIVITY_SCHEDULE,
        FeatureSwitchType::FEATURE_SWITCH_FARM_ELEMENT,
        FeatureSwitchType::FEATURE_SWITCH_ACHIEVEMENT_LEVEL,
        FeatureSwitchType::FEATURE_SWITCH_DAILY_ACTIVE_LEVEL,
        FeatureSwitchType::FEATURE_SWITCH_PLAYER_RETURN,
        FeatureSwitchType::FEATURE_SWITCH_FIRST_SET_NICKNAME,
        FeatureSwitchType::FEATURE_SWITCH_MAIN_MISSION_REWARD,
        FeatureSwitchType::FEATURE_SWITCH_SUB_MISSION_REWARD,
        FeatureSwitchType::FEATURE_SWITCH_PAM_MISSION,
        FeatureSwitchType::FEATURE_SWITCH_DESTROY_ITEM,
        FeatureSwitchType::FEATURE_SWITCH_CONSUME_ITEM_TURN,
        FeatureSwitchType::FEATURE_SWITCH_ROGUE_MODIFIER,
        FeatureSwitchType::FEATURE_SWITCH_CHESS_ROGUE,
        FeatureSwitchType::FEATURE_SWITCH_CHESS_ROGUE_BOARD,
        FeatureSwitchType::FEATURE_SWITCH_ROLL_SHOP,
        FeatureSwitchType::FEATURE_SWITCH_H5_RETURN,
        FeatureSwitchType::FEATURE_SWITCH_OFFERING,
        FeatureSwitchType::FEATURE_SWITCH_SERVER_RED_POINT,
        FeatureSwitchType::FEATURE_SWITCH_MONOPOLY_OPTION_RATIO,
        FeatureSwitchType::FEATURE_SWITCH_MONOPOLY_GET_RAFFLE_TICKET,
        FeatureSwitchType::FEATURE_SWITCH_MONOPOLY_TAKE_RAFFLE_REWARD,
        FeatureSwitchType::FEATURE_SWITCH_CHALLENGE_RECOMMEND_LINEUP,
        FeatureSwitchType::FEATURE_SWITCH_PSN_MEMBER_SHIP_CHECK,
        FeatureSwitchType::FEATURE_SWITCH_PLAYER_BOARD_DEVELOPMENT,
        FeatureSwitchType::FEATURE_SWITCH_PVP,
        FeatureSwitchType::FEATURE_SWITCH_ROGUE_MODE,
        FeatureSwitchType::FEATURE_SWITCH_ROGUE_TOURN_UGC,
        FeatureSwitchType::FEATURE_SWITCH_RELIC_FILTER_PLAN_NAME,
        FeatureSwitchType::FEATURE_SWITCH_MAZE_ITEM_USE_BUFF_DROP,
        FeatureSwitchType::FEATURE_SWITCH_RED_DOT,
        FeatureSwitchType::FEATURE_SWITCH_GAME_STATE_SERVICE,
        FeatureSwitchType::FEATURE_SWITCH_BENEFIT_INDEX,
        FeatureSwitchType::FEATURE_SWITCH_ROGUE_TOURN_BUILD_REF,
        FeatureSwitchType::FEATURE_SWITCH_PRE_AVATAR_SET_GROWTH_TARGET,
        FeatureSwitchType::FEATURE_SWITCH_IMPORT_RELIC_FILTER_PLAN,
        FeatureSwitchType::FEATURE_SWITCH_GACHA_DECIDE_ITEM,
        FeatureSwitchType::FEATURE_SWITCH_ITEM_SYNC,
        FeatureSwitchType::FEATURE_SWITCH_RECHARGE_BENEFIT,
        FeatureSwitchType::FEATURE_SWITCH_RECHARGE_GIFT,
        FeatureSwitchType::FEATURE_SWITCH_GACHA_AVATAR_TOAST,
        FeatureSwitchType::FEATURE_SWITCH_RELIC_SMART_DISCARD,
        FeatureSwitchType::FEATURE_SWITCH_PLANETFES_SOCIAL,
    ];
}

impl ::protobuf::EnumFull for FeatureSwitchType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("FeatureSwitchType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            FeatureSwitchType::FEATURE_SWITCH_NONE => 0,
            FeatureSwitchType::FEATURE_SWITCH_SHOP => 1,
            FeatureSwitchType::FEATURE_SWITCH_LINEUP_NAME => 2,
            FeatureSwitchType::FEATURE_SWITCH_RECHARGE_SHOP => 3,
            FeatureSwitchType::FEATURE_SWITCH_NICKNAME => 4,
            FeatureSwitchType::FEATURE_SWITCH_SIGNATURE => 5,
            FeatureSwitchType::FEATURE_SWITCH_BATTLEPASS => 6,
            FeatureSwitchType::FEATURE_SWITCH_PUNK_LORD => 7,
            FeatureSwitchType::FEATURE_SWITCH_MONTHCARD_DAILY => 8,
            FeatureSwitchType::FEATURE_SWITCH_PICTURE_SHARE => 9,
            FeatureSwitchType::FEATURE_SWITCH_ROGUE => 10,
            FeatureSwitchType::FEATURE_SWITCH_CHALLENGE => 11,
            FeatureSwitchType::FEATURE_SWITCH_COCOON => 12,
            FeatureSwitchType::FEATURE_SWITCH_RAID => 13,
            FeatureSwitchType::FEATURE_SWITCH_MAZE_PLANE_EVENT => 14,
            FeatureSwitchType::FEATURE_SWITCH_ACTIVITY_PANEL => 15,
            FeatureSwitchType::FEATURE_SWITCH_MAILBOX => 16,
            FeatureSwitchType::FEATURE_SWITCH_QUEST => 17,
            FeatureSwitchType::FEATURE_SWITCH_GACHA => 18,
            FeatureSwitchType::FEATURE_SWITCH_CHAT => 19,
            FeatureSwitchType::FEATURE_SWITCH_MODIFY_FRIEND_ALIAS => 20,
            FeatureSwitchType::FEATURE_SWITCH_USE_ITEM => 21,
            FeatureSwitchType::FEATURE_SWITCH_ACTIVITY_SCHEDULE => 22,
            FeatureSwitchType::FEATURE_SWITCH_FARM_ELEMENT => 23,
            FeatureSwitchType::FEATURE_SWITCH_ACHIEVEMENT_LEVEL => 24,
            FeatureSwitchType::FEATURE_SWITCH_DAILY_ACTIVE_LEVEL => 25,
            FeatureSwitchType::FEATURE_SWITCH_PLAYER_RETURN => 26,
            FeatureSwitchType::FEATURE_SWITCH_FIRST_SET_NICKNAME => 27,
            FeatureSwitchType::FEATURE_SWITCH_MAIN_MISSION_REWARD => 28,
            FeatureSwitchType::FEATURE_SWITCH_SUB_MISSION_REWARD => 29,
            FeatureSwitchType::FEATURE_SWITCH_PAM_MISSION => 30,
            FeatureSwitchType::FEATURE_SWITCH_DESTROY_ITEM => 31,
            FeatureSwitchType::FEATURE_SWITCH_CONSUME_ITEM_TURN => 32,
            FeatureSwitchType::FEATURE_SWITCH_ROGUE_MODIFIER => 33,
            FeatureSwitchType::FEATURE_SWITCH_CHESS_ROGUE => 34,
            FeatureSwitchType::FEATURE_SWITCH_CHESS_ROGUE_BOARD => 35,
            FeatureSwitchType::FEATURE_SWITCH_ROLL_SHOP => 36,
            FeatureSwitchType::FEATURE_SWITCH_H5_RETURN => 37,
            FeatureSwitchType::FEATURE_SWITCH_OFFERING => 38,
            FeatureSwitchType::FEATURE_SWITCH_SERVER_RED_POINT => 39,
            FeatureSwitchType::FEATURE_SWITCH_MONOPOLY_OPTION_RATIO => 40,
            FeatureSwitchType::FEATURE_SWITCH_MONOPOLY_GET_RAFFLE_TICKET => 41,
            FeatureSwitchType::FEATURE_SWITCH_MONOPOLY_TAKE_RAFFLE_REWARD => 42,
            FeatureSwitchType::FEATURE_SWITCH_CHALLENGE_RECOMMEND_LINEUP => 43,
            FeatureSwitchType::FEATURE_SWITCH_PSN_MEMBER_SHIP_CHECK => 44,
            FeatureSwitchType::FEATURE_SWITCH_PLAYER_BOARD_DEVELOPMENT => 45,
            FeatureSwitchType::FEATURE_SWITCH_PVP => 46,
            FeatureSwitchType::FEATURE_SWITCH_ROGUE_MODE => 47,
            FeatureSwitchType::FEATURE_SWITCH_ROGUE_TOURN_UGC => 48,
            FeatureSwitchType::FEATURE_SWITCH_RELIC_FILTER_PLAN_NAME => 49,
            FeatureSwitchType::FEATURE_SWITCH_MAZE_ITEM_USE_BUFF_DROP => 50,
            FeatureSwitchType::FEATURE_SWITCH_RED_DOT => 51,
            FeatureSwitchType::FEATURE_SWITCH_GAME_STATE_SERVICE => 52,
            FeatureSwitchType::FEATURE_SWITCH_BENEFIT_INDEX => 53,
            FeatureSwitchType::FEATURE_SWITCH_ROGUE_TOURN_BUILD_REF => 54,
            FeatureSwitchType::FEATURE_SWITCH_PRE_AVATAR_SET_GROWTH_TARGET => 55,
            FeatureSwitchType::FEATURE_SWITCH_IMPORT_RELIC_FILTER_PLAN => 56,
            FeatureSwitchType::FEATURE_SWITCH_GACHA_DECIDE_ITEM => 57,
            FeatureSwitchType::FEATURE_SWITCH_ITEM_SYNC => 58,
            FeatureSwitchType::FEATURE_SWITCH_RECHARGE_BENEFIT => 59,
            FeatureSwitchType::FEATURE_SWITCH_RECHARGE_GIFT => 60,
            FeatureSwitchType::FEATURE_SWITCH_GACHA_AVATAR_TOAST => 61,
            FeatureSwitchType::FEATURE_SWITCH_RELIC_SMART_DISCARD => 62,
            FeatureSwitchType::FEATURE_SWITCH_PLANETFES_SOCIAL => 63,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for FeatureSwitchType {
    fn default() -> Self {
        FeatureSwitchType::FEATURE_SWITCH_NONE
    }
}

impl FeatureSwitchType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<FeatureSwitchType>("FeatureSwitchType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17FeatureSwitchType.proto*\xce\x11\n\x11FeatureSwitchType\x12\x17\n\
    \x13FEATURE_SWITCH_NONE\x10\0\x12\x17\n\x13FEATURE_SWITCH_SHOP\x10\x01\
    \x12\x1e\n\x1aFEATURE_SWITCH_LINEUP_NAME\x10\x02\x12\x20\n\x1cFEATURE_SW\
    ITCH_RECHARGE_SHOP\x10\x03\x12\x1b\n\x17FEATURE_SWITCH_NICKNAME\x10\x04\
    \x12\x1c\n\x18FEATURE_SWITCH_SIGNATURE\x10\x05\x12\x1d\n\x19FEATURE_SWIT\
    CH_BATTLEPASS\x10\x06\x12\x1c\n\x18FEATURE_SWITCH_PUNK_LORD\x10\x07\x12\
    \"\n\x1eFEATURE_SWITCH_MONTHCARD_DAILY\x10\x08\x12\x20\n\x1cFEATURE_SWIT\
    CH_PICTURE_SHARE\x10\t\x12\x18\n\x14FEATURE_SWITCH_ROGUE\x10\n\x12\x1c\n\
    \x18FEATURE_SWITCH_CHALLENGE\x10\x0b\x12\x19\n\x15FEATURE_SWITCH_COCOON\
    \x10\x0c\x12\x17\n\x13FEATURE_SWITCH_RAID\x10\r\x12#\n\x1fFEATURE_SWITCH\
    _MAZE_PLANE_EVENT\x10\x0e\x12!\n\x1dFEATURE_SWITCH_ACTIVITY_PANEL\x10\
    \x0f\x12\x1a\n\x16FEATURE_SWITCH_MAILBOX\x10\x10\x12\x18\n\x14FEATURE_SW\
    ITCH_QUEST\x10\x11\x12\x18\n\x14FEATURE_SWITCH_GACHA\x10\x12\x12\x17\n\
    \x13FEATURE_SWITCH_CHAT\x10\x13\x12&\n\"FEATURE_SWITCH_MODIFY_FRIEND_ALI\
    AS\x10\x14\x12\x1b\n\x17FEATURE_SWITCH_USE_ITEM\x10\x15\x12$\n\x20FEATUR\
    E_SWITCH_ACTIVITY_SCHEDULE\x10\x16\x12\x1f\n\x1bFEATURE_SWITCH_FARM_ELEM\
    ENT\x10\x17\x12$\n\x20FEATURE_SWITCH_ACHIEVEMENT_LEVEL\x10\x18\x12%\n!FE\
    ATURE_SWITCH_DAILY_ACTIVE_LEVEL\x10\x19\x12\x20\n\x1cFEATURE_SWITCH_PLAY\
    ER_RETURN\x10\x1a\x12%\n!FEATURE_SWITCH_FIRST_SET_NICKNAME\x10\x1b\x12&\
    \n\"FEATURE_SWITCH_MAIN_MISSION_REWARD\x10\x1c\x12%\n!FEATURE_SWITCH_SUB\
    _MISSION_REWARD\x10\x1d\x12\x1e\n\x1aFEATURE_SWITCH_PAM_MISSION\x10\x1e\
    \x12\x1f\n\x1bFEATURE_SWITCH_DESTROY_ITEM\x10\x20\x12$\n\x20FEATURE_SWIT\
    CH_CONSUME_ITEM_TURN\x10!\x12!\n\x1dFEATURE_SWITCH_ROGUE_MODIFIER\x10\"\
    \x12\x1e\n\x1aFEATURE_SWITCH_CHESS_ROGUE\x10#\x12$\n\x20FEATURE_SWITCH_C\
    HESS_ROGUE_BOARD\x10$\x12\x1c\n\x18FEATURE_SWITCH_ROLL_SHOP\x10%\x12\x1c\
    \n\x18FEATURE_SWITCH_H5_RETURN\x10&\x12\x1b\n\x17FEATURE_SWITCH_OFFERING\
    \x10'\x12#\n\x1fFEATURE_SWITCH_SERVER_RED_POINT\x10(\x12(\n$FEATURE_SWIT\
    CH_MONOPOLY_OPTION_RATIO\x10)\x12-\n)FEATURE_SWITCH_MONOPOLY_GET_RAFFLE_\
    TICKET\x10*\x12.\n*FEATURE_SWITCH_MONOPOLY_TAKE_RAFFLE_REWARD\x10+\x12-\
    \n)FEATURE_SWITCH_CHALLENGE_RECOMMEND_LINEUP\x10,\x12(\n$FEATURE_SWITCH_\
    PSN_MEMBER_SHIP_CHECK\x10-\x12+\n'FEATURE_SWITCH_PLAYER_BOARD_DEVELOPMEN\
    T\x10.\x12\x16\n\x12FEATURE_SWITCH_PVP\x10/\x12\x1d\n\x19FEATURE_SWITCH_\
    ROGUE_MODE\x100\x12\"\n\x1eFEATURE_SWITCH_ROGUE_TOURN_UGC\x101\x12)\n%FE\
    ATURE_SWITCH_RELIC_FILTER_PLAN_NAME\x102\x12*\n&FEATURE_SWITCH_MAZE_ITEM\
    _USE_BUFF_DROP\x103\x12\x1a\n\x16FEATURE_SWITCH_RED_DOT\x104\x12%\n!FEAT\
    URE_SWITCH_GAME_STATE_SERVICE\x105\x12\x20\n\x1cFEATURE_SWITCH_BENEFIT_I\
    NDEX\x106\x12(\n$FEATURE_SWITCH_ROGUE_TOURN_BUILD_REF\x107\x12/\n+FEATUR\
    E_SWITCH_PRE_AVATAR_SET_GROWTH_TARGET\x108\x12+\n'FEATURE_SWITCH_IMPORT_\
    RELIC_FILTER_PLAN\x10:\x12$\n\x20FEATURE_SWITCH_GACHA_DECIDE_ITEM\x10;\
    \x12\x1c\n\x18FEATURE_SWITCH_ITEM_SYNC\x10<\x12#\n\x1fFEATURE_SWITCH_REC\
    HARGE_BENEFIT\x10=\x12\x20\n\x1cFEATURE_SWITCH_RECHARGE_GIFT\x10>\x12%\n\
    !FEATURE_SWITCH_GACHA_AVATAR_TOAST\x10@\x12&\n\"FEATURE_SWITCH_RELIC_SMA\
    RT_DISCARD\x10B\x12#\n\x1fFEATURE_SWITCH_PLANETFES_SOCIAL\x10Cb\x06proto\
    3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(0);
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(FeatureSwitchType::generated_enum_descriptor_data());
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
