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

//! Generated file from `CAAAKPFOEJI.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:CAAAKPFOEJI)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CAAAKPFOEJI {
    // message fields
    // @@protoc_insertion_point(field:CAAAKPFOEJI.AHINPCKGKJG)
    pub AHINPCKGKJG: ::protobuf::MessageField<super::JPFJGFOPKHB::JPFJGFOPKHB>,
    // @@protoc_insertion_point(field:CAAAKPFOEJI.ABLICGMCINN)
    pub ABLICGMCINN: u32,
    // @@protoc_insertion_point(field:CAAAKPFOEJI.JGBAINFDBAN)
    pub JGBAINFDBAN: u32,
    // @@protoc_insertion_point(field:CAAAKPFOEJI.DKKLLMOHGFD)
    pub DKKLLMOHGFD: u32,
    // @@protoc_insertion_point(field:CAAAKPFOEJI.CFONLBPOABP)
    pub CFONLBPOABP: u32,
    // special fields
    // @@protoc_insertion_point(special_field:CAAAKPFOEJI.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CAAAKPFOEJI {
    fn default() -> &'a CAAAKPFOEJI {
        <CAAAKPFOEJI as ::protobuf::Message>::default_instance()
    }
}

impl CAAAKPFOEJI {
    pub fn new() -> CAAAKPFOEJI {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::JPFJGFOPKHB::JPFJGFOPKHB>(
            "AHINPCKGKJG",
            |m: &CAAAKPFOEJI| { &m.AHINPCKGKJG },
            |m: &mut CAAAKPFOEJI| { &mut m.AHINPCKGKJG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ABLICGMCINN",
            |m: &CAAAKPFOEJI| { &m.ABLICGMCINN },
            |m: &mut CAAAKPFOEJI| { &mut m.ABLICGMCINN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JGBAINFDBAN",
            |m: &CAAAKPFOEJI| { &m.JGBAINFDBAN },
            |m: &mut CAAAKPFOEJI| { &mut m.JGBAINFDBAN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DKKLLMOHGFD",
            |m: &CAAAKPFOEJI| { &m.DKKLLMOHGFD },
            |m: &mut CAAAKPFOEJI| { &mut m.DKKLLMOHGFD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CFONLBPOABP",
            |m: &CAAAKPFOEJI| { &m.CFONLBPOABP },
            |m: &mut CAAAKPFOEJI| { &mut m.CFONLBPOABP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CAAAKPFOEJI>(
            "CAAAKPFOEJI",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CAAAKPFOEJI {
    const NAME: &'static str = "CAAAKPFOEJI";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.AHINPCKGKJG)?;
                },
                88 => {
                    self.ABLICGMCINN = is.read_uint32()?;
                },
                112 => {
                    self.JGBAINFDBAN = is.read_uint32()?;
                },
                32 => {
                    self.DKKLLMOHGFD = is.read_uint32()?;
                },
                96 => {
                    self.CFONLBPOABP = is.read_uint32()?;
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
        if let Some(v) = self.AHINPCKGKJG.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.ABLICGMCINN != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.ABLICGMCINN);
        }
        if self.JGBAINFDBAN != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.JGBAINFDBAN);
        }
        if self.DKKLLMOHGFD != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.DKKLLMOHGFD);
        }
        if self.CFONLBPOABP != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.CFONLBPOABP);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.AHINPCKGKJG.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if self.ABLICGMCINN != 0 {
            os.write_uint32(11, self.ABLICGMCINN)?;
        }
        if self.JGBAINFDBAN != 0 {
            os.write_uint32(14, self.JGBAINFDBAN)?;
        }
        if self.DKKLLMOHGFD != 0 {
            os.write_uint32(4, self.DKKLLMOHGFD)?;
        }
        if self.CFONLBPOABP != 0 {
            os.write_uint32(12, self.CFONLBPOABP)?;
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

    fn new() -> CAAAKPFOEJI {
        CAAAKPFOEJI::new()
    }

    fn clear(&mut self) {
        self.AHINPCKGKJG.clear();
        self.ABLICGMCINN = 0;
        self.JGBAINFDBAN = 0;
        self.DKKLLMOHGFD = 0;
        self.CFONLBPOABP = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CAAAKPFOEJI {
        static instance: CAAAKPFOEJI = CAAAKPFOEJI {
            AHINPCKGKJG: ::protobuf::MessageField::none(),
            ABLICGMCINN: 0,
            JGBAINFDBAN: 0,
            DKKLLMOHGFD: 0,
            CFONLBPOABP: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CAAAKPFOEJI {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CAAAKPFOEJI").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CAAAKPFOEJI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CAAAKPFOEJI {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CAAAKPFOEJI.proto\x1a\x11JPFJGFOPKHB.proto\"\xc5\x01\n\x0bCAAAKPFO\
    EJI\x12.\n\x0bAHINPCKGKJG\x18\x07\x20\x01(\x0b2\x0c.JPFJGFOPKHBR\x0bAHIN\
    PCKGKJG\x12\x20\n\x0bABLICGMCINN\x18\x0b\x20\x01(\rR\x0bABLICGMCINN\x12\
    \x20\n\x0bJGBAINFDBAN\x18\x0e\x20\x01(\rR\x0bJGBAINFDBAN\x12\x20\n\x0bDK\
    KLLMOHGFD\x18\x04\x20\x01(\rR\x0bDKKLLMOHGFD\x12\x20\n\x0bCFONLBPOABP\
    \x18\x0c\x20\x01(\rR\x0bCFONLBPOABPb\x06proto3\
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
            deps.push(super::JPFJGFOPKHB::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CAAAKPFOEJI::generated_message_descriptor_data());
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
