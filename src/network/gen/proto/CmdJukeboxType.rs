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

//! Generated file from `CmdJukeboxType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdJukeboxType)
pub enum CmdJukeboxType {
    // @@protoc_insertion_point(enum_value:CmdJukeboxType.CmdJukeboxTypeNone)
    CmdJukeboxTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdJukeboxType.CmdUnlockBackGroundMusicCsReq)
    CmdUnlockBackGroundMusicCsReq = 3167,
    // @@protoc_insertion_point(enum_value:CmdJukeboxType.CmdGetJukeboxDataCsReq)
    CmdGetJukeboxDataCsReq = 3136,
    // @@protoc_insertion_point(enum_value:CmdJukeboxType.CmdTrialBackGroundMusicCsReq)
    CmdTrialBackGroundMusicCsReq = 3152,
    // @@protoc_insertion_point(enum_value:CmdJukeboxType.CmdTrialBackGroundMusicScRsp)
    CmdTrialBackGroundMusicScRsp = 3174,
    // @@protoc_insertion_point(enum_value:CmdJukeboxType.CmdGetJukeboxDataScRsp)
    CmdGetJukeboxDataScRsp = 3195,
    // @@protoc_insertion_point(enum_value:CmdJukeboxType.CmdPlayBackGroundMusicCsReq)
    CmdPlayBackGroundMusicCsReq = 3184,
    // @@protoc_insertion_point(enum_value:CmdJukeboxType.CmdPlayBackGroundMusicScRsp)
    CmdPlayBackGroundMusicScRsp = 3127,
    // @@protoc_insertion_point(enum_value:CmdJukeboxType.CmdUnlockBackGroundMusicScRsp)
    CmdUnlockBackGroundMusicScRsp = 3128,
}

impl ::protobuf::Enum for CmdJukeboxType {
    const NAME: &'static str = "CmdJukeboxType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdJukeboxType> {
        match value {
            0 => ::std::option::Option::Some(CmdJukeboxType::CmdJukeboxTypeNone),
            3167 => ::std::option::Option::Some(CmdJukeboxType::CmdUnlockBackGroundMusicCsReq),
            3136 => ::std::option::Option::Some(CmdJukeboxType::CmdGetJukeboxDataCsReq),
            3152 => ::std::option::Option::Some(CmdJukeboxType::CmdTrialBackGroundMusicCsReq),
            3174 => ::std::option::Option::Some(CmdJukeboxType::CmdTrialBackGroundMusicScRsp),
            3195 => ::std::option::Option::Some(CmdJukeboxType::CmdGetJukeboxDataScRsp),
            3184 => ::std::option::Option::Some(CmdJukeboxType::CmdPlayBackGroundMusicCsReq),
            3127 => ::std::option::Option::Some(CmdJukeboxType::CmdPlayBackGroundMusicScRsp),
            3128 => ::std::option::Option::Some(CmdJukeboxType::CmdUnlockBackGroundMusicScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdJukeboxType> {
        match str {
            "CmdJukeboxTypeNone" => ::std::option::Option::Some(CmdJukeboxType::CmdJukeboxTypeNone),
            "CmdUnlockBackGroundMusicCsReq" => ::std::option::Option::Some(CmdJukeboxType::CmdUnlockBackGroundMusicCsReq),
            "CmdGetJukeboxDataCsReq" => ::std::option::Option::Some(CmdJukeboxType::CmdGetJukeboxDataCsReq),
            "CmdTrialBackGroundMusicCsReq" => ::std::option::Option::Some(CmdJukeboxType::CmdTrialBackGroundMusicCsReq),
            "CmdTrialBackGroundMusicScRsp" => ::std::option::Option::Some(CmdJukeboxType::CmdTrialBackGroundMusicScRsp),
            "CmdGetJukeboxDataScRsp" => ::std::option::Option::Some(CmdJukeboxType::CmdGetJukeboxDataScRsp),
            "CmdPlayBackGroundMusicCsReq" => ::std::option::Option::Some(CmdJukeboxType::CmdPlayBackGroundMusicCsReq),
            "CmdPlayBackGroundMusicScRsp" => ::std::option::Option::Some(CmdJukeboxType::CmdPlayBackGroundMusicScRsp),
            "CmdUnlockBackGroundMusicScRsp" => ::std::option::Option::Some(CmdJukeboxType::CmdUnlockBackGroundMusicScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdJukeboxType] = &[
        CmdJukeboxType::CmdJukeboxTypeNone,
        CmdJukeboxType::CmdUnlockBackGroundMusicCsReq,
        CmdJukeboxType::CmdGetJukeboxDataCsReq,
        CmdJukeboxType::CmdTrialBackGroundMusicCsReq,
        CmdJukeboxType::CmdTrialBackGroundMusicScRsp,
        CmdJukeboxType::CmdGetJukeboxDataScRsp,
        CmdJukeboxType::CmdPlayBackGroundMusicCsReq,
        CmdJukeboxType::CmdPlayBackGroundMusicScRsp,
        CmdJukeboxType::CmdUnlockBackGroundMusicScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdJukeboxType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdJukeboxType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdJukeboxType::CmdJukeboxTypeNone => 0,
            CmdJukeboxType::CmdUnlockBackGroundMusicCsReq => 1,
            CmdJukeboxType::CmdGetJukeboxDataCsReq => 2,
            CmdJukeboxType::CmdTrialBackGroundMusicCsReq => 3,
            CmdJukeboxType::CmdTrialBackGroundMusicScRsp => 4,
            CmdJukeboxType::CmdGetJukeboxDataScRsp => 5,
            CmdJukeboxType::CmdPlayBackGroundMusicCsReq => 6,
            CmdJukeboxType::CmdPlayBackGroundMusicScRsp => 7,
            CmdJukeboxType::CmdUnlockBackGroundMusicScRsp => 8,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdJukeboxType {
    fn default() -> Self {
        CmdJukeboxType::CmdJukeboxTypeNone
    }
}

impl CmdJukeboxType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdJukeboxType>("CmdJukeboxType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14CmdJukeboxType.proto*\xb4\x02\n\x0eCmdJukeboxType\x12\x16\n\x12Cmd\
    JukeboxTypeNone\x10\0\x12\"\n\x1dCmdUnlockBackGroundMusicCsReq\x10\xdf\
    \x18\x12\x1b\n\x16CmdGetJukeboxDataCsReq\x10\xc0\x18\x12!\n\x1cCmdTrialB\
    ackGroundMusicCsReq\x10\xd0\x18\x12!\n\x1cCmdTrialBackGroundMusicScRsp\
    \x10\xe6\x18\x12\x1b\n\x16CmdGetJukeboxDataScRsp\x10\xfb\x18\x12\x20\n\
    \x1bCmdPlayBackGroundMusicCsReq\x10\xf0\x18\x12\x20\n\x1bCmdPlayBackGrou\
    ndMusicScRsp\x10\xb7\x18\x12\"\n\x1dCmdUnlockBackGroundMusicScRsp\x10\
    \xb8\x18b\x06proto3\
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
            enums.push(CmdJukeboxType::generated_enum_descriptor_data());
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
