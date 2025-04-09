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

//! Generated file from `GetSingleRedDotParamGroupCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:GetSingleRedDotParamGroupCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetSingleRedDotParamGroupCsReq {
    // message fields
    // @@protoc_insertion_point(field:GetSingleRedDotParamGroupCsReq.NOPDKLDEKKF)
    pub NOPDKLDEKKF: u32,
    // @@protoc_insertion_point(field:GetSingleRedDotParamGroupCsReq.FJNHDHOHBCL)
    pub FJNHDHOHBCL: u32,
    // @@protoc_insertion_point(field:GetSingleRedDotParamGroupCsReq.DKKLLMOHGFD)
    pub DKKLLMOHGFD: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GetSingleRedDotParamGroupCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetSingleRedDotParamGroupCsReq {
    fn default() -> &'a GetSingleRedDotParamGroupCsReq {
        <GetSingleRedDotParamGroupCsReq as ::protobuf::Message>::default_instance()
    }
}

impl GetSingleRedDotParamGroupCsReq {
    pub fn new() -> GetSingleRedDotParamGroupCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NOPDKLDEKKF",
            |m: &GetSingleRedDotParamGroupCsReq| { &m.NOPDKLDEKKF },
            |m: &mut GetSingleRedDotParamGroupCsReq| { &mut m.NOPDKLDEKKF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FJNHDHOHBCL",
            |m: &GetSingleRedDotParamGroupCsReq| { &m.FJNHDHOHBCL },
            |m: &mut GetSingleRedDotParamGroupCsReq| { &mut m.FJNHDHOHBCL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DKKLLMOHGFD",
            |m: &GetSingleRedDotParamGroupCsReq| { &m.DKKLLMOHGFD },
            |m: &mut GetSingleRedDotParamGroupCsReq| { &mut m.DKKLLMOHGFD },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetSingleRedDotParamGroupCsReq>(
            "GetSingleRedDotParamGroupCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetSingleRedDotParamGroupCsReq {
    const NAME: &'static str = "GetSingleRedDotParamGroupCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.NOPDKLDEKKF = is.read_uint32()?;
                },
                72 => {
                    self.FJNHDHOHBCL = is.read_uint32()?;
                },
                88 => {
                    self.DKKLLMOHGFD = is.read_uint32()?;
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
        if self.NOPDKLDEKKF != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.NOPDKLDEKKF);
        }
        if self.FJNHDHOHBCL != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.FJNHDHOHBCL);
        }
        if self.DKKLLMOHGFD != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.DKKLLMOHGFD);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.NOPDKLDEKKF != 0 {
            os.write_uint32(13, self.NOPDKLDEKKF)?;
        }
        if self.FJNHDHOHBCL != 0 {
            os.write_uint32(9, self.FJNHDHOHBCL)?;
        }
        if self.DKKLLMOHGFD != 0 {
            os.write_uint32(11, self.DKKLLMOHGFD)?;
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

    fn new() -> GetSingleRedDotParamGroupCsReq {
        GetSingleRedDotParamGroupCsReq::new()
    }

    fn clear(&mut self) {
        self.NOPDKLDEKKF = 0;
        self.FJNHDHOHBCL = 0;
        self.DKKLLMOHGFD = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetSingleRedDotParamGroupCsReq {
        static instance: GetSingleRedDotParamGroupCsReq = GetSingleRedDotParamGroupCsReq {
            NOPDKLDEKKF: 0,
            FJNHDHOHBCL: 0,
            DKKLLMOHGFD: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetSingleRedDotParamGroupCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetSingleRedDotParamGroupCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetSingleRedDotParamGroupCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetSingleRedDotParamGroupCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n$GetSingleRedDotParamGroupCsReq.proto\"\x86\x01\n\x1eGetSingleRedDotPa\
    ramGroupCsReq\x12\x20\n\x0bNOPDKLDEKKF\x18\r\x20\x01(\rR\x0bNOPDKLDEKKF\
    \x12\x20\n\x0bFJNHDHOHBCL\x18\t\x20\x01(\rR\x0bFJNHDHOHBCL\x12\x20\n\x0b\
    DKKLLMOHGFD\x18\x0b\x20\x01(\rR\x0bDKKLLMOHGFDb\x06proto3\
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
            messages.push(GetSingleRedDotParamGroupCsReq::generated_message_descriptor_data());
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
