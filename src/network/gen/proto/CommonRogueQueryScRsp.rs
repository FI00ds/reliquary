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

//! Generated file from `CommonRogueQueryScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:CommonRogueQueryScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CommonRogueQueryScRsp {
    // message fields
    // @@protoc_insertion_point(field:CommonRogueQueryScRsp.FHHBJLHAJFJ)
    pub FHHBJLHAJFJ: u32,
    // @@protoc_insertion_point(field:CommonRogueQueryScRsp.LAGBGLDLGCB)
    pub LAGBGLDLGCB: ::protobuf::MessageField<super::LGJMDNNMPPE::LGJMDNNMPPE>,
    // @@protoc_insertion_point(field:CommonRogueQueryScRsp.retcode)
    pub retcode: u32,
    // special fields
    // @@protoc_insertion_point(special_field:CommonRogueQueryScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CommonRogueQueryScRsp {
    fn default() -> &'a CommonRogueQueryScRsp {
        <CommonRogueQueryScRsp as ::protobuf::Message>::default_instance()
    }
}

impl CommonRogueQueryScRsp {
    pub fn new() -> CommonRogueQueryScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FHHBJLHAJFJ",
            |m: &CommonRogueQueryScRsp| { &m.FHHBJLHAJFJ },
            |m: &mut CommonRogueQueryScRsp| { &mut m.FHHBJLHAJFJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::LGJMDNNMPPE::LGJMDNNMPPE>(
            "LAGBGLDLGCB",
            |m: &CommonRogueQueryScRsp| { &m.LAGBGLDLGCB },
            |m: &mut CommonRogueQueryScRsp| { &mut m.LAGBGLDLGCB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &CommonRogueQueryScRsp| { &m.retcode },
            |m: &mut CommonRogueQueryScRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CommonRogueQueryScRsp>(
            "CommonRogueQueryScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CommonRogueQueryScRsp {
    const NAME: &'static str = "CommonRogueQueryScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.FHHBJLHAJFJ = is.read_uint32()?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LAGBGLDLGCB)?;
                },
                32 => {
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
        if self.FHHBJLHAJFJ != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.FHHBJLHAJFJ);
        }
        if let Some(v) = self.LAGBGLDLGCB.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.FHHBJLHAJFJ != 0 {
            os.write_uint32(6, self.FHHBJLHAJFJ)?;
        }
        if let Some(v) = self.LAGBGLDLGCB.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if self.retcode != 0 {
            os.write_uint32(4, self.retcode)?;
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

    fn new() -> CommonRogueQueryScRsp {
        CommonRogueQueryScRsp::new()
    }

    fn clear(&mut self) {
        self.FHHBJLHAJFJ = 0;
        self.LAGBGLDLGCB.clear();
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CommonRogueQueryScRsp {
        static instance: CommonRogueQueryScRsp = CommonRogueQueryScRsp {
            FHHBJLHAJFJ: 0,
            LAGBGLDLGCB: ::protobuf::MessageField::none(),
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CommonRogueQueryScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CommonRogueQueryScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CommonRogueQueryScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CommonRogueQueryScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bCommonRogueQueryScRsp.proto\x1a\x11LGJMDNNMPPE.proto\"\x83\x01\n\
    \x15CommonRogueQueryScRsp\x12\x20\n\x0bFHHBJLHAJFJ\x18\x06\x20\x01(\rR\
    \x0bFHHBJLHAJFJ\x12.\n\x0bLAGBGLDLGCB\x18\t\x20\x01(\x0b2\x0c.LGJMDNNMPP\
    ER\x0bLAGBGLDLGCB\x12\x18\n\x07retcode\x18\x04\x20\x01(\rR\x07retcodeb\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::LGJMDNNMPPE::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CommonRogueQueryScRsp::generated_message_descriptor_data());
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
