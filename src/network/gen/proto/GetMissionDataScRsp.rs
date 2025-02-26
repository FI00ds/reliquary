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

//! Generated file from `GetMissionDataScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:GetMissionDataScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetMissionDataScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetMissionDataScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:GetMissionDataScRsp.OJOMOCGIAIC)
    pub OJOMOCGIAIC: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetMissionDataScRsp.NEBOJOCKGCL)
    pub NEBOJOCKGCL: ::std::vec::Vec<super::IOPMFBIAFJH::IOPMFBIAFJH>,
    // @@protoc_insertion_point(field:GetMissionDataScRsp.MMAOHNCJMOB)
    pub MMAOHNCJMOB: ::std::vec::Vec<super::BNPOGAMNNAJ::BNPOGAMNNAJ>,
    // @@protoc_insertion_point(field:GetMissionDataScRsp.LFLBIOPJFGE)
    pub LFLBIOPJFGE: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GetMissionDataScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetMissionDataScRsp {
    fn default() -> &'a GetMissionDataScRsp {
        <GetMissionDataScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetMissionDataScRsp {
    pub fn new() -> GetMissionDataScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetMissionDataScRsp| { &m.retcode },
            |m: &mut GetMissionDataScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "OJOMOCGIAIC",
            |m: &GetMissionDataScRsp| { &m.OJOMOCGIAIC },
            |m: &mut GetMissionDataScRsp| { &mut m.OJOMOCGIAIC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NEBOJOCKGCL",
            |m: &GetMissionDataScRsp| { &m.NEBOJOCKGCL },
            |m: &mut GetMissionDataScRsp| { &mut m.NEBOJOCKGCL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MMAOHNCJMOB",
            |m: &GetMissionDataScRsp| { &m.MMAOHNCJMOB },
            |m: &mut GetMissionDataScRsp| { &mut m.MMAOHNCJMOB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LFLBIOPJFGE",
            |m: &GetMissionDataScRsp| { &m.LFLBIOPJFGE },
            |m: &mut GetMissionDataScRsp| { &mut m.LFLBIOPJFGE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetMissionDataScRsp>(
            "GetMissionDataScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetMissionDataScRsp {
    const NAME: &'static str = "GetMissionDataScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.retcode = is.read_uint32()?;
                },
                42 => {
                    is.read_repeated_packed_uint32_into(&mut self.OJOMOCGIAIC)?;
                },
                40 => {
                    self.OJOMOCGIAIC.push(is.read_uint32()?);
                },
                122 => {
                    self.NEBOJOCKGCL.push(is.read_message()?);
                },
                50 => {
                    self.MMAOHNCJMOB.push(is.read_message()?);
                },
                32 => {
                    self.LFLBIOPJFGE = is.read_uint32()?;
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
            my_size += ::protobuf::rt::uint32_size(11, self.retcode);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(5, &self.OJOMOCGIAIC);
        for value in &self.NEBOJOCKGCL {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.MMAOHNCJMOB {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.LFLBIOPJFGE != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.LFLBIOPJFGE);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.retcode != 0 {
            os.write_uint32(11, self.retcode)?;
        }
        os.write_repeated_packed_uint32(5, &self.OJOMOCGIAIC)?;
        for v in &self.NEBOJOCKGCL {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        };
        for v in &self.MMAOHNCJMOB {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        };
        if self.LFLBIOPJFGE != 0 {
            os.write_uint32(4, self.LFLBIOPJFGE)?;
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

    fn new() -> GetMissionDataScRsp {
        GetMissionDataScRsp::new()
    }

    fn clear(&mut self) {
        self.retcode = 0;
        self.OJOMOCGIAIC.clear();
        self.NEBOJOCKGCL.clear();
        self.MMAOHNCJMOB.clear();
        self.LFLBIOPJFGE = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetMissionDataScRsp {
        static instance: GetMissionDataScRsp = GetMissionDataScRsp {
            retcode: 0,
            OJOMOCGIAIC: ::std::vec::Vec::new(),
            NEBOJOCKGCL: ::std::vec::Vec::new(),
            MMAOHNCJMOB: ::std::vec::Vec::new(),
            LFLBIOPJFGE: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetMissionDataScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetMissionDataScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetMissionDataScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetMissionDataScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19GetMissionDataScRsp.proto\x1a\x11BNPOGAMNNAJ.proto\x1a\x11IOPMFBIA\
    FJH.proto\"\xd3\x01\n\x13GetMissionDataScRsp\x12\x18\n\x07retcode\x18\
    \x0b\x20\x01(\rR\x07retcode\x12\x20\n\x0bOJOMOCGIAIC\x18\x05\x20\x03(\rR\
    \x0bOJOMOCGIAIC\x12.\n\x0bNEBOJOCKGCL\x18\x0f\x20\x03(\x0b2\x0c.IOPMFBIA\
    FJHR\x0bNEBOJOCKGCL\x12.\n\x0bMMAOHNCJMOB\x18\x06\x20\x03(\x0b2\x0c.BNPO\
    GAMNNAJR\x0bMMAOHNCJMOB\x12\x20\n\x0bLFLBIOPJFGE\x18\x04\x20\x01(\rR\x0b\
    LFLBIOPJFGEb\x06proto3\
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
            deps.push(super::BNPOGAMNNAJ::file_descriptor().clone());
            deps.push(super::IOPMFBIAFJH::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetMissionDataScRsp::generated_message_descriptor_data());
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
