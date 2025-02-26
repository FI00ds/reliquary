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

//! Generated file from `CmdOfferingType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdOfferingType)
pub enum CmdOfferingType {
    // @@protoc_insertion_point(enum_value:CmdOfferingType.CmdOfferingTypeNone)
    CmdOfferingTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdOfferingType.CmdSubmitOfferingItemScRsp)
    CmdSubmitOfferingItemScRsp = 6922,
    // @@protoc_insertion_point(enum_value:CmdOfferingType.CmdTakeOfferingRewardScRsp)
    CmdTakeOfferingRewardScRsp = 6921,
    // @@protoc_insertion_point(enum_value:CmdOfferingType.CmdGetOfferingInfoCsReq)
    CmdGetOfferingInfoCsReq = 6937,
    // @@protoc_insertion_point(enum_value:CmdOfferingType.CmdSubmitOfferingItemCsReq)
    CmdSubmitOfferingItemCsReq = 6932,
    // @@protoc_insertion_point(enum_value:CmdOfferingType.CmdTakeOfferingRewardCsReq)
    CmdTakeOfferingRewardCsReq = 6924,
    // @@protoc_insertion_point(enum_value:CmdOfferingType.CmdOfferingInfoScNotify)
    CmdOfferingInfoScNotify = 6939,
    // @@protoc_insertion_point(enum_value:CmdOfferingType.CmdGetOfferingInfoScRsp)
    CmdGetOfferingInfoScRsp = 6928,
}

impl ::protobuf::Enum for CmdOfferingType {
    const NAME: &'static str = "CmdOfferingType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdOfferingType> {
        match value {
            0 => ::std::option::Option::Some(CmdOfferingType::CmdOfferingTypeNone),
            6922 => ::std::option::Option::Some(CmdOfferingType::CmdSubmitOfferingItemScRsp),
            6921 => ::std::option::Option::Some(CmdOfferingType::CmdTakeOfferingRewardScRsp),
            6937 => ::std::option::Option::Some(CmdOfferingType::CmdGetOfferingInfoCsReq),
            6932 => ::std::option::Option::Some(CmdOfferingType::CmdSubmitOfferingItemCsReq),
            6924 => ::std::option::Option::Some(CmdOfferingType::CmdTakeOfferingRewardCsReq),
            6939 => ::std::option::Option::Some(CmdOfferingType::CmdOfferingInfoScNotify),
            6928 => ::std::option::Option::Some(CmdOfferingType::CmdGetOfferingInfoScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdOfferingType> {
        match str {
            "CmdOfferingTypeNone" => ::std::option::Option::Some(CmdOfferingType::CmdOfferingTypeNone),
            "CmdSubmitOfferingItemScRsp" => ::std::option::Option::Some(CmdOfferingType::CmdSubmitOfferingItemScRsp),
            "CmdTakeOfferingRewardScRsp" => ::std::option::Option::Some(CmdOfferingType::CmdTakeOfferingRewardScRsp),
            "CmdGetOfferingInfoCsReq" => ::std::option::Option::Some(CmdOfferingType::CmdGetOfferingInfoCsReq),
            "CmdSubmitOfferingItemCsReq" => ::std::option::Option::Some(CmdOfferingType::CmdSubmitOfferingItemCsReq),
            "CmdTakeOfferingRewardCsReq" => ::std::option::Option::Some(CmdOfferingType::CmdTakeOfferingRewardCsReq),
            "CmdOfferingInfoScNotify" => ::std::option::Option::Some(CmdOfferingType::CmdOfferingInfoScNotify),
            "CmdGetOfferingInfoScRsp" => ::std::option::Option::Some(CmdOfferingType::CmdGetOfferingInfoScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdOfferingType] = &[
        CmdOfferingType::CmdOfferingTypeNone,
        CmdOfferingType::CmdSubmitOfferingItemScRsp,
        CmdOfferingType::CmdTakeOfferingRewardScRsp,
        CmdOfferingType::CmdGetOfferingInfoCsReq,
        CmdOfferingType::CmdSubmitOfferingItemCsReq,
        CmdOfferingType::CmdTakeOfferingRewardCsReq,
        CmdOfferingType::CmdOfferingInfoScNotify,
        CmdOfferingType::CmdGetOfferingInfoScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdOfferingType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdOfferingType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdOfferingType::CmdOfferingTypeNone => 0,
            CmdOfferingType::CmdSubmitOfferingItemScRsp => 1,
            CmdOfferingType::CmdTakeOfferingRewardScRsp => 2,
            CmdOfferingType::CmdGetOfferingInfoCsReq => 3,
            CmdOfferingType::CmdSubmitOfferingItemCsReq => 4,
            CmdOfferingType::CmdTakeOfferingRewardCsReq => 5,
            CmdOfferingType::CmdOfferingInfoScNotify => 6,
            CmdOfferingType::CmdGetOfferingInfoScRsp => 7,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdOfferingType {
    fn default() -> Self {
        CmdOfferingType::CmdOfferingTypeNone
    }
}

impl CmdOfferingType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdOfferingType>("CmdOfferingType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15CmdOfferingType.proto*\x88\x02\n\x0fCmdOfferingType\x12\x17\n\x13C\
    mdOfferingTypeNone\x10\0\x12\x1f\n\x1aCmdSubmitOfferingItemScRsp\x10\x8a\
    6\x12\x1f\n\x1aCmdTakeOfferingRewardScRsp\x10\x896\x12\x1c\n\x17CmdGetOf\
    feringInfoCsReq\x10\x996\x12\x1f\n\x1aCmdSubmitOfferingItemCsReq\x10\x94\
    6\x12\x1f\n\x1aCmdTakeOfferingRewardCsReq\x10\x8c6\x12\x1c\n\x17CmdOffer\
    ingInfoScNotify\x10\x9b6\x12\x1c\n\x17CmdGetOfferingInfoScRsp\x10\x906b\
    \x06proto3\
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
            enums.push(CmdOfferingType::generated_enum_descriptor_data());
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
