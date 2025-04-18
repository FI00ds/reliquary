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

//! Generated file from `HDMKPHALALG.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:HDMKPHALALG)
pub enum HDMKPHALALG {
    // @@protoc_insertion_point(enum_value:HDMKPHALALG.SWORD_TRAINING_GAME_SETTLE_NONE)
    SWORD_TRAINING_GAME_SETTLE_NONE = 0,
    // @@protoc_insertion_point(enum_value:HDMKPHALALG.SWORD_TRAINING_GAME_SETTLE_FINISH)
    SWORD_TRAINING_GAME_SETTLE_FINISH = 1,
    // @@protoc_insertion_point(enum_value:HDMKPHALALG.SWORD_TRAINING_GAME_SETTLE_GIVE_UP)
    SWORD_TRAINING_GAME_SETTLE_GIVE_UP = 2,
    // @@protoc_insertion_point(enum_value:HDMKPHALALG.SWORD_TRAINING_GAME_SETTLE_BATTLE_FAILED)
    SWORD_TRAINING_GAME_SETTLE_BATTLE_FAILED = 3,
    // @@protoc_insertion_point(enum_value:HDMKPHALALG.SWORD_TRAINING_GAME_SETTLE_FORCE)
    SWORD_TRAINING_GAME_SETTLE_FORCE = 4,
    // @@protoc_insertion_point(enum_value:HDMKPHALALG.SWORD_TRAINING_GAME_SETTLE_BY_RESTORE)
    SWORD_TRAINING_GAME_SETTLE_BY_RESTORE = 5,
}

impl ::protobuf::Enum for HDMKPHALALG {
    const NAME: &'static str = "HDMKPHALALG";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<HDMKPHALALG> {
        match value {
            0 => ::std::option::Option::Some(HDMKPHALALG::SWORD_TRAINING_GAME_SETTLE_NONE),
            1 => ::std::option::Option::Some(HDMKPHALALG::SWORD_TRAINING_GAME_SETTLE_FINISH),
            2 => ::std::option::Option::Some(HDMKPHALALG::SWORD_TRAINING_GAME_SETTLE_GIVE_UP),
            3 => ::std::option::Option::Some(HDMKPHALALG::SWORD_TRAINING_GAME_SETTLE_BATTLE_FAILED),
            4 => ::std::option::Option::Some(HDMKPHALALG::SWORD_TRAINING_GAME_SETTLE_FORCE),
            5 => ::std::option::Option::Some(HDMKPHALALG::SWORD_TRAINING_GAME_SETTLE_BY_RESTORE),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<HDMKPHALALG> {
        match str {
            "SWORD_TRAINING_GAME_SETTLE_NONE" => ::std::option::Option::Some(HDMKPHALALG::SWORD_TRAINING_GAME_SETTLE_NONE),
            "SWORD_TRAINING_GAME_SETTLE_FINISH" => ::std::option::Option::Some(HDMKPHALALG::SWORD_TRAINING_GAME_SETTLE_FINISH),
            "SWORD_TRAINING_GAME_SETTLE_GIVE_UP" => ::std::option::Option::Some(HDMKPHALALG::SWORD_TRAINING_GAME_SETTLE_GIVE_UP),
            "SWORD_TRAINING_GAME_SETTLE_BATTLE_FAILED" => ::std::option::Option::Some(HDMKPHALALG::SWORD_TRAINING_GAME_SETTLE_BATTLE_FAILED),
            "SWORD_TRAINING_GAME_SETTLE_FORCE" => ::std::option::Option::Some(HDMKPHALALG::SWORD_TRAINING_GAME_SETTLE_FORCE),
            "SWORD_TRAINING_GAME_SETTLE_BY_RESTORE" => ::std::option::Option::Some(HDMKPHALALG::SWORD_TRAINING_GAME_SETTLE_BY_RESTORE),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [HDMKPHALALG] = &[
        HDMKPHALALG::SWORD_TRAINING_GAME_SETTLE_NONE,
        HDMKPHALALG::SWORD_TRAINING_GAME_SETTLE_FINISH,
        HDMKPHALALG::SWORD_TRAINING_GAME_SETTLE_GIVE_UP,
        HDMKPHALALG::SWORD_TRAINING_GAME_SETTLE_BATTLE_FAILED,
        HDMKPHALALG::SWORD_TRAINING_GAME_SETTLE_FORCE,
        HDMKPHALALG::SWORD_TRAINING_GAME_SETTLE_BY_RESTORE,
    ];
}

impl ::protobuf::EnumFull for HDMKPHALALG {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("HDMKPHALALG").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for HDMKPHALALG {
    fn default() -> Self {
        HDMKPHALALG::SWORD_TRAINING_GAME_SETTLE_NONE
    }
}

impl HDMKPHALALG {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<HDMKPHALALG>("HDMKPHALALG")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11HDMKPHALALG.proto*\x80\x02\n\x0bHDMKPHALALG\x12#\n\x1fSWORD_TRAINI\
    NG_GAME_SETTLE_NONE\x10\0\x12%\n!SWORD_TRAINING_GAME_SETTLE_FINISH\x10\
    \x01\x12&\n\"SWORD_TRAINING_GAME_SETTLE_GIVE_UP\x10\x02\x12,\n(SWORD_TRA\
    INING_GAME_SETTLE_BATTLE_FAILED\x10\x03\x12$\n\x20SWORD_TRAINING_GAME_SE\
    TTLE_FORCE\x10\x04\x12)\n%SWORD_TRAINING_GAME_SETTLE_BY_RESTORE\x10\x05b\
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
            enums.push(HDMKPHALALG::generated_enum_descriptor_data());
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
