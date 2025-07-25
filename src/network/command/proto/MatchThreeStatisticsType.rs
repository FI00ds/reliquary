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

//! Generated file from `MatchThreeStatisticsType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:MatchThreeStatisticsType)
pub enum MatchThreeStatisticsType {
    // @@protoc_insertion_point(enum_value:MatchThreeStatisticsType.MatchThreeStatistics_None)
    MatchThreeStatistics_None = 0,
    // @@protoc_insertion_point(enum_value:MatchThreeStatisticsType.MatchThreeStatistics_First)
    MatchThreeStatistics_First = 1,
    // @@protoc_insertion_point(enum_value:MatchThreeStatisticsType.MatchThreeStatistics_Second)
    MatchThreeStatistics_Second = 2,
    // @@protoc_insertion_point(enum_value:MatchThreeStatisticsType.MatchThreeStatistics_Third)
    MatchThreeStatistics_Third = 3,
    // @@protoc_insertion_point(enum_value:MatchThreeStatisticsType.MatchThreeStatistics_Fruit)
    MatchThreeStatistics_Fruit = 4,
    // @@protoc_insertion_point(enum_value:MatchThreeStatisticsType.MatchThreeStatistics_Skill)
    MatchThreeStatistics_Skill = 5,
    // @@protoc_insertion_point(enum_value:MatchThreeStatisticsType.MatchThreeStatistics_Defeat)
    MatchThreeStatistics_Defeat = 6,
    // @@protoc_insertion_point(enum_value:MatchThreeStatisticsType.MatchThreeStatistics_Bomb)
    MatchThreeStatistics_Bomb = 7,
    // @@protoc_insertion_point(enum_value:MatchThreeStatisticsType.MatchThreeStatistics_Damage)
    MatchThreeStatistics_Damage = 8,
    // @@protoc_insertion_point(enum_value:MatchThreeStatisticsType.MatchThreeStatistics_Energy)
    MatchThreeStatistics_Energy = 9,
    // @@protoc_insertion_point(enum_value:MatchThreeStatisticsType.MatchThreeStatistics_SwapBomb)
    MatchThreeStatistics_SwapBomb = 10,
    // @@protoc_insertion_point(enum_value:MatchThreeStatisticsType.MatchThreeStatistics_UseItem)
    MatchThreeStatistics_UseItem = 11,
    // @@protoc_insertion_point(enum_value:MatchThreeStatisticsType.MatchThreeStatistics_SoloMaxStep)
    MatchThreeStatistics_SoloMaxStep = 12,
    // @@protoc_insertion_point(enum_value:MatchThreeStatisticsType.MatchThreeStatistics_SoloScore)
    MatchThreeStatistics_SoloScore = 13,
}

impl ::protobuf::Enum for MatchThreeStatisticsType {
    const NAME: &'static str = "MatchThreeStatisticsType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MatchThreeStatisticsType> {
        match value {
            0 => ::std::option::Option::Some(MatchThreeStatisticsType::MatchThreeStatistics_None),
            1 => ::std::option::Option::Some(MatchThreeStatisticsType::MatchThreeStatistics_First),
            2 => ::std::option::Option::Some(MatchThreeStatisticsType::MatchThreeStatistics_Second),
            3 => ::std::option::Option::Some(MatchThreeStatisticsType::MatchThreeStatistics_Third),
            4 => ::std::option::Option::Some(MatchThreeStatisticsType::MatchThreeStatistics_Fruit),
            5 => ::std::option::Option::Some(MatchThreeStatisticsType::MatchThreeStatistics_Skill),
            6 => ::std::option::Option::Some(MatchThreeStatisticsType::MatchThreeStatistics_Defeat),
            7 => ::std::option::Option::Some(MatchThreeStatisticsType::MatchThreeStatistics_Bomb),
            8 => ::std::option::Option::Some(MatchThreeStatisticsType::MatchThreeStatistics_Damage),
            9 => ::std::option::Option::Some(MatchThreeStatisticsType::MatchThreeStatistics_Energy),
            10 => ::std::option::Option::Some(MatchThreeStatisticsType::MatchThreeStatistics_SwapBomb),
            11 => ::std::option::Option::Some(MatchThreeStatisticsType::MatchThreeStatistics_UseItem),
            12 => ::std::option::Option::Some(MatchThreeStatisticsType::MatchThreeStatistics_SoloMaxStep),
            13 => ::std::option::Option::Some(MatchThreeStatisticsType::MatchThreeStatistics_SoloScore),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<MatchThreeStatisticsType> {
        match str {
            "MatchThreeStatistics_None" => ::std::option::Option::Some(MatchThreeStatisticsType::MatchThreeStatistics_None),
            "MatchThreeStatistics_First" => ::std::option::Option::Some(MatchThreeStatisticsType::MatchThreeStatistics_First),
            "MatchThreeStatistics_Second" => ::std::option::Option::Some(MatchThreeStatisticsType::MatchThreeStatistics_Second),
            "MatchThreeStatistics_Third" => ::std::option::Option::Some(MatchThreeStatisticsType::MatchThreeStatistics_Third),
            "MatchThreeStatistics_Fruit" => ::std::option::Option::Some(MatchThreeStatisticsType::MatchThreeStatistics_Fruit),
            "MatchThreeStatistics_Skill" => ::std::option::Option::Some(MatchThreeStatisticsType::MatchThreeStatistics_Skill),
            "MatchThreeStatistics_Defeat" => ::std::option::Option::Some(MatchThreeStatisticsType::MatchThreeStatistics_Defeat),
            "MatchThreeStatistics_Bomb" => ::std::option::Option::Some(MatchThreeStatisticsType::MatchThreeStatistics_Bomb),
            "MatchThreeStatistics_Damage" => ::std::option::Option::Some(MatchThreeStatisticsType::MatchThreeStatistics_Damage),
            "MatchThreeStatistics_Energy" => ::std::option::Option::Some(MatchThreeStatisticsType::MatchThreeStatistics_Energy),
            "MatchThreeStatistics_SwapBomb" => ::std::option::Option::Some(MatchThreeStatisticsType::MatchThreeStatistics_SwapBomb),
            "MatchThreeStatistics_UseItem" => ::std::option::Option::Some(MatchThreeStatisticsType::MatchThreeStatistics_UseItem),
            "MatchThreeStatistics_SoloMaxStep" => ::std::option::Option::Some(MatchThreeStatisticsType::MatchThreeStatistics_SoloMaxStep),
            "MatchThreeStatistics_SoloScore" => ::std::option::Option::Some(MatchThreeStatisticsType::MatchThreeStatistics_SoloScore),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [MatchThreeStatisticsType] = &[
        MatchThreeStatisticsType::MatchThreeStatistics_None,
        MatchThreeStatisticsType::MatchThreeStatistics_First,
        MatchThreeStatisticsType::MatchThreeStatistics_Second,
        MatchThreeStatisticsType::MatchThreeStatistics_Third,
        MatchThreeStatisticsType::MatchThreeStatistics_Fruit,
        MatchThreeStatisticsType::MatchThreeStatistics_Skill,
        MatchThreeStatisticsType::MatchThreeStatistics_Defeat,
        MatchThreeStatisticsType::MatchThreeStatistics_Bomb,
        MatchThreeStatisticsType::MatchThreeStatistics_Damage,
        MatchThreeStatisticsType::MatchThreeStatistics_Energy,
        MatchThreeStatisticsType::MatchThreeStatistics_SwapBomb,
        MatchThreeStatisticsType::MatchThreeStatistics_UseItem,
        MatchThreeStatisticsType::MatchThreeStatistics_SoloMaxStep,
        MatchThreeStatisticsType::MatchThreeStatistics_SoloScore,
    ];
}

impl ::protobuf::EnumFull for MatchThreeStatisticsType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("MatchThreeStatisticsType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for MatchThreeStatisticsType {
    fn default() -> Self {
        MatchThreeStatisticsType::MatchThreeStatistics_None
    }
}

impl MatchThreeStatisticsType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<MatchThreeStatisticsType>("MatchThreeStatisticsType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1eMatchThreeStatisticsType.proto*\xeb\x03\n\x18MatchThreeStatisticsT\
    ype\x12\x1d\n\x19MatchThreeStatistics_None\x10\0\x12\x1e\n\x1aMatchThree\
    Statistics_First\x10\x01\x12\x1f\n\x1bMatchThreeStatistics_Second\x10\
    \x02\x12\x1e\n\x1aMatchThreeStatistics_Third\x10\x03\x12\x1e\n\x1aMatchT\
    hreeStatistics_Fruit\x10\x04\x12\x1e\n\x1aMatchThreeStatistics_Skill\x10\
    \x05\x12\x1f\n\x1bMatchThreeStatistics_Defeat\x10\x06\x12\x1d\n\x19Match\
    ThreeStatistics_Bomb\x10\x07\x12\x1f\n\x1bMatchThreeStatistics_Damage\
    \x10\x08\x12\x1f\n\x1bMatchThreeStatistics_Energy\x10\t\x12!\n\x1dMatchT\
    hreeStatistics_SwapBomb\x10\n\x12\x20\n\x1cMatchThreeStatistics_UseItem\
    \x10\x0b\x12$\n\x20MatchThreeStatistics_SoloMaxStep\x10\x0c\x12\"\n\x1eM\
    atchThreeStatistics_SoloScore\x10\rb\x06proto3\
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
            enums.push(MatchThreeStatisticsType::generated_enum_descriptor_data());
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
