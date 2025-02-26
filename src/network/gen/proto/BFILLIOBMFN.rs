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

//! Generated file from `BFILLIOBMFN.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:BFILLIOBMFN)
pub enum BFILLIOBMFN {
    // @@protoc_insertion_point(enum_value:BFILLIOBMFN.EVENT_BEGIN)
    EVENT_BEGIN = 0,
    // @@protoc_insertion_point(enum_value:BFILLIOBMFN.EVENT_BREAK)
    EVENT_BREAK = 1,
    // @@protoc_insertion_point(enum_value:BFILLIOBMFN.EVENT_FALL)
    EVENT_FALL = 2,
    // @@protoc_insertion_point(enum_value:BFILLIOBMFN.EVENT_REFRESH)
    EVENT_REFRESH = 3,
    // @@protoc_insertion_point(enum_value:BFILLIOBMFN.EVENT_BIRD_SKILL)
    EVENT_BIRD_SKILL = 4,
    // @@protoc_insertion_point(enum_value:BFILLIOBMFN.EVENT_ENV)
    EVENT_ENV = 5,
    // @@protoc_insertion_point(enum_value:BFILLIOBMFN.EVENT_SHUFFLE)
    EVENT_SHUFFLE = 6,
    // @@protoc_insertion_point(enum_value:BFILLIOBMFN.EVENT_SETTLE_TAG)
    EVENT_SETTLE_TAG = 7,
}

impl ::protobuf::Enum for BFILLIOBMFN {
    const NAME: &'static str = "BFILLIOBMFN";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<BFILLIOBMFN> {
        match value {
            0 => ::std::option::Option::Some(BFILLIOBMFN::EVENT_BEGIN),
            1 => ::std::option::Option::Some(BFILLIOBMFN::EVENT_BREAK),
            2 => ::std::option::Option::Some(BFILLIOBMFN::EVENT_FALL),
            3 => ::std::option::Option::Some(BFILLIOBMFN::EVENT_REFRESH),
            4 => ::std::option::Option::Some(BFILLIOBMFN::EVENT_BIRD_SKILL),
            5 => ::std::option::Option::Some(BFILLIOBMFN::EVENT_ENV),
            6 => ::std::option::Option::Some(BFILLIOBMFN::EVENT_SHUFFLE),
            7 => ::std::option::Option::Some(BFILLIOBMFN::EVENT_SETTLE_TAG),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<BFILLIOBMFN> {
        match str {
            "EVENT_BEGIN" => ::std::option::Option::Some(BFILLIOBMFN::EVENT_BEGIN),
            "EVENT_BREAK" => ::std::option::Option::Some(BFILLIOBMFN::EVENT_BREAK),
            "EVENT_FALL" => ::std::option::Option::Some(BFILLIOBMFN::EVENT_FALL),
            "EVENT_REFRESH" => ::std::option::Option::Some(BFILLIOBMFN::EVENT_REFRESH),
            "EVENT_BIRD_SKILL" => ::std::option::Option::Some(BFILLIOBMFN::EVENT_BIRD_SKILL),
            "EVENT_ENV" => ::std::option::Option::Some(BFILLIOBMFN::EVENT_ENV),
            "EVENT_SHUFFLE" => ::std::option::Option::Some(BFILLIOBMFN::EVENT_SHUFFLE),
            "EVENT_SETTLE_TAG" => ::std::option::Option::Some(BFILLIOBMFN::EVENT_SETTLE_TAG),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [BFILLIOBMFN] = &[
        BFILLIOBMFN::EVENT_BEGIN,
        BFILLIOBMFN::EVENT_BREAK,
        BFILLIOBMFN::EVENT_FALL,
        BFILLIOBMFN::EVENT_REFRESH,
        BFILLIOBMFN::EVENT_BIRD_SKILL,
        BFILLIOBMFN::EVENT_ENV,
        BFILLIOBMFN::EVENT_SHUFFLE,
        BFILLIOBMFN::EVENT_SETTLE_TAG,
    ];
}

impl ::protobuf::EnumFull for BFILLIOBMFN {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("BFILLIOBMFN").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for BFILLIOBMFN {
    fn default() -> Self {
        BFILLIOBMFN::EVENT_BEGIN
    }
}

impl BFILLIOBMFN {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<BFILLIOBMFN>("BFILLIOBMFN")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11BFILLIOBMFN.proto*\xa0\x01\n\x0bBFILLIOBMFN\x12\x0f\n\x0bEVENT_BEG\
    IN\x10\0\x12\x0f\n\x0bEVENT_BREAK\x10\x01\x12\x0e\n\nEVENT_FALL\x10\x02\
    \x12\x11\n\rEVENT_REFRESH\x10\x03\x12\x14\n\x10EVENT_BIRD_SKILL\x10\x04\
    \x12\r\n\tEVENT_ENV\x10\x05\x12\x11\n\rEVENT_SHUFFLE\x10\x06\x12\x14\n\
    \x10EVENT_SETTLE_TAG\x10\x07b\x06proto3\
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
            enums.push(BFILLIOBMFN::generated_enum_descriptor_data());
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
