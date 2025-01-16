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

//! Generated file from `FKFMMNOCIPK.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:FKFMMNOCIPK)
pub enum FKFMMNOCIPK {
    // @@protoc_insertion_point(enum_value:FKFMMNOCIPK.REBATTLE_TYPE_NONE)
    REBATTLE_TYPE_NONE = 0,
    // @@protoc_insertion_point(enum_value:FKFMMNOCIPK.REBATTLE_TYPE_REBATTLE_MIDWAY)
    REBATTLE_TYPE_REBATTLE_MIDWAY = 1,
    // @@protoc_insertion_point(enum_value:FKFMMNOCIPK.REBATTLE_TYPE_REBATTLE_LOSE)
    REBATTLE_TYPE_REBATTLE_LOSE = 2,
    // @@protoc_insertion_point(enum_value:FKFMMNOCIPK.REBATTLE_TYPE_REBATTLE_MIDWAY_LINEUP)
    REBATTLE_TYPE_REBATTLE_MIDWAY_LINEUP = 3,
    // @@protoc_insertion_point(enum_value:FKFMMNOCIPK.REBATTLE_TYPE_REBATTLE_LOSE_LINEUP)
    REBATTLE_TYPE_REBATTLE_LOSE_LINEUP = 4,
    // @@protoc_insertion_point(enum_value:FKFMMNOCIPK.REBATTLE_TYPE_QUIT_MIDWAY)
    REBATTLE_TYPE_QUIT_MIDWAY = 5,
    // @@protoc_insertion_point(enum_value:FKFMMNOCIPK.REBATTLE_TYPE_QUIT_LOSE)
    REBATTLE_TYPE_QUIT_LOSE = 6,
}

impl ::protobuf::Enum for FKFMMNOCIPK {
    const NAME: &'static str = "FKFMMNOCIPK";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<FKFMMNOCIPK> {
        match value {
            0 => ::std::option::Option::Some(FKFMMNOCIPK::REBATTLE_TYPE_NONE),
            1 => ::std::option::Option::Some(FKFMMNOCIPK::REBATTLE_TYPE_REBATTLE_MIDWAY),
            2 => ::std::option::Option::Some(FKFMMNOCIPK::REBATTLE_TYPE_REBATTLE_LOSE),
            3 => ::std::option::Option::Some(FKFMMNOCIPK::REBATTLE_TYPE_REBATTLE_MIDWAY_LINEUP),
            4 => ::std::option::Option::Some(FKFMMNOCIPK::REBATTLE_TYPE_REBATTLE_LOSE_LINEUP),
            5 => ::std::option::Option::Some(FKFMMNOCIPK::REBATTLE_TYPE_QUIT_MIDWAY),
            6 => ::std::option::Option::Some(FKFMMNOCIPK::REBATTLE_TYPE_QUIT_LOSE),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<FKFMMNOCIPK> {
        match str {
            "REBATTLE_TYPE_NONE" => ::std::option::Option::Some(FKFMMNOCIPK::REBATTLE_TYPE_NONE),
            "REBATTLE_TYPE_REBATTLE_MIDWAY" => ::std::option::Option::Some(FKFMMNOCIPK::REBATTLE_TYPE_REBATTLE_MIDWAY),
            "REBATTLE_TYPE_REBATTLE_LOSE" => ::std::option::Option::Some(FKFMMNOCIPK::REBATTLE_TYPE_REBATTLE_LOSE),
            "REBATTLE_TYPE_REBATTLE_MIDWAY_LINEUP" => ::std::option::Option::Some(FKFMMNOCIPK::REBATTLE_TYPE_REBATTLE_MIDWAY_LINEUP),
            "REBATTLE_TYPE_REBATTLE_LOSE_LINEUP" => ::std::option::Option::Some(FKFMMNOCIPK::REBATTLE_TYPE_REBATTLE_LOSE_LINEUP),
            "REBATTLE_TYPE_QUIT_MIDWAY" => ::std::option::Option::Some(FKFMMNOCIPK::REBATTLE_TYPE_QUIT_MIDWAY),
            "REBATTLE_TYPE_QUIT_LOSE" => ::std::option::Option::Some(FKFMMNOCIPK::REBATTLE_TYPE_QUIT_LOSE),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [FKFMMNOCIPK] = &[
        FKFMMNOCIPK::REBATTLE_TYPE_NONE,
        FKFMMNOCIPK::REBATTLE_TYPE_REBATTLE_MIDWAY,
        FKFMMNOCIPK::REBATTLE_TYPE_REBATTLE_LOSE,
        FKFMMNOCIPK::REBATTLE_TYPE_REBATTLE_MIDWAY_LINEUP,
        FKFMMNOCIPK::REBATTLE_TYPE_REBATTLE_LOSE_LINEUP,
        FKFMMNOCIPK::REBATTLE_TYPE_QUIT_MIDWAY,
        FKFMMNOCIPK::REBATTLE_TYPE_QUIT_LOSE,
    ];
}

impl ::protobuf::EnumFull for FKFMMNOCIPK {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("FKFMMNOCIPK").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for FKFMMNOCIPK {
    fn default() -> Self {
        FKFMMNOCIPK::REBATTLE_TYPE_NONE
    }
}

impl FKFMMNOCIPK {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<FKFMMNOCIPK>("FKFMMNOCIPK")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11FKFMMNOCIPK.proto*\xf7\x01\n\x0bFKFMMNOCIPK\x12\x16\n\x12REBATTLE_\
    TYPE_NONE\x10\0\x12!\n\x1dREBATTLE_TYPE_REBATTLE_MIDWAY\x10\x01\x12\x1f\
    \n\x1bREBATTLE_TYPE_REBATTLE_LOSE\x10\x02\x12(\n$REBATTLE_TYPE_REBATTLE_\
    MIDWAY_LINEUP\x10\x03\x12&\n\"REBATTLE_TYPE_REBATTLE_LOSE_LINEUP\x10\x04\
    \x12\x1d\n\x19REBATTLE_TYPE_QUIT_MIDWAY\x10\x05\x12\x1b\n\x17REBATTLE_TY\
    PE_QUIT_LOSE\x10\x06b\x06proto3\
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
            enums.push(FKFMMNOCIPK::generated_enum_descriptor_data());
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
