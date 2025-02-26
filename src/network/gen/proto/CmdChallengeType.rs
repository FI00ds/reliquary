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

//! Generated file from `CmdChallengeType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdChallengeType)
pub enum CmdChallengeType {
    // @@protoc_insertion_point(enum_value:CmdChallengeType.CmdChallengeTypeNone)
    CmdChallengeTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdChallengeType.CmdStartChallengeCsReq)
    CmdStartChallengeCsReq = 1758,
    // @@protoc_insertion_point(enum_value:CmdChallengeType.CmdChallengeBossPhaseSettleNotify)
    CmdChallengeBossPhaseSettleNotify = 1748,
    // @@protoc_insertion_point(enum_value:CmdChallengeType.CmdLeaveChallengeScRsp)
    CmdLeaveChallengeScRsp = 1756,
    // @@protoc_insertion_point(enum_value:CmdChallengeType.CmdStartPartialChallengeScRsp)
    CmdStartPartialChallengeScRsp = 1712,
    // @@protoc_insertion_point(enum_value:CmdChallengeType.CmdStartChallengeScRsp)
    CmdStartChallengeScRsp = 1724,
    // @@protoc_insertion_point(enum_value:CmdChallengeType.CmdGetChallengeGroupStatisticsCsReq)
    CmdGetChallengeGroupStatisticsCsReq = 1739,
    // @@protoc_insertion_point(enum_value:CmdChallengeType.CmdRestartChallengePhaseCsReq)
    CmdRestartChallengePhaseCsReq = 1791,
    // @@protoc_insertion_point(enum_value:CmdChallengeType.CmdRestartChallengePhaseScRsp)
    CmdRestartChallengePhaseScRsp = 1718,
    // @@protoc_insertion_point(enum_value:CmdChallengeType.CmdTakeChallengeRewardScRsp)
    CmdTakeChallengeRewardScRsp = 1780,
    // @@protoc_insertion_point(enum_value:CmdChallengeType.CmdGetCurChallengeCsReq)
    CmdGetCurChallengeCsReq = 1711,
    // @@protoc_insertion_point(enum_value:CmdChallengeType.CmdGetCurChallengeScRsp)
    CmdGetCurChallengeScRsp = 1705,
    // @@protoc_insertion_point(enum_value:CmdChallengeType.CmdChallengeLineupNotify)
    CmdChallengeLineupNotify = 1714,
    // @@protoc_insertion_point(enum_value:CmdChallengeType.CmdLeaveChallengeCsReq)
    CmdLeaveChallengeCsReq = 1730,
    // @@protoc_insertion_point(enum_value:CmdChallengeType.CmdGetChallengeCsReq)
    CmdGetChallengeCsReq = 1701,
    // @@protoc_insertion_point(enum_value:CmdChallengeType.CmdGetChallengeGroupStatisticsScRsp)
    CmdGetChallengeGroupStatisticsScRsp = 1796,
    // @@protoc_insertion_point(enum_value:CmdChallengeType.CmdChallengeSettleNotify)
    CmdChallengeSettleNotify = 1797,
    // @@protoc_insertion_point(enum_value:CmdChallengeType.CmdTakeChallengeRewardCsReq)
    CmdTakeChallengeRewardCsReq = 1747,
    // @@protoc_insertion_point(enum_value:CmdChallengeType.CmdStartPartialChallengeCsReq)
    CmdStartPartialChallengeCsReq = 1723,
    // @@protoc_insertion_point(enum_value:CmdChallengeType.CmdGetChallengeScRsp)
    CmdGetChallengeScRsp = 1768,
    // @@protoc_insertion_point(enum_value:CmdChallengeType.CmdEnterChallengeNextPhaseScRsp)
    CmdEnterChallengeNextPhaseScRsp = 1750,
    // @@protoc_insertion_point(enum_value:CmdChallengeType.CmdEnterChallengeNextPhaseCsReq)
    CmdEnterChallengeNextPhaseCsReq = 1795,
}

impl ::protobuf::Enum for CmdChallengeType {
    const NAME: &'static str = "CmdChallengeType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdChallengeType> {
        match value {
            0 => ::std::option::Option::Some(CmdChallengeType::CmdChallengeTypeNone),
            1758 => ::std::option::Option::Some(CmdChallengeType::CmdStartChallengeCsReq),
            1748 => ::std::option::Option::Some(CmdChallengeType::CmdChallengeBossPhaseSettleNotify),
            1756 => ::std::option::Option::Some(CmdChallengeType::CmdLeaveChallengeScRsp),
            1712 => ::std::option::Option::Some(CmdChallengeType::CmdStartPartialChallengeScRsp),
            1724 => ::std::option::Option::Some(CmdChallengeType::CmdStartChallengeScRsp),
            1739 => ::std::option::Option::Some(CmdChallengeType::CmdGetChallengeGroupStatisticsCsReq),
            1791 => ::std::option::Option::Some(CmdChallengeType::CmdRestartChallengePhaseCsReq),
            1718 => ::std::option::Option::Some(CmdChallengeType::CmdRestartChallengePhaseScRsp),
            1780 => ::std::option::Option::Some(CmdChallengeType::CmdTakeChallengeRewardScRsp),
            1711 => ::std::option::Option::Some(CmdChallengeType::CmdGetCurChallengeCsReq),
            1705 => ::std::option::Option::Some(CmdChallengeType::CmdGetCurChallengeScRsp),
            1714 => ::std::option::Option::Some(CmdChallengeType::CmdChallengeLineupNotify),
            1730 => ::std::option::Option::Some(CmdChallengeType::CmdLeaveChallengeCsReq),
            1701 => ::std::option::Option::Some(CmdChallengeType::CmdGetChallengeCsReq),
            1796 => ::std::option::Option::Some(CmdChallengeType::CmdGetChallengeGroupStatisticsScRsp),
            1797 => ::std::option::Option::Some(CmdChallengeType::CmdChallengeSettleNotify),
            1747 => ::std::option::Option::Some(CmdChallengeType::CmdTakeChallengeRewardCsReq),
            1723 => ::std::option::Option::Some(CmdChallengeType::CmdStartPartialChallengeCsReq),
            1768 => ::std::option::Option::Some(CmdChallengeType::CmdGetChallengeScRsp),
            1750 => ::std::option::Option::Some(CmdChallengeType::CmdEnterChallengeNextPhaseScRsp),
            1795 => ::std::option::Option::Some(CmdChallengeType::CmdEnterChallengeNextPhaseCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdChallengeType> {
        match str {
            "CmdChallengeTypeNone" => ::std::option::Option::Some(CmdChallengeType::CmdChallengeTypeNone),
            "CmdStartChallengeCsReq" => ::std::option::Option::Some(CmdChallengeType::CmdStartChallengeCsReq),
            "CmdChallengeBossPhaseSettleNotify" => ::std::option::Option::Some(CmdChallengeType::CmdChallengeBossPhaseSettleNotify),
            "CmdLeaveChallengeScRsp" => ::std::option::Option::Some(CmdChallengeType::CmdLeaveChallengeScRsp),
            "CmdStartPartialChallengeScRsp" => ::std::option::Option::Some(CmdChallengeType::CmdStartPartialChallengeScRsp),
            "CmdStartChallengeScRsp" => ::std::option::Option::Some(CmdChallengeType::CmdStartChallengeScRsp),
            "CmdGetChallengeGroupStatisticsCsReq" => ::std::option::Option::Some(CmdChallengeType::CmdGetChallengeGroupStatisticsCsReq),
            "CmdRestartChallengePhaseCsReq" => ::std::option::Option::Some(CmdChallengeType::CmdRestartChallengePhaseCsReq),
            "CmdRestartChallengePhaseScRsp" => ::std::option::Option::Some(CmdChallengeType::CmdRestartChallengePhaseScRsp),
            "CmdTakeChallengeRewardScRsp" => ::std::option::Option::Some(CmdChallengeType::CmdTakeChallengeRewardScRsp),
            "CmdGetCurChallengeCsReq" => ::std::option::Option::Some(CmdChallengeType::CmdGetCurChallengeCsReq),
            "CmdGetCurChallengeScRsp" => ::std::option::Option::Some(CmdChallengeType::CmdGetCurChallengeScRsp),
            "CmdChallengeLineupNotify" => ::std::option::Option::Some(CmdChallengeType::CmdChallengeLineupNotify),
            "CmdLeaveChallengeCsReq" => ::std::option::Option::Some(CmdChallengeType::CmdLeaveChallengeCsReq),
            "CmdGetChallengeCsReq" => ::std::option::Option::Some(CmdChallengeType::CmdGetChallengeCsReq),
            "CmdGetChallengeGroupStatisticsScRsp" => ::std::option::Option::Some(CmdChallengeType::CmdGetChallengeGroupStatisticsScRsp),
            "CmdChallengeSettleNotify" => ::std::option::Option::Some(CmdChallengeType::CmdChallengeSettleNotify),
            "CmdTakeChallengeRewardCsReq" => ::std::option::Option::Some(CmdChallengeType::CmdTakeChallengeRewardCsReq),
            "CmdStartPartialChallengeCsReq" => ::std::option::Option::Some(CmdChallengeType::CmdStartPartialChallengeCsReq),
            "CmdGetChallengeScRsp" => ::std::option::Option::Some(CmdChallengeType::CmdGetChallengeScRsp),
            "CmdEnterChallengeNextPhaseScRsp" => ::std::option::Option::Some(CmdChallengeType::CmdEnterChallengeNextPhaseScRsp),
            "CmdEnterChallengeNextPhaseCsReq" => ::std::option::Option::Some(CmdChallengeType::CmdEnterChallengeNextPhaseCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdChallengeType] = &[
        CmdChallengeType::CmdChallengeTypeNone,
        CmdChallengeType::CmdStartChallengeCsReq,
        CmdChallengeType::CmdChallengeBossPhaseSettleNotify,
        CmdChallengeType::CmdLeaveChallengeScRsp,
        CmdChallengeType::CmdStartPartialChallengeScRsp,
        CmdChallengeType::CmdStartChallengeScRsp,
        CmdChallengeType::CmdGetChallengeGroupStatisticsCsReq,
        CmdChallengeType::CmdRestartChallengePhaseCsReq,
        CmdChallengeType::CmdRestartChallengePhaseScRsp,
        CmdChallengeType::CmdTakeChallengeRewardScRsp,
        CmdChallengeType::CmdGetCurChallengeCsReq,
        CmdChallengeType::CmdGetCurChallengeScRsp,
        CmdChallengeType::CmdChallengeLineupNotify,
        CmdChallengeType::CmdLeaveChallengeCsReq,
        CmdChallengeType::CmdGetChallengeCsReq,
        CmdChallengeType::CmdGetChallengeGroupStatisticsScRsp,
        CmdChallengeType::CmdChallengeSettleNotify,
        CmdChallengeType::CmdTakeChallengeRewardCsReq,
        CmdChallengeType::CmdStartPartialChallengeCsReq,
        CmdChallengeType::CmdGetChallengeScRsp,
        CmdChallengeType::CmdEnterChallengeNextPhaseScRsp,
        CmdChallengeType::CmdEnterChallengeNextPhaseCsReq,
    ];
}

impl ::protobuf::EnumFull for CmdChallengeType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdChallengeType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdChallengeType::CmdChallengeTypeNone => 0,
            CmdChallengeType::CmdStartChallengeCsReq => 1,
            CmdChallengeType::CmdChallengeBossPhaseSettleNotify => 2,
            CmdChallengeType::CmdLeaveChallengeScRsp => 3,
            CmdChallengeType::CmdStartPartialChallengeScRsp => 4,
            CmdChallengeType::CmdStartChallengeScRsp => 5,
            CmdChallengeType::CmdGetChallengeGroupStatisticsCsReq => 6,
            CmdChallengeType::CmdRestartChallengePhaseCsReq => 7,
            CmdChallengeType::CmdRestartChallengePhaseScRsp => 8,
            CmdChallengeType::CmdTakeChallengeRewardScRsp => 9,
            CmdChallengeType::CmdGetCurChallengeCsReq => 10,
            CmdChallengeType::CmdGetCurChallengeScRsp => 11,
            CmdChallengeType::CmdChallengeLineupNotify => 12,
            CmdChallengeType::CmdLeaveChallengeCsReq => 13,
            CmdChallengeType::CmdGetChallengeCsReq => 14,
            CmdChallengeType::CmdGetChallengeGroupStatisticsScRsp => 15,
            CmdChallengeType::CmdChallengeSettleNotify => 16,
            CmdChallengeType::CmdTakeChallengeRewardCsReq => 17,
            CmdChallengeType::CmdStartPartialChallengeCsReq => 18,
            CmdChallengeType::CmdGetChallengeScRsp => 19,
            CmdChallengeType::CmdEnterChallengeNextPhaseScRsp => 20,
            CmdChallengeType::CmdEnterChallengeNextPhaseCsReq => 21,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdChallengeType {
    fn default() -> Self {
        CmdChallengeType::CmdChallengeTypeNone
    }
}

impl CmdChallengeType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdChallengeType>("CmdChallengeType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16CmdChallengeType.proto*\xec\x05\n\x10CmdChallengeType\x12\x18\n\
    \x14CmdChallengeTypeNone\x10\0\x12\x1b\n\x16CmdStartChallengeCsReq\x10\
    \xde\r\x12&\n!CmdChallengeBossPhaseSettleNotify\x10\xd4\r\x12\x1b\n\x16C\
    mdLeaveChallengeScRsp\x10\xdc\r\x12\"\n\x1dCmdStartPartialChallengeScRsp\
    \x10\xb0\r\x12\x1b\n\x16CmdStartChallengeScRsp\x10\xbc\r\x12(\n#CmdGetCh\
    allengeGroupStatisticsCsReq\x10\xcb\r\x12\"\n\x1dCmdRestartChallengePhas\
    eCsReq\x10\xff\r\x12\"\n\x1dCmdRestartChallengePhaseScRsp\x10\xb6\r\x12\
    \x20\n\x1bCmdTakeChallengeRewardScRsp\x10\xf4\r\x12\x1c\n\x17CmdGetCurCh\
    allengeCsReq\x10\xaf\r\x12\x1c\n\x17CmdGetCurChallengeScRsp\x10\xa9\r\
    \x12\x1d\n\x18CmdChallengeLineupNotify\x10\xb2\r\x12\x1b\n\x16CmdLeaveCh\
    allengeCsReq\x10\xc2\r\x12\x19\n\x14CmdGetChallengeCsReq\x10\xa5\r\x12(\
    \n#CmdGetChallengeGroupStatisticsScRsp\x10\x84\x0e\x12\x1d\n\x18CmdChall\
    engeSettleNotify\x10\x85\x0e\x12\x20\n\x1bCmdTakeChallengeRewardCsReq\
    \x10\xd3\r\x12\"\n\x1dCmdStartPartialChallengeCsReq\x10\xbb\r\x12\x19\n\
    \x14CmdGetChallengeScRsp\x10\xe8\r\x12$\n\x1fCmdEnterChallengeNextPhaseS\
    cRsp\x10\xd6\r\x12$\n\x1fCmdEnterChallengeNextPhaseCsReq\x10\x83\x0eb\
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
            enums.push(CmdChallengeType::generated_enum_descriptor_data());
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
