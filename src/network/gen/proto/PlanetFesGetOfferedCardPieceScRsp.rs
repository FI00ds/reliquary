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

//! Generated file from `PlanetFesGetOfferedCardPieceScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:PlanetFesGetOfferedCardPieceScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PlanetFesGetOfferedCardPieceScRsp {
    // message fields
    // @@protoc_insertion_point(field:PlanetFesGetOfferedCardPieceScRsp.HPJJDCJHHOA)
    pub HPJJDCJHHOA: u64,
    // @@protoc_insertion_point(field:PlanetFesGetOfferedCardPieceScRsp.CABEHKOFLPG)
    pub CABEHKOFLPG: bool,
    // @@protoc_insertion_point(field:PlanetFesGetOfferedCardPieceScRsp.ONJGMGHCPEF)
    pub ONJGMGHCPEF: ::std::vec::Vec<super::BKBILPDKOIL::BKBILPDKOIL>,
    // @@protoc_insertion_point(field:PlanetFesGetOfferedCardPieceScRsp.PBFHDNBGMBP)
    pub PBFHDNBGMBP: ::std::vec::Vec<super::CEODDCEIDDL::CEODDCEIDDL>,
    // @@protoc_insertion_point(field:PlanetFesGetOfferedCardPieceScRsp.DLJCKCMADHJ)
    pub DLJCKCMADHJ: i64,
    // @@protoc_insertion_point(field:PlanetFesGetOfferedCardPieceScRsp.retcode)
    pub retcode: u32,
    // special fields
    // @@protoc_insertion_point(special_field:PlanetFesGetOfferedCardPieceScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PlanetFesGetOfferedCardPieceScRsp {
    fn default() -> &'a PlanetFesGetOfferedCardPieceScRsp {
        <PlanetFesGetOfferedCardPieceScRsp as ::protobuf::Message>::default_instance()
    }
}

impl PlanetFesGetOfferedCardPieceScRsp {
    pub fn new() -> PlanetFesGetOfferedCardPieceScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HPJJDCJHHOA",
            |m: &PlanetFesGetOfferedCardPieceScRsp| { &m.HPJJDCJHHOA },
            |m: &mut PlanetFesGetOfferedCardPieceScRsp| { &mut m.HPJJDCJHHOA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CABEHKOFLPG",
            |m: &PlanetFesGetOfferedCardPieceScRsp| { &m.CABEHKOFLPG },
            |m: &mut PlanetFesGetOfferedCardPieceScRsp| { &mut m.CABEHKOFLPG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "ONJGMGHCPEF",
            |m: &PlanetFesGetOfferedCardPieceScRsp| { &m.ONJGMGHCPEF },
            |m: &mut PlanetFesGetOfferedCardPieceScRsp| { &mut m.ONJGMGHCPEF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PBFHDNBGMBP",
            |m: &PlanetFesGetOfferedCardPieceScRsp| { &m.PBFHDNBGMBP },
            |m: &mut PlanetFesGetOfferedCardPieceScRsp| { &mut m.PBFHDNBGMBP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DLJCKCMADHJ",
            |m: &PlanetFesGetOfferedCardPieceScRsp| { &m.DLJCKCMADHJ },
            |m: &mut PlanetFesGetOfferedCardPieceScRsp| { &mut m.DLJCKCMADHJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &PlanetFesGetOfferedCardPieceScRsp| { &m.retcode },
            |m: &mut PlanetFesGetOfferedCardPieceScRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PlanetFesGetOfferedCardPieceScRsp>(
            "PlanetFesGetOfferedCardPieceScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PlanetFesGetOfferedCardPieceScRsp {
    const NAME: &'static str = "PlanetFesGetOfferedCardPieceScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.HPJJDCJHHOA = is.read_uint64()?;
                },
                64 => {
                    self.CABEHKOFLPG = is.read_bool()?;
                },
                34 => {
                    self.ONJGMGHCPEF.push(is.read_message()?);
                },
                58 => {
                    self.PBFHDNBGMBP.push(is.read_message()?);
                },
                104 => {
                    self.DLJCKCMADHJ = is.read_int64()?;
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
        if self.HPJJDCJHHOA != 0 {
            my_size += ::protobuf::rt::uint64_size(3, self.HPJJDCJHHOA);
        }
        if self.CABEHKOFLPG != false {
            my_size += 1 + 1;
        }
        for value in &self.ONJGMGHCPEF {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.PBFHDNBGMBP {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.DLJCKCMADHJ != 0 {
            my_size += ::protobuf::rt::int64_size(13, self.DLJCKCMADHJ);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.HPJJDCJHHOA != 0 {
            os.write_uint64(3, self.HPJJDCJHHOA)?;
        }
        if self.CABEHKOFLPG != false {
            os.write_bool(8, self.CABEHKOFLPG)?;
        }
        for v in &self.ONJGMGHCPEF {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        for v in &self.PBFHDNBGMBP {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        };
        if self.DLJCKCMADHJ != 0 {
            os.write_int64(13, self.DLJCKCMADHJ)?;
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

    fn new() -> PlanetFesGetOfferedCardPieceScRsp {
        PlanetFesGetOfferedCardPieceScRsp::new()
    }

    fn clear(&mut self) {
        self.HPJJDCJHHOA = 0;
        self.CABEHKOFLPG = false;
        self.ONJGMGHCPEF.clear();
        self.PBFHDNBGMBP.clear();
        self.DLJCKCMADHJ = 0;
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PlanetFesGetOfferedCardPieceScRsp {
        static instance: PlanetFesGetOfferedCardPieceScRsp = PlanetFesGetOfferedCardPieceScRsp {
            HPJJDCJHHOA: 0,
            CABEHKOFLPG: false,
            ONJGMGHCPEF: ::std::vec::Vec::new(),
            PBFHDNBGMBP: ::std::vec::Vec::new(),
            DLJCKCMADHJ: 0,
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PlanetFesGetOfferedCardPieceScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PlanetFesGetOfferedCardPieceScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PlanetFesGetOfferedCardPieceScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlanetFesGetOfferedCardPieceScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'PlanetFesGetOfferedCardPieceScRsp.proto\x1a\x11BKBILPDKOIL.proto\x1a\
    \x11CEODDCEIDDL.proto\"\x83\x02\n!PlanetFesGetOfferedCardPieceScRsp\x12\
    \x20\n\x0bHPJJDCJHHOA\x18\x03\x20\x01(\x04R\x0bHPJJDCJHHOA\x12\x20\n\x0b\
    CABEHKOFLPG\x18\x08\x20\x01(\x08R\x0bCABEHKOFLPG\x12.\n\x0bONJGMGHCPEF\
    \x18\x04\x20\x03(\x0b2\x0c.BKBILPDKOILR\x0bONJGMGHCPEF\x12.\n\x0bPBFHDNB\
    GMBP\x18\x07\x20\x03(\x0b2\x0c.CEODDCEIDDLR\x0bPBFHDNBGMBP\x12\x20\n\x0b\
    DLJCKCMADHJ\x18\r\x20\x01(\x03R\x0bDLJCKCMADHJ\x12\x18\n\x07retcode\x18\
    \t\x20\x01(\rR\x07retcodeb\x06proto3\
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
            deps.push(super::BKBILPDKOIL::file_descriptor().clone());
            deps.push(super::CEODDCEIDDL::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PlanetFesGetOfferedCardPieceScRsp::generated_message_descriptor_data());
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
