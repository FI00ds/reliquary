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

//! Generated file from `ServerLogScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:ServerLogScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ServerLogScNotify {
    // message fields
    // @@protoc_insertion_point(field:ServerLogScNotify.level)
    pub level: ::protobuf::EnumOrUnknown<super::ServerLogLevel::ServerLogLevel>,
    // @@protoc_insertion_point(field:ServerLogScNotify.HKLIGCHHIEG)
    pub HKLIGCHHIEG: ::std::string::String,
    // @@protoc_insertion_point(field:ServerLogScNotify.EAMAJGPCGFD)
    pub EAMAJGPCGFD: ::protobuf::EnumOrUnknown<super::ServerLogTag::ServerLogTag>,
    // @@protoc_insertion_point(field:ServerLogScNotify.LCPLLGNJNAJ)
    pub LCPLLGNJNAJ: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:ServerLogScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ServerLogScNotify {
    fn default() -> &'a ServerLogScNotify {
        <ServerLogScNotify as ::protobuf::Message>::default_instance()
    }
}

impl ServerLogScNotify {
    pub fn new() -> ServerLogScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &ServerLogScNotify| { &m.level },
            |m: &mut ServerLogScNotify| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HKLIGCHHIEG",
            |m: &ServerLogScNotify| { &m.HKLIGCHHIEG },
            |m: &mut ServerLogScNotify| { &mut m.HKLIGCHHIEG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EAMAJGPCGFD",
            |m: &ServerLogScNotify| { &m.EAMAJGPCGFD },
            |m: &mut ServerLogScNotify| { &mut m.EAMAJGPCGFD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LCPLLGNJNAJ",
            |m: &ServerLogScNotify| { &m.LCPLLGNJNAJ },
            |m: &mut ServerLogScNotify| { &mut m.LCPLLGNJNAJ },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ServerLogScNotify>(
            "ServerLogScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ServerLogScNotify {
    const NAME: &'static str = "ServerLogScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.level = is.read_enum_or_unknown()?;
                },
                10 => {
                    self.HKLIGCHHIEG = is.read_string()?;
                },
                112 => {
                    self.EAMAJGPCGFD = is.read_enum_or_unknown()?;
                },
                74 => {
                    self.LCPLLGNJNAJ = is.read_string()?;
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
        if self.level != ::protobuf::EnumOrUnknown::new(super::ServerLogLevel::ServerLogLevel::SERVER_LOG_LEVEL_NONE) {
            my_size += ::protobuf::rt::int32_size(3, self.level.value());
        }
        if !self.HKLIGCHHIEG.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.HKLIGCHHIEG);
        }
        if self.EAMAJGPCGFD != ::protobuf::EnumOrUnknown::new(super::ServerLogTag::ServerLogTag::SERVER_LOG_TAG_DEFAULT) {
            my_size += ::protobuf::rt::int32_size(14, self.EAMAJGPCGFD.value());
        }
        if !self.LCPLLGNJNAJ.is_empty() {
            my_size += ::protobuf::rt::string_size(9, &self.LCPLLGNJNAJ);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.level != ::protobuf::EnumOrUnknown::new(super::ServerLogLevel::ServerLogLevel::SERVER_LOG_LEVEL_NONE) {
            os.write_enum(3, ::protobuf::EnumOrUnknown::value(&self.level))?;
        }
        if !self.HKLIGCHHIEG.is_empty() {
            os.write_string(1, &self.HKLIGCHHIEG)?;
        }
        if self.EAMAJGPCGFD != ::protobuf::EnumOrUnknown::new(super::ServerLogTag::ServerLogTag::SERVER_LOG_TAG_DEFAULT) {
            os.write_enum(14, ::protobuf::EnumOrUnknown::value(&self.EAMAJGPCGFD))?;
        }
        if !self.LCPLLGNJNAJ.is_empty() {
            os.write_string(9, &self.LCPLLGNJNAJ)?;
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

    fn new() -> ServerLogScNotify {
        ServerLogScNotify::new()
    }

    fn clear(&mut self) {
        self.level = ::protobuf::EnumOrUnknown::new(super::ServerLogLevel::ServerLogLevel::SERVER_LOG_LEVEL_NONE);
        self.HKLIGCHHIEG.clear();
        self.EAMAJGPCGFD = ::protobuf::EnumOrUnknown::new(super::ServerLogTag::ServerLogTag::SERVER_LOG_TAG_DEFAULT);
        self.LCPLLGNJNAJ.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ServerLogScNotify {
        static instance: ServerLogScNotify = ServerLogScNotify {
            level: ::protobuf::EnumOrUnknown::from_i32(0),
            HKLIGCHHIEG: ::std::string::String::new(),
            EAMAJGPCGFD: ::protobuf::EnumOrUnknown::from_i32(0),
            LCPLLGNJNAJ: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ServerLogScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ServerLogScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ServerLogScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ServerLogScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17ServerLogScNotify.proto\x1a\x14ServerLogLevel.proto\x1a\x12ServerL\
    ogTag.proto\"\xaf\x01\n\x11ServerLogScNotify\x12%\n\x05level\x18\x03\x20\
    \x01(\x0e2\x0f.ServerLogLevelR\x05level\x12\x20\n\x0bHKLIGCHHIEG\x18\x01\
    \x20\x01(\tR\x0bHKLIGCHHIEG\x12/\n\x0bEAMAJGPCGFD\x18\x0e\x20\x01(\x0e2\
    \r.ServerLogTagR\x0bEAMAJGPCGFD\x12\x20\n\x0bLCPLLGNJNAJ\x18\t\x20\x01(\
    \tR\x0bLCPLLGNJNAJb\x06proto3\
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
            deps.push(super::ServerLogLevel::file_descriptor().clone());
            deps.push(super::ServerLogTag::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ServerLogScNotify::generated_message_descriptor_data());
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
