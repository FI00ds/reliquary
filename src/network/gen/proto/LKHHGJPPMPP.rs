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

//! Generated file from `LKHHGJPPMPP.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:LKHHGJPPMPP)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LKHHGJPPMPP {
    // message fields
    // @@protoc_insertion_point(field:LKHHGJPPMPP.avatar_id)
    pub avatar_id: u32,
    // @@protoc_insertion_point(field:LKHHGJPPMPP.IHLEAMDIKKN)
    pub IHLEAMDIKKN: u32,
    // special fields
    // @@protoc_insertion_point(special_field:LKHHGJPPMPP.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LKHHGJPPMPP {
    fn default() -> &'a LKHHGJPPMPP {
        <LKHHGJPPMPP as ::protobuf::Message>::default_instance()
    }
}

impl LKHHGJPPMPP {
    pub fn new() -> LKHHGJPPMPP {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_id",
            |m: &LKHHGJPPMPP| { &m.avatar_id },
            |m: &mut LKHHGJPPMPP| { &mut m.avatar_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IHLEAMDIKKN",
            |m: &LKHHGJPPMPP| { &m.IHLEAMDIKKN },
            |m: &mut LKHHGJPPMPP| { &mut m.IHLEAMDIKKN },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LKHHGJPPMPP>(
            "LKHHGJPPMPP",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LKHHGJPPMPP {
    const NAME: &'static str = "LKHHGJPPMPP";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.avatar_id = is.read_uint32()?;
                },
                96 => {
                    self.IHLEAMDIKKN = is.read_uint32()?;
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
        if self.avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.avatar_id);
        }
        if self.IHLEAMDIKKN != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.IHLEAMDIKKN);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.avatar_id != 0 {
            os.write_uint32(7, self.avatar_id)?;
        }
        if self.IHLEAMDIKKN != 0 {
            os.write_uint32(12, self.IHLEAMDIKKN)?;
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

    fn new() -> LKHHGJPPMPP {
        LKHHGJPPMPP::new()
    }

    fn clear(&mut self) {
        self.avatar_id = 0;
        self.IHLEAMDIKKN = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LKHHGJPPMPP {
        static instance: LKHHGJPPMPP = LKHHGJPPMPP {
            avatar_id: 0,
            IHLEAMDIKKN: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LKHHGJPPMPP {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LKHHGJPPMPP").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LKHHGJPPMPP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LKHHGJPPMPP {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LKHHGJPPMPP.proto\"L\n\x0bLKHHGJPPMPP\x12\x1b\n\tavatar_id\x18\x07\
    \x20\x01(\rR\x08avatarId\x12\x20\n\x0bIHLEAMDIKKN\x18\x0c\x20\x01(\rR\
    \x0bIHLEAMDIKKNb\x06proto3\
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
            messages.push(LKHHGJPPMPP::generated_message_descriptor_data());
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
