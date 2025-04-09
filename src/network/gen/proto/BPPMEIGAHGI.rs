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

//! Generated file from `BPPMEIGAHGI.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:BPPMEIGAHGI)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BPPMEIGAHGI {
    // message fields
    // @@protoc_insertion_point(field:BPPMEIGAHGI.MIGFMPJBELG)
    pub MIGFMPJBELG: ::std::vec::Vec<super::MMOIBACBPKA::MMOIBACBPKA>,
    // @@protoc_insertion_point(field:BPPMEIGAHGI.IOMGDIKELIA)
    pub IOMGDIKELIA: u32,
    // @@protoc_insertion_point(field:BPPMEIGAHGI.PDMDKAPCOJM)
    pub PDMDKAPCOJM: u32,
    // special fields
    // @@protoc_insertion_point(special_field:BPPMEIGAHGI.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BPPMEIGAHGI {
    fn default() -> &'a BPPMEIGAHGI {
        <BPPMEIGAHGI as ::protobuf::Message>::default_instance()
    }
}

impl BPPMEIGAHGI {
    pub fn new() -> BPPMEIGAHGI {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MIGFMPJBELG",
            |m: &BPPMEIGAHGI| { &m.MIGFMPJBELG },
            |m: &mut BPPMEIGAHGI| { &mut m.MIGFMPJBELG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IOMGDIKELIA",
            |m: &BPPMEIGAHGI| { &m.IOMGDIKELIA },
            |m: &mut BPPMEIGAHGI| { &mut m.IOMGDIKELIA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PDMDKAPCOJM",
            |m: &BPPMEIGAHGI| { &m.PDMDKAPCOJM },
            |m: &mut BPPMEIGAHGI| { &mut m.PDMDKAPCOJM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BPPMEIGAHGI>(
            "BPPMEIGAHGI",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BPPMEIGAHGI {
    const NAME: &'static str = "BPPMEIGAHGI";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                74 => {
                    self.MIGFMPJBELG.push(is.read_message()?);
                },
                40 => {
                    self.IOMGDIKELIA = is.read_uint32()?;
                },
                88 => {
                    self.PDMDKAPCOJM = is.read_uint32()?;
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
        for value in &self.MIGFMPJBELG {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.IOMGDIKELIA != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.IOMGDIKELIA);
        }
        if self.PDMDKAPCOJM != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.PDMDKAPCOJM);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.MIGFMPJBELG {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        };
        if self.IOMGDIKELIA != 0 {
            os.write_uint32(5, self.IOMGDIKELIA)?;
        }
        if self.PDMDKAPCOJM != 0 {
            os.write_uint32(11, self.PDMDKAPCOJM)?;
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

    fn new() -> BPPMEIGAHGI {
        BPPMEIGAHGI::new()
    }

    fn clear(&mut self) {
        self.MIGFMPJBELG.clear();
        self.IOMGDIKELIA = 0;
        self.PDMDKAPCOJM = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BPPMEIGAHGI {
        static instance: BPPMEIGAHGI = BPPMEIGAHGI {
            MIGFMPJBELG: ::std::vec::Vec::new(),
            IOMGDIKELIA: 0,
            PDMDKAPCOJM: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BPPMEIGAHGI {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BPPMEIGAHGI").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BPPMEIGAHGI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BPPMEIGAHGI {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11BPPMEIGAHGI.proto\x1a\x11MMOIBACBPKA.proto\"\x81\x01\n\x0bBPPMEIGA\
    HGI\x12.\n\x0bMIGFMPJBELG\x18\t\x20\x03(\x0b2\x0c.MMOIBACBPKAR\x0bMIGFMP\
    JBELG\x12\x20\n\x0bIOMGDIKELIA\x18\x05\x20\x01(\rR\x0bIOMGDIKELIA\x12\
    \x20\n\x0bPDMDKAPCOJM\x18\x0b\x20\x01(\rR\x0bPDMDKAPCOJMb\x06proto3\
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
            deps.push(super::MMOIBACBPKA::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(BPPMEIGAHGI::generated_message_descriptor_data());
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
