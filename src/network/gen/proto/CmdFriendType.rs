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

//! Generated file from `CmdFriendType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdFriendType)
pub enum CmdFriendType {
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdFriendTypeNone)
    CmdFriendTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdNewAssistHistoryNotify)
    CmdNewAssistHistoryNotify = 2983,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdAddBlacklistCsReq)
    CmdAddBlacklistCsReq = 2977,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdSyncApplyFriendScNotify)
    CmdSyncApplyFriendScNotify = 2926,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdSetFriendMarkCsReq)
    CmdSetFriendMarkCsReq = 2914,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdGetFriendListInfoScRsp)
    CmdGetFriendListInfoScRsp = 2913,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdGetPlatformPlayerInfoScRsp)
    CmdGetPlatformPlayerInfoScRsp = 2987,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdGetAssistHistoryCsReq)
    CmdGetAssistHistoryCsReq = 2975,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdGetFriendAssistListCsReq)
    CmdGetFriendAssistListCsReq = 2917,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdTakeAssistRewardScRsp)
    CmdTakeAssistRewardScRsp = 2968,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdGetFriendLoginInfoCsReq)
    CmdGetFriendLoginInfoCsReq = 2956,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdGetFriendLoginInfoScRsp)
    CmdGetFriendLoginInfoScRsp = 2981,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdGetPlayerDetailInfoScRsp)
    CmdGetPlayerDetailInfoScRsp = 2909,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdApplyFriendCsReq)
    CmdApplyFriendCsReq = 2970,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdApplyFriendScRsp)
    CmdApplyFriendScRsp = 2989,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdGetAssistListCsReq)
    CmdGetAssistListCsReq = 2922,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdSearchPlayerCsReq)
    CmdSearchPlayerCsReq = 2965,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdGetAssistHistoryScRsp)
    CmdGetAssistHistoryScRsp = 2928,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdReportPlayerCsReq)
    CmdReportPlayerCsReq = 2971,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdSetFriendMarkScRsp)
    CmdSetFriendMarkScRsp = 2941,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdGetFriendBattleRecordDetailScRsp)
    CmdGetFriendBattleRecordDetailScRsp = 2999,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdSetFriendRemarkNameScRsp)
    CmdSetFriendRemarkNameScRsp = 2907,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdDeleteFriendScRsp)
    CmdDeleteFriendScRsp = 2950,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdSyncHandleFriendScNotify)
    CmdSyncHandleFriendScNotify = 2918,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdSearchPlayerScRsp)
    CmdSearchPlayerScRsp = 2952,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdAddBlacklistScRsp)
    CmdAddBlacklistScRsp = 2991,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdGetFriendRecommendListInfoCsReq)
    CmdGetFriendRecommendListInfoCsReq = 2957,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdGetFriendAssistListScRsp)
    CmdGetFriendAssistListScRsp = 2963,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdGetFriendApplyListInfoScRsp)
    CmdGetFriendApplyListInfoScRsp = 2906,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdSyncDeleteFriendScNotify)
    CmdSyncDeleteFriendScNotify = 2973,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdReportPlayerScRsp)
    CmdReportPlayerScRsp = 2982,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdGetCurAssistScRsp)
    CmdGetCurAssistScRsp = 2984,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdGetFriendChallengeDetailCsReq)
    CmdGetFriendChallengeDetailCsReq = 2996,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdGetFriendApplyListInfoCsReq)
    CmdGetFriendApplyListInfoCsReq = 2935,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdDeleteFriendCsReq)
    CmdDeleteFriendCsReq = 2936,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdGetAssistListScRsp)
    CmdGetAssistListScRsp = 2986,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdDeleteBlacklistScRsp)
    CmdDeleteBlacklistScRsp = 2937,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdGetCurAssistCsReq)
    CmdGetCurAssistCsReq = 2924,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdGetFriendBattleRecordDetailCsReq)
    CmdGetFriendBattleRecordDetailCsReq = 2966,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdSetForbidOtherApplyFriendCsReq)
    CmdSetForbidOtherApplyFriendCsReq = 3000,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdSetForbidOtherApplyFriendScRsp)
    CmdSetForbidOtherApplyFriendScRsp = 2946,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdSetAssistScRsp)
    CmdSetAssistScRsp = 2953,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdSetAssistCsReq)
    CmdSetAssistCsReq = 2992,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdDeleteBlacklistCsReq)
    CmdDeleteBlacklistCsReq = 2951,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdGetFriendDevelopmentInfoScRsp)
    CmdGetFriendDevelopmentInfoScRsp = 2908,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdGetFriendChallengeLineupCsReq)
    CmdGetFriendChallengeLineupCsReq = 2904,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdGetFriendChallengeDetailScRsp)
    CmdGetFriendChallengeDetailScRsp = 2969,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdGetFriendListInfoCsReq)
    CmdGetFriendListInfoCsReq = 2911,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdCurAssistChangedNotify)
    CmdCurAssistChangedNotify = 2960,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdGetPlayerDetailInfoCsReq)
    CmdGetPlayerDetailInfoCsReq = 2947,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdGetFriendChallengeLineupScRsp)
    CmdGetFriendChallengeLineupScRsp = 2978,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdHandleFriendScRsp)
    CmdHandleFriendScRsp = 2995,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdGetFriendRecommendListInfoScRsp)
    CmdGetFriendRecommendListInfoScRsp = 2925,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdGetPlatformPlayerInfoCsReq)
    CmdGetPlatformPlayerInfoCsReq = 2994,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdTakeAssistRewardCsReq)
    CmdTakeAssistRewardCsReq = 2958,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdGetFriendDevelopmentInfoCsReq)
    CmdGetFriendDevelopmentInfoCsReq = 2921,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdSyncAddBlacklistScNotify)
    CmdSyncAddBlacklistScNotify = 2993,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdHandleFriendCsReq)
    CmdHandleFriendCsReq = 2930,
    // @@protoc_insertion_point(enum_value:CmdFriendType.CmdSetFriendRemarkNameCsReq)
    CmdSetFriendRemarkNameCsReq = 2910,
}

impl ::protobuf::Enum for CmdFriendType {
    const NAME: &'static str = "CmdFriendType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdFriendType> {
        match value {
            0 => ::std::option::Option::Some(CmdFriendType::CmdFriendTypeNone),
            2983 => ::std::option::Option::Some(CmdFriendType::CmdNewAssistHistoryNotify),
            2977 => ::std::option::Option::Some(CmdFriendType::CmdAddBlacklistCsReq),
            2926 => ::std::option::Option::Some(CmdFriendType::CmdSyncApplyFriendScNotify),
            2914 => ::std::option::Option::Some(CmdFriendType::CmdSetFriendMarkCsReq),
            2913 => ::std::option::Option::Some(CmdFriendType::CmdGetFriendListInfoScRsp),
            2987 => ::std::option::Option::Some(CmdFriendType::CmdGetPlatformPlayerInfoScRsp),
            2975 => ::std::option::Option::Some(CmdFriendType::CmdGetAssistHistoryCsReq),
            2917 => ::std::option::Option::Some(CmdFriendType::CmdGetFriendAssistListCsReq),
            2968 => ::std::option::Option::Some(CmdFriendType::CmdTakeAssistRewardScRsp),
            2956 => ::std::option::Option::Some(CmdFriendType::CmdGetFriendLoginInfoCsReq),
            2981 => ::std::option::Option::Some(CmdFriendType::CmdGetFriendLoginInfoScRsp),
            2909 => ::std::option::Option::Some(CmdFriendType::CmdGetPlayerDetailInfoScRsp),
            2970 => ::std::option::Option::Some(CmdFriendType::CmdApplyFriendCsReq),
            2989 => ::std::option::Option::Some(CmdFriendType::CmdApplyFriendScRsp),
            2922 => ::std::option::Option::Some(CmdFriendType::CmdGetAssistListCsReq),
            2965 => ::std::option::Option::Some(CmdFriendType::CmdSearchPlayerCsReq),
            2928 => ::std::option::Option::Some(CmdFriendType::CmdGetAssistHistoryScRsp),
            2971 => ::std::option::Option::Some(CmdFriendType::CmdReportPlayerCsReq),
            2941 => ::std::option::Option::Some(CmdFriendType::CmdSetFriendMarkScRsp),
            2999 => ::std::option::Option::Some(CmdFriendType::CmdGetFriendBattleRecordDetailScRsp),
            2907 => ::std::option::Option::Some(CmdFriendType::CmdSetFriendRemarkNameScRsp),
            2950 => ::std::option::Option::Some(CmdFriendType::CmdDeleteFriendScRsp),
            2918 => ::std::option::Option::Some(CmdFriendType::CmdSyncHandleFriendScNotify),
            2952 => ::std::option::Option::Some(CmdFriendType::CmdSearchPlayerScRsp),
            2991 => ::std::option::Option::Some(CmdFriendType::CmdAddBlacklistScRsp),
            2957 => ::std::option::Option::Some(CmdFriendType::CmdGetFriendRecommendListInfoCsReq),
            2963 => ::std::option::Option::Some(CmdFriendType::CmdGetFriendAssistListScRsp),
            2906 => ::std::option::Option::Some(CmdFriendType::CmdGetFriendApplyListInfoScRsp),
            2973 => ::std::option::Option::Some(CmdFriendType::CmdSyncDeleteFriendScNotify),
            2982 => ::std::option::Option::Some(CmdFriendType::CmdReportPlayerScRsp),
            2984 => ::std::option::Option::Some(CmdFriendType::CmdGetCurAssistScRsp),
            2996 => ::std::option::Option::Some(CmdFriendType::CmdGetFriendChallengeDetailCsReq),
            2935 => ::std::option::Option::Some(CmdFriendType::CmdGetFriendApplyListInfoCsReq),
            2936 => ::std::option::Option::Some(CmdFriendType::CmdDeleteFriendCsReq),
            2986 => ::std::option::Option::Some(CmdFriendType::CmdGetAssistListScRsp),
            2937 => ::std::option::Option::Some(CmdFriendType::CmdDeleteBlacklistScRsp),
            2924 => ::std::option::Option::Some(CmdFriendType::CmdGetCurAssistCsReq),
            2966 => ::std::option::Option::Some(CmdFriendType::CmdGetFriendBattleRecordDetailCsReq),
            3000 => ::std::option::Option::Some(CmdFriendType::CmdSetForbidOtherApplyFriendCsReq),
            2946 => ::std::option::Option::Some(CmdFriendType::CmdSetForbidOtherApplyFriendScRsp),
            2953 => ::std::option::Option::Some(CmdFriendType::CmdSetAssistScRsp),
            2992 => ::std::option::Option::Some(CmdFriendType::CmdSetAssistCsReq),
            2951 => ::std::option::Option::Some(CmdFriendType::CmdDeleteBlacklistCsReq),
            2908 => ::std::option::Option::Some(CmdFriendType::CmdGetFriendDevelopmentInfoScRsp),
            2904 => ::std::option::Option::Some(CmdFriendType::CmdGetFriendChallengeLineupCsReq),
            2969 => ::std::option::Option::Some(CmdFriendType::CmdGetFriendChallengeDetailScRsp),
            2911 => ::std::option::Option::Some(CmdFriendType::CmdGetFriendListInfoCsReq),
            2960 => ::std::option::Option::Some(CmdFriendType::CmdCurAssistChangedNotify),
            2947 => ::std::option::Option::Some(CmdFriendType::CmdGetPlayerDetailInfoCsReq),
            2978 => ::std::option::Option::Some(CmdFriendType::CmdGetFriendChallengeLineupScRsp),
            2995 => ::std::option::Option::Some(CmdFriendType::CmdHandleFriendScRsp),
            2925 => ::std::option::Option::Some(CmdFriendType::CmdGetFriendRecommendListInfoScRsp),
            2994 => ::std::option::Option::Some(CmdFriendType::CmdGetPlatformPlayerInfoCsReq),
            2958 => ::std::option::Option::Some(CmdFriendType::CmdTakeAssistRewardCsReq),
            2921 => ::std::option::Option::Some(CmdFriendType::CmdGetFriendDevelopmentInfoCsReq),
            2993 => ::std::option::Option::Some(CmdFriendType::CmdSyncAddBlacklistScNotify),
            2930 => ::std::option::Option::Some(CmdFriendType::CmdHandleFriendCsReq),
            2910 => ::std::option::Option::Some(CmdFriendType::CmdSetFriendRemarkNameCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdFriendType> {
        match str {
            "CmdFriendTypeNone" => ::std::option::Option::Some(CmdFriendType::CmdFriendTypeNone),
            "CmdNewAssistHistoryNotify" => ::std::option::Option::Some(CmdFriendType::CmdNewAssistHistoryNotify),
            "CmdAddBlacklistCsReq" => ::std::option::Option::Some(CmdFriendType::CmdAddBlacklistCsReq),
            "CmdSyncApplyFriendScNotify" => ::std::option::Option::Some(CmdFriendType::CmdSyncApplyFriendScNotify),
            "CmdSetFriendMarkCsReq" => ::std::option::Option::Some(CmdFriendType::CmdSetFriendMarkCsReq),
            "CmdGetFriendListInfoScRsp" => ::std::option::Option::Some(CmdFriendType::CmdGetFriendListInfoScRsp),
            "CmdGetPlatformPlayerInfoScRsp" => ::std::option::Option::Some(CmdFriendType::CmdGetPlatformPlayerInfoScRsp),
            "CmdGetAssistHistoryCsReq" => ::std::option::Option::Some(CmdFriendType::CmdGetAssistHistoryCsReq),
            "CmdGetFriendAssistListCsReq" => ::std::option::Option::Some(CmdFriendType::CmdGetFriendAssistListCsReq),
            "CmdTakeAssistRewardScRsp" => ::std::option::Option::Some(CmdFriendType::CmdTakeAssistRewardScRsp),
            "CmdGetFriendLoginInfoCsReq" => ::std::option::Option::Some(CmdFriendType::CmdGetFriendLoginInfoCsReq),
            "CmdGetFriendLoginInfoScRsp" => ::std::option::Option::Some(CmdFriendType::CmdGetFriendLoginInfoScRsp),
            "CmdGetPlayerDetailInfoScRsp" => ::std::option::Option::Some(CmdFriendType::CmdGetPlayerDetailInfoScRsp),
            "CmdApplyFriendCsReq" => ::std::option::Option::Some(CmdFriendType::CmdApplyFriendCsReq),
            "CmdApplyFriendScRsp" => ::std::option::Option::Some(CmdFriendType::CmdApplyFriendScRsp),
            "CmdGetAssistListCsReq" => ::std::option::Option::Some(CmdFriendType::CmdGetAssistListCsReq),
            "CmdSearchPlayerCsReq" => ::std::option::Option::Some(CmdFriendType::CmdSearchPlayerCsReq),
            "CmdGetAssistHistoryScRsp" => ::std::option::Option::Some(CmdFriendType::CmdGetAssistHistoryScRsp),
            "CmdReportPlayerCsReq" => ::std::option::Option::Some(CmdFriendType::CmdReportPlayerCsReq),
            "CmdSetFriendMarkScRsp" => ::std::option::Option::Some(CmdFriendType::CmdSetFriendMarkScRsp),
            "CmdGetFriendBattleRecordDetailScRsp" => ::std::option::Option::Some(CmdFriendType::CmdGetFriendBattleRecordDetailScRsp),
            "CmdSetFriendRemarkNameScRsp" => ::std::option::Option::Some(CmdFriendType::CmdSetFriendRemarkNameScRsp),
            "CmdDeleteFriendScRsp" => ::std::option::Option::Some(CmdFriendType::CmdDeleteFriendScRsp),
            "CmdSyncHandleFriendScNotify" => ::std::option::Option::Some(CmdFriendType::CmdSyncHandleFriendScNotify),
            "CmdSearchPlayerScRsp" => ::std::option::Option::Some(CmdFriendType::CmdSearchPlayerScRsp),
            "CmdAddBlacklistScRsp" => ::std::option::Option::Some(CmdFriendType::CmdAddBlacklistScRsp),
            "CmdGetFriendRecommendListInfoCsReq" => ::std::option::Option::Some(CmdFriendType::CmdGetFriendRecommendListInfoCsReq),
            "CmdGetFriendAssistListScRsp" => ::std::option::Option::Some(CmdFriendType::CmdGetFriendAssistListScRsp),
            "CmdGetFriendApplyListInfoScRsp" => ::std::option::Option::Some(CmdFriendType::CmdGetFriendApplyListInfoScRsp),
            "CmdSyncDeleteFriendScNotify" => ::std::option::Option::Some(CmdFriendType::CmdSyncDeleteFriendScNotify),
            "CmdReportPlayerScRsp" => ::std::option::Option::Some(CmdFriendType::CmdReportPlayerScRsp),
            "CmdGetCurAssistScRsp" => ::std::option::Option::Some(CmdFriendType::CmdGetCurAssistScRsp),
            "CmdGetFriendChallengeDetailCsReq" => ::std::option::Option::Some(CmdFriendType::CmdGetFriendChallengeDetailCsReq),
            "CmdGetFriendApplyListInfoCsReq" => ::std::option::Option::Some(CmdFriendType::CmdGetFriendApplyListInfoCsReq),
            "CmdDeleteFriendCsReq" => ::std::option::Option::Some(CmdFriendType::CmdDeleteFriendCsReq),
            "CmdGetAssistListScRsp" => ::std::option::Option::Some(CmdFriendType::CmdGetAssistListScRsp),
            "CmdDeleteBlacklistScRsp" => ::std::option::Option::Some(CmdFriendType::CmdDeleteBlacklistScRsp),
            "CmdGetCurAssistCsReq" => ::std::option::Option::Some(CmdFriendType::CmdGetCurAssistCsReq),
            "CmdGetFriendBattleRecordDetailCsReq" => ::std::option::Option::Some(CmdFriendType::CmdGetFriendBattleRecordDetailCsReq),
            "CmdSetForbidOtherApplyFriendCsReq" => ::std::option::Option::Some(CmdFriendType::CmdSetForbidOtherApplyFriendCsReq),
            "CmdSetForbidOtherApplyFriendScRsp" => ::std::option::Option::Some(CmdFriendType::CmdSetForbidOtherApplyFriendScRsp),
            "CmdSetAssistScRsp" => ::std::option::Option::Some(CmdFriendType::CmdSetAssistScRsp),
            "CmdSetAssistCsReq" => ::std::option::Option::Some(CmdFriendType::CmdSetAssistCsReq),
            "CmdDeleteBlacklistCsReq" => ::std::option::Option::Some(CmdFriendType::CmdDeleteBlacklistCsReq),
            "CmdGetFriendDevelopmentInfoScRsp" => ::std::option::Option::Some(CmdFriendType::CmdGetFriendDevelopmentInfoScRsp),
            "CmdGetFriendChallengeLineupCsReq" => ::std::option::Option::Some(CmdFriendType::CmdGetFriendChallengeLineupCsReq),
            "CmdGetFriendChallengeDetailScRsp" => ::std::option::Option::Some(CmdFriendType::CmdGetFriendChallengeDetailScRsp),
            "CmdGetFriendListInfoCsReq" => ::std::option::Option::Some(CmdFriendType::CmdGetFriendListInfoCsReq),
            "CmdCurAssistChangedNotify" => ::std::option::Option::Some(CmdFriendType::CmdCurAssistChangedNotify),
            "CmdGetPlayerDetailInfoCsReq" => ::std::option::Option::Some(CmdFriendType::CmdGetPlayerDetailInfoCsReq),
            "CmdGetFriendChallengeLineupScRsp" => ::std::option::Option::Some(CmdFriendType::CmdGetFriendChallengeLineupScRsp),
            "CmdHandleFriendScRsp" => ::std::option::Option::Some(CmdFriendType::CmdHandleFriendScRsp),
            "CmdGetFriendRecommendListInfoScRsp" => ::std::option::Option::Some(CmdFriendType::CmdGetFriendRecommendListInfoScRsp),
            "CmdGetPlatformPlayerInfoCsReq" => ::std::option::Option::Some(CmdFriendType::CmdGetPlatformPlayerInfoCsReq),
            "CmdTakeAssistRewardCsReq" => ::std::option::Option::Some(CmdFriendType::CmdTakeAssistRewardCsReq),
            "CmdGetFriendDevelopmentInfoCsReq" => ::std::option::Option::Some(CmdFriendType::CmdGetFriendDevelopmentInfoCsReq),
            "CmdSyncAddBlacklistScNotify" => ::std::option::Option::Some(CmdFriendType::CmdSyncAddBlacklistScNotify),
            "CmdHandleFriendCsReq" => ::std::option::Option::Some(CmdFriendType::CmdHandleFriendCsReq),
            "CmdSetFriendRemarkNameCsReq" => ::std::option::Option::Some(CmdFriendType::CmdSetFriendRemarkNameCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdFriendType] = &[
        CmdFriendType::CmdFriendTypeNone,
        CmdFriendType::CmdNewAssistHistoryNotify,
        CmdFriendType::CmdAddBlacklistCsReq,
        CmdFriendType::CmdSyncApplyFriendScNotify,
        CmdFriendType::CmdSetFriendMarkCsReq,
        CmdFriendType::CmdGetFriendListInfoScRsp,
        CmdFriendType::CmdGetPlatformPlayerInfoScRsp,
        CmdFriendType::CmdGetAssistHistoryCsReq,
        CmdFriendType::CmdGetFriendAssistListCsReq,
        CmdFriendType::CmdTakeAssistRewardScRsp,
        CmdFriendType::CmdGetFriendLoginInfoCsReq,
        CmdFriendType::CmdGetFriendLoginInfoScRsp,
        CmdFriendType::CmdGetPlayerDetailInfoScRsp,
        CmdFriendType::CmdApplyFriendCsReq,
        CmdFriendType::CmdApplyFriendScRsp,
        CmdFriendType::CmdGetAssistListCsReq,
        CmdFriendType::CmdSearchPlayerCsReq,
        CmdFriendType::CmdGetAssistHistoryScRsp,
        CmdFriendType::CmdReportPlayerCsReq,
        CmdFriendType::CmdSetFriendMarkScRsp,
        CmdFriendType::CmdGetFriendBattleRecordDetailScRsp,
        CmdFriendType::CmdSetFriendRemarkNameScRsp,
        CmdFriendType::CmdDeleteFriendScRsp,
        CmdFriendType::CmdSyncHandleFriendScNotify,
        CmdFriendType::CmdSearchPlayerScRsp,
        CmdFriendType::CmdAddBlacklistScRsp,
        CmdFriendType::CmdGetFriendRecommendListInfoCsReq,
        CmdFriendType::CmdGetFriendAssistListScRsp,
        CmdFriendType::CmdGetFriendApplyListInfoScRsp,
        CmdFriendType::CmdSyncDeleteFriendScNotify,
        CmdFriendType::CmdReportPlayerScRsp,
        CmdFriendType::CmdGetCurAssistScRsp,
        CmdFriendType::CmdGetFriendChallengeDetailCsReq,
        CmdFriendType::CmdGetFriendApplyListInfoCsReq,
        CmdFriendType::CmdDeleteFriendCsReq,
        CmdFriendType::CmdGetAssistListScRsp,
        CmdFriendType::CmdDeleteBlacklistScRsp,
        CmdFriendType::CmdGetCurAssistCsReq,
        CmdFriendType::CmdGetFriendBattleRecordDetailCsReq,
        CmdFriendType::CmdSetForbidOtherApplyFriendCsReq,
        CmdFriendType::CmdSetForbidOtherApplyFriendScRsp,
        CmdFriendType::CmdSetAssistScRsp,
        CmdFriendType::CmdSetAssistCsReq,
        CmdFriendType::CmdDeleteBlacklistCsReq,
        CmdFriendType::CmdGetFriendDevelopmentInfoScRsp,
        CmdFriendType::CmdGetFriendChallengeLineupCsReq,
        CmdFriendType::CmdGetFriendChallengeDetailScRsp,
        CmdFriendType::CmdGetFriendListInfoCsReq,
        CmdFriendType::CmdCurAssistChangedNotify,
        CmdFriendType::CmdGetPlayerDetailInfoCsReq,
        CmdFriendType::CmdGetFriendChallengeLineupScRsp,
        CmdFriendType::CmdHandleFriendScRsp,
        CmdFriendType::CmdGetFriendRecommendListInfoScRsp,
        CmdFriendType::CmdGetPlatformPlayerInfoCsReq,
        CmdFriendType::CmdTakeAssistRewardCsReq,
        CmdFriendType::CmdGetFriendDevelopmentInfoCsReq,
        CmdFriendType::CmdSyncAddBlacklistScNotify,
        CmdFriendType::CmdHandleFriendCsReq,
        CmdFriendType::CmdSetFriendRemarkNameCsReq,
    ];
}

impl ::protobuf::EnumFull for CmdFriendType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdFriendType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdFriendType::CmdFriendTypeNone => 0,
            CmdFriendType::CmdNewAssistHistoryNotify => 1,
            CmdFriendType::CmdAddBlacklistCsReq => 2,
            CmdFriendType::CmdSyncApplyFriendScNotify => 3,
            CmdFriendType::CmdSetFriendMarkCsReq => 4,
            CmdFriendType::CmdGetFriendListInfoScRsp => 5,
            CmdFriendType::CmdGetPlatformPlayerInfoScRsp => 6,
            CmdFriendType::CmdGetAssistHistoryCsReq => 7,
            CmdFriendType::CmdGetFriendAssistListCsReq => 8,
            CmdFriendType::CmdTakeAssistRewardScRsp => 9,
            CmdFriendType::CmdGetFriendLoginInfoCsReq => 10,
            CmdFriendType::CmdGetFriendLoginInfoScRsp => 11,
            CmdFriendType::CmdGetPlayerDetailInfoScRsp => 12,
            CmdFriendType::CmdApplyFriendCsReq => 13,
            CmdFriendType::CmdApplyFriendScRsp => 14,
            CmdFriendType::CmdGetAssistListCsReq => 15,
            CmdFriendType::CmdSearchPlayerCsReq => 16,
            CmdFriendType::CmdGetAssistHistoryScRsp => 17,
            CmdFriendType::CmdReportPlayerCsReq => 18,
            CmdFriendType::CmdSetFriendMarkScRsp => 19,
            CmdFriendType::CmdGetFriendBattleRecordDetailScRsp => 20,
            CmdFriendType::CmdSetFriendRemarkNameScRsp => 21,
            CmdFriendType::CmdDeleteFriendScRsp => 22,
            CmdFriendType::CmdSyncHandleFriendScNotify => 23,
            CmdFriendType::CmdSearchPlayerScRsp => 24,
            CmdFriendType::CmdAddBlacklistScRsp => 25,
            CmdFriendType::CmdGetFriendRecommendListInfoCsReq => 26,
            CmdFriendType::CmdGetFriendAssistListScRsp => 27,
            CmdFriendType::CmdGetFriendApplyListInfoScRsp => 28,
            CmdFriendType::CmdSyncDeleteFriendScNotify => 29,
            CmdFriendType::CmdReportPlayerScRsp => 30,
            CmdFriendType::CmdGetCurAssistScRsp => 31,
            CmdFriendType::CmdGetFriendChallengeDetailCsReq => 32,
            CmdFriendType::CmdGetFriendApplyListInfoCsReq => 33,
            CmdFriendType::CmdDeleteFriendCsReq => 34,
            CmdFriendType::CmdGetAssistListScRsp => 35,
            CmdFriendType::CmdDeleteBlacklistScRsp => 36,
            CmdFriendType::CmdGetCurAssistCsReq => 37,
            CmdFriendType::CmdGetFriendBattleRecordDetailCsReq => 38,
            CmdFriendType::CmdSetForbidOtherApplyFriendCsReq => 39,
            CmdFriendType::CmdSetForbidOtherApplyFriendScRsp => 40,
            CmdFriendType::CmdSetAssistScRsp => 41,
            CmdFriendType::CmdSetAssistCsReq => 42,
            CmdFriendType::CmdDeleteBlacklistCsReq => 43,
            CmdFriendType::CmdGetFriendDevelopmentInfoScRsp => 44,
            CmdFriendType::CmdGetFriendChallengeLineupCsReq => 45,
            CmdFriendType::CmdGetFriendChallengeDetailScRsp => 46,
            CmdFriendType::CmdGetFriendListInfoCsReq => 47,
            CmdFriendType::CmdCurAssistChangedNotify => 48,
            CmdFriendType::CmdGetPlayerDetailInfoCsReq => 49,
            CmdFriendType::CmdGetFriendChallengeLineupScRsp => 50,
            CmdFriendType::CmdHandleFriendScRsp => 51,
            CmdFriendType::CmdGetFriendRecommendListInfoScRsp => 52,
            CmdFriendType::CmdGetPlatformPlayerInfoCsReq => 53,
            CmdFriendType::CmdTakeAssistRewardCsReq => 54,
            CmdFriendType::CmdGetFriendDevelopmentInfoCsReq => 55,
            CmdFriendType::CmdSyncAddBlacklistScNotify => 56,
            CmdFriendType::CmdHandleFriendCsReq => 57,
            CmdFriendType::CmdSetFriendRemarkNameCsReq => 58,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdFriendType {
    fn default() -> Self {
        CmdFriendType::CmdFriendTypeNone
    }
}

impl CmdFriendType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdFriendType>("CmdFriendType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13CmdFriendType.proto*\xfd\x0e\n\rCmdFriendType\x12\x15\n\x11CmdFrie\
    ndTypeNone\x10\0\x12\x1e\n\x19CmdNewAssistHistoryNotify\x10\xa7\x17\x12\
    \x19\n\x14CmdAddBlacklistCsReq\x10\xa1\x17\x12\x1f\n\x1aCmdSyncApplyFrie\
    ndScNotify\x10\xee\x16\x12\x1a\n\x15CmdSetFriendMarkCsReq\x10\xe2\x16\
    \x12\x1e\n\x19CmdGetFriendListInfoScRsp\x10\xe1\x16\x12\"\n\x1dCmdGetPla\
    tformPlayerInfoScRsp\x10\xab\x17\x12\x1d\n\x18CmdGetAssistHistoryCsReq\
    \x10\x9f\x17\x12\x20\n\x1bCmdGetFriendAssistListCsReq\x10\xe5\x16\x12\
    \x1d\n\x18CmdTakeAssistRewardScRsp\x10\x98\x17\x12\x1f\n\x1aCmdGetFriend\
    LoginInfoCsReq\x10\x8c\x17\x12\x1f\n\x1aCmdGetFriendLoginInfoScRsp\x10\
    \xa5\x17\x12\x20\n\x1bCmdGetPlayerDetailInfoScRsp\x10\xdd\x16\x12\x18\n\
    \x13CmdApplyFriendCsReq\x10\x9a\x17\x12\x18\n\x13CmdApplyFriendScRsp\x10\
    \xad\x17\x12\x1a\n\x15CmdGetAssistListCsReq\x10\xea\x16\x12\x19\n\x14Cmd\
    SearchPlayerCsReq\x10\x95\x17\x12\x1d\n\x18CmdGetAssistHistoryScRsp\x10\
    \xf0\x16\x12\x19\n\x14CmdReportPlayerCsReq\x10\x9b\x17\x12\x1a\n\x15CmdS\
    etFriendMarkScRsp\x10\xfd\x16\x12(\n#CmdGetFriendBattleRecordDetailScRsp\
    \x10\xb7\x17\x12\x20\n\x1bCmdSetFriendRemarkNameScRsp\x10\xdb\x16\x12\
    \x19\n\x14CmdDeleteFriendScRsp\x10\x86\x17\x12\x20\n\x1bCmdSyncHandleFri\
    endScNotify\x10\xe6\x16\x12\x19\n\x14CmdSearchPlayerScRsp\x10\x88\x17\
    \x12\x19\n\x14CmdAddBlacklistScRsp\x10\xaf\x17\x12'\n\"CmdGetFriendRecom\
    mendListInfoCsReq\x10\x8d\x17\x12\x20\n\x1bCmdGetFriendAssistListScRsp\
    \x10\x93\x17\x12#\n\x1eCmdGetFriendApplyListInfoScRsp\x10\xda\x16\x12\
    \x20\n\x1bCmdSyncDeleteFriendScNotify\x10\x9d\x17\x12\x19\n\x14CmdReport\
    PlayerScRsp\x10\xa6\x17\x12\x19\n\x14CmdGetCurAssistScRsp\x10\xa8\x17\
    \x12%\n\x20CmdGetFriendChallengeDetailCsReq\x10\xb4\x17\x12#\n\x1eCmdGet\
    FriendApplyListInfoCsReq\x10\xf7\x16\x12\x19\n\x14CmdDeleteFriendCsReq\
    \x10\xf8\x16\x12\x1a\n\x15CmdGetAssistListScRsp\x10\xaa\x17\x12\x1c\n\
    \x17CmdDeleteBlacklistScRsp\x10\xf9\x16\x12\x19\n\x14CmdGetCurAssistCsRe\
    q\x10\xec\x16\x12(\n#CmdGetFriendBattleRecordDetailCsReq\x10\x96\x17\x12\
    &\n!CmdSetForbidOtherApplyFriendCsReq\x10\xb8\x17\x12&\n!CmdSetForbidOth\
    erApplyFriendScRsp\x10\x82\x17\x12\x16\n\x11CmdSetAssistScRsp\x10\x89\
    \x17\x12\x16\n\x11CmdSetAssistCsReq\x10\xb0\x17\x12\x1c\n\x17CmdDeleteBl\
    acklistCsReq\x10\x87\x17\x12%\n\x20CmdGetFriendDevelopmentInfoScRsp\x10\
    \xdc\x16\x12%\n\x20CmdGetFriendChallengeLineupCsReq\x10\xd8\x16\x12%\n\
    \x20CmdGetFriendChallengeDetailScRsp\x10\x99\x17\x12\x1e\n\x19CmdGetFrie\
    ndListInfoCsReq\x10\xdf\x16\x12\x1e\n\x19CmdCurAssistChangedNotify\x10\
    \x90\x17\x12\x20\n\x1bCmdGetPlayerDetailInfoCsReq\x10\x83\x17\x12%\n\x20\
    CmdGetFriendChallengeLineupScRsp\x10\xa2\x17\x12\x19\n\x14CmdHandleFrien\
    dScRsp\x10\xb3\x17\x12'\n\"CmdGetFriendRecommendListInfoScRsp\x10\xed\
    \x16\x12\"\n\x1dCmdGetPlatformPlayerInfoCsReq\x10\xb2\x17\x12\x1d\n\x18C\
    mdTakeAssistRewardCsReq\x10\x8e\x17\x12%\n\x20CmdGetFriendDevelopmentInf\
    oCsReq\x10\xe9\x16\x12\x20\n\x1bCmdSyncAddBlacklistScNotify\x10\xb1\x17\
    \x12\x19\n\x14CmdHandleFriendCsReq\x10\xf2\x16\x12\x20\n\x1bCmdSetFriend\
    RemarkNameCsReq\x10\xde\x16b\x06proto3\
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
            enums.push(CmdFriendType::generated_enum_descriptor_data());
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
