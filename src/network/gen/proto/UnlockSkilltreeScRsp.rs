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

//! Generated file from `UnlockSkilltreeScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:UnlockSkilltreeScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct UnlockSkilltreeScRsp {
    // message fields
    // @@protoc_insertion_point(field:UnlockSkilltreeScRsp.level)
    pub level: u32,
    // @@protoc_insertion_point(field:UnlockSkilltreeScRsp.point_id)
    pub point_id: u32,
    // @@protoc_insertion_point(field:UnlockSkilltreeScRsp.retcode)
    pub retcode: u32,
    // special fields
    // @@protoc_insertion_point(special_field:UnlockSkilltreeScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a UnlockSkilltreeScRsp {
    fn default() -> &'a UnlockSkilltreeScRsp {
        <UnlockSkilltreeScRsp as ::protobuf::Message>::default_instance()
    }
}

impl UnlockSkilltreeScRsp {
    pub fn new() -> UnlockSkilltreeScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &UnlockSkilltreeScRsp| { &m.level },
            |m: &mut UnlockSkilltreeScRsp| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "point_id",
            |m: &UnlockSkilltreeScRsp| { &m.point_id },
            |m: &mut UnlockSkilltreeScRsp| { &mut m.point_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &UnlockSkilltreeScRsp| { &m.retcode },
            |m: &mut UnlockSkilltreeScRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<UnlockSkilltreeScRsp>(
            "UnlockSkilltreeScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for UnlockSkilltreeScRsp {
    const NAME: &'static str = "UnlockSkilltreeScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                96 => {
                    self.level = is.read_uint32()?;
                },
                112 => {
                    self.point_id = is.read_uint32()?;
                },
                48 => {
                    self.retcode = is.read_uint32()?;
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
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.level);
        }
        if self.point_id != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.point_id);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.level != 0 {
            os.write_uint32(12, self.level)?;
        }
        if self.point_id != 0 {
            os.write_uint32(14, self.point_id)?;
        }
        if self.retcode != 0 {
            os.write_uint32(6, self.retcode)?;
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

    fn new() -> UnlockSkilltreeScRsp {
        UnlockSkilltreeScRsp::new()
    }

    fn clear(&mut self) {
        self.level = 0;
        self.point_id = 0;
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static UnlockSkilltreeScRsp {
        static instance: UnlockSkilltreeScRsp = UnlockSkilltreeScRsp {
            level: 0,
            point_id: 0,
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for UnlockSkilltreeScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("UnlockSkilltreeScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for UnlockSkilltreeScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnlockSkilltreeScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aUnlockSkilltreeScRsp.proto\"a\n\x14UnlockSkilltreeScRsp\x12\x14\n\
    \x05level\x18\x0c\x20\x01(\rR\x05level\x12\x19\n\x08point_id\x18\x0e\x20\
    \x01(\rR\x07pointId\x12\x18\n\x07retcode\x18\x06\x20\x01(\rR\x07retcodeb\
    \x06proto3\
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
            messages.push(UnlockSkilltreeScRsp::generated_message_descriptor_data());
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
