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

//! Generated file from `EnterMapRotationRegionScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:EnterMapRotationRegionScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EnterMapRotationRegionScRsp {
    // message fields
    // @@protoc_insertion_point(field:EnterMapRotationRegionScRsp.NFLBONDJAIE)
    pub NFLBONDJAIE: u32,
    // @@protoc_insertion_point(field:EnterMapRotationRegionScRsp.ACNPBBNLMIE)
    pub ACNPBBNLMIE: u32,
    // @@protoc_insertion_point(field:EnterMapRotationRegionScRsp.NMCNCKKMMOD)
    pub NMCNCKKMMOD: u32,
    // @@protoc_insertion_point(field:EnterMapRotationRegionScRsp.MFNBANEDODD)
    pub MFNBANEDODD: ::protobuf::MessageField<super::CFKHKILIHHF::CFKHKILIHHF>,
    // @@protoc_insertion_point(field:EnterMapRotationRegionScRsp.FGNHLHEDLPN)
    pub FGNHLHEDLPN: ::protobuf::MessageField<super::PBNBNCJCPEI::PBNBNCJCPEI>,
    // @@protoc_insertion_point(field:EnterMapRotationRegionScRsp.retcode)
    pub retcode: u32,
    // special fields
    // @@protoc_insertion_point(special_field:EnterMapRotationRegionScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EnterMapRotationRegionScRsp {
    fn default() -> &'a EnterMapRotationRegionScRsp {
        <EnterMapRotationRegionScRsp as ::protobuf::Message>::default_instance()
    }
}

impl EnterMapRotationRegionScRsp {
    pub fn new() -> EnterMapRotationRegionScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NFLBONDJAIE",
            |m: &EnterMapRotationRegionScRsp| { &m.NFLBONDJAIE },
            |m: &mut EnterMapRotationRegionScRsp| { &mut m.NFLBONDJAIE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ACNPBBNLMIE",
            |m: &EnterMapRotationRegionScRsp| { &m.ACNPBBNLMIE },
            |m: &mut EnterMapRotationRegionScRsp| { &mut m.ACNPBBNLMIE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NMCNCKKMMOD",
            |m: &EnterMapRotationRegionScRsp| { &m.NMCNCKKMMOD },
            |m: &mut EnterMapRotationRegionScRsp| { &mut m.NMCNCKKMMOD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::CFKHKILIHHF::CFKHKILIHHF>(
            "MFNBANEDODD",
            |m: &EnterMapRotationRegionScRsp| { &m.MFNBANEDODD },
            |m: &mut EnterMapRotationRegionScRsp| { &mut m.MFNBANEDODD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PBNBNCJCPEI::PBNBNCJCPEI>(
            "FGNHLHEDLPN",
            |m: &EnterMapRotationRegionScRsp| { &m.FGNHLHEDLPN },
            |m: &mut EnterMapRotationRegionScRsp| { &mut m.FGNHLHEDLPN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &EnterMapRotationRegionScRsp| { &m.retcode },
            |m: &mut EnterMapRotationRegionScRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EnterMapRotationRegionScRsp>(
            "EnterMapRotationRegionScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EnterMapRotationRegionScRsp {
    const NAME: &'static str = "EnterMapRotationRegionScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.NFLBONDJAIE = is.read_uint32()?;
                },
                8 => {
                    self.ACNPBBNLMIE = is.read_uint32()?;
                },
                88 => {
                    self.NMCNCKKMMOD = is.read_uint32()?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.MFNBANEDODD)?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.FGNHLHEDLPN)?;
                },
                72 => {
                    self.retcode = is.read_uint32()?;
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
        if self.NFLBONDJAIE != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.NFLBONDJAIE);
        }
        if self.ACNPBBNLMIE != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.ACNPBBNLMIE);
        }
        if self.NMCNCKKMMOD != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.NMCNCKKMMOD);
        }
        if let Some(v) = self.MFNBANEDODD.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.FGNHLHEDLPN.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.NFLBONDJAIE != 0 {
            os.write_uint32(7, self.NFLBONDJAIE)?;
        }
        if self.ACNPBBNLMIE != 0 {
            os.write_uint32(1, self.ACNPBBNLMIE)?;
        }
        if self.NMCNCKKMMOD != 0 {
            os.write_uint32(11, self.NMCNCKKMMOD)?;
        }
        if let Some(v) = self.MFNBANEDODD.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if let Some(v) = self.FGNHLHEDLPN.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if self.retcode != 0 {
            os.write_uint32(9, self.retcode)?;
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

    fn new() -> EnterMapRotationRegionScRsp {
        EnterMapRotationRegionScRsp::new()
    }

    fn clear(&mut self) {
        self.NFLBONDJAIE = 0;
        self.ACNPBBNLMIE = 0;
        self.NMCNCKKMMOD = 0;
        self.MFNBANEDODD.clear();
        self.FGNHLHEDLPN.clear();
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EnterMapRotationRegionScRsp {
        static instance: EnterMapRotationRegionScRsp = EnterMapRotationRegionScRsp {
            NFLBONDJAIE: 0,
            ACNPBBNLMIE: 0,
            NMCNCKKMMOD: 0,
            MFNBANEDODD: ::protobuf::MessageField::none(),
            FGNHLHEDLPN: ::protobuf::MessageField::none(),
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EnterMapRotationRegionScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EnterMapRotationRegionScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EnterMapRotationRegionScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EnterMapRotationRegionScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n!EnterMapRotationRegionScRsp.proto\x1a\x11CFKHKILIHHF.proto\x1a\x11PBN\
    BNCJCPEI.proto\"\xfd\x01\n\x1bEnterMapRotationRegionScRsp\x12\x20\n\x0bN\
    FLBONDJAIE\x18\x07\x20\x01(\rR\x0bNFLBONDJAIE\x12\x20\n\x0bACNPBBNLMIE\
    \x18\x01\x20\x01(\rR\x0bACNPBBNLMIE\x12\x20\n\x0bNMCNCKKMMOD\x18\x0b\x20\
    \x01(\rR\x0bNMCNCKKMMOD\x12.\n\x0bMFNBANEDODD\x18\x03\x20\x01(\x0b2\x0c.\
    CFKHKILIHHFR\x0bMFNBANEDODD\x12.\n\x0bFGNHLHEDLPN\x18\x04\x20\x01(\x0b2\
    \x0c.PBNBNCJCPEIR\x0bFGNHLHEDLPN\x12\x18\n\x07retcode\x18\t\x20\x01(\rR\
    \x07retcodeb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::CFKHKILIHHF::file_descriptor().clone());
            deps.push(super::PBNBNCJCPEI::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EnterMapRotationRegionScRsp::generated_message_descriptor_data());
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
