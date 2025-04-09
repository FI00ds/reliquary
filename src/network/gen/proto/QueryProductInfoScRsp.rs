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

//! Generated file from `QueryProductInfoScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:QueryProductInfoScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct QueryProductInfoScRsp {
    // message fields
    // @@protoc_insertion_point(field:QueryProductInfoScRsp.EPCFJNOHJCL)
    pub EPCFJNOHJCL: u64,
    // @@protoc_insertion_point(field:QueryProductInfoScRsp.PIDHEGPBCJI)
    pub PIDHEGPBCJI: ::std::vec::Vec<super::AAPNHPAMDCK::AAPNHPAMDCK>,
    // @@protoc_insertion_point(field:QueryProductInfoScRsp.CMGHDMPEAKO)
    pub CMGHDMPEAKO: u32,
    // @@protoc_insertion_point(field:QueryProductInfoScRsp.ODNFIAAHKCI)
    pub ODNFIAAHKCI: u32,
    // @@protoc_insertion_point(field:QueryProductInfoScRsp.retcode)
    pub retcode: u32,
    // special fields
    // @@protoc_insertion_point(special_field:QueryProductInfoScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a QueryProductInfoScRsp {
    fn default() -> &'a QueryProductInfoScRsp {
        <QueryProductInfoScRsp as ::protobuf::Message>::default_instance()
    }
}

impl QueryProductInfoScRsp {
    pub fn new() -> QueryProductInfoScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EPCFJNOHJCL",
            |m: &QueryProductInfoScRsp| { &m.EPCFJNOHJCL },
            |m: &mut QueryProductInfoScRsp| { &mut m.EPCFJNOHJCL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PIDHEGPBCJI",
            |m: &QueryProductInfoScRsp| { &m.PIDHEGPBCJI },
            |m: &mut QueryProductInfoScRsp| { &mut m.PIDHEGPBCJI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CMGHDMPEAKO",
            |m: &QueryProductInfoScRsp| { &m.CMGHDMPEAKO },
            |m: &mut QueryProductInfoScRsp| { &mut m.CMGHDMPEAKO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ODNFIAAHKCI",
            |m: &QueryProductInfoScRsp| { &m.ODNFIAAHKCI },
            |m: &mut QueryProductInfoScRsp| { &mut m.ODNFIAAHKCI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &QueryProductInfoScRsp| { &m.retcode },
            |m: &mut QueryProductInfoScRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<QueryProductInfoScRsp>(
            "QueryProductInfoScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for QueryProductInfoScRsp {
    const NAME: &'static str = "QueryProductInfoScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.EPCFJNOHJCL = is.read_uint64()?;
                },
                82 => {
                    self.PIDHEGPBCJI.push(is.read_message()?);
                },
                104 => {
                    self.CMGHDMPEAKO = is.read_uint32()?;
                },
                8 => {
                    self.ODNFIAAHKCI = is.read_uint32()?;
                },
                24 => {
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
        if self.EPCFJNOHJCL != 0 {
            my_size += ::protobuf::rt::uint64_size(8, self.EPCFJNOHJCL);
        }
        for value in &self.PIDHEGPBCJI {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.CMGHDMPEAKO != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.CMGHDMPEAKO);
        }
        if self.ODNFIAAHKCI != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.ODNFIAAHKCI);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.EPCFJNOHJCL != 0 {
            os.write_uint64(8, self.EPCFJNOHJCL)?;
        }
        for v in &self.PIDHEGPBCJI {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        };
        if self.CMGHDMPEAKO != 0 {
            os.write_uint32(13, self.CMGHDMPEAKO)?;
        }
        if self.ODNFIAAHKCI != 0 {
            os.write_uint32(1, self.ODNFIAAHKCI)?;
        }
        if self.retcode != 0 {
            os.write_uint32(3, self.retcode)?;
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

    fn new() -> QueryProductInfoScRsp {
        QueryProductInfoScRsp::new()
    }

    fn clear(&mut self) {
        self.EPCFJNOHJCL = 0;
        self.PIDHEGPBCJI.clear();
        self.CMGHDMPEAKO = 0;
        self.ODNFIAAHKCI = 0;
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static QueryProductInfoScRsp {
        static instance: QueryProductInfoScRsp = QueryProductInfoScRsp {
            EPCFJNOHJCL: 0,
            PIDHEGPBCJI: ::std::vec::Vec::new(),
            CMGHDMPEAKO: 0,
            ODNFIAAHKCI: 0,
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for QueryProductInfoScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("QueryProductInfoScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for QueryProductInfoScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueryProductInfoScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bQueryProductInfoScRsp.proto\x1a\x11AAPNHPAMDCK.proto\"\xc7\x01\n\
    \x15QueryProductInfoScRsp\x12\x20\n\x0bEPCFJNOHJCL\x18\x08\x20\x01(\x04R\
    \x0bEPCFJNOHJCL\x12.\n\x0bPIDHEGPBCJI\x18\n\x20\x03(\x0b2\x0c.AAPNHPAMDC\
    KR\x0bPIDHEGPBCJI\x12\x20\n\x0bCMGHDMPEAKO\x18\r\x20\x01(\rR\x0bCMGHDMPE\
    AKO\x12\x20\n\x0bODNFIAAHKCI\x18\x01\x20\x01(\rR\x0bODNFIAAHKCI\x12\x18\
    \n\x07retcode\x18\x03\x20\x01(\rR\x07retcodeb\x06proto3\
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
            deps.push(super::AAPNHPAMDCK::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(QueryProductInfoScRsp::generated_message_descriptor_data());
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
