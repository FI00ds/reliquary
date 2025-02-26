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

//! Generated file from `MBMCFOLIOLO.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:MBMCFOLIOLO)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MBMCFOLIOLO {
    // message fields
    // @@protoc_insertion_point(field:MBMCFOLIOLO.CDNOJCACELJ)
    pub CDNOJCACELJ: bool,
    // @@protoc_insertion_point(field:MBMCFOLIOLO.HANEGINLANI)
    pub HANEGINLANI: ::std::collections::HashMap<::std::string::String, super::DKFLALJDIFL::DKFLALJDIFL>,
    // @@protoc_insertion_point(field:MBMCFOLIOLO.CJKMGENOJBE)
    pub CJKMGENOJBE: ::std::vec::Vec<u8>,
    // special fields
    // @@protoc_insertion_point(special_field:MBMCFOLIOLO.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MBMCFOLIOLO {
    fn default() -> &'a MBMCFOLIOLO {
        <MBMCFOLIOLO as ::protobuf::Message>::default_instance()
    }
}

impl MBMCFOLIOLO {
    pub fn new() -> MBMCFOLIOLO {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CDNOJCACELJ",
            |m: &MBMCFOLIOLO| { &m.CDNOJCACELJ },
            |m: &mut MBMCFOLIOLO| { &mut m.CDNOJCACELJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor_new::<_, _>(
            "HANEGINLANI",
            |m: &MBMCFOLIOLO| { &m.HANEGINLANI },
            |m: &mut MBMCFOLIOLO| { &mut m.HANEGINLANI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CJKMGENOJBE",
            |m: &MBMCFOLIOLO| { &m.CJKMGENOJBE },
            |m: &mut MBMCFOLIOLO| { &mut m.CJKMGENOJBE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MBMCFOLIOLO>(
            "MBMCFOLIOLO",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MBMCFOLIOLO {
    const NAME: &'static str = "MBMCFOLIOLO";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.CDNOJCACELJ = is.read_bool()?;
                },
                18 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            10 => key = is.read_string()?,
                            18 => value = is.read_message()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.HANEGINLANI.insert(key, value);
                },
                26 => {
                    self.CJKMGENOJBE = is.read_bytes()?;
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
        if self.CDNOJCACELJ != false {
            my_size += 1 + 1;
        }
        for (k, v) in &self.HANEGINLANI {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            let len = v.compute_size();
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if !self.CJKMGENOJBE.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.CJKMGENOJBE);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.CDNOJCACELJ != false {
            os.write_bool(1, self.CDNOJCACELJ)?;
        }
        for (k, v) in &self.HANEGINLANI {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            let len = v.cached_size() as u64;
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            os.write_raw_varint32(18)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_string(1, &k)?;
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        if !self.CJKMGENOJBE.is_empty() {
            os.write_bytes(3, &self.CJKMGENOJBE)?;
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

    fn new() -> MBMCFOLIOLO {
        MBMCFOLIOLO::new()
    }

    fn clear(&mut self) {
        self.CDNOJCACELJ = false;
        self.HANEGINLANI.clear();
        self.CJKMGENOJBE.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MBMCFOLIOLO {
        static instance: ::protobuf::rt::Lazy<MBMCFOLIOLO> = ::protobuf::rt::Lazy::new();
        instance.get(MBMCFOLIOLO::new)
    }
}

impl ::protobuf::MessageFull for MBMCFOLIOLO {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MBMCFOLIOLO").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MBMCFOLIOLO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MBMCFOLIOLO {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11MBMCFOLIOLO.proto\x1a\x11DKFLALJDIFL.proto\"\xe0\x01\n\x0bMBMCFOLI\
    OLO\x12\x20\n\x0bCDNOJCACELJ\x18\x01\x20\x01(\x08R\x0bCDNOJCACELJ\x12?\n\
    \x0bHANEGINLANI\x18\x02\x20\x03(\x0b2\x1d.MBMCFOLIOLO.HANEGINLANIEntryR\
    \x0bHANEGINLANI\x12\x20\n\x0bCJKMGENOJBE\x18\x03\x20\x01(\x0cR\x0bCJKMGE\
    NOJBE\x1aL\n\x10HANEGINLANIEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\
    \x03key\x12\"\n\x05value\x18\x02\x20\x01(\x0b2\x0c.DKFLALJDIFLR\x05value\
    :\x028\x01b\x06proto3\
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
            deps.push(super::DKFLALJDIFL::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MBMCFOLIOLO::generated_message_descriptor_data());
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
