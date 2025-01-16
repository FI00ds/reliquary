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

//! Generated file from `MDLBADPPHIH.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:MDLBADPPHIH)
pub enum MDLBADPPHIH {
    // @@protoc_insertion_point(enum_value:MDLBADPPHIH.ROGUE_TOURN_HANDBOOK_NONE)
    ROGUE_TOURN_HANDBOOK_NONE = 0,
    // @@protoc_insertion_point(enum_value:MDLBADPPHIH.ROGUE_TOURN_HANDBOOK_SIMPLE_MIRACLE)
    ROGUE_TOURN_HANDBOOK_SIMPLE_MIRACLE = 1,
    // @@protoc_insertion_point(enum_value:MDLBADPPHIH.ROGUE_TOURN_HANDBOOK_HEX_MIRACLE)
    ROGUE_TOURN_HANDBOOK_HEX_MIRACLE = 2,
    // @@protoc_insertion_point(enum_value:MDLBADPPHIH.ROGUE_TOURN_HANDBOOK_BUFF)
    ROGUE_TOURN_HANDBOOK_BUFF = 3,
    // @@protoc_insertion_point(enum_value:MDLBADPPHIH.ROGUE_TOURN_HANDBOOK_EVENT)
    ROGUE_TOURN_HANDBOOK_EVENT = 4,
    // @@protoc_insertion_point(enum_value:MDLBADPPHIH.ROGUE_TOURN_HANDBOOK_FORMULA)
    ROGUE_TOURN_HANDBOOK_FORMULA = 5,
}

impl ::protobuf::Enum for MDLBADPPHIH {
    const NAME: &'static str = "MDLBADPPHIH";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MDLBADPPHIH> {
        match value {
            0 => ::std::option::Option::Some(MDLBADPPHIH::ROGUE_TOURN_HANDBOOK_NONE),
            1 => ::std::option::Option::Some(MDLBADPPHIH::ROGUE_TOURN_HANDBOOK_SIMPLE_MIRACLE),
            2 => ::std::option::Option::Some(MDLBADPPHIH::ROGUE_TOURN_HANDBOOK_HEX_MIRACLE),
            3 => ::std::option::Option::Some(MDLBADPPHIH::ROGUE_TOURN_HANDBOOK_BUFF),
            4 => ::std::option::Option::Some(MDLBADPPHIH::ROGUE_TOURN_HANDBOOK_EVENT),
            5 => ::std::option::Option::Some(MDLBADPPHIH::ROGUE_TOURN_HANDBOOK_FORMULA),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<MDLBADPPHIH> {
        match str {
            "ROGUE_TOURN_HANDBOOK_NONE" => ::std::option::Option::Some(MDLBADPPHIH::ROGUE_TOURN_HANDBOOK_NONE),
            "ROGUE_TOURN_HANDBOOK_SIMPLE_MIRACLE" => ::std::option::Option::Some(MDLBADPPHIH::ROGUE_TOURN_HANDBOOK_SIMPLE_MIRACLE),
            "ROGUE_TOURN_HANDBOOK_HEX_MIRACLE" => ::std::option::Option::Some(MDLBADPPHIH::ROGUE_TOURN_HANDBOOK_HEX_MIRACLE),
            "ROGUE_TOURN_HANDBOOK_BUFF" => ::std::option::Option::Some(MDLBADPPHIH::ROGUE_TOURN_HANDBOOK_BUFF),
            "ROGUE_TOURN_HANDBOOK_EVENT" => ::std::option::Option::Some(MDLBADPPHIH::ROGUE_TOURN_HANDBOOK_EVENT),
            "ROGUE_TOURN_HANDBOOK_FORMULA" => ::std::option::Option::Some(MDLBADPPHIH::ROGUE_TOURN_HANDBOOK_FORMULA),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [MDLBADPPHIH] = &[
        MDLBADPPHIH::ROGUE_TOURN_HANDBOOK_NONE,
        MDLBADPPHIH::ROGUE_TOURN_HANDBOOK_SIMPLE_MIRACLE,
        MDLBADPPHIH::ROGUE_TOURN_HANDBOOK_HEX_MIRACLE,
        MDLBADPPHIH::ROGUE_TOURN_HANDBOOK_BUFF,
        MDLBADPPHIH::ROGUE_TOURN_HANDBOOK_EVENT,
        MDLBADPPHIH::ROGUE_TOURN_HANDBOOK_FORMULA,
    ];
}

impl ::protobuf::EnumFull for MDLBADPPHIH {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("MDLBADPPHIH").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for MDLBADPPHIH {
    fn default() -> Self {
        MDLBADPPHIH::ROGUE_TOURN_HANDBOOK_NONE
    }
}

impl MDLBADPPHIH {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<MDLBADPPHIH>("MDLBADPPHIH")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11MDLBADPPHIH.proto*\xdc\x01\n\x0bMDLBADPPHIH\x12\x1d\n\x19ROGUE_TOU\
    RN_HANDBOOK_NONE\x10\0\x12'\n#ROGUE_TOURN_HANDBOOK_SIMPLE_MIRACLE\x10\
    \x01\x12$\n\x20ROGUE_TOURN_HANDBOOK_HEX_MIRACLE\x10\x02\x12\x1d\n\x19ROG\
    UE_TOURN_HANDBOOK_BUFF\x10\x03\x12\x1e\n\x1aROGUE_TOURN_HANDBOOK_EVENT\
    \x10\x04\x12\x20\n\x1cROGUE_TOURN_HANDBOOK_FORMULA\x10\x05b\x06proto3\
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
            enums.push(MDLBADPPHIH::generated_enum_descriptor_data());
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
