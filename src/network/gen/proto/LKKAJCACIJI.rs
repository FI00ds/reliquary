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

//! Generated file from `LKKAJCACIJI.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:LKKAJCACIJI)
pub enum LKKAJCACIJI {
    // @@protoc_insertion_point(enum_value:LKKAJCACIJI.MARBLE_FACTION_TYPE_NONE)
    MARBLE_FACTION_TYPE_NONE = 0,
    // @@protoc_insertion_point(enum_value:LKKAJCACIJI.MARBLE_FACTION_TYPE_ALL)
    MARBLE_FACTION_TYPE_ALL = 1,
    // @@protoc_insertion_point(enum_value:LKKAJCACIJI.MARBLE_FACTION_TYPE_ENEMY)
    MARBLE_FACTION_TYPE_ENEMY = 2,
    // @@protoc_insertion_point(enum_value:LKKAJCACIJI.MARBLE_FACTION_TYPE_ALLY)
    MARBLE_FACTION_TYPE_ALLY = 3,
    // @@protoc_insertion_point(enum_value:LKKAJCACIJI.MARBLE_FACTION_TYPE_FIELD)
    MARBLE_FACTION_TYPE_FIELD = 4,
}

impl ::protobuf::Enum for LKKAJCACIJI {
    const NAME: &'static str = "LKKAJCACIJI";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<LKKAJCACIJI> {
        match value {
            0 => ::std::option::Option::Some(LKKAJCACIJI::MARBLE_FACTION_TYPE_NONE),
            1 => ::std::option::Option::Some(LKKAJCACIJI::MARBLE_FACTION_TYPE_ALL),
            2 => ::std::option::Option::Some(LKKAJCACIJI::MARBLE_FACTION_TYPE_ENEMY),
            3 => ::std::option::Option::Some(LKKAJCACIJI::MARBLE_FACTION_TYPE_ALLY),
            4 => ::std::option::Option::Some(LKKAJCACIJI::MARBLE_FACTION_TYPE_FIELD),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<LKKAJCACIJI> {
        match str {
            "MARBLE_FACTION_TYPE_NONE" => ::std::option::Option::Some(LKKAJCACIJI::MARBLE_FACTION_TYPE_NONE),
            "MARBLE_FACTION_TYPE_ALL" => ::std::option::Option::Some(LKKAJCACIJI::MARBLE_FACTION_TYPE_ALL),
            "MARBLE_FACTION_TYPE_ENEMY" => ::std::option::Option::Some(LKKAJCACIJI::MARBLE_FACTION_TYPE_ENEMY),
            "MARBLE_FACTION_TYPE_ALLY" => ::std::option::Option::Some(LKKAJCACIJI::MARBLE_FACTION_TYPE_ALLY),
            "MARBLE_FACTION_TYPE_FIELD" => ::std::option::Option::Some(LKKAJCACIJI::MARBLE_FACTION_TYPE_FIELD),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [LKKAJCACIJI] = &[
        LKKAJCACIJI::MARBLE_FACTION_TYPE_NONE,
        LKKAJCACIJI::MARBLE_FACTION_TYPE_ALL,
        LKKAJCACIJI::MARBLE_FACTION_TYPE_ENEMY,
        LKKAJCACIJI::MARBLE_FACTION_TYPE_ALLY,
        LKKAJCACIJI::MARBLE_FACTION_TYPE_FIELD,
    ];
}

impl ::protobuf::EnumFull for LKKAJCACIJI {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("LKKAJCACIJI").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for LKKAJCACIJI {
    fn default() -> Self {
        LKKAJCACIJI::MARBLE_FACTION_TYPE_NONE
    }
}

impl LKKAJCACIJI {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<LKKAJCACIJI>("LKKAJCACIJI")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LKKAJCACIJI.proto*\xa4\x01\n\x0bLKKAJCACIJI\x12\x1c\n\x18MARBLE_FA\
    CTION_TYPE_NONE\x10\0\x12\x1b\n\x17MARBLE_FACTION_TYPE_ALL\x10\x01\x12\
    \x1d\n\x19MARBLE_FACTION_TYPE_ENEMY\x10\x02\x12\x1c\n\x18MARBLE_FACTION_\
    TYPE_ALLY\x10\x03\x12\x1d\n\x19MARBLE_FACTION_TYPE_FIELD\x10\x04b\x06pro\
    to3\
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
            enums.push(LKKAJCACIJI::generated_enum_descriptor_data());
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
