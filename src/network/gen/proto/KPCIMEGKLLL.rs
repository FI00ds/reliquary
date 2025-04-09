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

//! Generated file from `KPCIMEGKLLL.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:KPCIMEGKLLL)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct KPCIMEGKLLL {
    // message fields
    // @@protoc_insertion_point(field:KPCIMEGKLLL.IAOGFCMADLC)
    pub IAOGFCMADLC: ::std::string::String,
    // @@protoc_insertion_point(field:KPCIMEGKLLL.slot)
    pub slot: ::protobuf::EnumOrUnknown<super::JPOKODCNPOA::JPOKODCNPOA>,
    // special fields
    // @@protoc_insertion_point(special_field:KPCIMEGKLLL.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a KPCIMEGKLLL {
    fn default() -> &'a KPCIMEGKLLL {
        <KPCIMEGKLLL as ::protobuf::Message>::default_instance()
    }
}

impl KPCIMEGKLLL {
    pub fn new() -> KPCIMEGKLLL {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IAOGFCMADLC",
            |m: &KPCIMEGKLLL| { &m.IAOGFCMADLC },
            |m: &mut KPCIMEGKLLL| { &mut m.IAOGFCMADLC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "slot",
            |m: &KPCIMEGKLLL| { &m.slot },
            |m: &mut KPCIMEGKLLL| { &mut m.slot },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<KPCIMEGKLLL>(
            "KPCIMEGKLLL",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for KPCIMEGKLLL {
    const NAME: &'static str = "KPCIMEGKLLL";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                34 => {
                    self.IAOGFCMADLC = is.read_string()?;
                },
                56 => {
                    self.slot = is.read_enum_or_unknown()?;
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
        if !self.IAOGFCMADLC.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.IAOGFCMADLC);
        }
        if self.slot != ::protobuf::EnumOrUnknown::new(super::JPOKODCNPOA::JPOKODCNPOA::SECRET_KEY_NONE) {
            my_size += ::protobuf::rt::int32_size(7, self.slot.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.IAOGFCMADLC.is_empty() {
            os.write_string(4, &self.IAOGFCMADLC)?;
        }
        if self.slot != ::protobuf::EnumOrUnknown::new(super::JPOKODCNPOA::JPOKODCNPOA::SECRET_KEY_NONE) {
            os.write_enum(7, ::protobuf::EnumOrUnknown::value(&self.slot))?;
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

    fn new() -> KPCIMEGKLLL {
        KPCIMEGKLLL::new()
    }

    fn clear(&mut self) {
        self.IAOGFCMADLC.clear();
        self.slot = ::protobuf::EnumOrUnknown::new(super::JPOKODCNPOA::JPOKODCNPOA::SECRET_KEY_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static KPCIMEGKLLL {
        static instance: KPCIMEGKLLL = KPCIMEGKLLL {
            IAOGFCMADLC: ::std::string::String::new(),
            slot: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for KPCIMEGKLLL {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("KPCIMEGKLLL").unwrap()).clone()
    }
}

impl ::std::fmt::Display for KPCIMEGKLLL {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KPCIMEGKLLL {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11KPCIMEGKLLL.proto\x1a\x11JPOKODCNPOA.proto\"Q\n\x0bKPCIMEGKLLL\x12\
    \x20\n\x0bIAOGFCMADLC\x18\x04\x20\x01(\tR\x0bIAOGFCMADLC\x12\x20\n\x04sl\
    ot\x18\x07\x20\x01(\x0e2\x0c.JPOKODCNPOAR\x04slotb\x06proto3\
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
            deps.push(super::JPOKODCNPOA::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(KPCIMEGKLLL::generated_message_descriptor_data());
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
