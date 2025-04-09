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

//! Generated file from `FAEBINHJGJK.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:FAEBINHJGJK)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FAEBINHJGJK {
    // message fields
    // @@protoc_insertion_point(field:FAEBINHJGJK.IFJFCEJJBPE)
    pub IFJFCEJJBPE: u32,
    // @@protoc_insertion_point(field:FAEBINHJGJK.FBJPBDIJPFK)
    pub FBJPBDIJPFK: u32,
    // @@protoc_insertion_point(field:FAEBINHJGJK.MPFEDFBKKDF)
    pub MPFEDFBKKDF: bool,
    // @@protoc_insertion_point(field:FAEBINHJGJK.AGBHAJDNHEH)
    pub AGBHAJDNHEH: ::protobuf::MessageField<super::EDBJJKNJPFM::EDBJJKNJPFM>,
    // @@protoc_insertion_point(field:FAEBINHJGJK.IDPJIDNLEHH)
    pub IDPJIDNLEHH: bool,
    // @@protoc_insertion_point(field:FAEBINHJGJK.FHICMGDFGBC)
    pub FHICMGDFGBC: u32,
    // special fields
    // @@protoc_insertion_point(special_field:FAEBINHJGJK.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FAEBINHJGJK {
    fn default() -> &'a FAEBINHJGJK {
        <FAEBINHJGJK as ::protobuf::Message>::default_instance()
    }
}

impl FAEBINHJGJK {
    pub fn new() -> FAEBINHJGJK {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IFJFCEJJBPE",
            |m: &FAEBINHJGJK| { &m.IFJFCEJJBPE },
            |m: &mut FAEBINHJGJK| { &mut m.IFJFCEJJBPE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FBJPBDIJPFK",
            |m: &FAEBINHJGJK| { &m.FBJPBDIJPFK },
            |m: &mut FAEBINHJGJK| { &mut m.FBJPBDIJPFK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MPFEDFBKKDF",
            |m: &FAEBINHJGJK| { &m.MPFEDFBKKDF },
            |m: &mut FAEBINHJGJK| { &mut m.MPFEDFBKKDF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::EDBJJKNJPFM::EDBJJKNJPFM>(
            "AGBHAJDNHEH",
            |m: &FAEBINHJGJK| { &m.AGBHAJDNHEH },
            |m: &mut FAEBINHJGJK| { &mut m.AGBHAJDNHEH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IDPJIDNLEHH",
            |m: &FAEBINHJGJK| { &m.IDPJIDNLEHH },
            |m: &mut FAEBINHJGJK| { &mut m.IDPJIDNLEHH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FHICMGDFGBC",
            |m: &FAEBINHJGJK| { &m.FHICMGDFGBC },
            |m: &mut FAEBINHJGJK| { &mut m.FHICMGDFGBC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FAEBINHJGJK>(
            "FAEBINHJGJK",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FAEBINHJGJK {
    const NAME: &'static str = "FAEBINHJGJK";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                80 => {
                    self.IFJFCEJJBPE = is.read_uint32()?;
                },
                48 => {
                    self.FBJPBDIJPFK = is.read_uint32()?;
                },
                104 => {
                    self.MPFEDFBKKDF = is.read_bool()?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.AGBHAJDNHEH)?;
                },
                56 => {
                    self.IDPJIDNLEHH = is.read_bool()?;
                },
                112 => {
                    self.FHICMGDFGBC = is.read_uint32()?;
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
        if self.IFJFCEJJBPE != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.IFJFCEJJBPE);
        }
        if self.FBJPBDIJPFK != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.FBJPBDIJPFK);
        }
        if self.MPFEDFBKKDF != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.AGBHAJDNHEH.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.IDPJIDNLEHH != false {
            my_size += 1 + 1;
        }
        if self.FHICMGDFGBC != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.FHICMGDFGBC);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IFJFCEJJBPE != 0 {
            os.write_uint32(10, self.IFJFCEJJBPE)?;
        }
        if self.FBJPBDIJPFK != 0 {
            os.write_uint32(6, self.FBJPBDIJPFK)?;
        }
        if self.MPFEDFBKKDF != false {
            os.write_bool(13, self.MPFEDFBKKDF)?;
        }
        if let Some(v) = self.AGBHAJDNHEH.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if self.IDPJIDNLEHH != false {
            os.write_bool(7, self.IDPJIDNLEHH)?;
        }
        if self.FHICMGDFGBC != 0 {
            os.write_uint32(14, self.FHICMGDFGBC)?;
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

    fn new() -> FAEBINHJGJK {
        FAEBINHJGJK::new()
    }

    fn clear(&mut self) {
        self.IFJFCEJJBPE = 0;
        self.FBJPBDIJPFK = 0;
        self.MPFEDFBKKDF = false;
        self.AGBHAJDNHEH.clear();
        self.IDPJIDNLEHH = false;
        self.FHICMGDFGBC = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FAEBINHJGJK {
        static instance: FAEBINHJGJK = FAEBINHJGJK {
            IFJFCEJJBPE: 0,
            FBJPBDIJPFK: 0,
            MPFEDFBKKDF: false,
            AGBHAJDNHEH: ::protobuf::MessageField::none(),
            IDPJIDNLEHH: false,
            FHICMGDFGBC: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FAEBINHJGJK {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FAEBINHJGJK").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FAEBINHJGJK {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FAEBINHJGJK {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11FAEBINHJGJK.proto\x1a\x11EDBJJKNJPFM.proto\"\xe7\x01\n\x0bFAEBINHJ\
    GJK\x12\x20\n\x0bIFJFCEJJBPE\x18\n\x20\x01(\rR\x0bIFJFCEJJBPE\x12\x20\n\
    \x0bFBJPBDIJPFK\x18\x06\x20\x01(\rR\x0bFBJPBDIJPFK\x12\x20\n\x0bMPFEDFBK\
    KDF\x18\r\x20\x01(\x08R\x0bMPFEDFBKKDF\x12.\n\x0bAGBHAJDNHEH\x18\x04\x20\
    \x01(\x0b2\x0c.EDBJJKNJPFMR\x0bAGBHAJDNHEH\x12\x20\n\x0bIDPJIDNLEHH\x18\
    \x07\x20\x01(\x08R\x0bIDPJIDNLEHH\x12\x20\n\x0bFHICMGDFGBC\x18\x0e\x20\
    \x01(\rR\x0bFHICMGDFGBCb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::EDBJJKNJPFM::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(FAEBINHJGJK::generated_message_descriptor_data());
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
