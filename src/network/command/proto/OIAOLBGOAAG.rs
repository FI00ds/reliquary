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

//! Generated file from `OIAOLBGOAAG.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:OIAOLBGOAAG)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct OIAOLBGOAAG {
    // message fields
    // @@protoc_insertion_point(field:OIAOLBGOAAG.NOAEDNNIBAF)
    pub NOAEDNNIBAF: ::protobuf::MessageField<super::RogueSyncContextBoardEvent::RogueSyncContextBoardEvent>,
    // @@protoc_insertion_point(field:OIAOLBGOAAG.HHPHLEGCLDM)
    pub HHPHLEGCLDM: ::protobuf::MessageField<super::CFELLCPMONH::CFELLCPMONH>,
    // special fields
    // @@protoc_insertion_point(special_field:OIAOLBGOAAG.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a OIAOLBGOAAG {
    fn default() -> &'a OIAOLBGOAAG {
        <OIAOLBGOAAG as ::protobuf::Message>::default_instance()
    }
}

impl OIAOLBGOAAG {
    pub fn new() -> OIAOLBGOAAG {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::RogueSyncContextBoardEvent::RogueSyncContextBoardEvent>(
            "NOAEDNNIBAF",
            |m: &OIAOLBGOAAG| { &m.NOAEDNNIBAF },
            |m: &mut OIAOLBGOAAG| { &mut m.NOAEDNNIBAF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::CFELLCPMONH::CFELLCPMONH>(
            "HHPHLEGCLDM",
            |m: &OIAOLBGOAAG| { &m.HHPHLEGCLDM },
            |m: &mut OIAOLBGOAAG| { &mut m.HHPHLEGCLDM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<OIAOLBGOAAG>(
            "OIAOLBGOAAG",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for OIAOLBGOAAG {
    const NAME: &'static str = "OIAOLBGOAAG";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.NOAEDNNIBAF)?;
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.HHPHLEGCLDM)?;
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
        if let Some(v) = self.NOAEDNNIBAF.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.HHPHLEGCLDM.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.NOAEDNNIBAF.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if let Some(v) = self.HHPHLEGCLDM.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
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

    fn new() -> OIAOLBGOAAG {
        OIAOLBGOAAG::new()
    }

    fn clear(&mut self) {
        self.NOAEDNNIBAF.clear();
        self.HHPHLEGCLDM.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static OIAOLBGOAAG {
        static instance: OIAOLBGOAAG = OIAOLBGOAAG {
            NOAEDNNIBAF: ::protobuf::MessageField::none(),
            HHPHLEGCLDM: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for OIAOLBGOAAG {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("OIAOLBGOAAG").unwrap()).clone()
    }
}

impl ::std::fmt::Display for OIAOLBGOAAG {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OIAOLBGOAAG {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11OIAOLBGOAAG.proto\x1a\x11CFELLCPMONH.proto\x1a\x20RogueSyncContext\
    BoardEvent.proto\"|\n\x0bOIAOLBGOAAG\x12=\n\x0bNOAEDNNIBAF\x18\x04\x20\
    \x01(\x0b2\x1b.RogueSyncContextBoardEventR\x0bNOAEDNNIBAF\x12.\n\x0bHHPH\
    LEGCLDM\x18\r\x20\x01(\x0b2\x0c.CFELLCPMONHR\x0bHHPHLEGCLDMb\x06proto3\
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
            deps.push(super::CFELLCPMONH::file_descriptor().clone());
            deps.push(super::RogueSyncContextBoardEvent::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(OIAOLBGOAAG::generated_message_descriptor_data());
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
