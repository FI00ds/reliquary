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

//! Generated file from `MKHMHPEFDHG.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MKHMHPEFDHG)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MKHMHPEFDHG {
    // message fields
    // @@protoc_insertion_point(field:MKHMHPEFDHG.EHJDKABPDBC)
    pub EHJDKABPDBC: u32,
    // @@protoc_insertion_point(field:MKHMHPEFDHG.AELGPBEJJNI)
    pub AELGPBEJJNI: u32,
    // @@protoc_insertion_point(field:MKHMHPEFDHG.AAOHALKFFNM)
    pub AAOHALKFFNM: bool,
    // @@protoc_insertion_point(field:MKHMHPEFDHG.ONNPOMBHIHB)
    pub ONNPOMBHIHB: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:MKHMHPEFDHG.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MKHMHPEFDHG {
    fn default() -> &'a MKHMHPEFDHG {
        <MKHMHPEFDHG as ::protobuf::Message>::default_instance()
    }
}

impl MKHMHPEFDHG {
    pub fn new() -> MKHMHPEFDHG {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EHJDKABPDBC",
            |m: &MKHMHPEFDHG| { &m.EHJDKABPDBC },
            |m: &mut MKHMHPEFDHG| { &mut m.EHJDKABPDBC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AELGPBEJJNI",
            |m: &MKHMHPEFDHG| { &m.AELGPBEJJNI },
            |m: &mut MKHMHPEFDHG| { &mut m.AELGPBEJJNI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AAOHALKFFNM",
            |m: &MKHMHPEFDHG| { &m.AAOHALKFFNM },
            |m: &mut MKHMHPEFDHG| { &mut m.AAOHALKFFNM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "ONNPOMBHIHB",
            |m: &MKHMHPEFDHG| { &m.ONNPOMBHIHB },
            |m: &mut MKHMHPEFDHG| { &mut m.ONNPOMBHIHB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MKHMHPEFDHG>(
            "MKHMHPEFDHG",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MKHMHPEFDHG {
    const NAME: &'static str = "MKHMHPEFDHG";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                40 => {
                    self.EHJDKABPDBC = is.read_uint32()?;
                },
                64 => {
                    self.AELGPBEJJNI = is.read_uint32()?;
                },
                56 => {
                    self.AAOHALKFFNM = is.read_bool()?;
                },
                106 => {
                    is.read_repeated_packed_uint32_into(&mut self.ONNPOMBHIHB)?;
                },
                104 => {
                    self.ONNPOMBHIHB.push(is.read_uint32()?);
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
        if self.EHJDKABPDBC != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.EHJDKABPDBC);
        }
        if self.AELGPBEJJNI != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.AELGPBEJJNI);
        }
        if self.AAOHALKFFNM != false {
            my_size += 1 + 1;
        }
        for value in &self.ONNPOMBHIHB {
            my_size += ::protobuf::rt::uint32_size(13, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.EHJDKABPDBC != 0 {
            os.write_uint32(5, self.EHJDKABPDBC)?;
        }
        if self.AELGPBEJJNI != 0 {
            os.write_uint32(8, self.AELGPBEJJNI)?;
        }
        if self.AAOHALKFFNM != false {
            os.write_bool(7, self.AAOHALKFFNM)?;
        }
        for v in &self.ONNPOMBHIHB {
            os.write_uint32(13, *v)?;
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

    fn new() -> MKHMHPEFDHG {
        MKHMHPEFDHG::new()
    }

    fn clear(&mut self) {
        self.EHJDKABPDBC = 0;
        self.AELGPBEJJNI = 0;
        self.AAOHALKFFNM = false;
        self.ONNPOMBHIHB.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MKHMHPEFDHG {
        static instance: MKHMHPEFDHG = MKHMHPEFDHG {
            EHJDKABPDBC: 0,
            AELGPBEJJNI: 0,
            AAOHALKFFNM: false,
            ONNPOMBHIHB: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MKHMHPEFDHG {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MKHMHPEFDHG").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MKHMHPEFDHG {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MKHMHPEFDHG {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11MKHMHPEFDHG.proto\"\x95\x01\n\x0bMKHMHPEFDHG\x12\x20\n\x0bEHJDKABP\
    DBC\x18\x05\x20\x01(\rR\x0bEHJDKABPDBC\x12\x20\n\x0bAELGPBEJJNI\x18\x08\
    \x20\x01(\rR\x0bAELGPBEJJNI\x12\x20\n\x0bAAOHALKFFNM\x18\x07\x20\x01(\
    \x08R\x0bAAOHALKFFNM\x12\x20\n\x0bONNPOMBHIHB\x18\r\x20\x03(\rR\x0bONNPO\
    MBHIHBb\x06proto3\
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
            messages.push(MKHMHPEFDHG::generated_message_descriptor_data());
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
