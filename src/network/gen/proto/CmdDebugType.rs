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

//! Generated file from `CmdDebugType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdDebugType)
pub enum CmdDebugType {
    // @@protoc_insertion_point(enum_value:CmdDebugType.CmdDebugTypeNone)
    CmdDebugTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdDebugType.CmdGetServerLogSettingsScRsp)
    CmdGetServerLogSettingsScRsp = 2468,
    // @@protoc_insertion_point(enum_value:CmdDebugType.CmdGetServerLogSettingsCsReq)
    CmdGetServerLogSettingsCsReq = 2492,
    // @@protoc_insertion_point(enum_value:CmdDebugType.CmdUpdateServerLogSettingsCsReq)
    CmdUpdateServerLogSettingsCsReq = 2464,
    // @@protoc_insertion_point(enum_value:CmdDebugType.CmdGetServerGraphDataCsReq)
    CmdGetServerGraphDataCsReq = 2459,
    // @@protoc_insertion_point(enum_value:CmdDebugType.CmdGetServerGraphDataScRsp)
    CmdGetServerGraphDataScRsp = 2479,
    // @@protoc_insertion_point(enum_value:CmdDebugType.CmdUpdateServerLogSettingsScRsp)
    CmdUpdateServerLogSettingsScRsp = 2456,
    // @@protoc_insertion_point(enum_value:CmdDebugType.CmdServerLogScNotify)
    CmdServerLogScNotify = 2491,
}

impl ::protobuf::Enum for CmdDebugType {
    const NAME: &'static str = "CmdDebugType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdDebugType> {
        match value {
            0 => ::std::option::Option::Some(CmdDebugType::CmdDebugTypeNone),
            2468 => ::std::option::Option::Some(CmdDebugType::CmdGetServerLogSettingsScRsp),
            2492 => ::std::option::Option::Some(CmdDebugType::CmdGetServerLogSettingsCsReq),
            2464 => ::std::option::Option::Some(CmdDebugType::CmdUpdateServerLogSettingsCsReq),
            2459 => ::std::option::Option::Some(CmdDebugType::CmdGetServerGraphDataCsReq),
            2479 => ::std::option::Option::Some(CmdDebugType::CmdGetServerGraphDataScRsp),
            2456 => ::std::option::Option::Some(CmdDebugType::CmdUpdateServerLogSettingsScRsp),
            2491 => ::std::option::Option::Some(CmdDebugType::CmdServerLogScNotify),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdDebugType> {
        match str {
            "CmdDebugTypeNone" => ::std::option::Option::Some(CmdDebugType::CmdDebugTypeNone),
            "CmdGetServerLogSettingsScRsp" => ::std::option::Option::Some(CmdDebugType::CmdGetServerLogSettingsScRsp),
            "CmdGetServerLogSettingsCsReq" => ::std::option::Option::Some(CmdDebugType::CmdGetServerLogSettingsCsReq),
            "CmdUpdateServerLogSettingsCsReq" => ::std::option::Option::Some(CmdDebugType::CmdUpdateServerLogSettingsCsReq),
            "CmdGetServerGraphDataCsReq" => ::std::option::Option::Some(CmdDebugType::CmdGetServerGraphDataCsReq),
            "CmdGetServerGraphDataScRsp" => ::std::option::Option::Some(CmdDebugType::CmdGetServerGraphDataScRsp),
            "CmdUpdateServerLogSettingsScRsp" => ::std::option::Option::Some(CmdDebugType::CmdUpdateServerLogSettingsScRsp),
            "CmdServerLogScNotify" => ::std::option::Option::Some(CmdDebugType::CmdServerLogScNotify),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdDebugType] = &[
        CmdDebugType::CmdDebugTypeNone,
        CmdDebugType::CmdGetServerLogSettingsScRsp,
        CmdDebugType::CmdGetServerLogSettingsCsReq,
        CmdDebugType::CmdUpdateServerLogSettingsCsReq,
        CmdDebugType::CmdGetServerGraphDataCsReq,
        CmdDebugType::CmdGetServerGraphDataScRsp,
        CmdDebugType::CmdUpdateServerLogSettingsScRsp,
        CmdDebugType::CmdServerLogScNotify,
    ];
}

impl ::protobuf::EnumFull for CmdDebugType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdDebugType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdDebugType::CmdDebugTypeNone => 0,
            CmdDebugType::CmdGetServerLogSettingsScRsp => 1,
            CmdDebugType::CmdGetServerLogSettingsCsReq => 2,
            CmdDebugType::CmdUpdateServerLogSettingsCsReq => 3,
            CmdDebugType::CmdGetServerGraphDataCsReq => 4,
            CmdDebugType::CmdGetServerGraphDataScRsp => 5,
            CmdDebugType::CmdUpdateServerLogSettingsScRsp => 6,
            CmdDebugType::CmdServerLogScNotify => 7,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdDebugType {
    fn default() -> Self {
        CmdDebugType::CmdDebugTypeNone
    }
}

impl CmdDebugType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdDebugType>("CmdDebugType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12CmdDebugType.proto*\x93\x02\n\x0cCmdDebugType\x12\x14\n\x10CmdDebu\
    gTypeNone\x10\0\x12!\n\x1cCmdGetServerLogSettingsScRsp\x10\xa4\x13\x12!\
    \n\x1cCmdGetServerLogSettingsCsReq\x10\xbc\x13\x12$\n\x1fCmdUpdateServer\
    LogSettingsCsReq\x10\xa0\x13\x12\x1f\n\x1aCmdGetServerGraphDataCsReq\x10\
    \x9b\x13\x12\x1f\n\x1aCmdGetServerGraphDataScRsp\x10\xaf\x13\x12$\n\x1fC\
    mdUpdateServerLogSettingsScRsp\x10\x98\x13\x12\x19\n\x14CmdServerLogScNo\
    tify\x10\xbb\x13b\x06proto3\
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
            enums.push(CmdDebugType::generated_enum_descriptor_data());
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
