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

//! Generated file from `BJHEBCCBANA.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:BJHEBCCBANA)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BJHEBCCBANA {
    // message fields
    // @@protoc_insertion_point(field:BJHEBCCBANA.LJPADNCGLOC)
    pub LJPADNCGLOC: bool,
    // @@protoc_insertion_point(field:BJHEBCCBANA.DDDHNAKLMHF)
    pub DDDHNAKLMHF: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:BJHEBCCBANA.JGJCDMJIMNN)
    pub JGJCDMJIMNN: u32,
    // @@protoc_insertion_point(field:BJHEBCCBANA.POFMKDABEHD)
    pub POFMKDABEHD: u32,
    // special fields
    // @@protoc_insertion_point(special_field:BJHEBCCBANA.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BJHEBCCBANA {
    fn default() -> &'a BJHEBCCBANA {
        <BJHEBCCBANA as ::protobuf::Message>::default_instance()
    }
}

impl BJHEBCCBANA {
    pub fn new() -> BJHEBCCBANA {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LJPADNCGLOC",
            |m: &BJHEBCCBANA| { &m.LJPADNCGLOC },
            |m: &mut BJHEBCCBANA| { &mut m.LJPADNCGLOC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "DDDHNAKLMHF",
            |m: &BJHEBCCBANA| { &m.DDDHNAKLMHF },
            |m: &mut BJHEBCCBANA| { &mut m.DDDHNAKLMHF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JGJCDMJIMNN",
            |m: &BJHEBCCBANA| { &m.JGJCDMJIMNN },
            |m: &mut BJHEBCCBANA| { &mut m.JGJCDMJIMNN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "POFMKDABEHD",
            |m: &BJHEBCCBANA| { &m.POFMKDABEHD },
            |m: &mut BJHEBCCBANA| { &mut m.POFMKDABEHD },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BJHEBCCBANA>(
            "BJHEBCCBANA",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BJHEBCCBANA {
    const NAME: &'static str = "BJHEBCCBANA";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.LJPADNCGLOC = is.read_bool()?;
                },
                18 => {
                    is.read_repeated_packed_uint32_into(&mut self.DDDHNAKLMHF)?;
                },
                16 => {
                    self.DDDHNAKLMHF.push(is.read_uint32()?);
                },
                24 => {
                    self.JGJCDMJIMNN = is.read_uint32()?;
                },
                32 => {
                    self.POFMKDABEHD = is.read_uint32()?;
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
        if self.LJPADNCGLOC != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(2, &self.DDDHNAKLMHF);
        if self.JGJCDMJIMNN != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.JGJCDMJIMNN);
        }
        if self.POFMKDABEHD != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.POFMKDABEHD);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.LJPADNCGLOC != false {
            os.write_bool(1, self.LJPADNCGLOC)?;
        }
        os.write_repeated_packed_uint32(2, &self.DDDHNAKLMHF)?;
        if self.JGJCDMJIMNN != 0 {
            os.write_uint32(3, self.JGJCDMJIMNN)?;
        }
        if self.POFMKDABEHD != 0 {
            os.write_uint32(4, self.POFMKDABEHD)?;
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

    fn new() -> BJHEBCCBANA {
        BJHEBCCBANA::new()
    }

    fn clear(&mut self) {
        self.LJPADNCGLOC = false;
        self.DDDHNAKLMHF.clear();
        self.JGJCDMJIMNN = 0;
        self.POFMKDABEHD = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BJHEBCCBANA {
        static instance: BJHEBCCBANA = BJHEBCCBANA {
            LJPADNCGLOC: false,
            DDDHNAKLMHF: ::std::vec::Vec::new(),
            JGJCDMJIMNN: 0,
            POFMKDABEHD: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BJHEBCCBANA {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BJHEBCCBANA").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BJHEBCCBANA {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BJHEBCCBANA {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11BJHEBCCBANA.proto\"\x95\x01\n\x0bBJHEBCCBANA\x12\x20\n\x0bLJPADNCG\
    LOC\x18\x01\x20\x01(\x08R\x0bLJPADNCGLOC\x12\x20\n\x0bDDDHNAKLMHF\x18\
    \x02\x20\x03(\rR\x0bDDDHNAKLMHF\x12\x20\n\x0bJGJCDMJIMNN\x18\x03\x20\x01\
    (\rR\x0bJGJCDMJIMNN\x12\x20\n\x0bPOFMKDABEHD\x18\x04\x20\x01(\rR\x0bPOFM\
    KDABEHDb\x06proto3\
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
            messages.push(BJHEBCCBANA::generated_message_descriptor_data());
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
