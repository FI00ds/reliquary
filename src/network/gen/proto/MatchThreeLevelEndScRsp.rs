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

//! Generated file from `MatchThreeLevelEndScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:MatchThreeLevelEndScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MatchThreeLevelEndScRsp {
    // message fields
    // @@protoc_insertion_point(field:MatchThreeLevelEndScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:MatchThreeLevelEndScRsp.EBGMBDMPEGM)
    pub EBGMBDMPEGM: u32,
    // @@protoc_insertion_point(field:MatchThreeLevelEndScRsp.ACJCPHIFMLN)
    pub ACJCPHIFMLN: u32,
    // special fields
    // @@protoc_insertion_point(special_field:MatchThreeLevelEndScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MatchThreeLevelEndScRsp {
    fn default() -> &'a MatchThreeLevelEndScRsp {
        <MatchThreeLevelEndScRsp as ::protobuf::Message>::default_instance()
    }
}

impl MatchThreeLevelEndScRsp {
    pub fn new() -> MatchThreeLevelEndScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &MatchThreeLevelEndScRsp| { &m.retcode },
            |m: &mut MatchThreeLevelEndScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EBGMBDMPEGM",
            |m: &MatchThreeLevelEndScRsp| { &m.EBGMBDMPEGM },
            |m: &mut MatchThreeLevelEndScRsp| { &mut m.EBGMBDMPEGM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ACJCPHIFMLN",
            |m: &MatchThreeLevelEndScRsp| { &m.ACJCPHIFMLN },
            |m: &mut MatchThreeLevelEndScRsp| { &mut m.ACJCPHIFMLN },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MatchThreeLevelEndScRsp>(
            "MatchThreeLevelEndScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MatchThreeLevelEndScRsp {
    const NAME: &'static str = "MatchThreeLevelEndScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.retcode = is.read_uint32()?;
                },
                112 => {
                    self.EBGMBDMPEGM = is.read_uint32()?;
                },
                88 => {
                    self.ACJCPHIFMLN = is.read_uint32()?;
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
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.retcode);
        }
        if self.EBGMBDMPEGM != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.EBGMBDMPEGM);
        }
        if self.ACJCPHIFMLN != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.ACJCPHIFMLN);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.retcode != 0 {
            os.write_uint32(8, self.retcode)?;
        }
        if self.EBGMBDMPEGM != 0 {
            os.write_uint32(14, self.EBGMBDMPEGM)?;
        }
        if self.ACJCPHIFMLN != 0 {
            os.write_uint32(11, self.ACJCPHIFMLN)?;
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

    fn new() -> MatchThreeLevelEndScRsp {
        MatchThreeLevelEndScRsp::new()
    }

    fn clear(&mut self) {
        self.retcode = 0;
        self.EBGMBDMPEGM = 0;
        self.ACJCPHIFMLN = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MatchThreeLevelEndScRsp {
        static instance: MatchThreeLevelEndScRsp = MatchThreeLevelEndScRsp {
            retcode: 0,
            EBGMBDMPEGM: 0,
            ACJCPHIFMLN: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MatchThreeLevelEndScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MatchThreeLevelEndScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MatchThreeLevelEndScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MatchThreeLevelEndScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dMatchThreeLevelEndScRsp.proto\"w\n\x17MatchThreeLevelEndScRsp\x12\
    \x18\n\x07retcode\x18\x08\x20\x01(\rR\x07retcode\x12\x20\n\x0bEBGMBDMPEG\
    M\x18\x0e\x20\x01(\rR\x0bEBGMBDMPEGM\x12\x20\n\x0bACJCPHIFMLN\x18\x0b\
    \x20\x01(\rR\x0bACJCPHIFMLNb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MatchThreeLevelEndScRsp::generated_message_descriptor_data());
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
