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

//! Generated file from `PlanetFesHandleCardPieceApplyScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:PlanetFesHandleCardPieceApplyScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PlanetFesHandleCardPieceApplyScRsp {
    // message fields
    // @@protoc_insertion_point(field:PlanetFesHandleCardPieceApplyScRsp.DLHBCOKCIDP)
    pub DLHBCOKCIDP: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:PlanetFesHandleCardPieceApplyScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:PlanetFesHandleCardPieceApplyScRsp.MDAPCFHELJL)
    pub MDAPCFHELJL: bool,
    // @@protoc_insertion_point(field:PlanetFesHandleCardPieceApplyScRsp.PNAKHNBDJAE)
    pub PNAKHNBDJAE: u32,
    // @@protoc_insertion_point(field:PlanetFesHandleCardPieceApplyScRsp.PPIOLCLPMPL)
    pub PPIOLCLPMPL: i64,
    // special fields
    // @@protoc_insertion_point(special_field:PlanetFesHandleCardPieceApplyScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PlanetFesHandleCardPieceApplyScRsp {
    fn default() -> &'a PlanetFesHandleCardPieceApplyScRsp {
        <PlanetFesHandleCardPieceApplyScRsp as ::protobuf::Message>::default_instance()
    }
}

impl PlanetFesHandleCardPieceApplyScRsp {
    pub fn new() -> PlanetFesHandleCardPieceApplyScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "DLHBCOKCIDP",
            |m: &PlanetFesHandleCardPieceApplyScRsp| { &m.DLHBCOKCIDP },
            |m: &mut PlanetFesHandleCardPieceApplyScRsp| { &mut m.DLHBCOKCIDP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &PlanetFesHandleCardPieceApplyScRsp| { &m.retcode },
            |m: &mut PlanetFesHandleCardPieceApplyScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MDAPCFHELJL",
            |m: &PlanetFesHandleCardPieceApplyScRsp| { &m.MDAPCFHELJL },
            |m: &mut PlanetFesHandleCardPieceApplyScRsp| { &mut m.MDAPCFHELJL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PNAKHNBDJAE",
            |m: &PlanetFesHandleCardPieceApplyScRsp| { &m.PNAKHNBDJAE },
            |m: &mut PlanetFesHandleCardPieceApplyScRsp| { &mut m.PNAKHNBDJAE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PPIOLCLPMPL",
            |m: &PlanetFesHandleCardPieceApplyScRsp| { &m.PPIOLCLPMPL },
            |m: &mut PlanetFesHandleCardPieceApplyScRsp| { &mut m.PPIOLCLPMPL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PlanetFesHandleCardPieceApplyScRsp>(
            "PlanetFesHandleCardPieceApplyScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PlanetFesHandleCardPieceApplyScRsp {
    const NAME: &'static str = "PlanetFesHandleCardPieceApplyScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                106 => {
                    is.read_repeated_packed_uint32_into(&mut self.DLHBCOKCIDP)?;
                },
                104 => {
                    self.DLHBCOKCIDP.push(is.read_uint32()?);
                },
                64 => {
                    self.retcode = is.read_uint32()?;
                },
                88 => {
                    self.MDAPCFHELJL = is.read_bool()?;
                },
                8 => {
                    self.PNAKHNBDJAE = is.read_uint32()?;
                },
                56 => {
                    self.PPIOLCLPMPL = is.read_int64()?;
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
        my_size += ::protobuf::rt::vec_packed_uint32_size(13, &self.DLHBCOKCIDP);
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.retcode);
        }
        if self.MDAPCFHELJL != false {
            my_size += 1 + 1;
        }
        if self.PNAKHNBDJAE != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.PNAKHNBDJAE);
        }
        if self.PPIOLCLPMPL != 0 {
            my_size += ::protobuf::rt::int64_size(7, self.PPIOLCLPMPL);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        os.write_repeated_packed_uint32(13, &self.DLHBCOKCIDP)?;
        if self.retcode != 0 {
            os.write_uint32(8, self.retcode)?;
        }
        if self.MDAPCFHELJL != false {
            os.write_bool(11, self.MDAPCFHELJL)?;
        }
        if self.PNAKHNBDJAE != 0 {
            os.write_uint32(1, self.PNAKHNBDJAE)?;
        }
        if self.PPIOLCLPMPL != 0 {
            os.write_int64(7, self.PPIOLCLPMPL)?;
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

    fn new() -> PlanetFesHandleCardPieceApplyScRsp {
        PlanetFesHandleCardPieceApplyScRsp::new()
    }

    fn clear(&mut self) {
        self.DLHBCOKCIDP.clear();
        self.retcode = 0;
        self.MDAPCFHELJL = false;
        self.PNAKHNBDJAE = 0;
        self.PPIOLCLPMPL = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PlanetFesHandleCardPieceApplyScRsp {
        static instance: PlanetFesHandleCardPieceApplyScRsp = PlanetFesHandleCardPieceApplyScRsp {
            DLHBCOKCIDP: ::std::vec::Vec::new(),
            retcode: 0,
            MDAPCFHELJL: false,
            PNAKHNBDJAE: 0,
            PPIOLCLPMPL: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PlanetFesHandleCardPieceApplyScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PlanetFesHandleCardPieceApplyScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PlanetFesHandleCardPieceApplyScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlanetFesHandleCardPieceApplyScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n(PlanetFesHandleCardPieceApplyScRsp.proto\"\xc6\x01\n\"PlanetFesHandle\
    CardPieceApplyScRsp\x12\x20\n\x0bDLHBCOKCIDP\x18\r\x20\x03(\rR\x0bDLHBCO\
    KCIDP\x12\x18\n\x07retcode\x18\x08\x20\x01(\rR\x07retcode\x12\x20\n\x0bM\
    DAPCFHELJL\x18\x0b\x20\x01(\x08R\x0bMDAPCFHELJL\x12\x20\n\x0bPNAKHNBDJAE\
    \x18\x01\x20\x01(\rR\x0bPNAKHNBDJAE\x12\x20\n\x0bPPIOLCLPMPL\x18\x07\x20\
    \x01(\x03R\x0bPPIOLCLPMPLb\x06proto3\
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
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PlanetFesHandleCardPieceApplyScRsp::generated_message_descriptor_data());
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
