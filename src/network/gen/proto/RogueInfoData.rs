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

//! Generated file from `RogueInfoData.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RogueInfoData)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueInfoData {
    // message fields
    // @@protoc_insertion_point(field:RogueInfoData.rogue_season_info)
    pub rogue_season_info: ::protobuf::MessageField<super::RogueSeasonInfo::RogueSeasonInfo>,
    // @@protoc_insertion_point(field:RogueInfoData.rogue_aeon_info)
    pub rogue_aeon_info: ::protobuf::MessageField<super::RogueAeonInfo::RogueAeonInfo>,
    // @@protoc_insertion_point(field:RogueInfoData.EEFGNNFCDNJ)
    pub EEFGNNFCDNJ: ::protobuf::MessageField<super::DMBBFODODOF::DMBBFODODOF>,
    // @@protoc_insertion_point(field:RogueInfoData.rogue_score_info)
    pub rogue_score_info: ::protobuf::MessageField<super::RogueScoreRewardInfo::RogueScoreRewardInfo>,
    // @@protoc_insertion_point(field:RogueInfoData.rogue_area_info)
    pub rogue_area_info: ::protobuf::MessageField<super::RogueAreaInfo::RogueAreaInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:RogueInfoData.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueInfoData {
    fn default() -> &'a RogueInfoData {
        <RogueInfoData as ::protobuf::Message>::default_instance()
    }
}

impl RogueInfoData {
    pub fn new() -> RogueInfoData {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::RogueSeasonInfo::RogueSeasonInfo>(
            "rogue_season_info",
            |m: &RogueInfoData| { &m.rogue_season_info },
            |m: &mut RogueInfoData| { &mut m.rogue_season_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::RogueAeonInfo::RogueAeonInfo>(
            "rogue_aeon_info",
            |m: &RogueInfoData| { &m.rogue_aeon_info },
            |m: &mut RogueInfoData| { &mut m.rogue_aeon_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::DMBBFODODOF::DMBBFODODOF>(
            "EEFGNNFCDNJ",
            |m: &RogueInfoData| { &m.EEFGNNFCDNJ },
            |m: &mut RogueInfoData| { &mut m.EEFGNNFCDNJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::RogueScoreRewardInfo::RogueScoreRewardInfo>(
            "rogue_score_info",
            |m: &RogueInfoData| { &m.rogue_score_info },
            |m: &mut RogueInfoData| { &mut m.rogue_score_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::RogueAreaInfo::RogueAreaInfo>(
            "rogue_area_info",
            |m: &RogueInfoData| { &m.rogue_area_info },
            |m: &mut RogueInfoData| { &mut m.rogue_area_info },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueInfoData>(
            "RogueInfoData",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueInfoData {
    const NAME: &'static str = "RogueInfoData";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.rogue_season_info)?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.rogue_aeon_info)?;
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.EEFGNNFCDNJ)?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.rogue_score_info)?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.rogue_area_info)?;
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
        if let Some(v) = self.rogue_season_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.rogue_aeon_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.EEFGNNFCDNJ.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.rogue_score_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.rogue_area_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.rogue_season_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if let Some(v) = self.rogue_aeon_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if let Some(v) = self.EEFGNNFCDNJ.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        if let Some(v) = self.rogue_score_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if let Some(v) = self.rogue_area_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
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

    fn new() -> RogueInfoData {
        RogueInfoData::new()
    }

    fn clear(&mut self) {
        self.rogue_season_info.clear();
        self.rogue_aeon_info.clear();
        self.EEFGNNFCDNJ.clear();
        self.rogue_score_info.clear();
        self.rogue_area_info.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueInfoData {
        static instance: RogueInfoData = RogueInfoData {
            rogue_season_info: ::protobuf::MessageField::none(),
            rogue_aeon_info: ::protobuf::MessageField::none(),
            EEFGNNFCDNJ: ::protobuf::MessageField::none(),
            rogue_score_info: ::protobuf::MessageField::none(),
            rogue_area_info: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RogueInfoData {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueInfoData").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueInfoData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueInfoData {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13RogueInfoData.proto\x1a\x15RogueSeasonInfo.proto\x1a\x1aRogueScore\
    RewardInfo.proto\x1a\x13RogueAeonInfo.proto\x1a\x11DMBBFODODOF.proto\x1a\
    \x13RogueAreaInfo.proto\"\xae\x02\n\rRogueInfoData\x12<\n\x11rogue_seaso\
    n_info\x18\x03\x20\x01(\x0b2\x10.RogueSeasonInfoR\x0frogueSeasonInfo\x12\
    6\n\x0frogue_aeon_info\x18\x0b\x20\x01(\x0b2\x0e.RogueAeonInfoR\rrogueAe\
    onInfo\x12.\n\x0bEEFGNNFCDNJ\x18\n\x20\x01(\x0b2\x0c.DMBBFODODOFR\x0bEEF\
    GNNFCDNJ\x12?\n\x10rogue_score_info\x18\x04\x20\x01(\x0b2\x15.RogueScore\
    RewardInfoR\x0erogueScoreInfo\x126\n\x0frogue_area_info\x18\x07\x20\x01(\
    \x0b2\x0e.RogueAreaInfoR\rrogueAreaInfoB\x15\n\x13emu.lunarcore.protob\
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
            let mut deps = ::std::vec::Vec::with_capacity(5);
            deps.push(super::RogueSeasonInfo::file_descriptor().clone());
            deps.push(super::RogueScoreRewardInfo::file_descriptor().clone());
            deps.push(super::RogueAeonInfo::file_descriptor().clone());
            deps.push(super::DMBBFODODOF::file_descriptor().clone());
            deps.push(super::RogueAreaInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RogueInfoData::generated_message_descriptor_data());
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
