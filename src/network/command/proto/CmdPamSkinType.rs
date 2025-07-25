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

//! Generated file from `CmdPamSkinType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdPamSkinType)
pub enum CmdPamSkinType {
    // @@protoc_insertion_point(enum_value:CmdPamSkinType.CmdPamSkinTypeNone)
    CmdPamSkinTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdPamSkinType.CmdSelectPamSkinScRsp)
    CmdSelectPamSkinScRsp = 8136,
    // @@protoc_insertion_point(enum_value:CmdPamSkinType.CmdGetPamSkinDataScRsp)
    CmdGetPamSkinDataScRsp = 8129,
    // @@protoc_insertion_point(enum_value:CmdPamSkinType.CmdUnlockPamSkinScNotify)
    CmdUnlockPamSkinScNotify = 8127,
    // @@protoc_insertion_point(enum_value:CmdPamSkinType.CmdGetPamSkinDataCsReq)
    CmdGetPamSkinDataCsReq = 8138,
    // @@protoc_insertion_point(enum_value:CmdPamSkinType.CmdSelectPamSkinCsReq)
    CmdSelectPamSkinCsReq = 8130,
}

impl ::protobuf::Enum for CmdPamSkinType {
    const NAME: &'static str = "CmdPamSkinType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdPamSkinType> {
        match value {
            0 => ::std::option::Option::Some(CmdPamSkinType::CmdPamSkinTypeNone),
            8136 => ::std::option::Option::Some(CmdPamSkinType::CmdSelectPamSkinScRsp),
            8129 => ::std::option::Option::Some(CmdPamSkinType::CmdGetPamSkinDataScRsp),
            8127 => ::std::option::Option::Some(CmdPamSkinType::CmdUnlockPamSkinScNotify),
            8138 => ::std::option::Option::Some(CmdPamSkinType::CmdGetPamSkinDataCsReq),
            8130 => ::std::option::Option::Some(CmdPamSkinType::CmdSelectPamSkinCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdPamSkinType> {
        match str {
            "CmdPamSkinTypeNone" => ::std::option::Option::Some(CmdPamSkinType::CmdPamSkinTypeNone),
            "CmdSelectPamSkinScRsp" => ::std::option::Option::Some(CmdPamSkinType::CmdSelectPamSkinScRsp),
            "CmdGetPamSkinDataScRsp" => ::std::option::Option::Some(CmdPamSkinType::CmdGetPamSkinDataScRsp),
            "CmdUnlockPamSkinScNotify" => ::std::option::Option::Some(CmdPamSkinType::CmdUnlockPamSkinScNotify),
            "CmdGetPamSkinDataCsReq" => ::std::option::Option::Some(CmdPamSkinType::CmdGetPamSkinDataCsReq),
            "CmdSelectPamSkinCsReq" => ::std::option::Option::Some(CmdPamSkinType::CmdSelectPamSkinCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdPamSkinType] = &[
        CmdPamSkinType::CmdPamSkinTypeNone,
        CmdPamSkinType::CmdSelectPamSkinScRsp,
        CmdPamSkinType::CmdGetPamSkinDataScRsp,
        CmdPamSkinType::CmdUnlockPamSkinScNotify,
        CmdPamSkinType::CmdGetPamSkinDataCsReq,
        CmdPamSkinType::CmdSelectPamSkinCsReq,
    ];
}

impl ::protobuf::EnumFull for CmdPamSkinType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdPamSkinType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdPamSkinType::CmdPamSkinTypeNone => 0,
            CmdPamSkinType::CmdSelectPamSkinScRsp => 1,
            CmdPamSkinType::CmdGetPamSkinDataScRsp => 2,
            CmdPamSkinType::CmdUnlockPamSkinScNotify => 3,
            CmdPamSkinType::CmdGetPamSkinDataCsReq => 4,
            CmdPamSkinType::CmdSelectPamSkinCsReq => 5,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdPamSkinType {
    fn default() -> Self {
        CmdPamSkinType::CmdPamSkinTypeNone
    }
}

impl CmdPamSkinType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdPamSkinType>("CmdPamSkinType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14CmdPamSkinType.proto*\xb9\x01\n\x0eCmdPamSkinType\x12\x16\n\x12Cmd\
    PamSkinTypeNone\x10\0\x12\x1a\n\x15CmdSelectPamSkinScRsp\x10\xc8?\x12\
    \x1b\n\x16CmdGetPamSkinDataScRsp\x10\xc1?\x12\x1d\n\x18CmdUnlockPamSkinS\
    cNotify\x10\xbf?\x12\x1b\n\x16CmdGetPamSkinDataCsReq\x10\xca?\x12\x1a\n\
    \x15CmdSelectPamSkinCsReq\x10\xc2?b\x06proto3\
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
            enums.push(CmdPamSkinType::generated_enum_descriptor_data());
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
