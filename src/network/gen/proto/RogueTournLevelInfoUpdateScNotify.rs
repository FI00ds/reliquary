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

//! Generated file from `RogueTournLevelInfoUpdateScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:RogueTournLevelInfoUpdateScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueTournLevelInfoUpdateScNotify {
    // message fields
    // @@protoc_insertion_point(field:RogueTournLevelInfoUpdateScNotify.LFCDODFMHHN)
    pub LFCDODFMHHN: u32,
    // @@protoc_insertion_point(field:RogueTournLevelInfoUpdateScNotify.AHOOAFGDEHF)
    pub AHOOAFGDEHF: ::std::vec::Vec<super::BABHBOMOMDF::BABHBOMOMDF>,
    // @@protoc_insertion_point(field:RogueTournLevelInfoUpdateScNotify.PBLFLJNHMIL)
    pub PBLFLJNHMIL: ::protobuf::EnumOrUnknown<super::RogueTournLevelStatus::RogueTournLevelStatus>,
    // @@protoc_insertion_point(field:RogueTournLevelInfoUpdateScNotify.ALIFPIHNMEK)
    pub ALIFPIHNMEK: ::protobuf::EnumOrUnknown<super::AKKHKMECAFL::AKKHKMECAFL>,
    // special fields
    // @@protoc_insertion_point(special_field:RogueTournLevelInfoUpdateScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueTournLevelInfoUpdateScNotify {
    fn default() -> &'a RogueTournLevelInfoUpdateScNotify {
        <RogueTournLevelInfoUpdateScNotify as ::protobuf::Message>::default_instance()
    }
}

impl RogueTournLevelInfoUpdateScNotify {
    pub fn new() -> RogueTournLevelInfoUpdateScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LFCDODFMHHN",
            |m: &RogueTournLevelInfoUpdateScNotify| { &m.LFCDODFMHHN },
            |m: &mut RogueTournLevelInfoUpdateScNotify| { &mut m.LFCDODFMHHN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "AHOOAFGDEHF",
            |m: &RogueTournLevelInfoUpdateScNotify| { &m.AHOOAFGDEHF },
            |m: &mut RogueTournLevelInfoUpdateScNotify| { &mut m.AHOOAFGDEHF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PBLFLJNHMIL",
            |m: &RogueTournLevelInfoUpdateScNotify| { &m.PBLFLJNHMIL },
            |m: &mut RogueTournLevelInfoUpdateScNotify| { &mut m.PBLFLJNHMIL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ALIFPIHNMEK",
            |m: &RogueTournLevelInfoUpdateScNotify| { &m.ALIFPIHNMEK },
            |m: &mut RogueTournLevelInfoUpdateScNotify| { &mut m.ALIFPIHNMEK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueTournLevelInfoUpdateScNotify>(
            "RogueTournLevelInfoUpdateScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueTournLevelInfoUpdateScNotify {
    const NAME: &'static str = "RogueTournLevelInfoUpdateScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.LFCDODFMHHN = is.read_uint32()?;
                },
                42 => {
                    self.AHOOAFGDEHF.push(is.read_message()?);
                },
                96 => {
                    self.PBLFLJNHMIL = is.read_enum_or_unknown()?;
                },
                48 => {
                    self.ALIFPIHNMEK = is.read_enum_or_unknown()?;
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
        if self.LFCDODFMHHN != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.LFCDODFMHHN);
        }
        for value in &self.AHOOAFGDEHF {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.PBLFLJNHMIL != ::protobuf::EnumOrUnknown::new(super::RogueTournLevelStatus::RogueTournLevelStatus::ROGUE_TOURN_LEVEL_STATUS_NONE) {
            my_size += ::protobuf::rt::int32_size(12, self.PBLFLJNHMIL.value());
        }
        if self.ALIFPIHNMEK != ::protobuf::EnumOrUnknown::new(super::AKKHKMECAFL::AKKHKMECAFL::ROGUE_TOURN_SETTLE_REASON_NONE) {
            my_size += ::protobuf::rt::int32_size(6, self.ALIFPIHNMEK.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.LFCDODFMHHN != 0 {
            os.write_uint32(7, self.LFCDODFMHHN)?;
        }
        for v in &self.AHOOAFGDEHF {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        if self.PBLFLJNHMIL != ::protobuf::EnumOrUnknown::new(super::RogueTournLevelStatus::RogueTournLevelStatus::ROGUE_TOURN_LEVEL_STATUS_NONE) {
            os.write_enum(12, ::protobuf::EnumOrUnknown::value(&self.PBLFLJNHMIL))?;
        }
        if self.ALIFPIHNMEK != ::protobuf::EnumOrUnknown::new(super::AKKHKMECAFL::AKKHKMECAFL::ROGUE_TOURN_SETTLE_REASON_NONE) {
            os.write_enum(6, ::protobuf::EnumOrUnknown::value(&self.ALIFPIHNMEK))?;
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

    fn new() -> RogueTournLevelInfoUpdateScNotify {
        RogueTournLevelInfoUpdateScNotify::new()
    }

    fn clear(&mut self) {
        self.LFCDODFMHHN = 0;
        self.AHOOAFGDEHF.clear();
        self.PBLFLJNHMIL = ::protobuf::EnumOrUnknown::new(super::RogueTournLevelStatus::RogueTournLevelStatus::ROGUE_TOURN_LEVEL_STATUS_NONE);
        self.ALIFPIHNMEK = ::protobuf::EnumOrUnknown::new(super::AKKHKMECAFL::AKKHKMECAFL::ROGUE_TOURN_SETTLE_REASON_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueTournLevelInfoUpdateScNotify {
        static instance: RogueTournLevelInfoUpdateScNotify = RogueTournLevelInfoUpdateScNotify {
            LFCDODFMHHN: 0,
            AHOOAFGDEHF: ::std::vec::Vec::new(),
            PBLFLJNHMIL: ::protobuf::EnumOrUnknown::from_i32(0),
            ALIFPIHNMEK: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RogueTournLevelInfoUpdateScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueTournLevelInfoUpdateScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueTournLevelInfoUpdateScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueTournLevelInfoUpdateScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'RogueTournLevelInfoUpdateScNotify.proto\x1a\x11AKKHKMECAFL.proto\x1a\
    \x11BABHBOMOMDF.proto\x1a\x1bRogueTournLevelStatus.proto\"\xdf\x01\n!Rog\
    ueTournLevelInfoUpdateScNotify\x12\x20\n\x0bLFCDODFMHHN\x18\x07\x20\x01(\
    \rR\x0bLFCDODFMHHN\x12.\n\x0bAHOOAFGDEHF\x18\x05\x20\x03(\x0b2\x0c.BABHB\
    OMOMDFR\x0bAHOOAFGDEHF\x128\n\x0bPBLFLJNHMIL\x18\x0c\x20\x01(\x0e2\x16.R\
    ogueTournLevelStatusR\x0bPBLFLJNHMIL\x12.\n\x0bALIFPIHNMEK\x18\x06\x20\
    \x01(\x0e2\x0c.AKKHKMECAFLR\x0bALIFPIHNMEKb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::AKKHKMECAFL::file_descriptor().clone());
            deps.push(super::BABHBOMOMDF::file_descriptor().clone());
            deps.push(super::RogueTournLevelStatus::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RogueTournLevelInfoUpdateScNotify::generated_message_descriptor_data());
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
