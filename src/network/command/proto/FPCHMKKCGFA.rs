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

//! Generated file from `FPCHMKKCGFA.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:FPCHMKKCGFA)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FPCHMKKCGFA {
    // message fields
    // @@protoc_insertion_point(field:FPCHMKKCGFA.BNCKFBFMGMI)
    pub BNCKFBFMGMI: u32,
    // @@protoc_insertion_point(field:FPCHMKKCGFA.AFFJHMJDIBN)
    pub AFFJHMJDIBN: u32,
    // @@protoc_insertion_point(field:FPCHMKKCGFA.LGHOKGABGCK)
    pub LGHOKGABGCK: ::protobuf::EnumOrUnknown<super::NJCHLJFIODM::NJCHLJFIODM>,
    // special fields
    // @@protoc_insertion_point(special_field:FPCHMKKCGFA.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FPCHMKKCGFA {
    fn default() -> &'a FPCHMKKCGFA {
        <FPCHMKKCGFA as ::protobuf::Message>::default_instance()
    }
}

impl FPCHMKKCGFA {
    pub fn new() -> FPCHMKKCGFA {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BNCKFBFMGMI",
            |m: &FPCHMKKCGFA| { &m.BNCKFBFMGMI },
            |m: &mut FPCHMKKCGFA| { &mut m.BNCKFBFMGMI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AFFJHMJDIBN",
            |m: &FPCHMKKCGFA| { &m.AFFJHMJDIBN },
            |m: &mut FPCHMKKCGFA| { &mut m.AFFJHMJDIBN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LGHOKGABGCK",
            |m: &FPCHMKKCGFA| { &m.LGHOKGABGCK },
            |m: &mut FPCHMKKCGFA| { &mut m.LGHOKGABGCK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FPCHMKKCGFA>(
            "FPCHMKKCGFA",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FPCHMKKCGFA {
    const NAME: &'static str = "FPCHMKKCGFA";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.BNCKFBFMGMI = is.read_uint32()?;
                },
                24 => {
                    self.AFFJHMJDIBN = is.read_uint32()?;
                },
                56 => {
                    self.LGHOKGABGCK = is.read_enum_or_unknown()?;
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
        if self.BNCKFBFMGMI != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.BNCKFBFMGMI);
        }
        if self.AFFJHMJDIBN != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.AFFJHMJDIBN);
        }
        if self.LGHOKGABGCK != ::protobuf::EnumOrUnknown::new(super::NJCHLJFIODM::NJCHLJFIODM::ROGUE_MODIFIER_CONTENT_DEFINITE) {
            my_size += ::protobuf::rt::int32_size(7, self.LGHOKGABGCK.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.BNCKFBFMGMI != 0 {
            os.write_uint32(8, self.BNCKFBFMGMI)?;
        }
        if self.AFFJHMJDIBN != 0 {
            os.write_uint32(3, self.AFFJHMJDIBN)?;
        }
        if self.LGHOKGABGCK != ::protobuf::EnumOrUnknown::new(super::NJCHLJFIODM::NJCHLJFIODM::ROGUE_MODIFIER_CONTENT_DEFINITE) {
            os.write_enum(7, ::protobuf::EnumOrUnknown::value(&self.LGHOKGABGCK))?;
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

    fn new() -> FPCHMKKCGFA {
        FPCHMKKCGFA::new()
    }

    fn clear(&mut self) {
        self.BNCKFBFMGMI = 0;
        self.AFFJHMJDIBN = 0;
        self.LGHOKGABGCK = ::protobuf::EnumOrUnknown::new(super::NJCHLJFIODM::NJCHLJFIODM::ROGUE_MODIFIER_CONTENT_DEFINITE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FPCHMKKCGFA {
        static instance: FPCHMKKCGFA = FPCHMKKCGFA {
            BNCKFBFMGMI: 0,
            AFFJHMJDIBN: 0,
            LGHOKGABGCK: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FPCHMKKCGFA {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FPCHMKKCGFA").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FPCHMKKCGFA {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FPCHMKKCGFA {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11FPCHMKKCGFA.proto\x1a\x11NJCHLJFIODM.proto\"\x81\x01\n\x0bFPCHMKKC\
    GFA\x12\x20\n\x0bBNCKFBFMGMI\x18\x08\x20\x01(\rR\x0bBNCKFBFMGMI\x12\x20\
    \n\x0bAFFJHMJDIBN\x18\x03\x20\x01(\rR\x0bAFFJHMJDIBN\x12.\n\x0bLGHOKGABG\
    CK\x18\x07\x20\x01(\x0e2\x0c.NJCHLJFIODMR\x0bLGHOKGABGCKb\x06proto3\
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
            deps.push(super::NJCHLJFIODM::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(FPCHMKKCGFA::generated_message_descriptor_data());
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
