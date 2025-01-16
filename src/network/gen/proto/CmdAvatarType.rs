// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `CmdAvatarType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdAvatarType)
pub enum CmdAvatarType {
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdAvatarTypeNone)
    CmdAvatarTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdDressRelicAvatarScRsp)
    CmdDressRelicAvatarScRsp = 375,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdTakePromotionRewardCsReq)
    CmdTakePromotionRewardCsReq = 307,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdGrowthTargetAvatarChangedScNotify)
    CmdGrowthTargetAvatarChangedScNotify = 376,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdTakeOffEquipmentCsReq)
    CmdTakeOffEquipmentCsReq = 334,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdRankUpAvatarScRsp)
    CmdRankUpAvatarScRsp = 396,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdTakePromotionRewardScRsp)
    CmdTakePromotionRewardScRsp = 353,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdRankUpAvatarCsReq)
    CmdRankUpAvatarCsReq = 325,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdDressAvatarScRsp)
    CmdDressAvatarScRsp = 393,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdSetMarSeventhSkinHintFinishScRsp)
    CmdSetMarSeventhSkinHintFinishScRsp = 313,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdGetMarSeventhSkinHintStatusCsReq)
    CmdGetMarSeventhSkinHintStatusCsReq = 335,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdAddMultiPathAvatarScNotify)
    CmdAddMultiPathAvatarScNotify = 388,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdMarkAvatarScRsp)
    CmdMarkAvatarScRsp = 348,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdDressAvatarSkinCsReq)
    CmdDressAvatarSkinCsReq = 390,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdPromoteAvatarCsReq)
    CmdPromoteAvatarCsReq = 352,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdGetAvatarDataCsReq)
    CmdGetAvatarDataCsReq = 336,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdTakeOffEquipmentScRsp)
    CmdTakeOffEquipmentScRsp = 343,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdSetGrowthTargetAvatarScRsp)
    CmdSetGrowthTargetAvatarScRsp = 310,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdSetMarSeventhSkinHintFinishCsReq)
    CmdSetMarSeventhSkinHintFinishCsReq = 317,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdSetGrowthTargetAvatarCsReq)
    CmdSetGrowthTargetAvatarCsReq = 358,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdUnlockAvatarSkinScNotify)
    CmdUnlockAvatarSkinScNotify = 329,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdAvatarExpUpCsReq)
    CmdAvatarExpUpCsReq = 384,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdUnlockSkilltreeCsReq)
    CmdUnlockSkilltreeCsReq = 367,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdDressAvatarSkinScRsp)
    CmdDressAvatarSkinScRsp = 359,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdTakeOffAvatarSkinScRsp)
    CmdTakeOffAvatarSkinScRsp = 378,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdTakeOffAvatarSkinCsReq)
    CmdTakeOffAvatarSkinCsReq = 312,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdAvatarExpUpScRsp)
    CmdAvatarExpUpScRsp = 327,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdTakeOffRelicScRsp)
    CmdTakeOffRelicScRsp = 326,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdMarkAvatarCsReq)
    CmdMarkAvatarCsReq = 350,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdAddAvatarScNotify)
    CmdAddAvatarScNotify = 346,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdGetMarSeventhSkinHintStatusScRsp)
    CmdGetMarSeventhSkinHintStatusScRsp = 373,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdTakeOffRelicCsReq)
    CmdTakeOffRelicCsReq = 319,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdGetAvatarDataScRsp)
    CmdGetAvatarDataScRsp = 395,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdDressRelicAvatarCsReq)
    CmdDressRelicAvatarCsReq = 305,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdPromoteAvatarScRsp)
    CmdPromoteAvatarScRsp = 374,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdDressAvatarCsReq)
    CmdDressAvatarCsReq = 324,
    // @@protoc_insertion_point(enum_value:CmdAvatarType.CmdUnlockSkilltreeScRsp)
    CmdUnlockSkilltreeScRsp = 328,
}

impl ::protobuf::Enum for CmdAvatarType {
    const NAME: &'static str = "CmdAvatarType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdAvatarType> {
        match value {
            0 => ::std::option::Option::Some(CmdAvatarType::CmdAvatarTypeNone),
            375 => ::std::option::Option::Some(CmdAvatarType::CmdDressRelicAvatarScRsp),
            307 => ::std::option::Option::Some(CmdAvatarType::CmdTakePromotionRewardCsReq),
            376 => ::std::option::Option::Some(CmdAvatarType::CmdGrowthTargetAvatarChangedScNotify),
            334 => ::std::option::Option::Some(CmdAvatarType::CmdTakeOffEquipmentCsReq),
            396 => ::std::option::Option::Some(CmdAvatarType::CmdRankUpAvatarScRsp),
            353 => ::std::option::Option::Some(CmdAvatarType::CmdTakePromotionRewardScRsp),
            325 => ::std::option::Option::Some(CmdAvatarType::CmdRankUpAvatarCsReq),
            393 => ::std::option::Option::Some(CmdAvatarType::CmdDressAvatarScRsp),
            313 => ::std::option::Option::Some(CmdAvatarType::CmdSetMarSeventhSkinHintFinishScRsp),
            335 => ::std::option::Option::Some(CmdAvatarType::CmdGetMarSeventhSkinHintStatusCsReq),
            388 => ::std::option::Option::Some(CmdAvatarType::CmdAddMultiPathAvatarScNotify),
            348 => ::std::option::Option::Some(CmdAvatarType::CmdMarkAvatarScRsp),
            390 => ::std::option::Option::Some(CmdAvatarType::CmdDressAvatarSkinCsReq),
            352 => ::std::option::Option::Some(CmdAvatarType::CmdPromoteAvatarCsReq),
            336 => ::std::option::Option::Some(CmdAvatarType::CmdGetAvatarDataCsReq),
            343 => ::std::option::Option::Some(CmdAvatarType::CmdTakeOffEquipmentScRsp),
            310 => ::std::option::Option::Some(CmdAvatarType::CmdSetGrowthTargetAvatarScRsp),
            317 => ::std::option::Option::Some(CmdAvatarType::CmdSetMarSeventhSkinHintFinishCsReq),
            358 => ::std::option::Option::Some(CmdAvatarType::CmdSetGrowthTargetAvatarCsReq),
            329 => ::std::option::Option::Some(CmdAvatarType::CmdUnlockAvatarSkinScNotify),
            384 => ::std::option::Option::Some(CmdAvatarType::CmdAvatarExpUpCsReq),
            367 => ::std::option::Option::Some(CmdAvatarType::CmdUnlockSkilltreeCsReq),
            359 => ::std::option::Option::Some(CmdAvatarType::CmdDressAvatarSkinScRsp),
            378 => ::std::option::Option::Some(CmdAvatarType::CmdTakeOffAvatarSkinScRsp),
            312 => ::std::option::Option::Some(CmdAvatarType::CmdTakeOffAvatarSkinCsReq),
            327 => ::std::option::Option::Some(CmdAvatarType::CmdAvatarExpUpScRsp),
            326 => ::std::option::Option::Some(CmdAvatarType::CmdTakeOffRelicScRsp),
            350 => ::std::option::Option::Some(CmdAvatarType::CmdMarkAvatarCsReq),
            346 => ::std::option::Option::Some(CmdAvatarType::CmdAddAvatarScNotify),
            373 => ::std::option::Option::Some(CmdAvatarType::CmdGetMarSeventhSkinHintStatusScRsp),
            319 => ::std::option::Option::Some(CmdAvatarType::CmdTakeOffRelicCsReq),
            395 => ::std::option::Option::Some(CmdAvatarType::CmdGetAvatarDataScRsp),
            305 => ::std::option::Option::Some(CmdAvatarType::CmdDressRelicAvatarCsReq),
            374 => ::std::option::Option::Some(CmdAvatarType::CmdPromoteAvatarScRsp),
            324 => ::std::option::Option::Some(CmdAvatarType::CmdDressAvatarCsReq),
            328 => ::std::option::Option::Some(CmdAvatarType::CmdUnlockSkilltreeScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdAvatarType> {
        match str {
            "CmdAvatarTypeNone" => ::std::option::Option::Some(CmdAvatarType::CmdAvatarTypeNone),
            "CmdDressRelicAvatarScRsp" => ::std::option::Option::Some(CmdAvatarType::CmdDressRelicAvatarScRsp),
            "CmdTakePromotionRewardCsReq" => ::std::option::Option::Some(CmdAvatarType::CmdTakePromotionRewardCsReq),
            "CmdGrowthTargetAvatarChangedScNotify" => ::std::option::Option::Some(CmdAvatarType::CmdGrowthTargetAvatarChangedScNotify),
            "CmdTakeOffEquipmentCsReq" => ::std::option::Option::Some(CmdAvatarType::CmdTakeOffEquipmentCsReq),
            "CmdRankUpAvatarScRsp" => ::std::option::Option::Some(CmdAvatarType::CmdRankUpAvatarScRsp),
            "CmdTakePromotionRewardScRsp" => ::std::option::Option::Some(CmdAvatarType::CmdTakePromotionRewardScRsp),
            "CmdRankUpAvatarCsReq" => ::std::option::Option::Some(CmdAvatarType::CmdRankUpAvatarCsReq),
            "CmdDressAvatarScRsp" => ::std::option::Option::Some(CmdAvatarType::CmdDressAvatarScRsp),
            "CmdSetMarSeventhSkinHintFinishScRsp" => ::std::option::Option::Some(CmdAvatarType::CmdSetMarSeventhSkinHintFinishScRsp),
            "CmdGetMarSeventhSkinHintStatusCsReq" => ::std::option::Option::Some(CmdAvatarType::CmdGetMarSeventhSkinHintStatusCsReq),
            "CmdAddMultiPathAvatarScNotify" => ::std::option::Option::Some(CmdAvatarType::CmdAddMultiPathAvatarScNotify),
            "CmdMarkAvatarScRsp" => ::std::option::Option::Some(CmdAvatarType::CmdMarkAvatarScRsp),
            "CmdDressAvatarSkinCsReq" => ::std::option::Option::Some(CmdAvatarType::CmdDressAvatarSkinCsReq),
            "CmdPromoteAvatarCsReq" => ::std::option::Option::Some(CmdAvatarType::CmdPromoteAvatarCsReq),
            "CmdGetAvatarDataCsReq" => ::std::option::Option::Some(CmdAvatarType::CmdGetAvatarDataCsReq),
            "CmdTakeOffEquipmentScRsp" => ::std::option::Option::Some(CmdAvatarType::CmdTakeOffEquipmentScRsp),
            "CmdSetGrowthTargetAvatarScRsp" => ::std::option::Option::Some(CmdAvatarType::CmdSetGrowthTargetAvatarScRsp),
            "CmdSetMarSeventhSkinHintFinishCsReq" => ::std::option::Option::Some(CmdAvatarType::CmdSetMarSeventhSkinHintFinishCsReq),
            "CmdSetGrowthTargetAvatarCsReq" => ::std::option::Option::Some(CmdAvatarType::CmdSetGrowthTargetAvatarCsReq),
            "CmdUnlockAvatarSkinScNotify" => ::std::option::Option::Some(CmdAvatarType::CmdUnlockAvatarSkinScNotify),
            "CmdAvatarExpUpCsReq" => ::std::option::Option::Some(CmdAvatarType::CmdAvatarExpUpCsReq),
            "CmdUnlockSkilltreeCsReq" => ::std::option::Option::Some(CmdAvatarType::CmdUnlockSkilltreeCsReq),
            "CmdDressAvatarSkinScRsp" => ::std::option::Option::Some(CmdAvatarType::CmdDressAvatarSkinScRsp),
            "CmdTakeOffAvatarSkinScRsp" => ::std::option::Option::Some(CmdAvatarType::CmdTakeOffAvatarSkinScRsp),
            "CmdTakeOffAvatarSkinCsReq" => ::std::option::Option::Some(CmdAvatarType::CmdTakeOffAvatarSkinCsReq),
            "CmdAvatarExpUpScRsp" => ::std::option::Option::Some(CmdAvatarType::CmdAvatarExpUpScRsp),
            "CmdTakeOffRelicScRsp" => ::std::option::Option::Some(CmdAvatarType::CmdTakeOffRelicScRsp),
            "CmdMarkAvatarCsReq" => ::std::option::Option::Some(CmdAvatarType::CmdMarkAvatarCsReq),
            "CmdAddAvatarScNotify" => ::std::option::Option::Some(CmdAvatarType::CmdAddAvatarScNotify),
            "CmdGetMarSeventhSkinHintStatusScRsp" => ::std::option::Option::Some(CmdAvatarType::CmdGetMarSeventhSkinHintStatusScRsp),
            "CmdTakeOffRelicCsReq" => ::std::option::Option::Some(CmdAvatarType::CmdTakeOffRelicCsReq),
            "CmdGetAvatarDataScRsp" => ::std::option::Option::Some(CmdAvatarType::CmdGetAvatarDataScRsp),
            "CmdDressRelicAvatarCsReq" => ::std::option::Option::Some(CmdAvatarType::CmdDressRelicAvatarCsReq),
            "CmdPromoteAvatarScRsp" => ::std::option::Option::Some(CmdAvatarType::CmdPromoteAvatarScRsp),
            "CmdDressAvatarCsReq" => ::std::option::Option::Some(CmdAvatarType::CmdDressAvatarCsReq),
            "CmdUnlockSkilltreeScRsp" => ::std::option::Option::Some(CmdAvatarType::CmdUnlockSkilltreeScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdAvatarType] = &[
        CmdAvatarType::CmdAvatarTypeNone,
        CmdAvatarType::CmdDressRelicAvatarScRsp,
        CmdAvatarType::CmdTakePromotionRewardCsReq,
        CmdAvatarType::CmdGrowthTargetAvatarChangedScNotify,
        CmdAvatarType::CmdTakeOffEquipmentCsReq,
        CmdAvatarType::CmdRankUpAvatarScRsp,
        CmdAvatarType::CmdTakePromotionRewardScRsp,
        CmdAvatarType::CmdRankUpAvatarCsReq,
        CmdAvatarType::CmdDressAvatarScRsp,
        CmdAvatarType::CmdSetMarSeventhSkinHintFinishScRsp,
        CmdAvatarType::CmdGetMarSeventhSkinHintStatusCsReq,
        CmdAvatarType::CmdAddMultiPathAvatarScNotify,
        CmdAvatarType::CmdMarkAvatarScRsp,
        CmdAvatarType::CmdDressAvatarSkinCsReq,
        CmdAvatarType::CmdPromoteAvatarCsReq,
        CmdAvatarType::CmdGetAvatarDataCsReq,
        CmdAvatarType::CmdTakeOffEquipmentScRsp,
        CmdAvatarType::CmdSetGrowthTargetAvatarScRsp,
        CmdAvatarType::CmdSetMarSeventhSkinHintFinishCsReq,
        CmdAvatarType::CmdSetGrowthTargetAvatarCsReq,
        CmdAvatarType::CmdUnlockAvatarSkinScNotify,
        CmdAvatarType::CmdAvatarExpUpCsReq,
        CmdAvatarType::CmdUnlockSkilltreeCsReq,
        CmdAvatarType::CmdDressAvatarSkinScRsp,
        CmdAvatarType::CmdTakeOffAvatarSkinScRsp,
        CmdAvatarType::CmdTakeOffAvatarSkinCsReq,
        CmdAvatarType::CmdAvatarExpUpScRsp,
        CmdAvatarType::CmdTakeOffRelicScRsp,
        CmdAvatarType::CmdMarkAvatarCsReq,
        CmdAvatarType::CmdAddAvatarScNotify,
        CmdAvatarType::CmdGetMarSeventhSkinHintStatusScRsp,
        CmdAvatarType::CmdTakeOffRelicCsReq,
        CmdAvatarType::CmdGetAvatarDataScRsp,
        CmdAvatarType::CmdDressRelicAvatarCsReq,
        CmdAvatarType::CmdPromoteAvatarScRsp,
        CmdAvatarType::CmdDressAvatarCsReq,
        CmdAvatarType::CmdUnlockSkilltreeScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdAvatarType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdAvatarType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdAvatarType::CmdAvatarTypeNone => 0,
            CmdAvatarType::CmdDressRelicAvatarScRsp => 1,
            CmdAvatarType::CmdTakePromotionRewardCsReq => 2,
            CmdAvatarType::CmdGrowthTargetAvatarChangedScNotify => 3,
            CmdAvatarType::CmdTakeOffEquipmentCsReq => 4,
            CmdAvatarType::CmdRankUpAvatarScRsp => 5,
            CmdAvatarType::CmdTakePromotionRewardScRsp => 6,
            CmdAvatarType::CmdRankUpAvatarCsReq => 7,
            CmdAvatarType::CmdDressAvatarScRsp => 8,
            CmdAvatarType::CmdSetMarSeventhSkinHintFinishScRsp => 9,
            CmdAvatarType::CmdGetMarSeventhSkinHintStatusCsReq => 10,
            CmdAvatarType::CmdAddMultiPathAvatarScNotify => 11,
            CmdAvatarType::CmdMarkAvatarScRsp => 12,
            CmdAvatarType::CmdDressAvatarSkinCsReq => 13,
            CmdAvatarType::CmdPromoteAvatarCsReq => 14,
            CmdAvatarType::CmdGetAvatarDataCsReq => 15,
            CmdAvatarType::CmdTakeOffEquipmentScRsp => 16,
            CmdAvatarType::CmdSetGrowthTargetAvatarScRsp => 17,
            CmdAvatarType::CmdSetMarSeventhSkinHintFinishCsReq => 18,
            CmdAvatarType::CmdSetGrowthTargetAvatarCsReq => 19,
            CmdAvatarType::CmdUnlockAvatarSkinScNotify => 20,
            CmdAvatarType::CmdAvatarExpUpCsReq => 21,
            CmdAvatarType::CmdUnlockSkilltreeCsReq => 22,
            CmdAvatarType::CmdDressAvatarSkinScRsp => 23,
            CmdAvatarType::CmdTakeOffAvatarSkinScRsp => 24,
            CmdAvatarType::CmdTakeOffAvatarSkinCsReq => 25,
            CmdAvatarType::CmdAvatarExpUpScRsp => 26,
            CmdAvatarType::CmdTakeOffRelicScRsp => 27,
            CmdAvatarType::CmdMarkAvatarCsReq => 28,
            CmdAvatarType::CmdAddAvatarScNotify => 29,
            CmdAvatarType::CmdGetMarSeventhSkinHintStatusScRsp => 30,
            CmdAvatarType::CmdTakeOffRelicCsReq => 31,
            CmdAvatarType::CmdGetAvatarDataScRsp => 32,
            CmdAvatarType::CmdDressRelicAvatarCsReq => 33,
            CmdAvatarType::CmdPromoteAvatarScRsp => 34,
            CmdAvatarType::CmdDressAvatarCsReq => 35,
            CmdAvatarType::CmdUnlockSkilltreeScRsp => 36,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdAvatarType {
    fn default() -> Self {
        CmdAvatarType::CmdAvatarTypeNone
    }
}

impl CmdAvatarType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdAvatarType>("CmdAvatarType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13CmdAvatarType.proto*\x90\t\n\rCmdAvatarType\x12\x15\n\x11CmdAvatar\
    TypeNone\x10\0\x12\x1d\n\x18CmdDressRelicAvatarScRsp\x10\xf7\x02\x12\x20\
    \n\x1bCmdTakePromotionRewardCsReq\x10\xb3\x02\x12)\n$CmdGrowthTargetAvat\
    arChangedScNotify\x10\xf8\x02\x12\x1d\n\x18CmdTakeOffEquipmentCsReq\x10\
    \xce\x02\x12\x19\n\x14CmdRankUpAvatarScRsp\x10\x8c\x03\x12\x20\n\x1bCmdT\
    akePromotionRewardScRsp\x10\xe1\x02\x12\x19\n\x14CmdRankUpAvatarCsReq\
    \x10\xc5\x02\x12\x18\n\x13CmdDressAvatarScRsp\x10\x89\x03\x12(\n#CmdSetM\
    arSeventhSkinHintFinishScRsp\x10\xb9\x02\x12(\n#CmdGetMarSeventhSkinHint\
    StatusCsReq\x10\xcf\x02\x12\"\n\x1dCmdAddMultiPathAvatarScNotify\x10\x84\
    \x03\x12\x17\n\x12CmdMarkAvatarScRsp\x10\xdc\x02\x12\x1c\n\x17CmdDressAv\
    atarSkinCsReq\x10\x86\x03\x12\x1a\n\x15CmdPromoteAvatarCsReq\x10\xe0\x02\
    \x12\x1a\n\x15CmdGetAvatarDataCsReq\x10\xd0\x02\x12\x1d\n\x18CmdTakeOffE\
    quipmentScRsp\x10\xd7\x02\x12\"\n\x1dCmdSetGrowthTargetAvatarScRsp\x10\
    \xb6\x02\x12(\n#CmdSetMarSeventhSkinHintFinishCsReq\x10\xbd\x02\x12\"\n\
    \x1dCmdSetGrowthTargetAvatarCsReq\x10\xe6\x02\x12\x20\n\x1bCmdUnlockAvat\
    arSkinScNotify\x10\xc9\x02\x12\x18\n\x13CmdAvatarExpUpCsReq\x10\x80\x03\
    \x12\x1c\n\x17CmdUnlockSkilltreeCsReq\x10\xef\x02\x12\x1c\n\x17CmdDressA\
    vatarSkinScRsp\x10\xe7\x02\x12\x1e\n\x19CmdTakeOffAvatarSkinScRsp\x10\
    \xfa\x02\x12\x1e\n\x19CmdTakeOffAvatarSkinCsReq\x10\xb8\x02\x12\x18\n\
    \x13CmdAvatarExpUpScRsp\x10\xc7\x02\x12\x19\n\x14CmdTakeOffRelicScRsp\
    \x10\xc6\x02\x12\x17\n\x12CmdMarkAvatarCsReq\x10\xde\x02\x12\x19\n\x14Cm\
    dAddAvatarScNotify\x10\xda\x02\x12(\n#CmdGetMarSeventhSkinHintStatusScRs\
    p\x10\xf5\x02\x12\x19\n\x14CmdTakeOffRelicCsReq\x10\xbf\x02\x12\x1a\n\
    \x15CmdGetAvatarDataScRsp\x10\x8b\x03\x12\x1d\n\x18CmdDressRelicAvatarCs\
    Req\x10\xb1\x02\x12\x1a\n\x15CmdPromoteAvatarScRsp\x10\xf6\x02\x12\x18\n\
    \x13CmdDressAvatarCsReq\x10\xc4\x02\x12\x1c\n\x17CmdUnlockSkilltreeScRsp\
    \x10\xc8\x02b\x06proto3\
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
            enums.push(CmdAvatarType::generated_enum_descriptor_data());
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
