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

//! Generated file from `SetLineupNameScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:SetLineupNameScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SetLineupNameScRsp {
    // message fields
    // @@protoc_insertion_point(field:SetLineupNameScRsp.MJJBNDAKOIF)
    pub MJJBNDAKOIF: ::std::string::String,
    // @@protoc_insertion_point(field:SetLineupNameScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:SetLineupNameScRsp.EGMAFIOOKJJ)
    pub EGMAFIOOKJJ: u32,
    // special fields
    // @@protoc_insertion_point(special_field:SetLineupNameScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SetLineupNameScRsp {
    fn default() -> &'a SetLineupNameScRsp {
        <SetLineupNameScRsp as ::protobuf::Message>::default_instance()
    }
}

impl SetLineupNameScRsp {
    pub fn new() -> SetLineupNameScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MJJBNDAKOIF",
            |m: &SetLineupNameScRsp| { &m.MJJBNDAKOIF },
            |m: &mut SetLineupNameScRsp| { &mut m.MJJBNDAKOIF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &SetLineupNameScRsp| { &m.retcode },
            |m: &mut SetLineupNameScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EGMAFIOOKJJ",
            |m: &SetLineupNameScRsp| { &m.EGMAFIOOKJJ },
            |m: &mut SetLineupNameScRsp| { &mut m.EGMAFIOOKJJ },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SetLineupNameScRsp>(
            "SetLineupNameScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SetLineupNameScRsp {
    const NAME: &'static str = "SetLineupNameScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                106 => {
                    self.MJJBNDAKOIF = is.read_string()?;
                },
                96 => {
                    self.retcode = is.read_uint32()?;
                },
                88 => {
                    self.EGMAFIOOKJJ = is.read_uint32()?;
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
        if !self.MJJBNDAKOIF.is_empty() {
            my_size += ::protobuf::rt::string_size(13, &self.MJJBNDAKOIF);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.retcode);
        }
        if self.EGMAFIOOKJJ != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.EGMAFIOOKJJ);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.MJJBNDAKOIF.is_empty() {
            os.write_string(13, &self.MJJBNDAKOIF)?;
        }
        if self.retcode != 0 {
            os.write_uint32(12, self.retcode)?;
        }
        if self.EGMAFIOOKJJ != 0 {
            os.write_uint32(11, self.EGMAFIOOKJJ)?;
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

    fn new() -> SetLineupNameScRsp {
        SetLineupNameScRsp::new()
    }

    fn clear(&mut self) {
        self.MJJBNDAKOIF.clear();
        self.retcode = 0;
        self.EGMAFIOOKJJ = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SetLineupNameScRsp {
        static instance: SetLineupNameScRsp = SetLineupNameScRsp {
            MJJBNDAKOIF: ::std::string::String::new(),
            retcode: 0,
            EGMAFIOOKJJ: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SetLineupNameScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SetLineupNameScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SetLineupNameScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetLineupNameScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18SetLineupNameScRsp.proto\"r\n\x12SetLineupNameScRsp\x12\x20\n\x0bM\
    JJBNDAKOIF\x18\r\x20\x01(\tR\x0bMJJBNDAKOIF\x12\x18\n\x07retcode\x18\x0c\
    \x20\x01(\rR\x07retcode\x12\x20\n\x0bEGMAFIOOKJJ\x18\x0b\x20\x01(\rR\x0b\
    EGMAFIOOKJJb\x06proto3\
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
            messages.push(SetLineupNameScRsp::generated_message_descriptor_data());
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
