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

//! Generated file from `HEHAOMIAMGL.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:HEHAOMIAMGL)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HEHAOMIAMGL {
    // message fields
    // @@protoc_insertion_point(field:HEHAOMIAMGL.KJDOLBOBKJF)
    pub KJDOLBOBKJF: ::protobuf::EnumOrUnknown<super::AlleyEventState::AlleyEventState>,
    // @@protoc_insertion_point(field:HEHAOMIAMGL.FHICMGDFGBC)
    pub FHICMGDFGBC: u32,
    // @@protoc_insertion_point(field:HEHAOMIAMGL.NINGBNBMKOP)
    pub NINGBNBMKOP: u32,
    // special fields
    // @@protoc_insertion_point(special_field:HEHAOMIAMGL.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HEHAOMIAMGL {
    fn default() -> &'a HEHAOMIAMGL {
        <HEHAOMIAMGL as ::protobuf::Message>::default_instance()
    }
}

impl HEHAOMIAMGL {
    pub fn new() -> HEHAOMIAMGL {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KJDOLBOBKJF",
            |m: &HEHAOMIAMGL| { &m.KJDOLBOBKJF },
            |m: &mut HEHAOMIAMGL| { &mut m.KJDOLBOBKJF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FHICMGDFGBC",
            |m: &HEHAOMIAMGL| { &m.FHICMGDFGBC },
            |m: &mut HEHAOMIAMGL| { &mut m.FHICMGDFGBC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NINGBNBMKOP",
            |m: &HEHAOMIAMGL| { &m.NINGBNBMKOP },
            |m: &mut HEHAOMIAMGL| { &mut m.NINGBNBMKOP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HEHAOMIAMGL>(
            "HEHAOMIAMGL",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HEHAOMIAMGL {
    const NAME: &'static str = "HEHAOMIAMGL";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.KJDOLBOBKJF = is.read_enum_or_unknown()?;
                },
                32 => {
                    self.FHICMGDFGBC = is.read_uint32()?;
                },
                80 => {
                    self.NINGBNBMKOP = is.read_uint32()?;
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
        if self.KJDOLBOBKJF != ::protobuf::EnumOrUnknown::new(super::AlleyEventState::AlleyEventState::ALLEY_STATE_NONE) {
            my_size += ::protobuf::rt::int32_size(1, self.KJDOLBOBKJF.value());
        }
        if self.FHICMGDFGBC != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.FHICMGDFGBC);
        }
        if self.NINGBNBMKOP != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.NINGBNBMKOP);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.KJDOLBOBKJF != ::protobuf::EnumOrUnknown::new(super::AlleyEventState::AlleyEventState::ALLEY_STATE_NONE) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.KJDOLBOBKJF))?;
        }
        if self.FHICMGDFGBC != 0 {
            os.write_uint32(4, self.FHICMGDFGBC)?;
        }
        if self.NINGBNBMKOP != 0 {
            os.write_uint32(10, self.NINGBNBMKOP)?;
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

    fn new() -> HEHAOMIAMGL {
        HEHAOMIAMGL::new()
    }

    fn clear(&mut self) {
        self.KJDOLBOBKJF = ::protobuf::EnumOrUnknown::new(super::AlleyEventState::AlleyEventState::ALLEY_STATE_NONE);
        self.FHICMGDFGBC = 0;
        self.NINGBNBMKOP = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HEHAOMIAMGL {
        static instance: HEHAOMIAMGL = HEHAOMIAMGL {
            KJDOLBOBKJF: ::protobuf::EnumOrUnknown::from_i32(0),
            FHICMGDFGBC: 0,
            NINGBNBMKOP: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HEHAOMIAMGL {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HEHAOMIAMGL").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HEHAOMIAMGL {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HEHAOMIAMGL {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11HEHAOMIAMGL.proto\x1a\x15AlleyEventState.proto\"\x85\x01\n\x0bHEHA\
    OMIAMGL\x122\n\x0bKJDOLBOBKJF\x18\x01\x20\x01(\x0e2\x10.AlleyEventStateR\
    \x0bKJDOLBOBKJF\x12\x20\n\x0bFHICMGDFGBC\x18\x04\x20\x01(\rR\x0bFHICMGDF\
    GBC\x12\x20\n\x0bNINGBNBMKOP\x18\n\x20\x01(\rR\x0bNINGBNBMKOPb\x06proto3\
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
            deps.push(super::AlleyEventState::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(HEHAOMIAMGL::generated_message_descriptor_data());
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
