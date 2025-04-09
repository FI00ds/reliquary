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

//! Generated file from `LIHOCEHEPDB.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:LIHOCEHEPDB)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LIHOCEHEPDB {
    // message fields
    // @@protoc_insertion_point(field:LIHOCEHEPDB.HFEJHLNIGGH)
    pub HFEJHLNIGGH: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:LIHOCEHEPDB.FHICMGDFGBC)
    pub FHICMGDFGBC: u32,
    // @@protoc_insertion_point(field:LIHOCEHEPDB.KDMLLLGHJON)
    pub KDMLLLGHJON: u32,
    // @@protoc_insertion_point(field:LIHOCEHEPDB.LFCMBGOAIBB)
    pub LFCMBGOAIBB: u32,
    // special fields
    // @@protoc_insertion_point(special_field:LIHOCEHEPDB.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LIHOCEHEPDB {
    fn default() -> &'a LIHOCEHEPDB {
        <LIHOCEHEPDB as ::protobuf::Message>::default_instance()
    }
}

impl LIHOCEHEPDB {
    pub fn new() -> LIHOCEHEPDB {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HFEJHLNIGGH",
            |m: &LIHOCEHEPDB| { &m.HFEJHLNIGGH },
            |m: &mut LIHOCEHEPDB| { &mut m.HFEJHLNIGGH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FHICMGDFGBC",
            |m: &LIHOCEHEPDB| { &m.FHICMGDFGBC },
            |m: &mut LIHOCEHEPDB| { &mut m.FHICMGDFGBC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KDMLLLGHJON",
            |m: &LIHOCEHEPDB| { &m.KDMLLLGHJON },
            |m: &mut LIHOCEHEPDB| { &mut m.KDMLLLGHJON },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LFCMBGOAIBB",
            |m: &LIHOCEHEPDB| { &m.LFCMBGOAIBB },
            |m: &mut LIHOCEHEPDB| { &mut m.LFCMBGOAIBB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LIHOCEHEPDB>(
            "LIHOCEHEPDB",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LIHOCEHEPDB {
    const NAME: &'static str = "LIHOCEHEPDB";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                18 => {
                    is.read_repeated_packed_uint32_into(&mut self.HFEJHLNIGGH)?;
                },
                16 => {
                    self.HFEJHLNIGGH.push(is.read_uint32()?);
                },
                80 => {
                    self.FHICMGDFGBC = is.read_uint32()?;
                },
                40 => {
                    self.KDMLLLGHJON = is.read_uint32()?;
                },
                24 => {
                    self.LFCMBGOAIBB = is.read_uint32()?;
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
        my_size += ::protobuf::rt::vec_packed_uint32_size(2, &self.HFEJHLNIGGH);
        if self.FHICMGDFGBC != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.FHICMGDFGBC);
        }
        if self.KDMLLLGHJON != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.KDMLLLGHJON);
        }
        if self.LFCMBGOAIBB != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.LFCMBGOAIBB);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        os.write_repeated_packed_uint32(2, &self.HFEJHLNIGGH)?;
        if self.FHICMGDFGBC != 0 {
            os.write_uint32(10, self.FHICMGDFGBC)?;
        }
        if self.KDMLLLGHJON != 0 {
            os.write_uint32(5, self.KDMLLLGHJON)?;
        }
        if self.LFCMBGOAIBB != 0 {
            os.write_uint32(3, self.LFCMBGOAIBB)?;
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

    fn new() -> LIHOCEHEPDB {
        LIHOCEHEPDB::new()
    }

    fn clear(&mut self) {
        self.HFEJHLNIGGH.clear();
        self.FHICMGDFGBC = 0;
        self.KDMLLLGHJON = 0;
        self.LFCMBGOAIBB = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LIHOCEHEPDB {
        static instance: LIHOCEHEPDB = LIHOCEHEPDB {
            HFEJHLNIGGH: ::std::vec::Vec::new(),
            FHICMGDFGBC: 0,
            KDMLLLGHJON: 0,
            LFCMBGOAIBB: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LIHOCEHEPDB {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LIHOCEHEPDB").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LIHOCEHEPDB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LIHOCEHEPDB {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LIHOCEHEPDB.proto\"\x95\x01\n\x0bLIHOCEHEPDB\x12\x20\n\x0bHFEJHLNI\
    GGH\x18\x02\x20\x03(\rR\x0bHFEJHLNIGGH\x12\x20\n\x0bFHICMGDFGBC\x18\n\
    \x20\x01(\rR\x0bFHICMGDFGBC\x12\x20\n\x0bKDMLLLGHJON\x18\x05\x20\x01(\rR\
    \x0bKDMLLLGHJON\x12\x20\n\x0bLFCMBGOAIBB\x18\x03\x20\x01(\rR\x0bLFCMBGOA\
    IBBb\x06proto3\
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
            messages.push(LIHOCEHEPDB::generated_message_descriptor_data());
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
