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

//! Generated file from `OPGBOLCJPJE.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:OPGBOLCJPJE)
pub enum OPGBOLCJPJE {
    // @@protoc_insertion_point(enum_value:OPGBOLCJPJE.FIGHT_ROOM_DESTROY_REASON_NONE)
    FIGHT_ROOM_DESTROY_REASON_NONE = 0,
    // @@protoc_insertion_point(enum_value:OPGBOLCJPJE.FIGHT_ROOM_DESTROY_REASON_SVR_STOP)
    FIGHT_ROOM_DESTROY_REASON_SVR_STOP = 1,
    // @@protoc_insertion_point(enum_value:OPGBOLCJPJE.FIGHT_ROOM_DESTROY_REASON_GAME_END)
    FIGHT_ROOM_DESTROY_REASON_GAME_END = 2,
}

impl ::protobuf::Enum for OPGBOLCJPJE {
    const NAME: &'static str = "OPGBOLCJPJE";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<OPGBOLCJPJE> {
        match value {
            0 => ::std::option::Option::Some(OPGBOLCJPJE::FIGHT_ROOM_DESTROY_REASON_NONE),
            1 => ::std::option::Option::Some(OPGBOLCJPJE::FIGHT_ROOM_DESTROY_REASON_SVR_STOP),
            2 => ::std::option::Option::Some(OPGBOLCJPJE::FIGHT_ROOM_DESTROY_REASON_GAME_END),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<OPGBOLCJPJE> {
        match str {
            "FIGHT_ROOM_DESTROY_REASON_NONE" => ::std::option::Option::Some(OPGBOLCJPJE::FIGHT_ROOM_DESTROY_REASON_NONE),
            "FIGHT_ROOM_DESTROY_REASON_SVR_STOP" => ::std::option::Option::Some(OPGBOLCJPJE::FIGHT_ROOM_DESTROY_REASON_SVR_STOP),
            "FIGHT_ROOM_DESTROY_REASON_GAME_END" => ::std::option::Option::Some(OPGBOLCJPJE::FIGHT_ROOM_DESTROY_REASON_GAME_END),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [OPGBOLCJPJE] = &[
        OPGBOLCJPJE::FIGHT_ROOM_DESTROY_REASON_NONE,
        OPGBOLCJPJE::FIGHT_ROOM_DESTROY_REASON_SVR_STOP,
        OPGBOLCJPJE::FIGHT_ROOM_DESTROY_REASON_GAME_END,
    ];
}

impl ::protobuf::EnumFull for OPGBOLCJPJE {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("OPGBOLCJPJE").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for OPGBOLCJPJE {
    fn default() -> Self {
        OPGBOLCJPJE::FIGHT_ROOM_DESTROY_REASON_NONE
    }
}

impl OPGBOLCJPJE {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<OPGBOLCJPJE>("OPGBOLCJPJE")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11OPGBOLCJPJE.proto*\x81\x01\n\x0bOPGBOLCJPJE\x12\"\n\x1eFIGHT_ROOM_\
    DESTROY_REASON_NONE\x10\0\x12&\n\"FIGHT_ROOM_DESTROY_REASON_SVR_STOP\x10\
    \x01\x12&\n\"FIGHT_ROOM_DESTROY_REASON_GAME_END\x10\x02b\x06proto3\
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
            enums.push(OPGBOLCJPJE::generated_enum_descriptor_data());
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
