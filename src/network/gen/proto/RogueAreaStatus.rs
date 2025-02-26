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

//! Generated file from `RogueAreaStatus.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:RogueAreaStatus)
pub enum RogueAreaStatus {
    // @@protoc_insertion_point(enum_value:RogueAreaStatus.ROGUE_AREA_STATUS_LOCK)
    ROGUE_AREA_STATUS_LOCK = 0,
    // @@protoc_insertion_point(enum_value:RogueAreaStatus.ROGUE_AREA_STATUS_UNLOCK)
    ROGUE_AREA_STATUS_UNLOCK = 1,
    // @@protoc_insertion_point(enum_value:RogueAreaStatus.ROGUE_AREA_STATUS_FIRST_PASS)
    ROGUE_AREA_STATUS_FIRST_PASS = 2,
    // @@protoc_insertion_point(enum_value:RogueAreaStatus.ROGUE_AREA_STATUS_CLOSE)
    ROGUE_AREA_STATUS_CLOSE = 3,
}

impl ::protobuf::Enum for RogueAreaStatus {
    const NAME: &'static str = "RogueAreaStatus";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RogueAreaStatus> {
        match value {
            0 => ::std::option::Option::Some(RogueAreaStatus::ROGUE_AREA_STATUS_LOCK),
            1 => ::std::option::Option::Some(RogueAreaStatus::ROGUE_AREA_STATUS_UNLOCK),
            2 => ::std::option::Option::Some(RogueAreaStatus::ROGUE_AREA_STATUS_FIRST_PASS),
            3 => ::std::option::Option::Some(RogueAreaStatus::ROGUE_AREA_STATUS_CLOSE),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<RogueAreaStatus> {
        match str {
            "ROGUE_AREA_STATUS_LOCK" => ::std::option::Option::Some(RogueAreaStatus::ROGUE_AREA_STATUS_LOCK),
            "ROGUE_AREA_STATUS_UNLOCK" => ::std::option::Option::Some(RogueAreaStatus::ROGUE_AREA_STATUS_UNLOCK),
            "ROGUE_AREA_STATUS_FIRST_PASS" => ::std::option::Option::Some(RogueAreaStatus::ROGUE_AREA_STATUS_FIRST_PASS),
            "ROGUE_AREA_STATUS_CLOSE" => ::std::option::Option::Some(RogueAreaStatus::ROGUE_AREA_STATUS_CLOSE),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [RogueAreaStatus] = &[
        RogueAreaStatus::ROGUE_AREA_STATUS_LOCK,
        RogueAreaStatus::ROGUE_AREA_STATUS_UNLOCK,
        RogueAreaStatus::ROGUE_AREA_STATUS_FIRST_PASS,
        RogueAreaStatus::ROGUE_AREA_STATUS_CLOSE,
    ];
}

impl ::protobuf::EnumFull for RogueAreaStatus {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("RogueAreaStatus").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for RogueAreaStatus {
    fn default() -> Self {
        RogueAreaStatus::ROGUE_AREA_STATUS_LOCK
    }
}

impl RogueAreaStatus {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<RogueAreaStatus>("RogueAreaStatus")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15RogueAreaStatus.proto*\x8a\x01\n\x0fRogueAreaStatus\x12\x1a\n\x16R\
    OGUE_AREA_STATUS_LOCK\x10\0\x12\x1c\n\x18ROGUE_AREA_STATUS_UNLOCK\x10\
    \x01\x12\x20\n\x1cROGUE_AREA_STATUS_FIRST_PASS\x10\x02\x12\x1b\n\x17ROGU\
    E_AREA_STATUS_CLOSE\x10\x03b\x06proto3\
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
            enums.push(RogueAreaStatus::generated_enum_descriptor_data());
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
