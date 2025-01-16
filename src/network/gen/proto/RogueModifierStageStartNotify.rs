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

//! Generated file from `RogueModifierStageStartNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RogueModifierStageStartNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueModifierStageStartNotify {
    // message fields
    // @@protoc_insertion_point(field:RogueModifierStageStartNotify.LEEACMNGDFL)
    pub LEEACMNGDFL: ::protobuf::EnumOrUnknown<super::RogueModifierSourceType::RogueModifierSourceType>,
    // special fields
    // @@protoc_insertion_point(special_field:RogueModifierStageStartNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueModifierStageStartNotify {
    fn default() -> &'a RogueModifierStageStartNotify {
        <RogueModifierStageStartNotify as ::protobuf::Message>::default_instance()
    }
}

impl RogueModifierStageStartNotify {
    pub fn new() -> RogueModifierStageStartNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LEEACMNGDFL",
            |m: &RogueModifierStageStartNotify| { &m.LEEACMNGDFL },
            |m: &mut RogueModifierStageStartNotify| { &mut m.LEEACMNGDFL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueModifierStageStartNotify>(
            "RogueModifierStageStartNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueModifierStageStartNotify {
    const NAME: &'static str = "RogueModifierStageStartNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                32 => {
                    self.LEEACMNGDFL = is.read_enum_or_unknown()?;
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
        if self.LEEACMNGDFL != ::protobuf::EnumOrUnknown::new(super::RogueModifierSourceType::RogueModifierSourceType::ROGUE_MODIFIER_SOURCE_NONE) {
            my_size += ::protobuf::rt::int32_size(4, self.LEEACMNGDFL.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.LEEACMNGDFL != ::protobuf::EnumOrUnknown::new(super::RogueModifierSourceType::RogueModifierSourceType::ROGUE_MODIFIER_SOURCE_NONE) {
            os.write_enum(4, ::protobuf::EnumOrUnknown::value(&self.LEEACMNGDFL))?;
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

    fn new() -> RogueModifierStageStartNotify {
        RogueModifierStageStartNotify::new()
    }

    fn clear(&mut self) {
        self.LEEACMNGDFL = ::protobuf::EnumOrUnknown::new(super::RogueModifierSourceType::RogueModifierSourceType::ROGUE_MODIFIER_SOURCE_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueModifierStageStartNotify {
        static instance: RogueModifierStageStartNotify = RogueModifierStageStartNotify {
            LEEACMNGDFL: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RogueModifierStageStartNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueModifierStageStartNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueModifierStageStartNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueModifierStageStartNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n#RogueModifierStageStartNotify.proto\x1a\x1dRogueModifierSourceType.pr\
    oto\"[\n\x1dRogueModifierStageStartNotify\x12:\n\x0bLEEACMNGDFL\x18\x04\
    \x20\x01(\x0e2\x18.RogueModifierSourceTypeR\x0bLEEACMNGDFLb\x06proto3\
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
            deps.push(super::RogueModifierSourceType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RogueModifierStageStartNotify::generated_message_descriptor_data());
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
