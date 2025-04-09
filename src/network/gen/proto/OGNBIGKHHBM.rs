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

//! Generated file from `OGNBIGKHHBM.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:OGNBIGKHHBM)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct OGNBIGKHHBM {
    // message fields
    // @@protoc_insertion_point(field:OGNBIGKHHBM.GCGLNKFDKKN)
    pub GCGLNKFDKKN: ::protobuf::MessageField<super::CACLANLOOLK::CACLANLOOLK>,
    // @@protoc_insertion_point(field:OGNBIGKHHBM.BHMHLPCHKLG)
    pub BHMHLPCHKLG: ::protobuf::MessageField<super::AAPKBPEGGBH::AAPKBPEGGBH>,
    // @@protoc_insertion_point(field:OGNBIGKHHBM.EMBAGMMHIPA)
    pub EMBAGMMHIPA: ::protobuf::MessageField<super::EIMJEAMDFKJ::EIMJEAMDFKJ>,
    // @@protoc_insertion_point(field:OGNBIGKHHBM.BJLEMFMCODD)
    pub BJLEMFMCODD: ::protobuf::MessageField<super::FJJDKDNDFDJ::FJJDKDNDFDJ>,
    // special fields
    // @@protoc_insertion_point(special_field:OGNBIGKHHBM.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a OGNBIGKHHBM {
    fn default() -> &'a OGNBIGKHHBM {
        <OGNBIGKHHBM as ::protobuf::Message>::default_instance()
    }
}

impl OGNBIGKHHBM {
    pub fn new() -> OGNBIGKHHBM {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::CACLANLOOLK::CACLANLOOLK>(
            "GCGLNKFDKKN",
            |m: &OGNBIGKHHBM| { &m.GCGLNKFDKKN },
            |m: &mut OGNBIGKHHBM| { &mut m.GCGLNKFDKKN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::AAPKBPEGGBH::AAPKBPEGGBH>(
            "BHMHLPCHKLG",
            |m: &OGNBIGKHHBM| { &m.BHMHLPCHKLG },
            |m: &mut OGNBIGKHHBM| { &mut m.BHMHLPCHKLG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::EIMJEAMDFKJ::EIMJEAMDFKJ>(
            "EMBAGMMHIPA",
            |m: &OGNBIGKHHBM| { &m.EMBAGMMHIPA },
            |m: &mut OGNBIGKHHBM| { &mut m.EMBAGMMHIPA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::FJJDKDNDFDJ::FJJDKDNDFDJ>(
            "BJLEMFMCODD",
            |m: &OGNBIGKHHBM| { &m.BJLEMFMCODD },
            |m: &mut OGNBIGKHHBM| { &mut m.BJLEMFMCODD },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<OGNBIGKHHBM>(
            "OGNBIGKHHBM",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for OGNBIGKHHBM {
    const NAME: &'static str = "OGNBIGKHHBM";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.GCGLNKFDKKN)?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.BHMHLPCHKLG)?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.EMBAGMMHIPA)?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.BJLEMFMCODD)?;
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
        if let Some(v) = self.GCGLNKFDKKN.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.BHMHLPCHKLG.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.EMBAGMMHIPA.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.BJLEMFMCODD.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.GCGLNKFDKKN.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        if let Some(v) = self.BHMHLPCHKLG.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        if let Some(v) = self.EMBAGMMHIPA.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if let Some(v) = self.BJLEMFMCODD.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
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

    fn new() -> OGNBIGKHHBM {
        OGNBIGKHHBM::new()
    }

    fn clear(&mut self) {
        self.GCGLNKFDKKN.clear();
        self.BHMHLPCHKLG.clear();
        self.EMBAGMMHIPA.clear();
        self.BJLEMFMCODD.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static OGNBIGKHHBM {
        static instance: OGNBIGKHHBM = OGNBIGKHHBM {
            GCGLNKFDKKN: ::protobuf::MessageField::none(),
            BHMHLPCHKLG: ::protobuf::MessageField::none(),
            EMBAGMMHIPA: ::protobuf::MessageField::none(),
            BJLEMFMCODD: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for OGNBIGKHHBM {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("OGNBIGKHHBM").unwrap()).clone()
    }
}

impl ::std::fmt::Display for OGNBIGKHHBM {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OGNBIGKHHBM {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11OGNBIGKHHBM.proto\x1a\x11AAPKBPEGGBH.proto\x1a\x11CACLANLOOLK.prot\
    o\x1a\x11EIMJEAMDFKJ.proto\x1a\x11FJJDKDNDFDJ.proto\"\xcd\x01\n\x0bOGNBI\
    GKHHBM\x12.\n\x0bGCGLNKFDKKN\x18\x06\x20\x01(\x0b2\x0c.CACLANLOOLKR\x0bG\
    CGLNKFDKKN\x12.\n\x0bBHMHLPCHKLG\x18\x0e\x20\x01(\x0b2\x0c.AAPKBPEGGBHR\
    \x0bBHMHLPCHKLG\x12.\n\x0bEMBAGMMHIPA\x18\t\x20\x01(\x0b2\x0c.EIMJEAMDFK\
    JR\x0bEMBAGMMHIPA\x12.\n\x0bBJLEMFMCODD\x18\x0b\x20\x01(\x0b2\x0c.FJJDKD\
    NDFDJR\x0bBJLEMFMCODDb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::AAPKBPEGGBH::file_descriptor().clone());
            deps.push(super::CACLANLOOLK::file_descriptor().clone());
            deps.push(super::EIMJEAMDFKJ::file_descriptor().clone());
            deps.push(super::FJJDKDNDFDJ::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(OGNBIGKHHBM::generated_message_descriptor_data());
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
