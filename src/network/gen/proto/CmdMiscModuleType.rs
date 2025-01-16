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

//! Generated file from `CmdMiscModuleType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdMiscModuleType)
pub enum CmdMiscModuleType {
    // @@protoc_insertion_point(enum_value:CmdMiscModuleType.CmdMiscModuleTypeNone)
    CmdMiscModuleTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdMiscModuleType.CmdTriggerVoiceCsReq)
    CmdTriggerVoiceCsReq = 4146,
    // @@protoc_insertion_point(enum_value:CmdMiscModuleType.CmdCancelCacheNotifyCsReq)
    CmdCancelCacheNotifyCsReq = 4124,
    // @@protoc_insertion_point(enum_value:CmdMiscModuleType.CmdSecurityReportCsReq)
    CmdSecurityReportCsReq = 4134,
    // @@protoc_insertion_point(enum_value:CmdMiscModuleType.CmdDifficultyAdjustmentGetDataCsReq)
    CmdDifficultyAdjustmentGetDataCsReq = 4188,
    // @@protoc_insertion_point(enum_value:CmdMiscModuleType.CmdMazeKillDirectCsReq)
    CmdMazeKillDirectCsReq = 4135,
    // @@protoc_insertion_point(enum_value:CmdMiscModuleType.CmdSubmitOrigamiItemCsReq)
    CmdSubmitOrigamiItemCsReq = 4196,
    // @@protoc_insertion_point(enum_value:CmdMiscModuleType.CmdMazeKillDirectScRsp)
    CmdMazeKillDirectScRsp = 4173,
    // @@protoc_insertion_point(enum_value:CmdMiscModuleType.CmdUpdateGunPlayDataScRsp)
    CmdUpdateGunPlayDataScRsp = 4148,
    // @@protoc_insertion_point(enum_value:CmdMiscModuleType.CmdGetGunPlayDataCsReq)
    CmdGetGunPlayDataCsReq = 4178,
    // @@protoc_insertion_point(enum_value:CmdMiscModuleType.CmdShareScRsp)
    CmdShareScRsp = 4195,
    // @@protoc_insertion_point(enum_value:CmdMiscModuleType.CmdDifficultyAdjustmentUpdateDataScRsp)
    CmdDifficultyAdjustmentUpdateDataScRsp = 4176,
    // @@protoc_insertion_point(enum_value:CmdMiscModuleType.CmdGetMovieRacingDataScRsp)
    CmdGetMovieRacingDataScRsp = 4190,
    // @@protoc_insertion_point(enum_value:CmdMiscModuleType.CmdGetMovieRacingDataCsReq)
    CmdGetMovieRacingDataCsReq = 4153,
    // @@protoc_insertion_point(enum_value:CmdMiscModuleType.CmdSecurityReportScRsp)
    CmdSecurityReportScRsp = 4143,
    // @@protoc_insertion_point(enum_value:CmdMiscModuleType.CmdGetGunPlayDataScRsp)
    CmdGetGunPlayDataScRsp = 4129,
    // @@protoc_insertion_point(enum_value:CmdMiscModuleType.CmdGetShareDataCsReq)
    CmdGetShareDataCsReq = 4184,
    // @@protoc_insertion_point(enum_value:CmdMiscModuleType.CmdTakePictureCsReq)
    CmdTakePictureCsReq = 4167,
    // @@protoc_insertion_point(enum_value:CmdMiscModuleType.CmdTriggerVoiceScRsp)
    CmdTriggerVoiceScRsp = 4125,
    // @@protoc_insertion_point(enum_value:CmdMiscModuleType.CmdUpdateMovieRacingDataScRsp)
    CmdUpdateMovieRacingDataScRsp = 4112,
    // @@protoc_insertion_point(enum_value:CmdMiscModuleType.CmdSubmitOrigamiItemScRsp)
    CmdSubmitOrigamiItemScRsp = 4105,
    // @@protoc_insertion_point(enum_value:CmdMiscModuleType.CmdShareCsReq)
    CmdShareCsReq = 4136,
    // @@protoc_insertion_point(enum_value:CmdMiscModuleType.CmdTakePictureScRsp)
    CmdTakePictureScRsp = 4128,
    // @@protoc_insertion_point(enum_value:CmdMiscModuleType.CmdUpdateMovieRacingDataCsReq)
    CmdUpdateMovieRacingDataCsReq = 4159,
    // @@protoc_insertion_point(enum_value:CmdMiscModuleType.CmdGetShareDataScRsp)
    CmdGetShareDataScRsp = 4127,
    // @@protoc_insertion_point(enum_value:CmdMiscModuleType.CmdDifficultyAdjustmentUpdateDataCsReq)
    CmdDifficultyAdjustmentUpdateDataCsReq = 4110,
    // @@protoc_insertion_point(enum_value:CmdMiscModuleType.CmdDifficultyAdjustmentGetDataScRsp)
    CmdDifficultyAdjustmentGetDataScRsp = 4158,
    // @@protoc_insertion_point(enum_value:CmdMiscModuleType.CmdUpdateGunPlayDataCsReq)
    CmdUpdateGunPlayDataCsReq = 4150,
    // @@protoc_insertion_point(enum_value:CmdMiscModuleType.CmdCancelCacheNotifyScRsp)
    CmdCancelCacheNotifyScRsp = 4193,
}

impl ::protobuf::Enum for CmdMiscModuleType {
    const NAME: &'static str = "CmdMiscModuleType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdMiscModuleType> {
        match value {
            0 => ::std::option::Option::Some(CmdMiscModuleType::CmdMiscModuleTypeNone),
            4146 => ::std::option::Option::Some(CmdMiscModuleType::CmdTriggerVoiceCsReq),
            4124 => ::std::option::Option::Some(CmdMiscModuleType::CmdCancelCacheNotifyCsReq),
            4134 => ::std::option::Option::Some(CmdMiscModuleType::CmdSecurityReportCsReq),
            4188 => ::std::option::Option::Some(CmdMiscModuleType::CmdDifficultyAdjustmentGetDataCsReq),
            4135 => ::std::option::Option::Some(CmdMiscModuleType::CmdMazeKillDirectCsReq),
            4196 => ::std::option::Option::Some(CmdMiscModuleType::CmdSubmitOrigamiItemCsReq),
            4173 => ::std::option::Option::Some(CmdMiscModuleType::CmdMazeKillDirectScRsp),
            4148 => ::std::option::Option::Some(CmdMiscModuleType::CmdUpdateGunPlayDataScRsp),
            4178 => ::std::option::Option::Some(CmdMiscModuleType::CmdGetGunPlayDataCsReq),
            4195 => ::std::option::Option::Some(CmdMiscModuleType::CmdShareScRsp),
            4176 => ::std::option::Option::Some(CmdMiscModuleType::CmdDifficultyAdjustmentUpdateDataScRsp),
            4190 => ::std::option::Option::Some(CmdMiscModuleType::CmdGetMovieRacingDataScRsp),
            4153 => ::std::option::Option::Some(CmdMiscModuleType::CmdGetMovieRacingDataCsReq),
            4143 => ::std::option::Option::Some(CmdMiscModuleType::CmdSecurityReportScRsp),
            4129 => ::std::option::Option::Some(CmdMiscModuleType::CmdGetGunPlayDataScRsp),
            4184 => ::std::option::Option::Some(CmdMiscModuleType::CmdGetShareDataCsReq),
            4167 => ::std::option::Option::Some(CmdMiscModuleType::CmdTakePictureCsReq),
            4125 => ::std::option::Option::Some(CmdMiscModuleType::CmdTriggerVoiceScRsp),
            4112 => ::std::option::Option::Some(CmdMiscModuleType::CmdUpdateMovieRacingDataScRsp),
            4105 => ::std::option::Option::Some(CmdMiscModuleType::CmdSubmitOrigamiItemScRsp),
            4136 => ::std::option::Option::Some(CmdMiscModuleType::CmdShareCsReq),
            4128 => ::std::option::Option::Some(CmdMiscModuleType::CmdTakePictureScRsp),
            4159 => ::std::option::Option::Some(CmdMiscModuleType::CmdUpdateMovieRacingDataCsReq),
            4127 => ::std::option::Option::Some(CmdMiscModuleType::CmdGetShareDataScRsp),
            4110 => ::std::option::Option::Some(CmdMiscModuleType::CmdDifficultyAdjustmentUpdateDataCsReq),
            4158 => ::std::option::Option::Some(CmdMiscModuleType::CmdDifficultyAdjustmentGetDataScRsp),
            4150 => ::std::option::Option::Some(CmdMiscModuleType::CmdUpdateGunPlayDataCsReq),
            4193 => ::std::option::Option::Some(CmdMiscModuleType::CmdCancelCacheNotifyScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdMiscModuleType> {
        match str {
            "CmdMiscModuleTypeNone" => ::std::option::Option::Some(CmdMiscModuleType::CmdMiscModuleTypeNone),
            "CmdTriggerVoiceCsReq" => ::std::option::Option::Some(CmdMiscModuleType::CmdTriggerVoiceCsReq),
            "CmdCancelCacheNotifyCsReq" => ::std::option::Option::Some(CmdMiscModuleType::CmdCancelCacheNotifyCsReq),
            "CmdSecurityReportCsReq" => ::std::option::Option::Some(CmdMiscModuleType::CmdSecurityReportCsReq),
            "CmdDifficultyAdjustmentGetDataCsReq" => ::std::option::Option::Some(CmdMiscModuleType::CmdDifficultyAdjustmentGetDataCsReq),
            "CmdMazeKillDirectCsReq" => ::std::option::Option::Some(CmdMiscModuleType::CmdMazeKillDirectCsReq),
            "CmdSubmitOrigamiItemCsReq" => ::std::option::Option::Some(CmdMiscModuleType::CmdSubmitOrigamiItemCsReq),
            "CmdMazeKillDirectScRsp" => ::std::option::Option::Some(CmdMiscModuleType::CmdMazeKillDirectScRsp),
            "CmdUpdateGunPlayDataScRsp" => ::std::option::Option::Some(CmdMiscModuleType::CmdUpdateGunPlayDataScRsp),
            "CmdGetGunPlayDataCsReq" => ::std::option::Option::Some(CmdMiscModuleType::CmdGetGunPlayDataCsReq),
            "CmdShareScRsp" => ::std::option::Option::Some(CmdMiscModuleType::CmdShareScRsp),
            "CmdDifficultyAdjustmentUpdateDataScRsp" => ::std::option::Option::Some(CmdMiscModuleType::CmdDifficultyAdjustmentUpdateDataScRsp),
            "CmdGetMovieRacingDataScRsp" => ::std::option::Option::Some(CmdMiscModuleType::CmdGetMovieRacingDataScRsp),
            "CmdGetMovieRacingDataCsReq" => ::std::option::Option::Some(CmdMiscModuleType::CmdGetMovieRacingDataCsReq),
            "CmdSecurityReportScRsp" => ::std::option::Option::Some(CmdMiscModuleType::CmdSecurityReportScRsp),
            "CmdGetGunPlayDataScRsp" => ::std::option::Option::Some(CmdMiscModuleType::CmdGetGunPlayDataScRsp),
            "CmdGetShareDataCsReq" => ::std::option::Option::Some(CmdMiscModuleType::CmdGetShareDataCsReq),
            "CmdTakePictureCsReq" => ::std::option::Option::Some(CmdMiscModuleType::CmdTakePictureCsReq),
            "CmdTriggerVoiceScRsp" => ::std::option::Option::Some(CmdMiscModuleType::CmdTriggerVoiceScRsp),
            "CmdUpdateMovieRacingDataScRsp" => ::std::option::Option::Some(CmdMiscModuleType::CmdUpdateMovieRacingDataScRsp),
            "CmdSubmitOrigamiItemScRsp" => ::std::option::Option::Some(CmdMiscModuleType::CmdSubmitOrigamiItemScRsp),
            "CmdShareCsReq" => ::std::option::Option::Some(CmdMiscModuleType::CmdShareCsReq),
            "CmdTakePictureScRsp" => ::std::option::Option::Some(CmdMiscModuleType::CmdTakePictureScRsp),
            "CmdUpdateMovieRacingDataCsReq" => ::std::option::Option::Some(CmdMiscModuleType::CmdUpdateMovieRacingDataCsReq),
            "CmdGetShareDataScRsp" => ::std::option::Option::Some(CmdMiscModuleType::CmdGetShareDataScRsp),
            "CmdDifficultyAdjustmentUpdateDataCsReq" => ::std::option::Option::Some(CmdMiscModuleType::CmdDifficultyAdjustmentUpdateDataCsReq),
            "CmdDifficultyAdjustmentGetDataScRsp" => ::std::option::Option::Some(CmdMiscModuleType::CmdDifficultyAdjustmentGetDataScRsp),
            "CmdUpdateGunPlayDataCsReq" => ::std::option::Option::Some(CmdMiscModuleType::CmdUpdateGunPlayDataCsReq),
            "CmdCancelCacheNotifyScRsp" => ::std::option::Option::Some(CmdMiscModuleType::CmdCancelCacheNotifyScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdMiscModuleType] = &[
        CmdMiscModuleType::CmdMiscModuleTypeNone,
        CmdMiscModuleType::CmdTriggerVoiceCsReq,
        CmdMiscModuleType::CmdCancelCacheNotifyCsReq,
        CmdMiscModuleType::CmdSecurityReportCsReq,
        CmdMiscModuleType::CmdDifficultyAdjustmentGetDataCsReq,
        CmdMiscModuleType::CmdMazeKillDirectCsReq,
        CmdMiscModuleType::CmdSubmitOrigamiItemCsReq,
        CmdMiscModuleType::CmdMazeKillDirectScRsp,
        CmdMiscModuleType::CmdUpdateGunPlayDataScRsp,
        CmdMiscModuleType::CmdGetGunPlayDataCsReq,
        CmdMiscModuleType::CmdShareScRsp,
        CmdMiscModuleType::CmdDifficultyAdjustmentUpdateDataScRsp,
        CmdMiscModuleType::CmdGetMovieRacingDataScRsp,
        CmdMiscModuleType::CmdGetMovieRacingDataCsReq,
        CmdMiscModuleType::CmdSecurityReportScRsp,
        CmdMiscModuleType::CmdGetGunPlayDataScRsp,
        CmdMiscModuleType::CmdGetShareDataCsReq,
        CmdMiscModuleType::CmdTakePictureCsReq,
        CmdMiscModuleType::CmdTriggerVoiceScRsp,
        CmdMiscModuleType::CmdUpdateMovieRacingDataScRsp,
        CmdMiscModuleType::CmdSubmitOrigamiItemScRsp,
        CmdMiscModuleType::CmdShareCsReq,
        CmdMiscModuleType::CmdTakePictureScRsp,
        CmdMiscModuleType::CmdUpdateMovieRacingDataCsReq,
        CmdMiscModuleType::CmdGetShareDataScRsp,
        CmdMiscModuleType::CmdDifficultyAdjustmentUpdateDataCsReq,
        CmdMiscModuleType::CmdDifficultyAdjustmentGetDataScRsp,
        CmdMiscModuleType::CmdUpdateGunPlayDataCsReq,
        CmdMiscModuleType::CmdCancelCacheNotifyScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdMiscModuleType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdMiscModuleType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdMiscModuleType::CmdMiscModuleTypeNone => 0,
            CmdMiscModuleType::CmdTriggerVoiceCsReq => 1,
            CmdMiscModuleType::CmdCancelCacheNotifyCsReq => 2,
            CmdMiscModuleType::CmdSecurityReportCsReq => 3,
            CmdMiscModuleType::CmdDifficultyAdjustmentGetDataCsReq => 4,
            CmdMiscModuleType::CmdMazeKillDirectCsReq => 5,
            CmdMiscModuleType::CmdSubmitOrigamiItemCsReq => 6,
            CmdMiscModuleType::CmdMazeKillDirectScRsp => 7,
            CmdMiscModuleType::CmdUpdateGunPlayDataScRsp => 8,
            CmdMiscModuleType::CmdGetGunPlayDataCsReq => 9,
            CmdMiscModuleType::CmdShareScRsp => 10,
            CmdMiscModuleType::CmdDifficultyAdjustmentUpdateDataScRsp => 11,
            CmdMiscModuleType::CmdGetMovieRacingDataScRsp => 12,
            CmdMiscModuleType::CmdGetMovieRacingDataCsReq => 13,
            CmdMiscModuleType::CmdSecurityReportScRsp => 14,
            CmdMiscModuleType::CmdGetGunPlayDataScRsp => 15,
            CmdMiscModuleType::CmdGetShareDataCsReq => 16,
            CmdMiscModuleType::CmdTakePictureCsReq => 17,
            CmdMiscModuleType::CmdTriggerVoiceScRsp => 18,
            CmdMiscModuleType::CmdUpdateMovieRacingDataScRsp => 19,
            CmdMiscModuleType::CmdSubmitOrigamiItemScRsp => 20,
            CmdMiscModuleType::CmdShareCsReq => 21,
            CmdMiscModuleType::CmdTakePictureScRsp => 22,
            CmdMiscModuleType::CmdUpdateMovieRacingDataCsReq => 23,
            CmdMiscModuleType::CmdGetShareDataScRsp => 24,
            CmdMiscModuleType::CmdDifficultyAdjustmentUpdateDataCsReq => 25,
            CmdMiscModuleType::CmdDifficultyAdjustmentGetDataScRsp => 26,
            CmdMiscModuleType::CmdUpdateGunPlayDataCsReq => 27,
            CmdMiscModuleType::CmdCancelCacheNotifyScRsp => 28,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdMiscModuleType {
    fn default() -> Self {
        CmdMiscModuleType::CmdMiscModuleTypeNone
    }
}

impl CmdMiscModuleType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdMiscModuleType>("CmdMiscModuleType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17CmdMiscModuleType.proto*\x9c\x07\n\x11CmdMiscModuleType\x12\x19\n\
    \x15CmdMiscModuleTypeNone\x10\0\x12\x19\n\x14CmdTriggerVoiceCsReq\x10\
    \xb2\x20\x12\x1e\n\x19CmdCancelCacheNotifyCsReq\x10\x9c\x20\x12\x1b\n\
    \x16CmdSecurityReportCsReq\x10\xa6\x20\x12(\n#CmdDifficultyAdjustmentGet\
    DataCsReq\x10\xdc\x20\x12\x1b\n\x16CmdMazeKillDirectCsReq\x10\xa7\x20\
    \x12\x1e\n\x19CmdSubmitOrigamiItemCsReq\x10\xe4\x20\x12\x1b\n\x16CmdMaze\
    KillDirectScRsp\x10\xcd\x20\x12\x1e\n\x19CmdUpdateGunPlayDataScRsp\x10\
    \xb4\x20\x12\x1b\n\x16CmdGetGunPlayDataCsReq\x10\xd2\x20\x12\x12\n\rCmdS\
    hareScRsp\x10\xe3\x20\x12+\n&CmdDifficultyAdjustmentUpdateDataScRsp\x10\
    \xd0\x20\x12\x1f\n\x1aCmdGetMovieRacingDataScRsp\x10\xde\x20\x12\x1f\n\
    \x1aCmdGetMovieRacingDataCsReq\x10\xb9\x20\x12\x1b\n\x16CmdSecurityRepor\
    tScRsp\x10\xaf\x20\x12\x1b\n\x16CmdGetGunPlayDataScRsp\x10\xa1\x20\x12\
    \x19\n\x14CmdGetShareDataCsReq\x10\xd8\x20\x12\x18\n\x13CmdTakePictureCs\
    Req\x10\xc7\x20\x12\x19\n\x14CmdTriggerVoiceScRsp\x10\x9d\x20\x12\"\n\
    \x1dCmdUpdateMovieRacingDataScRsp\x10\x90\x20\x12\x1e\n\x19CmdSubmitOrig\
    amiItemScRsp\x10\x89\x20\x12\x12\n\rCmdShareCsReq\x10\xa8\x20\x12\x18\n\
    \x13CmdTakePictureScRsp\x10\xa0\x20\x12\"\n\x1dCmdUpdateMovieRacingDataC\
    sReq\x10\xbf\x20\x12\x19\n\x14CmdGetShareDataScRsp\x10\x9f\x20\x12+\n&Cm\
    dDifficultyAdjustmentUpdateDataCsReq\x10\x8e\x20\x12(\n#CmdDifficultyAdj\
    ustmentGetDataScRsp\x10\xbe\x20\x12\x1e\n\x19CmdUpdateGunPlayDataCsReq\
    \x10\xb6\x20\x12\x1e\n\x19CmdCancelCacheNotifyScRsp\x10\xe1\x20b\x06prot\
    o3\
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
            enums.push(CmdMiscModuleType::generated_enum_descriptor_data());
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
