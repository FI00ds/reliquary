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

//! Generated file from `CmdTreasureDungeonType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdTreasureDungeonType)
pub enum CmdTreasureDungeonType {
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdTreasureDungeonTypeNone)
    CmdTreasureDungeonTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdEnterTreasureDungeonCsReq)
    CmdEnterTreasureDungeonCsReq = 4436,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdOpenTreasureDungeonGridCsReq)
    CmdOpenTreasureDungeonGridCsReq = 4473,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdTreasureDungeonDataScNotify)
    CmdTreasureDungeonDataScNotify = 4411,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdInteractTreasureDungeonGridScRsp)
    CmdInteractTreasureDungeonGridScRsp = 4425,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdInteractTreasureDungeonGridCsReq)
    CmdInteractTreasureDungeonGridCsReq = 4457,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdGetTreasureDungeonActivityDataScRsp)
    CmdGetTreasureDungeonActivityDataScRsp = 4418,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdFightTreasureDungeonMonsterScRsp)
    CmdFightTreasureDungeonMonsterScRsp = 4493,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdQuitTreasureDungeonCsReq)
    CmdQuitTreasureDungeonCsReq = 4471,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdFightTreasureDungeonMonsterCsReq)
    CmdFightTreasureDungeonMonsterCsReq = 4491,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdUseTreasureDungeonItemCsReq)
    CmdUseTreasureDungeonItemCsReq = 4410,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdOpenTreasureDungeonGridScRsp)
    CmdOpenTreasureDungeonGridScRsp = 4477,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdQuitTreasureDungeonScRsp)
    CmdQuitTreasureDungeonScRsp = 4482,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdTreasureDungeonFinishScNotify)
    CmdTreasureDungeonFinishScNotify = 4413,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdGetTreasureDungeonActivityDataCsReq)
    CmdGetTreasureDungeonActivityDataCsReq = 4495,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdUseTreasureDungeonItemScRsp)
    CmdUseTreasureDungeonItemScRsp = 4407,
    // @@protoc_insertion_point(enum_value:CmdTreasureDungeonType.CmdEnterTreasureDungeonScRsp)
    CmdEnterTreasureDungeonScRsp = 4450,
}

impl ::protobuf::Enum for CmdTreasureDungeonType {
    const NAME: &'static str = "CmdTreasureDungeonType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdTreasureDungeonType> {
        match value {
            0 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdTreasureDungeonTypeNone),
            4436 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdEnterTreasureDungeonCsReq),
            4473 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdOpenTreasureDungeonGridCsReq),
            4411 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdTreasureDungeonDataScNotify),
            4425 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdInteractTreasureDungeonGridScRsp),
            4457 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdInteractTreasureDungeonGridCsReq),
            4418 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdGetTreasureDungeonActivityDataScRsp),
            4493 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdFightTreasureDungeonMonsterScRsp),
            4471 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdQuitTreasureDungeonCsReq),
            4491 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdFightTreasureDungeonMonsterCsReq),
            4410 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdUseTreasureDungeonItemCsReq),
            4477 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdOpenTreasureDungeonGridScRsp),
            4482 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdQuitTreasureDungeonScRsp),
            4413 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdTreasureDungeonFinishScNotify),
            4495 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdGetTreasureDungeonActivityDataCsReq),
            4407 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdUseTreasureDungeonItemScRsp),
            4450 => ::std::option::Option::Some(CmdTreasureDungeonType::CmdEnterTreasureDungeonScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdTreasureDungeonType> {
        match str {
            "CmdTreasureDungeonTypeNone" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdTreasureDungeonTypeNone),
            "CmdEnterTreasureDungeonCsReq" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdEnterTreasureDungeonCsReq),
            "CmdOpenTreasureDungeonGridCsReq" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdOpenTreasureDungeonGridCsReq),
            "CmdTreasureDungeonDataScNotify" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdTreasureDungeonDataScNotify),
            "CmdInteractTreasureDungeonGridScRsp" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdInteractTreasureDungeonGridScRsp),
            "CmdInteractTreasureDungeonGridCsReq" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdInteractTreasureDungeonGridCsReq),
            "CmdGetTreasureDungeonActivityDataScRsp" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdGetTreasureDungeonActivityDataScRsp),
            "CmdFightTreasureDungeonMonsterScRsp" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdFightTreasureDungeonMonsterScRsp),
            "CmdQuitTreasureDungeonCsReq" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdQuitTreasureDungeonCsReq),
            "CmdFightTreasureDungeonMonsterCsReq" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdFightTreasureDungeonMonsterCsReq),
            "CmdUseTreasureDungeonItemCsReq" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdUseTreasureDungeonItemCsReq),
            "CmdOpenTreasureDungeonGridScRsp" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdOpenTreasureDungeonGridScRsp),
            "CmdQuitTreasureDungeonScRsp" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdQuitTreasureDungeonScRsp),
            "CmdTreasureDungeonFinishScNotify" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdTreasureDungeonFinishScNotify),
            "CmdGetTreasureDungeonActivityDataCsReq" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdGetTreasureDungeonActivityDataCsReq),
            "CmdUseTreasureDungeonItemScRsp" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdUseTreasureDungeonItemScRsp),
            "CmdEnterTreasureDungeonScRsp" => ::std::option::Option::Some(CmdTreasureDungeonType::CmdEnterTreasureDungeonScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdTreasureDungeonType] = &[
        CmdTreasureDungeonType::CmdTreasureDungeonTypeNone,
        CmdTreasureDungeonType::CmdEnterTreasureDungeonCsReq,
        CmdTreasureDungeonType::CmdOpenTreasureDungeonGridCsReq,
        CmdTreasureDungeonType::CmdTreasureDungeonDataScNotify,
        CmdTreasureDungeonType::CmdInteractTreasureDungeonGridScRsp,
        CmdTreasureDungeonType::CmdInteractTreasureDungeonGridCsReq,
        CmdTreasureDungeonType::CmdGetTreasureDungeonActivityDataScRsp,
        CmdTreasureDungeonType::CmdFightTreasureDungeonMonsterScRsp,
        CmdTreasureDungeonType::CmdQuitTreasureDungeonCsReq,
        CmdTreasureDungeonType::CmdFightTreasureDungeonMonsterCsReq,
        CmdTreasureDungeonType::CmdUseTreasureDungeonItemCsReq,
        CmdTreasureDungeonType::CmdOpenTreasureDungeonGridScRsp,
        CmdTreasureDungeonType::CmdQuitTreasureDungeonScRsp,
        CmdTreasureDungeonType::CmdTreasureDungeonFinishScNotify,
        CmdTreasureDungeonType::CmdGetTreasureDungeonActivityDataCsReq,
        CmdTreasureDungeonType::CmdUseTreasureDungeonItemScRsp,
        CmdTreasureDungeonType::CmdEnterTreasureDungeonScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdTreasureDungeonType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdTreasureDungeonType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdTreasureDungeonType::CmdTreasureDungeonTypeNone => 0,
            CmdTreasureDungeonType::CmdEnterTreasureDungeonCsReq => 1,
            CmdTreasureDungeonType::CmdOpenTreasureDungeonGridCsReq => 2,
            CmdTreasureDungeonType::CmdTreasureDungeonDataScNotify => 3,
            CmdTreasureDungeonType::CmdInteractTreasureDungeonGridScRsp => 4,
            CmdTreasureDungeonType::CmdInteractTreasureDungeonGridCsReq => 5,
            CmdTreasureDungeonType::CmdGetTreasureDungeonActivityDataScRsp => 6,
            CmdTreasureDungeonType::CmdFightTreasureDungeonMonsterScRsp => 7,
            CmdTreasureDungeonType::CmdQuitTreasureDungeonCsReq => 8,
            CmdTreasureDungeonType::CmdFightTreasureDungeonMonsterCsReq => 9,
            CmdTreasureDungeonType::CmdUseTreasureDungeonItemCsReq => 10,
            CmdTreasureDungeonType::CmdOpenTreasureDungeonGridScRsp => 11,
            CmdTreasureDungeonType::CmdQuitTreasureDungeonScRsp => 12,
            CmdTreasureDungeonType::CmdTreasureDungeonFinishScNotify => 13,
            CmdTreasureDungeonType::CmdGetTreasureDungeonActivityDataCsReq => 14,
            CmdTreasureDungeonType::CmdUseTreasureDungeonItemScRsp => 15,
            CmdTreasureDungeonType::CmdEnterTreasureDungeonScRsp => 16,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdTreasureDungeonType {
    fn default() -> Self {
        CmdTreasureDungeonType::CmdTreasureDungeonTypeNone
    }
}

impl CmdTreasureDungeonType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdTreasureDungeonType>("CmdTreasureDungeonType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cCmdTreasureDungeonType.proto*\xa6\x05\n\x16CmdTreasureDungeonType\
    \x12\x1e\n\x1aCmdTreasureDungeonTypeNone\x10\0\x12!\n\x1cCmdEnterTreasur\
    eDungeonCsReq\x10\xd4\"\x12$\n\x1fCmdOpenTreasureDungeonGridCsReq\x10\
    \xf9\"\x12#\n\x1eCmdTreasureDungeonDataScNotify\x10\xbb\"\x12(\n#CmdInte\
    ractTreasureDungeonGridScRsp\x10\xc9\"\x12(\n#CmdInteractTreasureDungeon\
    GridCsReq\x10\xe9\"\x12+\n&CmdGetTreasureDungeonActivityDataScRsp\x10\
    \xc2\"\x12(\n#CmdFightTreasureDungeonMonsterScRsp\x10\x8d#\x12\x20\n\x1b\
    CmdQuitTreasureDungeonCsReq\x10\xf7\"\x12(\n#CmdFightTreasureDungeonMons\
    terCsReq\x10\x8b#\x12#\n\x1eCmdUseTreasureDungeonItemCsReq\x10\xba\"\x12\
    $\n\x1fCmdOpenTreasureDungeonGridScRsp\x10\xfd\"\x12\x20\n\x1bCmdQuitTre\
    asureDungeonScRsp\x10\x82#\x12%\n\x20CmdTreasureDungeonFinishScNotify\
    \x10\xbd\"\x12+\n&CmdGetTreasureDungeonActivityDataCsReq\x10\x8f#\x12#\n\
    \x1eCmdUseTreasureDungeonItemScRsp\x10\xb7\"\x12!\n\x1cCmdEnterTreasureD\
    ungeonScRsp\x10\xe2\"b\x06proto3\
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
            enums.push(CmdTreasureDungeonType::generated_enum_descriptor_data());
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
