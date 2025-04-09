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

//! Generated file from `SetRogueExhibitionCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:SetRogueExhibitionCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SetRogueExhibitionCsReq {
    // message fields
    // @@protoc_insertion_point(field:SetRogueExhibitionCsReq.HEEPOEOLILO)
    pub HEEPOEOLILO: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:SetRogueExhibitionCsReq.ANEKPINLKFJ)
    pub ANEKPINLKFJ: ::std::vec::Vec<::protobuf::EnumOrUnknown<super::RogueCollectionExhibitionOperateType::RogueCollectionExhibitionOperateType>>,
    // @@protoc_insertion_point(field:SetRogueExhibitionCsReq.LDIFBJDGFFE)
    pub LDIFBJDGFFE: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:SetRogueExhibitionCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SetRogueExhibitionCsReq {
    fn default() -> &'a SetRogueExhibitionCsReq {
        <SetRogueExhibitionCsReq as ::protobuf::Message>::default_instance()
    }
}

impl SetRogueExhibitionCsReq {
    pub fn new() -> SetRogueExhibitionCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HEEPOEOLILO",
            |m: &SetRogueExhibitionCsReq| { &m.HEEPOEOLILO },
            |m: &mut SetRogueExhibitionCsReq| { &mut m.HEEPOEOLILO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "ANEKPINLKFJ",
            |m: &SetRogueExhibitionCsReq| { &m.ANEKPINLKFJ },
            |m: &mut SetRogueExhibitionCsReq| { &mut m.ANEKPINLKFJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LDIFBJDGFFE",
            |m: &SetRogueExhibitionCsReq| { &m.LDIFBJDGFFE },
            |m: &mut SetRogueExhibitionCsReq| { &mut m.LDIFBJDGFFE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SetRogueExhibitionCsReq>(
            "SetRogueExhibitionCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SetRogueExhibitionCsReq {
    const NAME: &'static str = "SetRogueExhibitionCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                90 => {
                    is.read_repeated_packed_uint32_into(&mut self.HEEPOEOLILO)?;
                },
                88 => {
                    self.HEEPOEOLILO.push(is.read_uint32()?);
                },
                112 => {
                    self.ANEKPINLKFJ.push(is.read_enum_or_unknown()?);
                },
                114 => {
                    ::protobuf::rt::read_repeated_packed_enum_or_unknown_into(is, &mut self.ANEKPINLKFJ)?
                },
                50 => {
                    is.read_repeated_packed_uint32_into(&mut self.LDIFBJDGFFE)?;
                },
                48 => {
                    self.LDIFBJDGFFE.push(is.read_uint32()?);
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
        my_size += ::protobuf::rt::vec_packed_uint32_size(11, &self.HEEPOEOLILO);
        my_size += ::protobuf::rt::vec_packed_enum_or_unknown_size(14, &self.ANEKPINLKFJ);
        my_size += ::protobuf::rt::vec_packed_uint32_size(6, &self.LDIFBJDGFFE);
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        os.write_repeated_packed_uint32(11, &self.HEEPOEOLILO)?;
        os.write_repeated_packed_enum_or_unknown(14, &self.ANEKPINLKFJ)?;
        os.write_repeated_packed_uint32(6, &self.LDIFBJDGFFE)?;
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> SetRogueExhibitionCsReq {
        SetRogueExhibitionCsReq::new()
    }

    fn clear(&mut self) {
        self.HEEPOEOLILO.clear();
        self.ANEKPINLKFJ.clear();
        self.LDIFBJDGFFE.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SetRogueExhibitionCsReq {
        static instance: SetRogueExhibitionCsReq = SetRogueExhibitionCsReq {
            HEEPOEOLILO: ::std::vec::Vec::new(),
            ANEKPINLKFJ: ::std::vec::Vec::new(),
            LDIFBJDGFFE: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SetRogueExhibitionCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SetRogueExhibitionCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SetRogueExhibitionCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetRogueExhibitionCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dSetRogueExhibitionCsReq.proto\x1a*RogueCollectionExhibitionOperate\
    Type.proto\"\xa6\x01\n\x17SetRogueExhibitionCsReq\x12\x20\n\x0bHEEPOEOLI\
    LO\x18\x0b\x20\x03(\rR\x0bHEEPOEOLILO\x12G\n\x0bANEKPINLKFJ\x18\x0e\x20\
    \x03(\x0e2%.RogueCollectionExhibitionOperateTypeR\x0bANEKPINLKFJ\x12\x20\
    \n\x0bLDIFBJDGFFE\x18\x06\x20\x03(\rR\x0bLDIFBJDGFFEb\x06proto3\
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
            deps.push(super::RogueCollectionExhibitionOperateType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SetRogueExhibitionCsReq::generated_message_descriptor_data());
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
