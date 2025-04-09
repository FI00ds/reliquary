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

//! Generated file from `CmdChimeraType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdChimeraType)
pub enum CmdChimeraType {
    // @@protoc_insertion_point(enum_value:CmdChimeraType.CmdChimeraTypeNone)
    CmdChimeraTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdChimeraType.CmdChimeraGetDataCsReq)
    CmdChimeraGetDataCsReq = 8177,
    // @@protoc_insertion_point(enum_value:CmdChimeraType.CmdChimeraFinishEndlessRoundScRsp)
    CmdChimeraFinishEndlessRoundScRsp = 8178,
    // @@protoc_insertion_point(enum_value:CmdChimeraType.CmdChimeraDoFinalRoundScRsp)
    CmdChimeraDoFinalRoundScRsp = 8163,
    // @@protoc_insertion_point(enum_value:CmdChimeraType.CmdChimeraFinishRoundCsReq)
    CmdChimeraFinishRoundCsReq = 8174,
    // @@protoc_insertion_point(enum_value:CmdChimeraType.CmdChimeraStartEndlessScRsp)
    CmdChimeraStartEndlessScRsp = 8171,
    // @@protoc_insertion_point(enum_value:CmdChimeraType.CmdChimeraGetDataScRsp)
    CmdChimeraGetDataScRsp = 8176,
    // @@protoc_insertion_point(enum_value:CmdChimeraType.CmdChimeraRoundWorkStartScRsp)
    CmdChimeraRoundWorkStartScRsp = 8172,
    // @@protoc_insertion_point(enum_value:CmdChimeraType.CmdChimeraFinishEndlessRoundCsReq)
    CmdChimeraFinishEndlessRoundCsReq = 8170,
    // @@protoc_insertion_point(enum_value:CmdChimeraType.CmdChimeraSetLineupCsReq)
    CmdChimeraSetLineupCsReq = 8180,
    // @@protoc_insertion_point(enum_value:CmdChimeraType.CmdChimeraSetLineupScRsp)
    CmdChimeraSetLineupScRsp = 8164,
    // @@protoc_insertion_point(enum_value:CmdChimeraType.CmdChimeraQuitEndlessScRsp)
    CmdChimeraQuitEndlessScRsp = 8175,
    // @@protoc_insertion_point(enum_value:CmdChimeraType.CmdChimeraDoFinalRoundCsReq)
    CmdChimeraDoFinalRoundCsReq = 8173,
    // @@protoc_insertion_point(enum_value:CmdChimeraType.CmdChimeraRoundWorkStartCsReq)
    CmdChimeraRoundWorkStartCsReq = 8165,
    // @@protoc_insertion_point(enum_value:CmdChimeraType.CmdChimeraFinishRoundScRsp)
    CmdChimeraFinishRoundScRsp = 8169,
    // @@protoc_insertion_point(enum_value:CmdChimeraType.CmdChimeraStartEndlessCsReq)
    CmdChimeraStartEndlessCsReq = 8179,
    // @@protoc_insertion_point(enum_value:CmdChimeraType.CmdChimeraQuitEndlessCsReq)
    CmdChimeraQuitEndlessCsReq = 8167,
}

impl ::protobuf::Enum for CmdChimeraType {
    const NAME: &'static str = "CmdChimeraType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdChimeraType> {
        match value {
            0 => ::std::option::Option::Some(CmdChimeraType::CmdChimeraTypeNone),
            8177 => ::std::option::Option::Some(CmdChimeraType::CmdChimeraGetDataCsReq),
            8178 => ::std::option::Option::Some(CmdChimeraType::CmdChimeraFinishEndlessRoundScRsp),
            8163 => ::std::option::Option::Some(CmdChimeraType::CmdChimeraDoFinalRoundScRsp),
            8174 => ::std::option::Option::Some(CmdChimeraType::CmdChimeraFinishRoundCsReq),
            8171 => ::std::option::Option::Some(CmdChimeraType::CmdChimeraStartEndlessScRsp),
            8176 => ::std::option::Option::Some(CmdChimeraType::CmdChimeraGetDataScRsp),
            8172 => ::std::option::Option::Some(CmdChimeraType::CmdChimeraRoundWorkStartScRsp),
            8170 => ::std::option::Option::Some(CmdChimeraType::CmdChimeraFinishEndlessRoundCsReq),
            8180 => ::std::option::Option::Some(CmdChimeraType::CmdChimeraSetLineupCsReq),
            8164 => ::std::option::Option::Some(CmdChimeraType::CmdChimeraSetLineupScRsp),
            8175 => ::std::option::Option::Some(CmdChimeraType::CmdChimeraQuitEndlessScRsp),
            8173 => ::std::option::Option::Some(CmdChimeraType::CmdChimeraDoFinalRoundCsReq),
            8165 => ::std::option::Option::Some(CmdChimeraType::CmdChimeraRoundWorkStartCsReq),
            8169 => ::std::option::Option::Some(CmdChimeraType::CmdChimeraFinishRoundScRsp),
            8179 => ::std::option::Option::Some(CmdChimeraType::CmdChimeraStartEndlessCsReq),
            8167 => ::std::option::Option::Some(CmdChimeraType::CmdChimeraQuitEndlessCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdChimeraType> {
        match str {
            "CmdChimeraTypeNone" => ::std::option::Option::Some(CmdChimeraType::CmdChimeraTypeNone),
            "CmdChimeraGetDataCsReq" => ::std::option::Option::Some(CmdChimeraType::CmdChimeraGetDataCsReq),
            "CmdChimeraFinishEndlessRoundScRsp" => ::std::option::Option::Some(CmdChimeraType::CmdChimeraFinishEndlessRoundScRsp),
            "CmdChimeraDoFinalRoundScRsp" => ::std::option::Option::Some(CmdChimeraType::CmdChimeraDoFinalRoundScRsp),
            "CmdChimeraFinishRoundCsReq" => ::std::option::Option::Some(CmdChimeraType::CmdChimeraFinishRoundCsReq),
            "CmdChimeraStartEndlessScRsp" => ::std::option::Option::Some(CmdChimeraType::CmdChimeraStartEndlessScRsp),
            "CmdChimeraGetDataScRsp" => ::std::option::Option::Some(CmdChimeraType::CmdChimeraGetDataScRsp),
            "CmdChimeraRoundWorkStartScRsp" => ::std::option::Option::Some(CmdChimeraType::CmdChimeraRoundWorkStartScRsp),
            "CmdChimeraFinishEndlessRoundCsReq" => ::std::option::Option::Some(CmdChimeraType::CmdChimeraFinishEndlessRoundCsReq),
            "CmdChimeraSetLineupCsReq" => ::std::option::Option::Some(CmdChimeraType::CmdChimeraSetLineupCsReq),
            "CmdChimeraSetLineupScRsp" => ::std::option::Option::Some(CmdChimeraType::CmdChimeraSetLineupScRsp),
            "CmdChimeraQuitEndlessScRsp" => ::std::option::Option::Some(CmdChimeraType::CmdChimeraQuitEndlessScRsp),
            "CmdChimeraDoFinalRoundCsReq" => ::std::option::Option::Some(CmdChimeraType::CmdChimeraDoFinalRoundCsReq),
            "CmdChimeraRoundWorkStartCsReq" => ::std::option::Option::Some(CmdChimeraType::CmdChimeraRoundWorkStartCsReq),
            "CmdChimeraFinishRoundScRsp" => ::std::option::Option::Some(CmdChimeraType::CmdChimeraFinishRoundScRsp),
            "CmdChimeraStartEndlessCsReq" => ::std::option::Option::Some(CmdChimeraType::CmdChimeraStartEndlessCsReq),
            "CmdChimeraQuitEndlessCsReq" => ::std::option::Option::Some(CmdChimeraType::CmdChimeraQuitEndlessCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdChimeraType] = &[
        CmdChimeraType::CmdChimeraTypeNone,
        CmdChimeraType::CmdChimeraGetDataCsReq,
        CmdChimeraType::CmdChimeraFinishEndlessRoundScRsp,
        CmdChimeraType::CmdChimeraDoFinalRoundScRsp,
        CmdChimeraType::CmdChimeraFinishRoundCsReq,
        CmdChimeraType::CmdChimeraStartEndlessScRsp,
        CmdChimeraType::CmdChimeraGetDataScRsp,
        CmdChimeraType::CmdChimeraRoundWorkStartScRsp,
        CmdChimeraType::CmdChimeraFinishEndlessRoundCsReq,
        CmdChimeraType::CmdChimeraSetLineupCsReq,
        CmdChimeraType::CmdChimeraSetLineupScRsp,
        CmdChimeraType::CmdChimeraQuitEndlessScRsp,
        CmdChimeraType::CmdChimeraDoFinalRoundCsReq,
        CmdChimeraType::CmdChimeraRoundWorkStartCsReq,
        CmdChimeraType::CmdChimeraFinishRoundScRsp,
        CmdChimeraType::CmdChimeraStartEndlessCsReq,
        CmdChimeraType::CmdChimeraQuitEndlessCsReq,
    ];
}

impl ::protobuf::EnumFull for CmdChimeraType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdChimeraType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdChimeraType::CmdChimeraTypeNone => 0,
            CmdChimeraType::CmdChimeraGetDataCsReq => 1,
            CmdChimeraType::CmdChimeraFinishEndlessRoundScRsp => 2,
            CmdChimeraType::CmdChimeraDoFinalRoundScRsp => 3,
            CmdChimeraType::CmdChimeraFinishRoundCsReq => 4,
            CmdChimeraType::CmdChimeraStartEndlessScRsp => 5,
            CmdChimeraType::CmdChimeraGetDataScRsp => 6,
            CmdChimeraType::CmdChimeraRoundWorkStartScRsp => 7,
            CmdChimeraType::CmdChimeraFinishEndlessRoundCsReq => 8,
            CmdChimeraType::CmdChimeraSetLineupCsReq => 9,
            CmdChimeraType::CmdChimeraSetLineupScRsp => 10,
            CmdChimeraType::CmdChimeraQuitEndlessScRsp => 11,
            CmdChimeraType::CmdChimeraDoFinalRoundCsReq => 12,
            CmdChimeraType::CmdChimeraRoundWorkStartCsReq => 13,
            CmdChimeraType::CmdChimeraFinishRoundScRsp => 14,
            CmdChimeraType::CmdChimeraStartEndlessCsReq => 15,
            CmdChimeraType::CmdChimeraQuitEndlessCsReq => 16,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdChimeraType {
    fn default() -> Self {
        CmdChimeraType::CmdChimeraTypeNone
    }
}

impl CmdChimeraType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdChimeraType>("CmdChimeraType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14CmdChimeraType.proto*\xc4\x04\n\x0eCmdChimeraType\x12\x16\n\x12Cmd\
    ChimeraTypeNone\x10\0\x12\x1b\n\x16CmdChimeraGetDataCsReq\x10\xf1?\x12&\
    \n!CmdChimeraFinishEndlessRoundScRsp\x10\xf2?\x12\x20\n\x1bCmdChimeraDoF\
    inalRoundScRsp\x10\xe3?\x12\x1f\n\x1aCmdChimeraFinishRoundCsReq\x10\xee?\
    \x12\x20\n\x1bCmdChimeraStartEndlessScRsp\x10\xeb?\x12\x1b\n\x16CmdChime\
    raGetDataScRsp\x10\xf0?\x12\"\n\x1dCmdChimeraRoundWorkStartScRsp\x10\xec\
    ?\x12&\n!CmdChimeraFinishEndlessRoundCsReq\x10\xea?\x12\x1d\n\x18CmdChim\
    eraSetLineupCsReq\x10\xf4?\x12\x1d\n\x18CmdChimeraSetLineupScRsp\x10\xe4\
    ?\x12\x1f\n\x1aCmdChimeraQuitEndlessScRsp\x10\xef?\x12\x20\n\x1bCmdChime\
    raDoFinalRoundCsReq\x10\xed?\x12\"\n\x1dCmdChimeraRoundWorkStartCsReq\
    \x10\xe5?\x12\x1f\n\x1aCmdChimeraFinishRoundScRsp\x10\xe9?\x12\x20\n\x1b\
    CmdChimeraStartEndlessCsReq\x10\xf3?\x12\x1f\n\x1aCmdChimeraQuitEndlessC\
    sReq\x10\xe7?b\x06proto3\
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
            enums.push(CmdChimeraType::generated_enum_descriptor_data());
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
