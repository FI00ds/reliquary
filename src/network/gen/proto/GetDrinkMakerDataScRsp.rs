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

//! Generated file from `GetDrinkMakerDataScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:GetDrinkMakerDataScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetDrinkMakerDataScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetDrinkMakerDataScRsp.EAOLMHOAAML)
    pub EAOLMHOAAML: u32,
    // @@protoc_insertion_point(field:GetDrinkMakerDataScRsp.AMEFGBICGDI)
    pub AMEFGBICGDI: u32,
    // @@protoc_insertion_point(field:GetDrinkMakerDataScRsp.PCNNPEJEGEF)
    pub PCNNPEJEGEF: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetDrinkMakerDataScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:GetDrinkMakerDataScRsp.HEPALNIOJNP)
    pub HEPALNIOJNP: u32,
    // @@protoc_insertion_point(field:GetDrinkMakerDataScRsp.level)
    pub level: u32,
    // @@protoc_insertion_point(field:GetDrinkMakerDataScRsp.DHAKOFAGDOF)
    pub DHAKOFAGDOF: u32,
    // @@protoc_insertion_point(field:GetDrinkMakerDataScRsp.PJKIBODPCKI)
    pub PJKIBODPCKI: ::std::vec::Vec<super::DrinkMakerGuest::DrinkMakerGuest>,
    // @@protoc_insertion_point(field:GetDrinkMakerDataScRsp.exp)
    pub exp: u32,
    // @@protoc_insertion_point(field:GetDrinkMakerDataScRsp.BOINOMBHPCL)
    pub BOINOMBHPCL: ::protobuf::MessageField<super::EEKFECDIHJE::EEKFECDIHJE>,
    // special fields
    // @@protoc_insertion_point(special_field:GetDrinkMakerDataScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetDrinkMakerDataScRsp {
    fn default() -> &'a GetDrinkMakerDataScRsp {
        <GetDrinkMakerDataScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetDrinkMakerDataScRsp {
    pub fn new() -> GetDrinkMakerDataScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(10);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EAOLMHOAAML",
            |m: &GetDrinkMakerDataScRsp| { &m.EAOLMHOAAML },
            |m: &mut GetDrinkMakerDataScRsp| { &mut m.EAOLMHOAAML },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AMEFGBICGDI",
            |m: &GetDrinkMakerDataScRsp| { &m.AMEFGBICGDI },
            |m: &mut GetDrinkMakerDataScRsp| { &mut m.AMEFGBICGDI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PCNNPEJEGEF",
            |m: &GetDrinkMakerDataScRsp| { &m.PCNNPEJEGEF },
            |m: &mut GetDrinkMakerDataScRsp| { &mut m.PCNNPEJEGEF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetDrinkMakerDataScRsp| { &m.retcode },
            |m: &mut GetDrinkMakerDataScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HEPALNIOJNP",
            |m: &GetDrinkMakerDataScRsp| { &m.HEPALNIOJNP },
            |m: &mut GetDrinkMakerDataScRsp| { &mut m.HEPALNIOJNP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &GetDrinkMakerDataScRsp| { &m.level },
            |m: &mut GetDrinkMakerDataScRsp| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DHAKOFAGDOF",
            |m: &GetDrinkMakerDataScRsp| { &m.DHAKOFAGDOF },
            |m: &mut GetDrinkMakerDataScRsp| { &mut m.DHAKOFAGDOF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PJKIBODPCKI",
            |m: &GetDrinkMakerDataScRsp| { &m.PJKIBODPCKI },
            |m: &mut GetDrinkMakerDataScRsp| { &mut m.PJKIBODPCKI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "exp",
            |m: &GetDrinkMakerDataScRsp| { &m.exp },
            |m: &mut GetDrinkMakerDataScRsp| { &mut m.exp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::EEKFECDIHJE::EEKFECDIHJE>(
            "BOINOMBHPCL",
            |m: &GetDrinkMakerDataScRsp| { &m.BOINOMBHPCL },
            |m: &mut GetDrinkMakerDataScRsp| { &mut m.BOINOMBHPCL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetDrinkMakerDataScRsp>(
            "GetDrinkMakerDataScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetDrinkMakerDataScRsp {
    const NAME: &'static str = "GetDrinkMakerDataScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.EAOLMHOAAML = is.read_uint32()?;
                },
                8 => {
                    self.AMEFGBICGDI = is.read_uint32()?;
                },
                114 => {
                    is.read_repeated_packed_uint32_into(&mut self.PCNNPEJEGEF)?;
                },
                112 => {
                    self.PCNNPEJEGEF.push(is.read_uint32()?);
                },
                96 => {
                    self.retcode = is.read_uint32()?;
                },
                104 => {
                    self.HEPALNIOJNP = is.read_uint32()?;
                },
                40 => {
                    self.level = is.read_uint32()?;
                },
                56 => {
                    self.DHAKOFAGDOF = is.read_uint32()?;
                },
                82 => {
                    self.PJKIBODPCKI.push(is.read_message()?);
                },
                88 => {
                    self.exp = is.read_uint32()?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.BOINOMBHPCL)?;
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
        if self.EAOLMHOAAML != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.EAOLMHOAAML);
        }
        if self.AMEFGBICGDI != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.AMEFGBICGDI);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(14, &self.PCNNPEJEGEF);
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.retcode);
        }
        if self.HEPALNIOJNP != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.HEPALNIOJNP);
        }
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.level);
        }
        if self.DHAKOFAGDOF != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.DHAKOFAGDOF);
        }
        for value in &self.PJKIBODPCKI {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.exp != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.exp);
        }
        if let Some(v) = self.BOINOMBHPCL.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.EAOLMHOAAML != 0 {
            os.write_uint32(8, self.EAOLMHOAAML)?;
        }
        if self.AMEFGBICGDI != 0 {
            os.write_uint32(1, self.AMEFGBICGDI)?;
        }
        os.write_repeated_packed_uint32(14, &self.PCNNPEJEGEF)?;
        if self.retcode != 0 {
            os.write_uint32(12, self.retcode)?;
        }
        if self.HEPALNIOJNP != 0 {
            os.write_uint32(13, self.HEPALNIOJNP)?;
        }
        if self.level != 0 {
            os.write_uint32(5, self.level)?;
        }
        if self.DHAKOFAGDOF != 0 {
            os.write_uint32(7, self.DHAKOFAGDOF)?;
        }
        for v in &self.PJKIBODPCKI {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        };
        if self.exp != 0 {
            os.write_uint32(11, self.exp)?;
        }
        if let Some(v) = self.BOINOMBHPCL.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
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

    fn new() -> GetDrinkMakerDataScRsp {
        GetDrinkMakerDataScRsp::new()
    }

    fn clear(&mut self) {
        self.EAOLMHOAAML = 0;
        self.AMEFGBICGDI = 0;
        self.PCNNPEJEGEF.clear();
        self.retcode = 0;
        self.HEPALNIOJNP = 0;
        self.level = 0;
        self.DHAKOFAGDOF = 0;
        self.PJKIBODPCKI.clear();
        self.exp = 0;
        self.BOINOMBHPCL.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetDrinkMakerDataScRsp {
        static instance: GetDrinkMakerDataScRsp = GetDrinkMakerDataScRsp {
            EAOLMHOAAML: 0,
            AMEFGBICGDI: 0,
            PCNNPEJEGEF: ::std::vec::Vec::new(),
            retcode: 0,
            HEPALNIOJNP: 0,
            level: 0,
            DHAKOFAGDOF: 0,
            PJKIBODPCKI: ::std::vec::Vec::new(),
            exp: 0,
            BOINOMBHPCL: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetDrinkMakerDataScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetDrinkMakerDataScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetDrinkMakerDataScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetDrinkMakerDataScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cGetDrinkMakerDataScRsp.proto\x1a\x15DrinkMakerGuest.proto\x1a\x11E\
    EKFECDIHJE.proto\"\xe8\x02\n\x16GetDrinkMakerDataScRsp\x12\x20\n\x0bEAOL\
    MHOAAML\x18\x08\x20\x01(\rR\x0bEAOLMHOAAML\x12\x20\n\x0bAMEFGBICGDI\x18\
    \x01\x20\x01(\rR\x0bAMEFGBICGDI\x12\x20\n\x0bPCNNPEJEGEF\x18\x0e\x20\x03\
    (\rR\x0bPCNNPEJEGEF\x12\x18\n\x07retcode\x18\x0c\x20\x01(\rR\x07retcode\
    \x12\x20\n\x0bHEPALNIOJNP\x18\r\x20\x01(\rR\x0bHEPALNIOJNP\x12\x14\n\x05\
    level\x18\x05\x20\x01(\rR\x05level\x12\x20\n\x0bDHAKOFAGDOF\x18\x07\x20\
    \x01(\rR\x0bDHAKOFAGDOF\x122\n\x0bPJKIBODPCKI\x18\n\x20\x03(\x0b2\x10.Dr\
    inkMakerGuestR\x0bPJKIBODPCKI\x12\x10\n\x03exp\x18\x0b\x20\x01(\rR\x03ex\
    p\x12.\n\x0bBOINOMBHPCL\x18\t\x20\x01(\x0b2\x0c.EEKFECDIHJER\x0bBOINOMBH\
    PCLb\x06proto3\
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
            deps.push(super::DrinkMakerGuest::file_descriptor().clone());
            deps.push(super::EEKFECDIHJE::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetDrinkMakerDataScRsp::generated_message_descriptor_data());
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
