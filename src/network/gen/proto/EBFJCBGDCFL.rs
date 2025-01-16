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

//! Generated file from `EBFJCBGDCFL.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EBFJCBGDCFL)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EBFJCBGDCFL {
    // message fields
    // @@protoc_insertion_point(field:EBFJCBGDCFL.INPPFDJHEJN)
    pub INPPFDJHEJN: ::protobuf::MessageField<super::IONNOLIHAAG::IONNOLIHAAG>,
    // @@protoc_insertion_point(field:EBFJCBGDCFL.PFKHHKIMNBP)
    pub PFKHHKIMNBP: u32,
    // @@protoc_insertion_point(field:EBFJCBGDCFL.PCFPDACIMII)
    pub PCFPDACIMII: ::std::vec::Vec<super::COBHIBLKKBB::COBHIBLKKBB>,
    // special fields
    // @@protoc_insertion_point(special_field:EBFJCBGDCFL.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EBFJCBGDCFL {
    fn default() -> &'a EBFJCBGDCFL {
        <EBFJCBGDCFL as ::protobuf::Message>::default_instance()
    }
}

impl EBFJCBGDCFL {
    pub fn new() -> EBFJCBGDCFL {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::IONNOLIHAAG::IONNOLIHAAG>(
            "INPPFDJHEJN",
            |m: &EBFJCBGDCFL| { &m.INPPFDJHEJN },
            |m: &mut EBFJCBGDCFL| { &mut m.INPPFDJHEJN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PFKHHKIMNBP",
            |m: &EBFJCBGDCFL| { &m.PFKHHKIMNBP },
            |m: &mut EBFJCBGDCFL| { &mut m.PFKHHKIMNBP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PCFPDACIMII",
            |m: &EBFJCBGDCFL| { &m.PCFPDACIMII },
            |m: &mut EBFJCBGDCFL| { &mut m.PCFPDACIMII },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EBFJCBGDCFL>(
            "EBFJCBGDCFL",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EBFJCBGDCFL {
    const NAME: &'static str = "EBFJCBGDCFL";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.INPPFDJHEJN)?;
                },
                96 => {
                    self.PFKHHKIMNBP = is.read_uint32()?;
                },
                18 => {
                    self.PCFPDACIMII.push(is.read_message()?);
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
        if let Some(v) = self.INPPFDJHEJN.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.PFKHHKIMNBP != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.PFKHHKIMNBP);
        }
        for value in &self.PCFPDACIMII {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.INPPFDJHEJN.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if self.PFKHHKIMNBP != 0 {
            os.write_uint32(12, self.PFKHHKIMNBP)?;
        }
        for v in &self.PCFPDACIMII {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> EBFJCBGDCFL {
        EBFJCBGDCFL::new()
    }

    fn clear(&mut self) {
        self.INPPFDJHEJN.clear();
        self.PFKHHKIMNBP = 0;
        self.PCFPDACIMII.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EBFJCBGDCFL {
        static instance: EBFJCBGDCFL = EBFJCBGDCFL {
            INPPFDJHEJN: ::protobuf::MessageField::none(),
            PFKHHKIMNBP: 0,
            PCFPDACIMII: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EBFJCBGDCFL {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EBFJCBGDCFL").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EBFJCBGDCFL {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EBFJCBGDCFL {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11EBFJCBGDCFL.proto\x1a\x11COBHIBLKKBB.proto\x1a\x11IONNOLIHAAG.prot\
    o\"\x8f\x01\n\x0bEBFJCBGDCFL\x12.\n\x0bINPPFDJHEJN\x18\x04\x20\x01(\x0b2\
    \x0c.IONNOLIHAAGR\x0bINPPFDJHEJN\x12\x20\n\x0bPFKHHKIMNBP\x18\x0c\x20\
    \x01(\rR\x0bPFKHHKIMNBP\x12.\n\x0bPCFPDACIMII\x18\x02\x20\x03(\x0b2\x0c.\
    COBHIBLKKBBR\x0bPCFPDACIMIIb\x06proto3\
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
            deps.push(super::COBHIBLKKBB::file_descriptor().clone());
            deps.push(super::IONNOLIHAAG::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EBFJCBGDCFL::generated_message_descriptor_data());
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
