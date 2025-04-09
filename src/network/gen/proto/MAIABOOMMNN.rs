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

//! Generated file from `MAIABOOMMNN.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:MAIABOOMMNN)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MAIABOOMMNN {
    // message fields
    // @@protoc_insertion_point(field:MAIABOOMMNN.JMPEJFICKJO)
    pub JMPEJFICKJO: bool,
    // @@protoc_insertion_point(field:MAIABOOMMNN.KKGFIGCHKIB)
    pub KKGFIGCHKIB: bool,
    // @@protoc_insertion_point(field:MAIABOOMMNN.CLKEOEHPLNG)
    pub CLKEOEHPLNG: u32,
    // @@protoc_insertion_point(field:MAIABOOMMNN.step)
    pub step: ::protobuf::EnumOrUnknown<super::AFEFBPABLHM::AFEFBPABLHM>,
    // @@protoc_insertion_point(field:MAIABOOMMNN.JIIMELJAONE)
    pub JIIMELJAONE: ::protobuf::EnumOrUnknown<super::BFDFLHEKFGK::BFDFLHEKFGK>,
    // special fields
    // @@protoc_insertion_point(special_field:MAIABOOMMNN.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MAIABOOMMNN {
    fn default() -> &'a MAIABOOMMNN {
        <MAIABOOMMNN as ::protobuf::Message>::default_instance()
    }
}

impl MAIABOOMMNN {
    pub fn new() -> MAIABOOMMNN {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JMPEJFICKJO",
            |m: &MAIABOOMMNN| { &m.JMPEJFICKJO },
            |m: &mut MAIABOOMMNN| { &mut m.JMPEJFICKJO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KKGFIGCHKIB",
            |m: &MAIABOOMMNN| { &m.KKGFIGCHKIB },
            |m: &mut MAIABOOMMNN| { &mut m.KKGFIGCHKIB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CLKEOEHPLNG",
            |m: &MAIABOOMMNN| { &m.CLKEOEHPLNG },
            |m: &mut MAIABOOMMNN| { &mut m.CLKEOEHPLNG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "step",
            |m: &MAIABOOMMNN| { &m.step },
            |m: &mut MAIABOOMMNN| { &mut m.step },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JIIMELJAONE",
            |m: &MAIABOOMMNN| { &m.JIIMELJAONE },
            |m: &mut MAIABOOMMNN| { &mut m.JIIMELJAONE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MAIABOOMMNN>(
            "MAIABOOMMNN",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MAIABOOMMNN {
    const NAME: &'static str = "MAIABOOMMNN";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.JMPEJFICKJO = is.read_bool()?;
                },
                16 => {
                    self.KKGFIGCHKIB = is.read_bool()?;
                },
                56 => {
                    self.CLKEOEHPLNG = is.read_uint32()?;
                },
                64 => {
                    self.step = is.read_enum_or_unknown()?;
                },
                104 => {
                    self.JIIMELJAONE = is.read_enum_or_unknown()?;
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
        if self.JMPEJFICKJO != false {
            my_size += 1 + 1;
        }
        if self.KKGFIGCHKIB != false {
            my_size += 1 + 1;
        }
        if self.CLKEOEHPLNG != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.CLKEOEHPLNG);
        }
        if self.step != ::protobuf::EnumOrUnknown::new(super::AFEFBPABLHM::AFEFBPABLHM::HEART_DIAL_STEP_TYPE_MISSING) {
            my_size += ::protobuf::rt::int32_size(8, self.step.value());
        }
        if self.JIIMELJAONE != ::protobuf::EnumOrUnknown::new(super::BFDFLHEKFGK::BFDFLHEKFGK::HEART_DIAL_EMOTION_TYPE_PEACE) {
            my_size += ::protobuf::rt::int32_size(13, self.JIIMELJAONE.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.JMPEJFICKJO != false {
            os.write_bool(3, self.JMPEJFICKJO)?;
        }
        if self.KKGFIGCHKIB != false {
            os.write_bool(2, self.KKGFIGCHKIB)?;
        }
        if self.CLKEOEHPLNG != 0 {
            os.write_uint32(7, self.CLKEOEHPLNG)?;
        }
        if self.step != ::protobuf::EnumOrUnknown::new(super::AFEFBPABLHM::AFEFBPABLHM::HEART_DIAL_STEP_TYPE_MISSING) {
            os.write_enum(8, ::protobuf::EnumOrUnknown::value(&self.step))?;
        }
        if self.JIIMELJAONE != ::protobuf::EnumOrUnknown::new(super::BFDFLHEKFGK::BFDFLHEKFGK::HEART_DIAL_EMOTION_TYPE_PEACE) {
            os.write_enum(13, ::protobuf::EnumOrUnknown::value(&self.JIIMELJAONE))?;
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

    fn new() -> MAIABOOMMNN {
        MAIABOOMMNN::new()
    }

    fn clear(&mut self) {
        self.JMPEJFICKJO = false;
        self.KKGFIGCHKIB = false;
        self.CLKEOEHPLNG = 0;
        self.step = ::protobuf::EnumOrUnknown::new(super::AFEFBPABLHM::AFEFBPABLHM::HEART_DIAL_STEP_TYPE_MISSING);
        self.JIIMELJAONE = ::protobuf::EnumOrUnknown::new(super::BFDFLHEKFGK::BFDFLHEKFGK::HEART_DIAL_EMOTION_TYPE_PEACE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MAIABOOMMNN {
        static instance: MAIABOOMMNN = MAIABOOMMNN {
            JMPEJFICKJO: false,
            KKGFIGCHKIB: false,
            CLKEOEHPLNG: 0,
            step: ::protobuf::EnumOrUnknown::from_i32(0),
            JIIMELJAONE: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MAIABOOMMNN {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MAIABOOMMNN").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MAIABOOMMNN {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MAIABOOMMNN {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11MAIABOOMMNN.proto\x1a\x11AFEFBPABLHM.proto\x1a\x11BFDFLHEKFGK.prot\
    o\"\xc5\x01\n\x0bMAIABOOMMNN\x12\x20\n\x0bJMPEJFICKJO\x18\x03\x20\x01(\
    \x08R\x0bJMPEJFICKJO\x12\x20\n\x0bKKGFIGCHKIB\x18\x02\x20\x01(\x08R\x0bK\
    KGFIGCHKIB\x12\x20\n\x0bCLKEOEHPLNG\x18\x07\x20\x01(\rR\x0bCLKEOEHPLNG\
    \x12\x20\n\x04step\x18\x08\x20\x01(\x0e2\x0c.AFEFBPABLHMR\x04step\x12.\n\
    \x0bJIIMELJAONE\x18\r\x20\x01(\x0e2\x0c.BFDFLHEKFGKR\x0bJIIMELJAONEb\x06\
    proto3\
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
            deps.push(super::AFEFBPABLHM::file_descriptor().clone());
            deps.push(super::BFDFLHEKFGK::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MAIABOOMMNN::generated_message_descriptor_data());
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
