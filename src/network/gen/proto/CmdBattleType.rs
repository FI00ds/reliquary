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

//! Generated file from `CmdBattleType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdBattleType)
pub enum CmdBattleType {
    // @@protoc_insertion_point(enum_value:CmdBattleType.CmdBattleTypeNone)
    CmdBattleTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdBattleType.CmdPVEBattleResultCsReq)
    CmdPVEBattleResultCsReq = 101,
    // @@protoc_insertion_point(enum_value:CmdBattleType.CmdBattleLogReportScRsp)
    CmdBattleLogReportScRsp = 105,
    // @@protoc_insertion_point(enum_value:CmdBattleType.CmdPVEBattleResultScRsp)
    CmdPVEBattleResultScRsp = 168,
    // @@protoc_insertion_point(enum_value:CmdBattleType.CmdGetCurBattleInfoCsReq)
    CmdGetCurBattleInfoCsReq = 130,
    // @@protoc_insertion_point(enum_value:CmdBattleType.CmdQuitBattleScRsp)
    CmdQuitBattleScRsp = 124,
    // @@protoc_insertion_point(enum_value:CmdBattleType.CmdSyncClientResVersionCsReq)
    CmdSyncClientResVersionCsReq = 197,
    // @@protoc_insertion_point(enum_value:CmdBattleType.CmdBattleLogReportCsReq)
    CmdBattleLogReportCsReq = 111,
    // @@protoc_insertion_point(enum_value:CmdBattleType.CmdServerSimulateBattleFinishScNotify)
    CmdServerSimulateBattleFinishScNotify = 114,
    // @@protoc_insertion_point(enum_value:CmdBattleType.CmdQuitBattleScNotify)
    CmdQuitBattleScNotify = 128,
    // @@protoc_insertion_point(enum_value:CmdBattleType.CmdSyncClientResVersionScRsp)
    CmdSyncClientResVersionScRsp = 176,
    // @@protoc_insertion_point(enum_value:CmdBattleType.CmdGetCurBattleInfoScRsp)
    CmdGetCurBattleInfoScRsp = 156,
    // @@protoc_insertion_point(enum_value:CmdBattleType.CmdRebattleByClientCsNotify)
    CmdRebattleByClientCsNotify = 122,
    // @@protoc_insertion_point(enum_value:CmdBattleType.CmdReBattleAfterBattleLoseCsNotify)
    CmdReBattleAfterBattleLoseCsNotify = 149,
    // @@protoc_insertion_point(enum_value:CmdBattleType.CmdQuitBattleCsReq)
    CmdQuitBattleCsReq = 158,
}

impl ::protobuf::Enum for CmdBattleType {
    const NAME: &'static str = "CmdBattleType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdBattleType> {
        match value {
            0 => ::std::option::Option::Some(CmdBattleType::CmdBattleTypeNone),
            101 => ::std::option::Option::Some(CmdBattleType::CmdPVEBattleResultCsReq),
            105 => ::std::option::Option::Some(CmdBattleType::CmdBattleLogReportScRsp),
            168 => ::std::option::Option::Some(CmdBattleType::CmdPVEBattleResultScRsp),
            130 => ::std::option::Option::Some(CmdBattleType::CmdGetCurBattleInfoCsReq),
            124 => ::std::option::Option::Some(CmdBattleType::CmdQuitBattleScRsp),
            197 => ::std::option::Option::Some(CmdBattleType::CmdSyncClientResVersionCsReq),
            111 => ::std::option::Option::Some(CmdBattleType::CmdBattleLogReportCsReq),
            114 => ::std::option::Option::Some(CmdBattleType::CmdServerSimulateBattleFinishScNotify),
            128 => ::std::option::Option::Some(CmdBattleType::CmdQuitBattleScNotify),
            176 => ::std::option::Option::Some(CmdBattleType::CmdSyncClientResVersionScRsp),
            156 => ::std::option::Option::Some(CmdBattleType::CmdGetCurBattleInfoScRsp),
            122 => ::std::option::Option::Some(CmdBattleType::CmdRebattleByClientCsNotify),
            149 => ::std::option::Option::Some(CmdBattleType::CmdReBattleAfterBattleLoseCsNotify),
            158 => ::std::option::Option::Some(CmdBattleType::CmdQuitBattleCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdBattleType> {
        match str {
            "CmdBattleTypeNone" => ::std::option::Option::Some(CmdBattleType::CmdBattleTypeNone),
            "CmdPVEBattleResultCsReq" => ::std::option::Option::Some(CmdBattleType::CmdPVEBattleResultCsReq),
            "CmdBattleLogReportScRsp" => ::std::option::Option::Some(CmdBattleType::CmdBattleLogReportScRsp),
            "CmdPVEBattleResultScRsp" => ::std::option::Option::Some(CmdBattleType::CmdPVEBattleResultScRsp),
            "CmdGetCurBattleInfoCsReq" => ::std::option::Option::Some(CmdBattleType::CmdGetCurBattleInfoCsReq),
            "CmdQuitBattleScRsp" => ::std::option::Option::Some(CmdBattleType::CmdQuitBattleScRsp),
            "CmdSyncClientResVersionCsReq" => ::std::option::Option::Some(CmdBattleType::CmdSyncClientResVersionCsReq),
            "CmdBattleLogReportCsReq" => ::std::option::Option::Some(CmdBattleType::CmdBattleLogReportCsReq),
            "CmdServerSimulateBattleFinishScNotify" => ::std::option::Option::Some(CmdBattleType::CmdServerSimulateBattleFinishScNotify),
            "CmdQuitBattleScNotify" => ::std::option::Option::Some(CmdBattleType::CmdQuitBattleScNotify),
            "CmdSyncClientResVersionScRsp" => ::std::option::Option::Some(CmdBattleType::CmdSyncClientResVersionScRsp),
            "CmdGetCurBattleInfoScRsp" => ::std::option::Option::Some(CmdBattleType::CmdGetCurBattleInfoScRsp),
            "CmdRebattleByClientCsNotify" => ::std::option::Option::Some(CmdBattleType::CmdRebattleByClientCsNotify),
            "CmdReBattleAfterBattleLoseCsNotify" => ::std::option::Option::Some(CmdBattleType::CmdReBattleAfterBattleLoseCsNotify),
            "CmdQuitBattleCsReq" => ::std::option::Option::Some(CmdBattleType::CmdQuitBattleCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdBattleType] = &[
        CmdBattleType::CmdBattleTypeNone,
        CmdBattleType::CmdPVEBattleResultCsReq,
        CmdBattleType::CmdBattleLogReportScRsp,
        CmdBattleType::CmdPVEBattleResultScRsp,
        CmdBattleType::CmdGetCurBattleInfoCsReq,
        CmdBattleType::CmdQuitBattleScRsp,
        CmdBattleType::CmdSyncClientResVersionCsReq,
        CmdBattleType::CmdBattleLogReportCsReq,
        CmdBattleType::CmdServerSimulateBattleFinishScNotify,
        CmdBattleType::CmdQuitBattleScNotify,
        CmdBattleType::CmdSyncClientResVersionScRsp,
        CmdBattleType::CmdGetCurBattleInfoScRsp,
        CmdBattleType::CmdRebattleByClientCsNotify,
        CmdBattleType::CmdReBattleAfterBattleLoseCsNotify,
        CmdBattleType::CmdQuitBattleCsReq,
    ];
}

impl ::protobuf::EnumFull for CmdBattleType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdBattleType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdBattleType::CmdBattleTypeNone => 0,
            CmdBattleType::CmdPVEBattleResultCsReq => 1,
            CmdBattleType::CmdBattleLogReportScRsp => 2,
            CmdBattleType::CmdPVEBattleResultScRsp => 3,
            CmdBattleType::CmdGetCurBattleInfoCsReq => 4,
            CmdBattleType::CmdQuitBattleScRsp => 5,
            CmdBattleType::CmdSyncClientResVersionCsReq => 6,
            CmdBattleType::CmdBattleLogReportCsReq => 7,
            CmdBattleType::CmdServerSimulateBattleFinishScNotify => 8,
            CmdBattleType::CmdQuitBattleScNotify => 9,
            CmdBattleType::CmdSyncClientResVersionScRsp => 10,
            CmdBattleType::CmdGetCurBattleInfoScRsp => 11,
            CmdBattleType::CmdRebattleByClientCsNotify => 12,
            CmdBattleType::CmdReBattleAfterBattleLoseCsNotify => 13,
            CmdBattleType::CmdQuitBattleCsReq => 14,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdBattleType {
    fn default() -> Self {
        CmdBattleType::CmdBattleTypeNone
    }
}

impl CmdBattleType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdBattleType>("CmdBattleType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13CmdBattleType.proto*\xe1\x03\n\rCmdBattleType\x12\x15\n\x11CmdBatt\
    leTypeNone\x10\0\x12\x1b\n\x17CmdPVEBattleResultCsReq\x10e\x12\x1b\n\x17\
    CmdBattleLogReportScRsp\x10i\x12\x1c\n\x17CmdPVEBattleResultScRsp\x10\
    \xa8\x01\x12\x1d\n\x18CmdGetCurBattleInfoCsReq\x10\x82\x01\x12\x16\n\x12\
    CmdQuitBattleScRsp\x10|\x12!\n\x1cCmdSyncClientResVersionCsReq\x10\xc5\
    \x01\x12\x1b\n\x17CmdBattleLogReportCsReq\x10o\x12)\n%CmdServerSimulateB\
    attleFinishScNotify\x10r\x12\x1a\n\x15CmdQuitBattleScNotify\x10\x80\x01\
    \x12!\n\x1cCmdSyncClientResVersionScRsp\x10\xb0\x01\x12\x1d\n\x18CmdGetC\
    urBattleInfoScRsp\x10\x9c\x01\x12\x1f\n\x1bCmdRebattleByClientCsNotify\
    \x10z\x12'\n\"CmdReBattleAfterBattleLoseCsNotify\x10\x95\x01\x12\x17\n\
    \x12CmdQuitBattleCsReq\x10\x9e\x01b\x06proto3\
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
            enums.push(CmdBattleType::generated_enum_descriptor_data());
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
