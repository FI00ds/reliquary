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

//! Generated file from `CmdTelevisionActivityType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdTelevisionActivityType)
pub enum CmdTelevisionActivityType {
    // @@protoc_insertion_point(enum_value:CmdTelevisionActivityType.CmdTelevisionActivityTypeNone)
    CmdTelevisionActivityTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdTelevisionActivityType.CmdGetTelevisionActivityDataCsReq)
    CmdGetTelevisionActivityDataCsReq = 6977,
    // @@protoc_insertion_point(enum_value:CmdTelevisionActivityType.CmdTelevisionActivityBattleEndScNotify)
    CmdTelevisionActivityBattleEndScNotify = 6969,
    // @@protoc_insertion_point(enum_value:CmdTelevisionActivityType.CmdEnterTelevisionActivityStageCsReq)
    CmdEnterTelevisionActivityStageCsReq = 6964,
    // @@protoc_insertion_point(enum_value:CmdTelevisionActivityType.CmdEnterTelevisionActivityStageScRsp)
    CmdEnterTelevisionActivityStageScRsp = 6974,
    // @@protoc_insertion_point(enum_value:CmdTelevisionActivityType.CmdTelevisionActivityDataChangeScNotify)
    CmdTelevisionActivityDataChangeScNotify = 6980,
    // @@protoc_insertion_point(enum_value:CmdTelevisionActivityType.CmdGetTelevisionActivityDataScRsp)
    CmdGetTelevisionActivityDataScRsp = 6976,
}

impl ::protobuf::Enum for CmdTelevisionActivityType {
    const NAME: &'static str = "CmdTelevisionActivityType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdTelevisionActivityType> {
        match value {
            0 => ::std::option::Option::Some(CmdTelevisionActivityType::CmdTelevisionActivityTypeNone),
            6977 => ::std::option::Option::Some(CmdTelevisionActivityType::CmdGetTelevisionActivityDataCsReq),
            6969 => ::std::option::Option::Some(CmdTelevisionActivityType::CmdTelevisionActivityBattleEndScNotify),
            6964 => ::std::option::Option::Some(CmdTelevisionActivityType::CmdEnterTelevisionActivityStageCsReq),
            6974 => ::std::option::Option::Some(CmdTelevisionActivityType::CmdEnterTelevisionActivityStageScRsp),
            6980 => ::std::option::Option::Some(CmdTelevisionActivityType::CmdTelevisionActivityDataChangeScNotify),
            6976 => ::std::option::Option::Some(CmdTelevisionActivityType::CmdGetTelevisionActivityDataScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdTelevisionActivityType> {
        match str {
            "CmdTelevisionActivityTypeNone" => ::std::option::Option::Some(CmdTelevisionActivityType::CmdTelevisionActivityTypeNone),
            "CmdGetTelevisionActivityDataCsReq" => ::std::option::Option::Some(CmdTelevisionActivityType::CmdGetTelevisionActivityDataCsReq),
            "CmdTelevisionActivityBattleEndScNotify" => ::std::option::Option::Some(CmdTelevisionActivityType::CmdTelevisionActivityBattleEndScNotify),
            "CmdEnterTelevisionActivityStageCsReq" => ::std::option::Option::Some(CmdTelevisionActivityType::CmdEnterTelevisionActivityStageCsReq),
            "CmdEnterTelevisionActivityStageScRsp" => ::std::option::Option::Some(CmdTelevisionActivityType::CmdEnterTelevisionActivityStageScRsp),
            "CmdTelevisionActivityDataChangeScNotify" => ::std::option::Option::Some(CmdTelevisionActivityType::CmdTelevisionActivityDataChangeScNotify),
            "CmdGetTelevisionActivityDataScRsp" => ::std::option::Option::Some(CmdTelevisionActivityType::CmdGetTelevisionActivityDataScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdTelevisionActivityType] = &[
        CmdTelevisionActivityType::CmdTelevisionActivityTypeNone,
        CmdTelevisionActivityType::CmdGetTelevisionActivityDataCsReq,
        CmdTelevisionActivityType::CmdTelevisionActivityBattleEndScNotify,
        CmdTelevisionActivityType::CmdEnterTelevisionActivityStageCsReq,
        CmdTelevisionActivityType::CmdEnterTelevisionActivityStageScRsp,
        CmdTelevisionActivityType::CmdTelevisionActivityDataChangeScNotify,
        CmdTelevisionActivityType::CmdGetTelevisionActivityDataScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdTelevisionActivityType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdTelevisionActivityType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdTelevisionActivityType::CmdTelevisionActivityTypeNone => 0,
            CmdTelevisionActivityType::CmdGetTelevisionActivityDataCsReq => 1,
            CmdTelevisionActivityType::CmdTelevisionActivityBattleEndScNotify => 2,
            CmdTelevisionActivityType::CmdEnterTelevisionActivityStageCsReq => 3,
            CmdTelevisionActivityType::CmdEnterTelevisionActivityStageScRsp => 4,
            CmdTelevisionActivityType::CmdTelevisionActivityDataChangeScNotify => 5,
            CmdTelevisionActivityType::CmdGetTelevisionActivityDataScRsp => 6,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdTelevisionActivityType {
    fn default() -> Self {
        CmdTelevisionActivityType::CmdTelevisionActivityTypeNone
    }
}

impl CmdTelevisionActivityType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdTelevisionActivityType>("CmdTelevisionActivityType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1fCmdTelevisionActivityType.proto*\xbf\x02\n\x19CmdTelevisionActivit\
    yType\x12!\n\x1dCmdTelevisionActivityTypeNone\x10\0\x12&\n!CmdGetTelevis\
    ionActivityDataCsReq\x10\xc16\x12+\n&CmdTelevisionActivityBattleEndScNot\
    ify\x10\xb96\x12)\n$CmdEnterTelevisionActivityStageCsReq\x10\xb46\x12)\n\
    $CmdEnterTelevisionActivityStageScRsp\x10\xbe6\x12,\n'CmdTelevisionActiv\
    ityDataChangeScNotify\x10\xc46\x12&\n!CmdGetTelevisionActivityDataScRsp\
    \x10\xc06b\x06proto3\
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
            enums.push(CmdTelevisionActivityType::generated_enum_descriptor_data());
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
