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

//! Generated file from `CmdDrinkMakerType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdDrinkMakerType)
pub enum CmdDrinkMakerType {
    // @@protoc_insertion_point(enum_value:CmdDrinkMakerType.CmdDrinkMakerTypeNone)
    CmdDrinkMakerTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdDrinkMakerType.CmdDrinkMakerChallengeCsReq)
    CmdDrinkMakerChallengeCsReq = 6986,
    // @@protoc_insertion_point(enum_value:CmdDrinkMakerType.CmdEndDrinkMakerSequenceScRsp)
    CmdEndDrinkMakerSequenceScRsp = 6981,
    // @@protoc_insertion_point(enum_value:CmdDrinkMakerType.CmdDrinkMakerChallengeScRsp)
    CmdDrinkMakerChallengeScRsp = 6998,
    // @@protoc_insertion_point(enum_value:CmdDrinkMakerType.CmdDrinkMakerDayEndScNotify)
    CmdDrinkMakerDayEndScNotify = 6995,
    // @@protoc_insertion_point(enum_value:CmdDrinkMakerType.CmdMakeMissionDrinkScRsp)
    CmdMakeMissionDrinkScRsp = 6994,
    // @@protoc_insertion_point(enum_value:CmdDrinkMakerType.CmdMakeDrinkCsReq)
    CmdMakeDrinkCsReq = 6992,
    // @@protoc_insertion_point(enum_value:CmdDrinkMakerType.CmdGetDrinkMakerDataScRsp)
    CmdGetDrinkMakerDataScRsp = 6988,
    // @@protoc_insertion_point(enum_value:CmdDrinkMakerType.CmdGetDrinkMakerDataCsReq)
    CmdGetDrinkMakerDataCsReq = 6997,
    // @@protoc_insertion_point(enum_value:CmdDrinkMakerType.CmdMakeMissionDrinkCsReq)
    CmdMakeMissionDrinkCsReq = 6999,
    // @@protoc_insertion_point(enum_value:CmdDrinkMakerType.CmdMakeDrinkScRsp)
    CmdMakeDrinkScRsp = 6982,
    // @@protoc_insertion_point(enum_value:CmdDrinkMakerType.CmdEndDrinkMakerSequenceCsReq)
    CmdEndDrinkMakerSequenceCsReq = 6984,
    // @@protoc_insertion_point(enum_value:CmdDrinkMakerType.CmdDrinkMakerUpdateTipsNotify)
    CmdDrinkMakerUpdateTipsNotify = 6985,
}

impl ::protobuf::Enum for CmdDrinkMakerType {
    const NAME: &'static str = "CmdDrinkMakerType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdDrinkMakerType> {
        match value {
            0 => ::std::option::Option::Some(CmdDrinkMakerType::CmdDrinkMakerTypeNone),
            6986 => ::std::option::Option::Some(CmdDrinkMakerType::CmdDrinkMakerChallengeCsReq),
            6981 => ::std::option::Option::Some(CmdDrinkMakerType::CmdEndDrinkMakerSequenceScRsp),
            6998 => ::std::option::Option::Some(CmdDrinkMakerType::CmdDrinkMakerChallengeScRsp),
            6995 => ::std::option::Option::Some(CmdDrinkMakerType::CmdDrinkMakerDayEndScNotify),
            6994 => ::std::option::Option::Some(CmdDrinkMakerType::CmdMakeMissionDrinkScRsp),
            6992 => ::std::option::Option::Some(CmdDrinkMakerType::CmdMakeDrinkCsReq),
            6988 => ::std::option::Option::Some(CmdDrinkMakerType::CmdGetDrinkMakerDataScRsp),
            6997 => ::std::option::Option::Some(CmdDrinkMakerType::CmdGetDrinkMakerDataCsReq),
            6999 => ::std::option::Option::Some(CmdDrinkMakerType::CmdMakeMissionDrinkCsReq),
            6982 => ::std::option::Option::Some(CmdDrinkMakerType::CmdMakeDrinkScRsp),
            6984 => ::std::option::Option::Some(CmdDrinkMakerType::CmdEndDrinkMakerSequenceCsReq),
            6985 => ::std::option::Option::Some(CmdDrinkMakerType::CmdDrinkMakerUpdateTipsNotify),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdDrinkMakerType> {
        match str {
            "CmdDrinkMakerTypeNone" => ::std::option::Option::Some(CmdDrinkMakerType::CmdDrinkMakerTypeNone),
            "CmdDrinkMakerChallengeCsReq" => ::std::option::Option::Some(CmdDrinkMakerType::CmdDrinkMakerChallengeCsReq),
            "CmdEndDrinkMakerSequenceScRsp" => ::std::option::Option::Some(CmdDrinkMakerType::CmdEndDrinkMakerSequenceScRsp),
            "CmdDrinkMakerChallengeScRsp" => ::std::option::Option::Some(CmdDrinkMakerType::CmdDrinkMakerChallengeScRsp),
            "CmdDrinkMakerDayEndScNotify" => ::std::option::Option::Some(CmdDrinkMakerType::CmdDrinkMakerDayEndScNotify),
            "CmdMakeMissionDrinkScRsp" => ::std::option::Option::Some(CmdDrinkMakerType::CmdMakeMissionDrinkScRsp),
            "CmdMakeDrinkCsReq" => ::std::option::Option::Some(CmdDrinkMakerType::CmdMakeDrinkCsReq),
            "CmdGetDrinkMakerDataScRsp" => ::std::option::Option::Some(CmdDrinkMakerType::CmdGetDrinkMakerDataScRsp),
            "CmdGetDrinkMakerDataCsReq" => ::std::option::Option::Some(CmdDrinkMakerType::CmdGetDrinkMakerDataCsReq),
            "CmdMakeMissionDrinkCsReq" => ::std::option::Option::Some(CmdDrinkMakerType::CmdMakeMissionDrinkCsReq),
            "CmdMakeDrinkScRsp" => ::std::option::Option::Some(CmdDrinkMakerType::CmdMakeDrinkScRsp),
            "CmdEndDrinkMakerSequenceCsReq" => ::std::option::Option::Some(CmdDrinkMakerType::CmdEndDrinkMakerSequenceCsReq),
            "CmdDrinkMakerUpdateTipsNotify" => ::std::option::Option::Some(CmdDrinkMakerType::CmdDrinkMakerUpdateTipsNotify),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdDrinkMakerType] = &[
        CmdDrinkMakerType::CmdDrinkMakerTypeNone,
        CmdDrinkMakerType::CmdDrinkMakerChallengeCsReq,
        CmdDrinkMakerType::CmdEndDrinkMakerSequenceScRsp,
        CmdDrinkMakerType::CmdDrinkMakerChallengeScRsp,
        CmdDrinkMakerType::CmdDrinkMakerDayEndScNotify,
        CmdDrinkMakerType::CmdMakeMissionDrinkScRsp,
        CmdDrinkMakerType::CmdMakeDrinkCsReq,
        CmdDrinkMakerType::CmdGetDrinkMakerDataScRsp,
        CmdDrinkMakerType::CmdGetDrinkMakerDataCsReq,
        CmdDrinkMakerType::CmdMakeMissionDrinkCsReq,
        CmdDrinkMakerType::CmdMakeDrinkScRsp,
        CmdDrinkMakerType::CmdEndDrinkMakerSequenceCsReq,
        CmdDrinkMakerType::CmdDrinkMakerUpdateTipsNotify,
    ];
}

impl ::protobuf::EnumFull for CmdDrinkMakerType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdDrinkMakerType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdDrinkMakerType::CmdDrinkMakerTypeNone => 0,
            CmdDrinkMakerType::CmdDrinkMakerChallengeCsReq => 1,
            CmdDrinkMakerType::CmdEndDrinkMakerSequenceScRsp => 2,
            CmdDrinkMakerType::CmdDrinkMakerChallengeScRsp => 3,
            CmdDrinkMakerType::CmdDrinkMakerDayEndScNotify => 4,
            CmdDrinkMakerType::CmdMakeMissionDrinkScRsp => 5,
            CmdDrinkMakerType::CmdMakeDrinkCsReq => 6,
            CmdDrinkMakerType::CmdGetDrinkMakerDataScRsp => 7,
            CmdDrinkMakerType::CmdGetDrinkMakerDataCsReq => 8,
            CmdDrinkMakerType::CmdMakeMissionDrinkCsReq => 9,
            CmdDrinkMakerType::CmdMakeDrinkScRsp => 10,
            CmdDrinkMakerType::CmdEndDrinkMakerSequenceCsReq => 11,
            CmdDrinkMakerType::CmdDrinkMakerUpdateTipsNotify => 12,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdDrinkMakerType {
    fn default() -> Self {
        CmdDrinkMakerType::CmdDrinkMakerTypeNone
    }
}

impl CmdDrinkMakerType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdDrinkMakerType>("CmdDrinkMakerType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17CmdDrinkMakerType.proto*\xae\x03\n\x11CmdDrinkMakerType\x12\x19\n\
    \x15CmdDrinkMakerTypeNone\x10\0\x12\x20\n\x1bCmdDrinkMakerChallengeCsReq\
    \x10\xca6\x12\"\n\x1dCmdEndDrinkMakerSequenceScRsp\x10\xc56\x12\x20\n\
    \x1bCmdDrinkMakerChallengeScRsp\x10\xd66\x12\x20\n\x1bCmdDrinkMakerDayEn\
    dScNotify\x10\xd36\x12\x1d\n\x18CmdMakeMissionDrinkScRsp\x10\xd26\x12\
    \x16\n\x11CmdMakeDrinkCsReq\x10\xd06\x12\x1e\n\x19CmdGetDrinkMakerDataSc\
    Rsp\x10\xcc6\x12\x1e\n\x19CmdGetDrinkMakerDataCsReq\x10\xd56\x12\x1d\n\
    \x18CmdMakeMissionDrinkCsReq\x10\xd76\x12\x16\n\x11CmdMakeDrinkScRsp\x10\
    \xc66\x12\"\n\x1dCmdEndDrinkMakerSequenceCsReq\x10\xc86\x12\"\n\x1dCmdDr\
    inkMakerUpdateTipsNotify\x10\xc96b\x06proto3\
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
            enums.push(CmdDrinkMakerType::generated_enum_descriptor_data());
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
