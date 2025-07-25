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

//! Generated file from `GetBigDataRecommendScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:GetBigDataRecommendScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetBigDataRecommendScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetBigDataRecommendScRsp.big_data_recommend_type)
    pub big_data_recommend_type: ::protobuf::EnumOrUnknown<super::BigDataRecommendType::BigDataRecommendType>,
    // @@protoc_insertion_point(field:GetBigDataRecommendScRsp.ROGUE_TALENT_STATUS_ENABLE)
    pub ROGUE_TALENT_STATUS_ENABLE: bool,
    // @@protoc_insertion_point(field:GetBigDataRecommendScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:GetBigDataRecommendScRsp.recommended_avatar_id)
    pub recommended_avatar_id: u32,
    // message oneof groups
    pub DPCNJILLEHJ: ::std::option::Option<get_big_data_recommend_sc_rsp::DPCNJILLEHJ>,
    // special fields
    // @@protoc_insertion_point(special_field:GetBigDataRecommendScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetBigDataRecommendScRsp {
    fn default() -> &'a GetBigDataRecommendScRsp {
        <GetBigDataRecommendScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetBigDataRecommendScRsp {
    pub fn new() -> GetBigDataRecommendScRsp {
        ::std::default::Default::default()
    }

    // .PPGMPBHKDCN GPNFOLHKODI = 14;

    pub fn GPNFOLHKODI(&self) -> &super::PPGMPBHKDCN::PPGMPBHKDCN {
        match self.DPCNJILLEHJ {
            ::std::option::Option::Some(get_big_data_recommend_sc_rsp::DPCNJILLEHJ::GPNFOLHKODI(ref v)) => v,
            _ => <super::PPGMPBHKDCN::PPGMPBHKDCN as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_GPNFOLHKODI(&mut self) {
        self.DPCNJILLEHJ = ::std::option::Option::None;
    }

    pub fn has_GPNFOLHKODI(&self) -> bool {
        match self.DPCNJILLEHJ {
            ::std::option::Option::Some(get_big_data_recommend_sc_rsp::DPCNJILLEHJ::GPNFOLHKODI(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_GPNFOLHKODI(&mut self, v: super::PPGMPBHKDCN::PPGMPBHKDCN) {
        self.DPCNJILLEHJ = ::std::option::Option::Some(get_big_data_recommend_sc_rsp::DPCNJILLEHJ::GPNFOLHKODI(v))
    }

    // Mutable pointer to the field.
    pub fn mut_GPNFOLHKODI(&mut self) -> &mut super::PPGMPBHKDCN::PPGMPBHKDCN {
        if let ::std::option::Option::Some(get_big_data_recommend_sc_rsp::DPCNJILLEHJ::GPNFOLHKODI(_)) = self.DPCNJILLEHJ {
        } else {
            self.DPCNJILLEHJ = ::std::option::Option::Some(get_big_data_recommend_sc_rsp::DPCNJILLEHJ::GPNFOLHKODI(super::PPGMPBHKDCN::PPGMPBHKDCN::new()));
        }
        match self.DPCNJILLEHJ {
            ::std::option::Option::Some(get_big_data_recommend_sc_rsp::DPCNJILLEHJ::GPNFOLHKODI(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_GPNFOLHKODI(&mut self) -> super::PPGMPBHKDCN::PPGMPBHKDCN {
        if self.has_GPNFOLHKODI() {
            match self.DPCNJILLEHJ.take() {
                ::std::option::Option::Some(get_big_data_recommend_sc_rsp::DPCNJILLEHJ::GPNFOLHKODI(v)) => v,
                _ => panic!(),
            }
        } else {
            super::PPGMPBHKDCN::PPGMPBHKDCN::new()
        }
    }

    // .GDGOCBPAPPB AEIGAHEEOCN = 7;

    pub fn AEIGAHEEOCN(&self) -> &super::GDGOCBPAPPB::GDGOCBPAPPB {
        match self.DPCNJILLEHJ {
            ::std::option::Option::Some(get_big_data_recommend_sc_rsp::DPCNJILLEHJ::AEIGAHEEOCN(ref v)) => v,
            _ => <super::GDGOCBPAPPB::GDGOCBPAPPB as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_AEIGAHEEOCN(&mut self) {
        self.DPCNJILLEHJ = ::std::option::Option::None;
    }

    pub fn has_AEIGAHEEOCN(&self) -> bool {
        match self.DPCNJILLEHJ {
            ::std::option::Option::Some(get_big_data_recommend_sc_rsp::DPCNJILLEHJ::AEIGAHEEOCN(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_AEIGAHEEOCN(&mut self, v: super::GDGOCBPAPPB::GDGOCBPAPPB) {
        self.DPCNJILLEHJ = ::std::option::Option::Some(get_big_data_recommend_sc_rsp::DPCNJILLEHJ::AEIGAHEEOCN(v))
    }

    // Mutable pointer to the field.
    pub fn mut_AEIGAHEEOCN(&mut self) -> &mut super::GDGOCBPAPPB::GDGOCBPAPPB {
        if let ::std::option::Option::Some(get_big_data_recommend_sc_rsp::DPCNJILLEHJ::AEIGAHEEOCN(_)) = self.DPCNJILLEHJ {
        } else {
            self.DPCNJILLEHJ = ::std::option::Option::Some(get_big_data_recommend_sc_rsp::DPCNJILLEHJ::AEIGAHEEOCN(super::GDGOCBPAPPB::GDGOCBPAPPB::new()));
        }
        match self.DPCNJILLEHJ {
            ::std::option::Option::Some(get_big_data_recommend_sc_rsp::DPCNJILLEHJ::AEIGAHEEOCN(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_AEIGAHEEOCN(&mut self) -> super::GDGOCBPAPPB::GDGOCBPAPPB {
        if self.has_AEIGAHEEOCN() {
            match self.DPCNJILLEHJ.take() {
                ::std::option::Option::Some(get_big_data_recommend_sc_rsp::DPCNJILLEHJ::AEIGAHEEOCN(v)) => v,
                _ => panic!(),
            }
        } else {
            super::GDGOCBPAPPB::GDGOCBPAPPB::new()
        }
    }

    // .GPCPGBHOFCF KGHLFCEOBKK = 3;

    pub fn KGHLFCEOBKK(&self) -> &super::GPCPGBHOFCF::GPCPGBHOFCF {
        match self.DPCNJILLEHJ {
            ::std::option::Option::Some(get_big_data_recommend_sc_rsp::DPCNJILLEHJ::KGHLFCEOBKK(ref v)) => v,
            _ => <super::GPCPGBHOFCF::GPCPGBHOFCF as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_KGHLFCEOBKK(&mut self) {
        self.DPCNJILLEHJ = ::std::option::Option::None;
    }

    pub fn has_KGHLFCEOBKK(&self) -> bool {
        match self.DPCNJILLEHJ {
            ::std::option::Option::Some(get_big_data_recommend_sc_rsp::DPCNJILLEHJ::KGHLFCEOBKK(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_KGHLFCEOBKK(&mut self, v: super::GPCPGBHOFCF::GPCPGBHOFCF) {
        self.DPCNJILLEHJ = ::std::option::Option::Some(get_big_data_recommend_sc_rsp::DPCNJILLEHJ::KGHLFCEOBKK(v))
    }

    // Mutable pointer to the field.
    pub fn mut_KGHLFCEOBKK(&mut self) -> &mut super::GPCPGBHOFCF::GPCPGBHOFCF {
        if let ::std::option::Option::Some(get_big_data_recommend_sc_rsp::DPCNJILLEHJ::KGHLFCEOBKK(_)) = self.DPCNJILLEHJ {
        } else {
            self.DPCNJILLEHJ = ::std::option::Option::Some(get_big_data_recommend_sc_rsp::DPCNJILLEHJ::KGHLFCEOBKK(super::GPCPGBHOFCF::GPCPGBHOFCF::new()));
        }
        match self.DPCNJILLEHJ {
            ::std::option::Option::Some(get_big_data_recommend_sc_rsp::DPCNJILLEHJ::KGHLFCEOBKK(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_KGHLFCEOBKK(&mut self) -> super::GPCPGBHOFCF::GPCPGBHOFCF {
        if self.has_KGHLFCEOBKK() {
            match self.DPCNJILLEHJ.take() {
                ::std::option::Option::Some(get_big_data_recommend_sc_rsp::DPCNJILLEHJ::KGHLFCEOBKK(v)) => v,
                _ => panic!(),
            }
        } else {
            super::GPCPGBHOFCF::GPCPGBHOFCF::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "big_data_recommend_type",
            |m: &GetBigDataRecommendScRsp| { &m.big_data_recommend_type },
            |m: &mut GetBigDataRecommendScRsp| { &mut m.big_data_recommend_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ROGUE_TALENT_STATUS_ENABLE",
            |m: &GetBigDataRecommendScRsp| { &m.ROGUE_TALENT_STATUS_ENABLE },
            |m: &mut GetBigDataRecommendScRsp| { &mut m.ROGUE_TALENT_STATUS_ENABLE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetBigDataRecommendScRsp| { &m.retcode },
            |m: &mut GetBigDataRecommendScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "recommended_avatar_id",
            |m: &GetBigDataRecommendScRsp| { &m.recommended_avatar_id },
            |m: &mut GetBigDataRecommendScRsp| { &mut m.recommended_avatar_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::PPGMPBHKDCN::PPGMPBHKDCN>(
            "GPNFOLHKODI",
            GetBigDataRecommendScRsp::has_GPNFOLHKODI,
            GetBigDataRecommendScRsp::GPNFOLHKODI,
            GetBigDataRecommendScRsp::mut_GPNFOLHKODI,
            GetBigDataRecommendScRsp::set_GPNFOLHKODI,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::GDGOCBPAPPB::GDGOCBPAPPB>(
            "AEIGAHEEOCN",
            GetBigDataRecommendScRsp::has_AEIGAHEEOCN,
            GetBigDataRecommendScRsp::AEIGAHEEOCN,
            GetBigDataRecommendScRsp::mut_AEIGAHEEOCN,
            GetBigDataRecommendScRsp::set_AEIGAHEEOCN,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::GPCPGBHOFCF::GPCPGBHOFCF>(
            "KGHLFCEOBKK",
            GetBigDataRecommendScRsp::has_KGHLFCEOBKK,
            GetBigDataRecommendScRsp::KGHLFCEOBKK,
            GetBigDataRecommendScRsp::mut_KGHLFCEOBKK,
            GetBigDataRecommendScRsp::set_KGHLFCEOBKK,
        ));
        oneofs.push(get_big_data_recommend_sc_rsp::DPCNJILLEHJ::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetBigDataRecommendScRsp>(
            "GetBigDataRecommendScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetBigDataRecommendScRsp {
    const NAME: &'static str = "GetBigDataRecommendScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.big_data_recommend_type = is.read_enum_or_unknown()?;
                },
                32 => {
                    self.ROGUE_TALENT_STATUS_ENABLE = is.read_bool()?;
                },
                16 => {
                    self.retcode = is.read_uint32()?;
                },
                40 => {
                    self.recommended_avatar_id = is.read_uint32()?;
                },
                114 => {
                    self.DPCNJILLEHJ = ::std::option::Option::Some(get_big_data_recommend_sc_rsp::DPCNJILLEHJ::GPNFOLHKODI(is.read_message()?));
                },
                58 => {
                    self.DPCNJILLEHJ = ::std::option::Option::Some(get_big_data_recommend_sc_rsp::DPCNJILLEHJ::AEIGAHEEOCN(is.read_message()?));
                },
                26 => {
                    self.DPCNJILLEHJ = ::std::option::Option::Some(get_big_data_recommend_sc_rsp::DPCNJILLEHJ::KGHLFCEOBKK(is.read_message()?));
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.big_data_recommend_type != ::protobuf::EnumOrUnknown::new(super::BigDataRecommendType::BigDataRecommendType::BIG_DATA_RECOMMEND_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(6, self.big_data_recommend_type.value());
        }
        if self.ROGUE_TALENT_STATUS_ENABLE != false {
            my_size += 1 + 1;
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.retcode);
        }
        if self.recommended_avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.recommended_avatar_id);
        }
        if let ::std::option::Option::Some(ref v) = self.DPCNJILLEHJ {
            match v {
                &get_big_data_recommend_sc_rsp::DPCNJILLEHJ::GPNFOLHKODI(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &get_big_data_recommend_sc_rsp::DPCNJILLEHJ::AEIGAHEEOCN(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &get_big_data_recommend_sc_rsp::DPCNJILLEHJ::KGHLFCEOBKK(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.big_data_recommend_type != ::protobuf::EnumOrUnknown::new(super::BigDataRecommendType::BigDataRecommendType::BIG_DATA_RECOMMEND_TYPE_NONE) {
            os.write_enum(6, ::protobuf::EnumOrUnknown::value(&self.big_data_recommend_type))?;
        }
        if self.ROGUE_TALENT_STATUS_ENABLE != false {
            os.write_bool(4, self.ROGUE_TALENT_STATUS_ENABLE)?;
        }
        if self.retcode != 0 {
            os.write_uint32(2, self.retcode)?;
        }
        if self.recommended_avatar_id != 0 {
            os.write_uint32(5, self.recommended_avatar_id)?;
        }
        if let ::std::option::Option::Some(ref v) = self.DPCNJILLEHJ {
            match v {
                &get_big_data_recommend_sc_rsp::DPCNJILLEHJ::GPNFOLHKODI(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
                },
                &get_big_data_recommend_sc_rsp::DPCNJILLEHJ::AEIGAHEEOCN(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
                },
                &get_big_data_recommend_sc_rsp::DPCNJILLEHJ::KGHLFCEOBKK(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
                },
            };
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> GetBigDataRecommendScRsp {
        GetBigDataRecommendScRsp::new()
    }

    fn clear(&mut self) {
        self.big_data_recommend_type = ::protobuf::EnumOrUnknown::new(super::BigDataRecommendType::BigDataRecommendType::BIG_DATA_RECOMMEND_TYPE_NONE);
        self.ROGUE_TALENT_STATUS_ENABLE = false;
        self.retcode = 0;
        self.recommended_avatar_id = 0;
        self.DPCNJILLEHJ = ::std::option::Option::None;
        self.DPCNJILLEHJ = ::std::option::Option::None;
        self.DPCNJILLEHJ = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetBigDataRecommendScRsp {
        static instance: GetBigDataRecommendScRsp = GetBigDataRecommendScRsp {
            big_data_recommend_type: ::protobuf::EnumOrUnknown::from_i32(0),
            ROGUE_TALENT_STATUS_ENABLE: false,
            retcode: 0,
            recommended_avatar_id: 0,
            DPCNJILLEHJ: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetBigDataRecommendScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetBigDataRecommendScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetBigDataRecommendScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetBigDataRecommendScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `GetBigDataRecommendScRsp`
pub mod get_big_data_recommend_sc_rsp {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:GetBigDataRecommendScRsp.DPCNJILLEHJ)
    pub enum DPCNJILLEHJ {
        // @@protoc_insertion_point(oneof_field:GetBigDataRecommendScRsp.GPNFOLHKODI)
        GPNFOLHKODI(super::super::PPGMPBHKDCN::PPGMPBHKDCN),
        // @@protoc_insertion_point(oneof_field:GetBigDataRecommendScRsp.AEIGAHEEOCN)
        AEIGAHEEOCN(super::super::GDGOCBPAPPB::GDGOCBPAPPB),
        // @@protoc_insertion_point(oneof_field:GetBigDataRecommendScRsp.KGHLFCEOBKK)
        KGHLFCEOBKK(super::super::GPCPGBHOFCF::GPCPGBHOFCF),
    }

    impl ::protobuf::Oneof for DPCNJILLEHJ {
    }

    impl ::protobuf::OneofFull for DPCNJILLEHJ {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::GetBigDataRecommendScRsp as ::protobuf::MessageFull>::descriptor().oneof_by_name("DPCNJILLEHJ").unwrap()).clone()
        }
    }

    impl DPCNJILLEHJ {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<DPCNJILLEHJ>("DPCNJILLEHJ")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1eGetBigDataRecommendScRsp.proto\x1a\x1aBigDataRecommendType.proto\
    \x1a\x11GDGOCBPAPPB.proto\x1a\x11GPCPGBHOFCF.proto\x1a\x11PPGMPBHKDCN.pr\
    oto\"\x98\x03\n\x18GetBigDataRecommendScRsp\x12L\n\x17big_data_recommend\
    _type\x18\x06\x20\x01(\x0e2\x15.BigDataRecommendTypeR\x14bigDataRecommen\
    dType\x12;\n\x1aROGUE_TALENT_STATUS_ENABLE\x18\x04\x20\x01(\x08R\x17ROGU\
    ETALENTSTATUSENABLE\x12\x18\n\x07retcode\x18\x02\x20\x01(\rR\x07retcode\
    \x122\n\x15recommended_avatar_id\x18\x05\x20\x01(\rR\x13recommendedAvata\
    rId\x120\n\x0bGPNFOLHKODI\x18\x0e\x20\x01(\x0b2\x0c.PPGMPBHKDCNH\0R\x0bG\
    PNFOLHKODI\x120\n\x0bAEIGAHEEOCN\x18\x07\x20\x01(\x0b2\x0c.GDGOCBPAPPBH\
    \0R\x0bAEIGAHEEOCN\x120\n\x0bKGHLFCEOBKK\x18\x03\x20\x01(\x0b2\x0c.GPCPG\
    BHOFCFH\0R\x0bKGHLFCEOBKKB\r\n\x0bDPCNJILLEHJb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::BigDataRecommendType::file_descriptor().clone());
            deps.push(super::GDGOCBPAPPB::file_descriptor().clone());
            deps.push(super::GPCPGBHOFCF::file_descriptor().clone());
            deps.push(super::PPGMPBHKDCN::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetBigDataRecommendScRsp::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
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
