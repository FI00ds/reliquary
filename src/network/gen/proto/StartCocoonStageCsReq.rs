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

//! Generated file from `StartCocoonStageCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:StartCocoonStageCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct StartCocoonStageCsReq {
    // message fields
    // @@protoc_insertion_point(field:StartCocoonStageCsReq.IFJFCEJJBPE)
    pub IFJFCEJJBPE: u32,
    // @@protoc_insertion_point(field:StartCocoonStageCsReq.HEAJKBNOGLG)
    pub HEAJKBNOGLG: u32,
    // @@protoc_insertion_point(field:StartCocoonStageCsReq.LMPHJMDONMH)
    pub LMPHJMDONMH: u32,
    // @@protoc_insertion_point(field:StartCocoonStageCsReq.NLLKJODHPCP)
    pub NLLKJODHPCP: u32,
    // special fields
    // @@protoc_insertion_point(special_field:StartCocoonStageCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a StartCocoonStageCsReq {
    fn default() -> &'a StartCocoonStageCsReq {
        <StartCocoonStageCsReq as ::protobuf::Message>::default_instance()
    }
}

impl StartCocoonStageCsReq {
    pub fn new() -> StartCocoonStageCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IFJFCEJJBPE",
            |m: &StartCocoonStageCsReq| { &m.IFJFCEJJBPE },
            |m: &mut StartCocoonStageCsReq| { &mut m.IFJFCEJJBPE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HEAJKBNOGLG",
            |m: &StartCocoonStageCsReq| { &m.HEAJKBNOGLG },
            |m: &mut StartCocoonStageCsReq| { &mut m.HEAJKBNOGLG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LMPHJMDONMH",
            |m: &StartCocoonStageCsReq| { &m.LMPHJMDONMH },
            |m: &mut StartCocoonStageCsReq| { &mut m.LMPHJMDONMH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NLLKJODHPCP",
            |m: &StartCocoonStageCsReq| { &m.NLLKJODHPCP },
            |m: &mut StartCocoonStageCsReq| { &mut m.NLLKJODHPCP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<StartCocoonStageCsReq>(
            "StartCocoonStageCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for StartCocoonStageCsReq {
    const NAME: &'static str = "StartCocoonStageCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.IFJFCEJJBPE = is.read_uint32()?;
                },
                96 => {
                    self.HEAJKBNOGLG = is.read_uint32()?;
                },
                40 => {
                    self.LMPHJMDONMH = is.read_uint32()?;
                },
                32 => {
                    self.NLLKJODHPCP = is.read_uint32()?;
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
            my_size += ::protobuf::rt::uint32_size(3, self.IFJFCEJJBPE);
        }
        if self.HEAJKBNOGLG != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.HEAJKBNOGLG);
        }
        if self.LMPHJMDONMH != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.LMPHJMDONMH);
        }
        if self.NLLKJODHPCP != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.NLLKJODHPCP);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IFJFCEJJBPE != 0 {
            os.write_uint32(3, self.IFJFCEJJBPE)?;
        }
        if self.HEAJKBNOGLG != 0 {
            os.write_uint32(12, self.HEAJKBNOGLG)?;
        }
        if self.LMPHJMDONMH != 0 {
            os.write_uint32(5, self.LMPHJMDONMH)?;
        }
        if self.NLLKJODHPCP != 0 {
            os.write_uint32(4, self.NLLKJODHPCP)?;
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

    fn new() -> StartCocoonStageCsReq {
        StartCocoonStageCsReq::new()
    }

    fn clear(&mut self) {
        self.IFJFCEJJBPE = 0;
        self.HEAJKBNOGLG = 0;
        self.LMPHJMDONMH = 0;
        self.NLLKJODHPCP = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static StartCocoonStageCsReq {
        static instance: StartCocoonStageCsReq = StartCocoonStageCsReq {
            IFJFCEJJBPE: 0,
            HEAJKBNOGLG: 0,
            LMPHJMDONMH: 0,
            NLLKJODHPCP: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for StartCocoonStageCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("StartCocoonStageCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for StartCocoonStageCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StartCocoonStageCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bStartCocoonStageCsReq.proto\"\x9f\x01\n\x15StartCocoonStageCsReq\
    \x12\x20\n\x0bIFJFCEJJBPE\x18\x03\x20\x01(\rR\x0bIFJFCEJJBPE\x12\x20\n\
    \x0bHEAJKBNOGLG\x18\x0c\x20\x01(\rR\x0bHEAJKBNOGLG\x12\x20\n\x0bLMPHJMDO\
    NMH\x18\x05\x20\x01(\rR\x0bLMPHJMDONMH\x12\x20\n\x0bNLLKJODHPCP\x18\x04\
    \x20\x01(\rR\x0bNLLKJODHPCPb\x06proto3\
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
            messages.push(StartCocoonStageCsReq::generated_message_descriptor_data());
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
