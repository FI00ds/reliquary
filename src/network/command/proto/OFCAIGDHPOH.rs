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

//! Generated file from `OFCAIGDHPOH.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:OFCAIGDHPOH)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct OFCAIGDHPOH {
    // message fields
    // @@protoc_insertion_point(field:OFCAIGDHPOH.config_id)
    pub config_id: u32,
    // @@protoc_insertion_point(field:OFCAIGDHPOH.state)
    pub state: u32,
    // @@protoc_insertion_point(field:OFCAIGDHPOH.group_id)
    pub group_id: u32,
    // @@protoc_insertion_point(field:OFCAIGDHPOH.extra_info)
    pub extra_info: ::protobuf::MessageField<super::PropExtraInfo::PropExtraInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:OFCAIGDHPOH.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a OFCAIGDHPOH {
    fn default() -> &'a OFCAIGDHPOH {
        <OFCAIGDHPOH as ::protobuf::Message>::default_instance()
    }
}

impl OFCAIGDHPOH {
    pub fn new() -> OFCAIGDHPOH {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "config_id",
            |m: &OFCAIGDHPOH| { &m.config_id },
            |m: &mut OFCAIGDHPOH| { &mut m.config_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "state",
            |m: &OFCAIGDHPOH| { &m.state },
            |m: &mut OFCAIGDHPOH| { &mut m.state },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "group_id",
            |m: &OFCAIGDHPOH| { &m.group_id },
            |m: &mut OFCAIGDHPOH| { &mut m.group_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PropExtraInfo::PropExtraInfo>(
            "extra_info",
            |m: &OFCAIGDHPOH| { &m.extra_info },
            |m: &mut OFCAIGDHPOH| { &mut m.extra_info },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<OFCAIGDHPOH>(
            "OFCAIGDHPOH",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for OFCAIGDHPOH {
    const NAME: &'static str = "OFCAIGDHPOH";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                32 => {
                    self.config_id = is.read_uint32()?;
                },
                96 => {
                    self.state = is.read_uint32()?;
                },
                48 => {
                    self.group_id = is.read_uint32()?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.extra_info)?;
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
        if self.config_id != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.config_id);
        }
        if self.state != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.state);
        }
        if self.group_id != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.group_id);
        }
        if let Some(v) = self.extra_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.config_id != 0 {
            os.write_uint32(4, self.config_id)?;
        }
        if self.state != 0 {
            os.write_uint32(12, self.state)?;
        }
        if self.group_id != 0 {
            os.write_uint32(6, self.group_id)?;
        }
        if let Some(v) = self.extra_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
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

    fn new() -> OFCAIGDHPOH {
        OFCAIGDHPOH::new()
    }

    fn clear(&mut self) {
        self.config_id = 0;
        self.state = 0;
        self.group_id = 0;
        self.extra_info.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static OFCAIGDHPOH {
        static instance: OFCAIGDHPOH = OFCAIGDHPOH {
            config_id: 0,
            state: 0,
            group_id: 0,
            extra_info: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for OFCAIGDHPOH {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("OFCAIGDHPOH").unwrap()).clone()
    }
}

impl ::std::fmt::Display for OFCAIGDHPOH {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OFCAIGDHPOH {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11OFCAIGDHPOH.proto\x1a\x13PropExtraInfo.proto\"\x8a\x01\n\x0bOFCAIG\
    DHPOH\x12\x1b\n\tconfig_id\x18\x04\x20\x01(\rR\x08configId\x12\x14\n\x05\
    state\x18\x0c\x20\x01(\rR\x05state\x12\x19\n\x08group_id\x18\x06\x20\x01\
    (\rR\x07groupId\x12-\n\nextra_info\x18\x0b\x20\x01(\x0b2\x0e.PropExtraIn\
    foR\textraInfob\x06proto3\
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
            deps.push(super::PropExtraInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(OFCAIGDHPOH::generated_message_descriptor_data());
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
