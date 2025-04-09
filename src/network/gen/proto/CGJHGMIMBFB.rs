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

//! Generated file from `CGJHGMIMBFB.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:CGJHGMIMBFB)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CGJHGMIMBFB {
    // message fields
    // @@protoc_insertion_point(field:CGJHGMIMBFB.MNCIHJHGNMJ)
    pub MNCIHJHGNMJ: u32,
    // @@protoc_insertion_point(field:CGJHGMIMBFB.KJDOLBOBKJF)
    pub KJDOLBOBKJF: u32,
    // @@protoc_insertion_point(field:CGJHGMIMBFB.FJNHDHOHBCL)
    pub FJNHDHOHBCL: u32,
    // special fields
    // @@protoc_insertion_point(special_field:CGJHGMIMBFB.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CGJHGMIMBFB {
    fn default() -> &'a CGJHGMIMBFB {
        <CGJHGMIMBFB as ::protobuf::Message>::default_instance()
    }
}

impl CGJHGMIMBFB {
    pub fn new() -> CGJHGMIMBFB {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MNCIHJHGNMJ",
            |m: &CGJHGMIMBFB| { &m.MNCIHJHGNMJ },
            |m: &mut CGJHGMIMBFB| { &mut m.MNCIHJHGNMJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KJDOLBOBKJF",
            |m: &CGJHGMIMBFB| { &m.KJDOLBOBKJF },
            |m: &mut CGJHGMIMBFB| { &mut m.KJDOLBOBKJF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FJNHDHOHBCL",
            |m: &CGJHGMIMBFB| { &m.FJNHDHOHBCL },
            |m: &mut CGJHGMIMBFB| { &mut m.FJNHDHOHBCL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CGJHGMIMBFB>(
            "CGJHGMIMBFB",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CGJHGMIMBFB {
    const NAME: &'static str = "CGJHGMIMBFB";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                40 => {
                    self.MNCIHJHGNMJ = is.read_uint32()?;
                },
                120 => {
                    self.KJDOLBOBKJF = is.read_uint32()?;
                },
                104 => {
                    self.FJNHDHOHBCL = is.read_uint32()?;
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
        if self.MNCIHJHGNMJ != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.MNCIHJHGNMJ);
        }
        if self.KJDOLBOBKJF != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.KJDOLBOBKJF);
        }
        if self.FJNHDHOHBCL != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.FJNHDHOHBCL);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.MNCIHJHGNMJ != 0 {
            os.write_uint32(5, self.MNCIHJHGNMJ)?;
        }
        if self.KJDOLBOBKJF != 0 {
            os.write_uint32(15, self.KJDOLBOBKJF)?;
        }
        if self.FJNHDHOHBCL != 0 {
            os.write_uint32(13, self.FJNHDHOHBCL)?;
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

    fn new() -> CGJHGMIMBFB {
        CGJHGMIMBFB::new()
    }

    fn clear(&mut self) {
        self.MNCIHJHGNMJ = 0;
        self.KJDOLBOBKJF = 0;
        self.FJNHDHOHBCL = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CGJHGMIMBFB {
        static instance: CGJHGMIMBFB = CGJHGMIMBFB {
            MNCIHJHGNMJ: 0,
            KJDOLBOBKJF: 0,
            FJNHDHOHBCL: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CGJHGMIMBFB {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CGJHGMIMBFB").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CGJHGMIMBFB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGJHGMIMBFB {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CGJHGMIMBFB.proto\"s\n\x0bCGJHGMIMBFB\x12\x20\n\x0bMNCIHJHGNMJ\x18\
    \x05\x20\x01(\rR\x0bMNCIHJHGNMJ\x12\x20\n\x0bKJDOLBOBKJF\x18\x0f\x20\x01\
    (\rR\x0bKJDOLBOBKJF\x12\x20\n\x0bFJNHDHOHBCL\x18\r\x20\x01(\rR\x0bFJNHDH\
    OHBCLb\x06proto3\
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
            messages.push(CGJHGMIMBFB::generated_message_descriptor_data());
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
