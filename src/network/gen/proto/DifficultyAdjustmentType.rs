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

//! Generated file from `DifficultyAdjustmentType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:DifficultyAdjustmentType)
pub enum DifficultyAdjustmentType {
    // @@protoc_insertion_point(enum_value:DifficultyAdjustmentType.DIFFICULTY_AJUSTMENT_TYPE_DEFAULT)
    DIFFICULTY_AJUSTMENT_TYPE_DEFAULT = 0,
    // @@protoc_insertion_point(enum_value:DifficultyAdjustmentType.DIFFICULTY_AJUSTMENT_TYPE_EASY)
    DIFFICULTY_AJUSTMENT_TYPE_EASY = 1,
}

impl ::protobuf::Enum for DifficultyAdjustmentType {
    const NAME: &'static str = "DifficultyAdjustmentType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DifficultyAdjustmentType> {
        match value {
            0 => ::std::option::Option::Some(DifficultyAdjustmentType::DIFFICULTY_AJUSTMENT_TYPE_DEFAULT),
            1 => ::std::option::Option::Some(DifficultyAdjustmentType::DIFFICULTY_AJUSTMENT_TYPE_EASY),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<DifficultyAdjustmentType> {
        match str {
            "DIFFICULTY_AJUSTMENT_TYPE_DEFAULT" => ::std::option::Option::Some(DifficultyAdjustmentType::DIFFICULTY_AJUSTMENT_TYPE_DEFAULT),
            "DIFFICULTY_AJUSTMENT_TYPE_EASY" => ::std::option::Option::Some(DifficultyAdjustmentType::DIFFICULTY_AJUSTMENT_TYPE_EASY),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [DifficultyAdjustmentType] = &[
        DifficultyAdjustmentType::DIFFICULTY_AJUSTMENT_TYPE_DEFAULT,
        DifficultyAdjustmentType::DIFFICULTY_AJUSTMENT_TYPE_EASY,
    ];
}

impl ::protobuf::EnumFull for DifficultyAdjustmentType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("DifficultyAdjustmentType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for DifficultyAdjustmentType {
    fn default() -> Self {
        DifficultyAdjustmentType::DIFFICULTY_AJUSTMENT_TYPE_DEFAULT
    }
}

impl DifficultyAdjustmentType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<DifficultyAdjustmentType>("DifficultyAdjustmentType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1eDifficultyAdjustmentType.proto*e\n\x18DifficultyAdjustmentType\x12\
    %\n!DIFFICULTY_AJUSTMENT_TYPE_DEFAULT\x10\0\x12\"\n\x1eDIFFICULTY_AJUSTM\
    ENT_TYPE_EASY\x10\x01b\x06proto3\
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
            enums.push(DifficultyAdjustmentType::generated_enum_descriptor_data());
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
