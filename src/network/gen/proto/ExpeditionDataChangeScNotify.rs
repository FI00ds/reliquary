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

//! Generated file from `ExpeditionDataChangeScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ExpeditionDataChangeScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ExpeditionDataChangeScNotify {
    // message fields
    // @@protoc_insertion_point(field:ExpeditionDataChangeScNotify.IJFKBLEOOIF)
    pub IJFKBLEOOIF: u32,
    // @@protoc_insertion_point(field:ExpeditionDataChangeScNotify.BFFHLICNPBA)
    pub BFFHLICNPBA: ::std::vec::Vec<super::HIMOBNCPCEK::HIMOBNCPCEK>,
    // @@protoc_insertion_point(field:ExpeditionDataChangeScNotify.JBFNFGBPLDB)
    pub JBFNFGBPLDB: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:ExpeditionDataChangeScNotify.OPKHGKIBJJH)
    pub OPKHGKIBJJH: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:ExpeditionDataChangeScNotify.OBGBOJNDEPK)
    pub OBGBOJNDEPK: ::std::vec::Vec<super::INPDKKMOBFL::INPDKKMOBFL>,
    // special fields
    // @@protoc_insertion_point(special_field:ExpeditionDataChangeScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ExpeditionDataChangeScNotify {
    fn default() -> &'a ExpeditionDataChangeScNotify {
        <ExpeditionDataChangeScNotify as ::protobuf::Message>::default_instance()
    }
}

impl ExpeditionDataChangeScNotify {
    pub fn new() -> ExpeditionDataChangeScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IJFKBLEOOIF",
            |m: &ExpeditionDataChangeScNotify| { &m.IJFKBLEOOIF },
            |m: &mut ExpeditionDataChangeScNotify| { &mut m.IJFKBLEOOIF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "BFFHLICNPBA",
            |m: &ExpeditionDataChangeScNotify| { &m.BFFHLICNPBA },
            |m: &mut ExpeditionDataChangeScNotify| { &mut m.BFFHLICNPBA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JBFNFGBPLDB",
            |m: &ExpeditionDataChangeScNotify| { &m.JBFNFGBPLDB },
            |m: &mut ExpeditionDataChangeScNotify| { &mut m.JBFNFGBPLDB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "OPKHGKIBJJH",
            |m: &ExpeditionDataChangeScNotify| { &m.OPKHGKIBJJH },
            |m: &mut ExpeditionDataChangeScNotify| { &mut m.OPKHGKIBJJH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "OBGBOJNDEPK",
            |m: &ExpeditionDataChangeScNotify| { &m.OBGBOJNDEPK },
            |m: &mut ExpeditionDataChangeScNotify| { &mut m.OBGBOJNDEPK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ExpeditionDataChangeScNotify>(
            "ExpeditionDataChangeScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ExpeditionDataChangeScNotify {
    const NAME: &'static str = "ExpeditionDataChangeScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.IJFKBLEOOIF = is.read_uint32()?;
                },
                82 => {
                    self.BFFHLICNPBA.push(is.read_message()?);
                },
                74 => {
                    is.read_repeated_packed_uint32_into(&mut self.JBFNFGBPLDB)?;
                },
                72 => {
                    self.JBFNFGBPLDB.push(is.read_uint32()?);
                },
                26 => {
                    is.read_repeated_packed_uint32_into(&mut self.OPKHGKIBJJH)?;
                },
                24 => {
                    self.OPKHGKIBJJH.push(is.read_uint32()?);
                },
                98 => {
                    self.OBGBOJNDEPK.push(is.read_message()?);
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
        if self.IJFKBLEOOIF != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.IJFKBLEOOIF);
        }
        for value in &self.BFFHLICNPBA {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.JBFNFGBPLDB {
            my_size += ::protobuf::rt::uint32_size(9, *value);
        };
        for value in &self.OPKHGKIBJJH {
            my_size += ::protobuf::rt::uint32_size(3, *value);
        };
        for value in &self.OBGBOJNDEPK {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IJFKBLEOOIF != 0 {
            os.write_uint32(7, self.IJFKBLEOOIF)?;
        }
        for v in &self.BFFHLICNPBA {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        };
        for v in &self.JBFNFGBPLDB {
            os.write_uint32(9, *v)?;
        };
        for v in &self.OPKHGKIBJJH {
            os.write_uint32(3, *v)?;
        };
        for v in &self.OBGBOJNDEPK {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
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

    fn new() -> ExpeditionDataChangeScNotify {
        ExpeditionDataChangeScNotify::new()
    }

    fn clear(&mut self) {
        self.IJFKBLEOOIF = 0;
        self.BFFHLICNPBA.clear();
        self.JBFNFGBPLDB.clear();
        self.OPKHGKIBJJH.clear();
        self.OBGBOJNDEPK.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ExpeditionDataChangeScNotify {
        static instance: ExpeditionDataChangeScNotify = ExpeditionDataChangeScNotify {
            IJFKBLEOOIF: 0,
            BFFHLICNPBA: ::std::vec::Vec::new(),
            JBFNFGBPLDB: ::std::vec::Vec::new(),
            OPKHGKIBJJH: ::std::vec::Vec::new(),
            OBGBOJNDEPK: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ExpeditionDataChangeScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ExpeditionDataChangeScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ExpeditionDataChangeScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExpeditionDataChangeScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"ExpeditionDataChangeScNotify.proto\x1a\x11HIMOBNCPCEK.proto\x1a\x11I\
    NPDKKMOBFL.proto\"\xe4\x01\n\x1cExpeditionDataChangeScNotify\x12\x20\n\
    \x0bIJFKBLEOOIF\x18\x07\x20\x01(\rR\x0bIJFKBLEOOIF\x12.\n\x0bBFFHLICNPBA\
    \x18\n\x20\x03(\x0b2\x0c.HIMOBNCPCEKR\x0bBFFHLICNPBA\x12\x20\n\x0bJBFNFG\
    BPLDB\x18\t\x20\x03(\rR\x0bJBFNFGBPLDB\x12\x20\n\x0bOPKHGKIBJJH\x18\x03\
    \x20\x03(\rR\x0bOPKHGKIBJJH\x12.\n\x0bOBGBOJNDEPK\x18\x0c\x20\x03(\x0b2\
    \x0c.INPDKKMOBFLR\x0bOBGBOJNDEPKb\x06proto3\
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
            deps.push(super::HIMOBNCPCEK::file_descriptor().clone());
            deps.push(super::INPDKKMOBFL::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ExpeditionDataChangeScNotify::generated_message_descriptor_data());
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
