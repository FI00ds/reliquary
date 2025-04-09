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

//! Generated file from `HeliobusActivityDataScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:HeliobusActivityDataScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HeliobusActivityDataScRsp {
    // message fields
    // @@protoc_insertion_point(field:HeliobusActivityDataScRsp.NFDBMHPPFIP)
    pub NFDBMHPPFIP: ::std::vec::Vec<super::GBJKKFHPFFN::GBJKKFHPFFN>,
    // @@protoc_insertion_point(field:HeliobusActivityDataScRsp.GCLJENGJICM)
    pub GCLJENGJICM: ::std::vec::Vec<super::HeliobusChallengeLineup::HeliobusChallengeLineup>,
    // @@protoc_insertion_point(field:HeliobusActivityDataScRsp.IBHAAEJEEHC)
    pub IBHAAEJEEHC: u32,
    // @@protoc_insertion_point(field:HeliobusActivityDataScRsp.BJAPDDEPHEL)
    pub BJAPDDEPHEL: ::protobuf::MessageField<super::DEJAKPOEPKN::DEJAKPOEPKN>,
    // @@protoc_insertion_point(field:HeliobusActivityDataScRsp.HFMLNLGIGHB)
    pub HFMLNLGIGHB: ::std::vec::Vec<super::JMIJJHKIBLB::JMIJJHKIBLB>,
    // @@protoc_insertion_point(field:HeliobusActivityDataScRsp.level)
    pub level: u32,
    // @@protoc_insertion_point(field:HeliobusActivityDataScRsp.HALGPJMCMFP)
    pub HALGPJMCMFP: u32,
    // @@protoc_insertion_point(field:HeliobusActivityDataScRsp.IPHKDELMOIH)
    pub IPHKDELMOIH: u32,
    // @@protoc_insertion_point(field:HeliobusActivityDataScRsp.EENJBPMNDOL)
    pub EENJBPMNDOL: u32,
    // @@protoc_insertion_point(field:HeliobusActivityDataScRsp.retcode)
    pub retcode: u32,
    // special fields
    // @@protoc_insertion_point(special_field:HeliobusActivityDataScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HeliobusActivityDataScRsp {
    fn default() -> &'a HeliobusActivityDataScRsp {
        <HeliobusActivityDataScRsp as ::protobuf::Message>::default_instance()
    }
}

impl HeliobusActivityDataScRsp {
    pub fn new() -> HeliobusActivityDataScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(10);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NFDBMHPPFIP",
            |m: &HeliobusActivityDataScRsp| { &m.NFDBMHPPFIP },
            |m: &mut HeliobusActivityDataScRsp| { &mut m.NFDBMHPPFIP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "GCLJENGJICM",
            |m: &HeliobusActivityDataScRsp| { &m.GCLJENGJICM },
            |m: &mut HeliobusActivityDataScRsp| { &mut m.GCLJENGJICM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IBHAAEJEEHC",
            |m: &HeliobusActivityDataScRsp| { &m.IBHAAEJEEHC },
            |m: &mut HeliobusActivityDataScRsp| { &mut m.IBHAAEJEEHC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::DEJAKPOEPKN::DEJAKPOEPKN>(
            "BJAPDDEPHEL",
            |m: &HeliobusActivityDataScRsp| { &m.BJAPDDEPHEL },
            |m: &mut HeliobusActivityDataScRsp| { &mut m.BJAPDDEPHEL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HFMLNLGIGHB",
            |m: &HeliobusActivityDataScRsp| { &m.HFMLNLGIGHB },
            |m: &mut HeliobusActivityDataScRsp| { &mut m.HFMLNLGIGHB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &HeliobusActivityDataScRsp| { &m.level },
            |m: &mut HeliobusActivityDataScRsp| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HALGPJMCMFP",
            |m: &HeliobusActivityDataScRsp| { &m.HALGPJMCMFP },
            |m: &mut HeliobusActivityDataScRsp| { &mut m.HALGPJMCMFP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IPHKDELMOIH",
            |m: &HeliobusActivityDataScRsp| { &m.IPHKDELMOIH },
            |m: &mut HeliobusActivityDataScRsp| { &mut m.IPHKDELMOIH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EENJBPMNDOL",
            |m: &HeliobusActivityDataScRsp| { &m.EENJBPMNDOL },
            |m: &mut HeliobusActivityDataScRsp| { &mut m.EENJBPMNDOL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &HeliobusActivityDataScRsp| { &m.retcode },
            |m: &mut HeliobusActivityDataScRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HeliobusActivityDataScRsp>(
            "HeliobusActivityDataScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HeliobusActivityDataScRsp {
    const NAME: &'static str = "HeliobusActivityDataScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                122 => {
                    self.NFDBMHPPFIP.push(is.read_message()?);
                },
                10 => {
                    self.GCLJENGJICM.push(is.read_message()?);
                },
                56 => {
                    self.IBHAAEJEEHC = is.read_uint32()?;
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.BJAPDDEPHEL)?;
                },
                82 => {
                    self.HFMLNLGIGHB.push(is.read_message()?);
                },
                64 => {
                    self.level = is.read_uint32()?;
                },
                96 => {
                    self.HALGPJMCMFP = is.read_uint32()?;
                },
                24 => {
                    self.IPHKDELMOIH = is.read_uint32()?;
                },
                112 => {
                    self.EENJBPMNDOL = is.read_uint32()?;
                },
                104 => {
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
        for value in &self.NFDBMHPPFIP {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.GCLJENGJICM {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.IBHAAEJEEHC != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.IBHAAEJEEHC);
        }
        if let Some(v) = self.BJAPDDEPHEL.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.HFMLNLGIGHB {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.level);
        }
        if self.HALGPJMCMFP != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.HALGPJMCMFP);
        }
        if self.IPHKDELMOIH != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.IPHKDELMOIH);
        }
        if self.EENJBPMNDOL != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.EENJBPMNDOL);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.NFDBMHPPFIP {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        };
        for v in &self.GCLJENGJICM {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        if self.IBHAAEJEEHC != 0 {
            os.write_uint32(7, self.IBHAAEJEEHC)?;
        }
        if let Some(v) = self.BJAPDDEPHEL.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        for v in &self.HFMLNLGIGHB {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        };
        if self.level != 0 {
            os.write_uint32(8, self.level)?;
        }
        if self.HALGPJMCMFP != 0 {
            os.write_uint32(12, self.HALGPJMCMFP)?;
        }
        if self.IPHKDELMOIH != 0 {
            os.write_uint32(3, self.IPHKDELMOIH)?;
        }
        if self.EENJBPMNDOL != 0 {
            os.write_uint32(14, self.EENJBPMNDOL)?;
        }
        if self.retcode != 0 {
            os.write_uint32(13, self.retcode)?;
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

    fn new() -> HeliobusActivityDataScRsp {
        HeliobusActivityDataScRsp::new()
    }

    fn clear(&mut self) {
        self.NFDBMHPPFIP.clear();
        self.GCLJENGJICM.clear();
        self.IBHAAEJEEHC = 0;
        self.BJAPDDEPHEL.clear();
        self.HFMLNLGIGHB.clear();
        self.level = 0;
        self.HALGPJMCMFP = 0;
        self.IPHKDELMOIH = 0;
        self.EENJBPMNDOL = 0;
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HeliobusActivityDataScRsp {
        static instance: HeliobusActivityDataScRsp = HeliobusActivityDataScRsp {
            NFDBMHPPFIP: ::std::vec::Vec::new(),
            GCLJENGJICM: ::std::vec::Vec::new(),
            IBHAAEJEEHC: 0,
            BJAPDDEPHEL: ::protobuf::MessageField::none(),
            HFMLNLGIGHB: ::std::vec::Vec::new(),
            level: 0,
            HALGPJMCMFP: 0,
            IPHKDELMOIH: 0,
            EENJBPMNDOL: 0,
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HeliobusActivityDataScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HeliobusActivityDataScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HeliobusActivityDataScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HeliobusActivityDataScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1fHeliobusActivityDataScRsp.proto\x1a\x11DEJAKPOEPKN.proto\x1a\x11GB\
    JKKFHPFFN.proto\x1a\x1dHeliobusChallengeLineup.proto\x1a\x11JMIJJHKIBLB.\
    proto\"\x9f\x03\n\x19HeliobusActivityDataScRsp\x12.\n\x0bNFDBMHPPFIP\x18\
    \x0f\x20\x03(\x0b2\x0c.GBJKKFHPFFNR\x0bNFDBMHPPFIP\x12:\n\x0bGCLJENGJICM\
    \x18\x01\x20\x03(\x0b2\x18.HeliobusChallengeLineupR\x0bGCLJENGJICM\x12\
    \x20\n\x0bIBHAAEJEEHC\x18\x07\x20\x01(\rR\x0bIBHAAEJEEHC\x12.\n\x0bBJAPD\
    DEPHEL\x18\x06\x20\x01(\x0b2\x0c.DEJAKPOEPKNR\x0bBJAPDDEPHEL\x12.\n\x0bH\
    FMLNLGIGHB\x18\n\x20\x03(\x0b2\x0c.JMIJJHKIBLBR\x0bHFMLNLGIGHB\x12\x14\n\
    \x05level\x18\x08\x20\x01(\rR\x05level\x12\x20\n\x0bHALGPJMCMFP\x18\x0c\
    \x20\x01(\rR\x0bHALGPJMCMFP\x12\x20\n\x0bIPHKDELMOIH\x18\x03\x20\x01(\rR\
    \x0bIPHKDELMOIH\x12\x20\n\x0bEENJBPMNDOL\x18\x0e\x20\x01(\rR\x0bEENJBPMN\
    DOL\x12\x18\n\x07retcode\x18\r\x20\x01(\rR\x07retcodeb\x06proto3\
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
            deps.push(super::DEJAKPOEPKN::file_descriptor().clone());
            deps.push(super::GBJKKFHPFFN::file_descriptor().clone());
            deps.push(super::HeliobusChallengeLineup::file_descriptor().clone());
            deps.push(super::JMIJJHKIBLB::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(HeliobusActivityDataScRsp::generated_message_descriptor_data());
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
