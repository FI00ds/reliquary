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

//! Generated file from `CmdTrainPartyType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdTrainPartyType)
pub enum CmdTrainPartyType {
    // @@protoc_insertion_point(enum_value:CmdTrainPartyType.CmdTrainPartyTypeNone)
    CmdTrainPartyTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdTrainPartyType.CmdTrainPartyLeaveCsReq)
    CmdTrainPartyLeaveCsReq = 8010,
    // @@protoc_insertion_point(enum_value:CmdTrainPartyType.CmdTrainPartyBuildDiyScRsp)
    CmdTrainPartyBuildDiyScRsp = 8077,
    // @@protoc_insertion_point(enum_value:CmdTrainPartyType.CmdTrainPartyGamePlayStartScRsp)
    CmdTrainPartyGamePlayStartScRsp = 8052,
    // @@protoc_insertion_point(enum_value:CmdTrainPartyType.CmdTrainPartyAddBuildDynamicBuffScRsp)
    CmdTrainPartyAddBuildDynamicBuffScRsp = 8028,
    // @@protoc_insertion_point(enum_value:CmdTrainPartyType.CmdTrainPartyGamePlaySettleNotify)
    CmdTrainPartyGamePlaySettleNotify = 8022,
    // @@protoc_insertion_point(enum_value:CmdTrainPartyType.CmdTrainPartyBuildDiyCsReq)
    CmdTrainPartyBuildDiyCsReq = 8073,
    // @@protoc_insertion_point(enum_value:CmdTrainPartyType.CmdTrainPartyGamePlayStartCsReq)
    CmdTrainPartyGamePlayStartCsReq = 8065,
    // @@protoc_insertion_point(enum_value:CmdTrainPartyType.CmdTrainPartyUpdatePosEnvScRsp)
    CmdTrainPartyUpdatePosEnvScRsp = 8053,
    // @@protoc_insertion_point(enum_value:CmdTrainPartyType.CmdTrainPartyAddBuildDynamicBuffCsReq)
    CmdTrainPartyAddBuildDynamicBuffCsReq = 8075,
    // @@protoc_insertion_point(enum_value:CmdTrainPartyType.CmdTrainPartyGetDataCsReq)
    CmdTrainPartyGetDataCsReq = 8011,
    // @@protoc_insertion_point(enum_value:CmdTrainPartyType.CmdTrainPartyMoveScNotify)
    CmdTrainPartyMoveScNotify = 8035,
    // @@protoc_insertion_point(enum_value:CmdTrainPartyType.CmdTrainPartyEnterScRsp)
    CmdTrainPartyEnterScRsp = 8025,
    // @@protoc_insertion_point(enum_value:CmdTrainPartyType.CmdTrainPartyEnterCsReq)
    CmdTrainPartyEnterCsReq = 8057,
    // @@protoc_insertion_point(enum_value:CmdTrainPartyType.CmdTrainPartySettleNotify)
    CmdTrainPartySettleNotify = 8070,
    // @@protoc_insertion_point(enum_value:CmdTrainPartyType.CmdTrainPartyTakeBuildLevelAwardCsReq)
    CmdTrainPartyTakeBuildLevelAwardCsReq = 8024,
    // @@protoc_insertion_point(enum_value:CmdTrainPartyType.CmdTrainPartyBuildStartStepCsReq)
    CmdTrainPartyBuildStartStepCsReq = 8036,
    // @@protoc_insertion_point(enum_value:CmdTrainPartyType.CmdTrainPartyUseCardCsReq)
    CmdTrainPartyUseCardCsReq = 8047,
    // @@protoc_insertion_point(enum_value:CmdTrainPartyType.CmdTrainPartyGetDataScRsp)
    CmdTrainPartyGetDataScRsp = 8013,
    // @@protoc_insertion_point(enum_value:CmdTrainPartyType.CmdTrainPartyLeaveScRsp)
    CmdTrainPartyLeaveScRsp = 8007,
    // @@protoc_insertion_point(enum_value:CmdTrainPartyType.CmdTrainPartyTakeBuildLevelAwardScRsp)
    CmdTrainPartyTakeBuildLevelAwardScRsp = 8084,
    // @@protoc_insertion_point(enum_value:CmdTrainPartyType.CmdTrainPartyBuildStartStepScRsp)
    CmdTrainPartyBuildStartStepScRsp = 8050,
    // @@protoc_insertion_point(enum_value:CmdTrainPartyType.CmdTrainPartyUseCardScRsp)
    CmdTrainPartyUseCardScRsp = 8009,
    // @@protoc_insertion_point(enum_value:CmdTrainPartyType.CmdTrainPartyHandlePendingActionCsReq)
    CmdTrainPartyHandlePendingActionCsReq = 8095,
    // @@protoc_insertion_point(enum_value:CmdTrainPartyType.CmdTrainPartyBuildingUpdateNotify)
    CmdTrainPartyBuildingUpdateNotify = 8091,
    // @@protoc_insertion_point(enum_value:CmdTrainPartyType.CmdTrainPartyHandlePendingActionScRsp)
    CmdTrainPartyHandlePendingActionScRsp = 8018,
    // @@protoc_insertion_point(enum_value:CmdTrainPartyType.CmdTrainPartyUpdatePosEnvCsReq)
    CmdTrainPartyUpdatePosEnvCsReq = 8092,
    // @@protoc_insertion_point(enum_value:CmdTrainPartyType.CmdTrainPartySyncUpdateScNotify)
    CmdTrainPartySyncUpdateScNotify = 8026,
}

impl ::protobuf::Enum for CmdTrainPartyType {
    const NAME: &'static str = "CmdTrainPartyType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdTrainPartyType> {
        match value {
            0 => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyTypeNone),
            8010 => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyLeaveCsReq),
            8077 => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyBuildDiyScRsp),
            8052 => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyGamePlayStartScRsp),
            8028 => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyAddBuildDynamicBuffScRsp),
            8022 => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyGamePlaySettleNotify),
            8073 => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyBuildDiyCsReq),
            8065 => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyGamePlayStartCsReq),
            8053 => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyUpdatePosEnvScRsp),
            8075 => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyAddBuildDynamicBuffCsReq),
            8011 => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyGetDataCsReq),
            8035 => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyMoveScNotify),
            8025 => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyEnterScRsp),
            8057 => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyEnterCsReq),
            8070 => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartySettleNotify),
            8024 => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyTakeBuildLevelAwardCsReq),
            8036 => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyBuildStartStepCsReq),
            8047 => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyUseCardCsReq),
            8013 => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyGetDataScRsp),
            8007 => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyLeaveScRsp),
            8084 => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyTakeBuildLevelAwardScRsp),
            8050 => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyBuildStartStepScRsp),
            8009 => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyUseCardScRsp),
            8095 => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyHandlePendingActionCsReq),
            8091 => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyBuildingUpdateNotify),
            8018 => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyHandlePendingActionScRsp),
            8092 => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyUpdatePosEnvCsReq),
            8026 => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartySyncUpdateScNotify),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdTrainPartyType> {
        match str {
            "CmdTrainPartyTypeNone" => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyTypeNone),
            "CmdTrainPartyLeaveCsReq" => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyLeaveCsReq),
            "CmdTrainPartyBuildDiyScRsp" => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyBuildDiyScRsp),
            "CmdTrainPartyGamePlayStartScRsp" => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyGamePlayStartScRsp),
            "CmdTrainPartyAddBuildDynamicBuffScRsp" => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyAddBuildDynamicBuffScRsp),
            "CmdTrainPartyGamePlaySettleNotify" => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyGamePlaySettleNotify),
            "CmdTrainPartyBuildDiyCsReq" => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyBuildDiyCsReq),
            "CmdTrainPartyGamePlayStartCsReq" => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyGamePlayStartCsReq),
            "CmdTrainPartyUpdatePosEnvScRsp" => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyUpdatePosEnvScRsp),
            "CmdTrainPartyAddBuildDynamicBuffCsReq" => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyAddBuildDynamicBuffCsReq),
            "CmdTrainPartyGetDataCsReq" => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyGetDataCsReq),
            "CmdTrainPartyMoveScNotify" => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyMoveScNotify),
            "CmdTrainPartyEnterScRsp" => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyEnterScRsp),
            "CmdTrainPartyEnterCsReq" => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyEnterCsReq),
            "CmdTrainPartySettleNotify" => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartySettleNotify),
            "CmdTrainPartyTakeBuildLevelAwardCsReq" => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyTakeBuildLevelAwardCsReq),
            "CmdTrainPartyBuildStartStepCsReq" => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyBuildStartStepCsReq),
            "CmdTrainPartyUseCardCsReq" => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyUseCardCsReq),
            "CmdTrainPartyGetDataScRsp" => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyGetDataScRsp),
            "CmdTrainPartyLeaveScRsp" => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyLeaveScRsp),
            "CmdTrainPartyTakeBuildLevelAwardScRsp" => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyTakeBuildLevelAwardScRsp),
            "CmdTrainPartyBuildStartStepScRsp" => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyBuildStartStepScRsp),
            "CmdTrainPartyUseCardScRsp" => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyUseCardScRsp),
            "CmdTrainPartyHandlePendingActionCsReq" => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyHandlePendingActionCsReq),
            "CmdTrainPartyBuildingUpdateNotify" => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyBuildingUpdateNotify),
            "CmdTrainPartyHandlePendingActionScRsp" => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyHandlePendingActionScRsp),
            "CmdTrainPartyUpdatePosEnvCsReq" => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartyUpdatePosEnvCsReq),
            "CmdTrainPartySyncUpdateScNotify" => ::std::option::Option::Some(CmdTrainPartyType::CmdTrainPartySyncUpdateScNotify),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdTrainPartyType] = &[
        CmdTrainPartyType::CmdTrainPartyTypeNone,
        CmdTrainPartyType::CmdTrainPartyLeaveCsReq,
        CmdTrainPartyType::CmdTrainPartyBuildDiyScRsp,
        CmdTrainPartyType::CmdTrainPartyGamePlayStartScRsp,
        CmdTrainPartyType::CmdTrainPartyAddBuildDynamicBuffScRsp,
        CmdTrainPartyType::CmdTrainPartyGamePlaySettleNotify,
        CmdTrainPartyType::CmdTrainPartyBuildDiyCsReq,
        CmdTrainPartyType::CmdTrainPartyGamePlayStartCsReq,
        CmdTrainPartyType::CmdTrainPartyUpdatePosEnvScRsp,
        CmdTrainPartyType::CmdTrainPartyAddBuildDynamicBuffCsReq,
        CmdTrainPartyType::CmdTrainPartyGetDataCsReq,
        CmdTrainPartyType::CmdTrainPartyMoveScNotify,
        CmdTrainPartyType::CmdTrainPartyEnterScRsp,
        CmdTrainPartyType::CmdTrainPartyEnterCsReq,
        CmdTrainPartyType::CmdTrainPartySettleNotify,
        CmdTrainPartyType::CmdTrainPartyTakeBuildLevelAwardCsReq,
        CmdTrainPartyType::CmdTrainPartyBuildStartStepCsReq,
        CmdTrainPartyType::CmdTrainPartyUseCardCsReq,
        CmdTrainPartyType::CmdTrainPartyGetDataScRsp,
        CmdTrainPartyType::CmdTrainPartyLeaveScRsp,
        CmdTrainPartyType::CmdTrainPartyTakeBuildLevelAwardScRsp,
        CmdTrainPartyType::CmdTrainPartyBuildStartStepScRsp,
        CmdTrainPartyType::CmdTrainPartyUseCardScRsp,
        CmdTrainPartyType::CmdTrainPartyHandlePendingActionCsReq,
        CmdTrainPartyType::CmdTrainPartyBuildingUpdateNotify,
        CmdTrainPartyType::CmdTrainPartyHandlePendingActionScRsp,
        CmdTrainPartyType::CmdTrainPartyUpdatePosEnvCsReq,
        CmdTrainPartyType::CmdTrainPartySyncUpdateScNotify,
    ];
}

impl ::protobuf::EnumFull for CmdTrainPartyType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdTrainPartyType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdTrainPartyType::CmdTrainPartyTypeNone => 0,
            CmdTrainPartyType::CmdTrainPartyLeaveCsReq => 1,
            CmdTrainPartyType::CmdTrainPartyBuildDiyScRsp => 2,
            CmdTrainPartyType::CmdTrainPartyGamePlayStartScRsp => 3,
            CmdTrainPartyType::CmdTrainPartyAddBuildDynamicBuffScRsp => 4,
            CmdTrainPartyType::CmdTrainPartyGamePlaySettleNotify => 5,
            CmdTrainPartyType::CmdTrainPartyBuildDiyCsReq => 6,
            CmdTrainPartyType::CmdTrainPartyGamePlayStartCsReq => 7,
            CmdTrainPartyType::CmdTrainPartyUpdatePosEnvScRsp => 8,
            CmdTrainPartyType::CmdTrainPartyAddBuildDynamicBuffCsReq => 9,
            CmdTrainPartyType::CmdTrainPartyGetDataCsReq => 10,
            CmdTrainPartyType::CmdTrainPartyMoveScNotify => 11,
            CmdTrainPartyType::CmdTrainPartyEnterScRsp => 12,
            CmdTrainPartyType::CmdTrainPartyEnterCsReq => 13,
            CmdTrainPartyType::CmdTrainPartySettleNotify => 14,
            CmdTrainPartyType::CmdTrainPartyTakeBuildLevelAwardCsReq => 15,
            CmdTrainPartyType::CmdTrainPartyBuildStartStepCsReq => 16,
            CmdTrainPartyType::CmdTrainPartyUseCardCsReq => 17,
            CmdTrainPartyType::CmdTrainPartyGetDataScRsp => 18,
            CmdTrainPartyType::CmdTrainPartyLeaveScRsp => 19,
            CmdTrainPartyType::CmdTrainPartyTakeBuildLevelAwardScRsp => 20,
            CmdTrainPartyType::CmdTrainPartyBuildStartStepScRsp => 21,
            CmdTrainPartyType::CmdTrainPartyUseCardScRsp => 22,
            CmdTrainPartyType::CmdTrainPartyHandlePendingActionCsReq => 23,
            CmdTrainPartyType::CmdTrainPartyBuildingUpdateNotify => 24,
            CmdTrainPartyType::CmdTrainPartyHandlePendingActionScRsp => 25,
            CmdTrainPartyType::CmdTrainPartyUpdatePosEnvCsReq => 26,
            CmdTrainPartyType::CmdTrainPartySyncUpdateScNotify => 27,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdTrainPartyType {
    fn default() -> Self {
        CmdTrainPartyType::CmdTrainPartyTypeNone
    }
}

impl CmdTrainPartyType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdTrainPartyType>("CmdTrainPartyType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17CmdTrainPartyType.proto*\x8a\x08\n\x11CmdTrainPartyType\x12\x19\n\
    \x15CmdTrainPartyTypeNone\x10\0\x12\x1c\n\x17CmdTrainPartyLeaveCsReq\x10\
    \xca>\x12\x1f\n\x1aCmdTrainPartyBuildDiyScRsp\x10\x8d?\x12$\n\x1fCmdTrai\
    nPartyGamePlayStartScRsp\x10\xf4>\x12*\n%CmdTrainPartyAddBuildDynamicBuf\
    fScRsp\x10\xdc>\x12&\n!CmdTrainPartyGamePlaySettleNotify\x10\xd6>\x12\
    \x1f\n\x1aCmdTrainPartyBuildDiyCsReq\x10\x89?\x12$\n\x1fCmdTrainPartyGam\
    ePlayStartCsReq\x10\x81?\x12#\n\x1eCmdTrainPartyUpdatePosEnvScRsp\x10\
    \xf5>\x12*\n%CmdTrainPartyAddBuildDynamicBuffCsReq\x10\x8b?\x12\x1e\n\
    \x19CmdTrainPartyGetDataCsReq\x10\xcb>\x12\x1e\n\x19CmdTrainPartyMoveScN\
    otify\x10\xe3>\x12\x1c\n\x17CmdTrainPartyEnterScRsp\x10\xd9>\x12\x1c\n\
    \x17CmdTrainPartyEnterCsReq\x10\xf9>\x12\x1e\n\x19CmdTrainPartySettleNot\
    ify\x10\x86?\x12*\n%CmdTrainPartyTakeBuildLevelAwardCsReq\x10\xd8>\x12%\
    \n\x20CmdTrainPartyBuildStartStepCsReq\x10\xe4>\x12\x1e\n\x19CmdTrainPar\
    tyUseCardCsReq\x10\xef>\x12\x1e\n\x19CmdTrainPartyGetDataScRsp\x10\xcd>\
    \x12\x1c\n\x17CmdTrainPartyLeaveScRsp\x10\xc7>\x12*\n%CmdTrainPartyTakeB\
    uildLevelAwardScRsp\x10\x94?\x12%\n\x20CmdTrainPartyBuildStartStepScRsp\
    \x10\xf2>\x12\x1e\n\x19CmdTrainPartyUseCardScRsp\x10\xc9>\x12*\n%CmdTrai\
    nPartyHandlePendingActionCsReq\x10\x9f?\x12&\n!CmdTrainPartyBuildingUpda\
    teNotify\x10\x9b?\x12*\n%CmdTrainPartyHandlePendingActionScRsp\x10\xd2>\
    \x12#\n\x1eCmdTrainPartyUpdatePosEnvCsReq\x10\x9c?\x12$\n\x1fCmdTrainPar\
    tySyncUpdateScNotify\x10\xda>b\x06proto3\
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
            enums.push(CmdTrainPartyType::generated_enum_descriptor_data());
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
