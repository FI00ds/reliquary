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

//! Generated file from `FKJJEBFJFDA.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:FKJJEBFJFDA)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FKJJEBFJFDA {
    // message fields
    // @@protoc_insertion_point(field:FKJJEBFJFDA.ONDFNMPEOEN)
    pub ONDFNMPEOEN: u64,
    // @@protoc_insertion_point(field:FKJJEBFJFDA.LJDBKCBFPPC)
    pub LJDBKCBFPPC: u32,
    // @@protoc_insertion_point(field:FKJJEBFJFDA.tid)
    pub tid: u32,
    // special fields
    // @@protoc_insertion_point(special_field:FKJJEBFJFDA.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FKJJEBFJFDA {
    fn default() -> &'a FKJJEBFJFDA {
        <FKJJEBFJFDA as ::protobuf::Message>::default_instance()
    }
}

impl FKJJEBFJFDA {
    pub fn new() -> FKJJEBFJFDA {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ONDFNMPEOEN",
            |m: &FKJJEBFJFDA| { &m.ONDFNMPEOEN },
            |m: &mut FKJJEBFJFDA| { &mut m.ONDFNMPEOEN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LJDBKCBFPPC",
            |m: &FKJJEBFJFDA| { &m.LJDBKCBFPPC },
            |m: &mut FKJJEBFJFDA| { &mut m.LJDBKCBFPPC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "tid",
            |m: &FKJJEBFJFDA| { &m.tid },
            |m: &mut FKJJEBFJFDA| { &mut m.tid },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FKJJEBFJFDA>(
            "FKJJEBFJFDA",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FKJJEBFJFDA {
    const NAME: &'static str = "FKJJEBFJFDA";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.ONDFNMPEOEN = is.read_uint64()?;
                },
                112 => {
                    self.LJDBKCBFPPC = is.read_uint32()?;
                },
                80 => {
                    self.tid = is.read_uint32()?;
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
        if self.ONDFNMPEOEN != 0 {
            my_size += ::protobuf::rt::uint64_size(9, self.ONDFNMPEOEN);
        }
        if self.LJDBKCBFPPC != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.LJDBKCBFPPC);
        }
        if self.tid != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.tid);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.ONDFNMPEOEN != 0 {
            os.write_uint64(9, self.ONDFNMPEOEN)?;
        }
        if self.LJDBKCBFPPC != 0 {
            os.write_uint32(14, self.LJDBKCBFPPC)?;
        }
        if self.tid != 0 {
            os.write_uint32(10, self.tid)?;
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

    fn new() -> FKJJEBFJFDA {
        FKJJEBFJFDA::new()
    }

    fn clear(&mut self) {
        self.ONDFNMPEOEN = 0;
        self.LJDBKCBFPPC = 0;
        self.tid = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FKJJEBFJFDA {
        static instance: FKJJEBFJFDA = FKJJEBFJFDA {
            ONDFNMPEOEN: 0,
            LJDBKCBFPPC: 0,
            tid: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FKJJEBFJFDA {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FKJJEBFJFDA").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FKJJEBFJFDA {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FKJJEBFJFDA {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11FKJJEBFJFDA.proto\"c\n\x0bFKJJEBFJFDA\x12\x20\n\x0bONDFNMPEOEN\x18\
    \t\x20\x01(\x04R\x0bONDFNMPEOEN\x12\x20\n\x0bLJDBKCBFPPC\x18\x0e\x20\x01\
    (\rR\x0bLJDBKCBFPPC\x12\x10\n\x03tid\x18\n\x20\x01(\rR\x03tidb\x06proto3\
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
            messages.push(FKJJEBFJFDA::generated_message_descriptor_data());
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
