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

//! Generated file from `GetFightFestDataScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:GetFightFestDataScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetFightFestDataScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetFightFestDataScRsp.HFMLNLGIGHB)
    pub HFMLNLGIGHB: ::std::vec::Vec<super::IKLNILKPENA::IKLNILKPENA>,
    // @@protoc_insertion_point(field:GetFightFestDataScRsp.MFGONHJGIPP)
    pub MFGONHJGIPP: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetFightFestDataScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:GetFightFestDataScRsp.CPIOGJKFECH)
    pub CPIOGJKFECH: u32,
    // @@protoc_insertion_point(field:GetFightFestDataScRsp.DNPHCJEBIKB)
    pub DNPHCJEBIKB: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GetFightFestDataScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetFightFestDataScRsp {
    fn default() -> &'a GetFightFestDataScRsp {
        <GetFightFestDataScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetFightFestDataScRsp {
    pub fn new() -> GetFightFestDataScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HFMLNLGIGHB",
            |m: &GetFightFestDataScRsp| { &m.HFMLNLGIGHB },
            |m: &mut GetFightFestDataScRsp| { &mut m.HFMLNLGIGHB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MFGONHJGIPP",
            |m: &GetFightFestDataScRsp| { &m.MFGONHJGIPP },
            |m: &mut GetFightFestDataScRsp| { &mut m.MFGONHJGIPP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetFightFestDataScRsp| { &m.retcode },
            |m: &mut GetFightFestDataScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CPIOGJKFECH",
            |m: &GetFightFestDataScRsp| { &m.CPIOGJKFECH },
            |m: &mut GetFightFestDataScRsp| { &mut m.CPIOGJKFECH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DNPHCJEBIKB",
            |m: &GetFightFestDataScRsp| { &m.DNPHCJEBIKB },
            |m: &mut GetFightFestDataScRsp| { &mut m.DNPHCJEBIKB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetFightFestDataScRsp>(
            "GetFightFestDataScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetFightFestDataScRsp {
    const NAME: &'static str = "GetFightFestDataScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                82 => {
                    self.HFMLNLGIGHB.push(is.read_message()?);
                },
                74 => {
                    is.read_repeated_packed_uint32_into(&mut self.MFGONHJGIPP)?;
                },
                72 => {
                    self.MFGONHJGIPP.push(is.read_uint32()?);
                },
                32 => {
                    self.retcode = is.read_uint32()?;
                },
                120 => {
                    self.CPIOGJKFECH = is.read_uint32()?;
                },
                24 => {
                    self.DNPHCJEBIKB = is.read_uint32()?;
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
        for value in &self.HFMLNLGIGHB {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::vec_packed_uint32_size(9, &self.MFGONHJGIPP);
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.retcode);
        }
        if self.CPIOGJKFECH != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.CPIOGJKFECH);
        }
        if self.DNPHCJEBIKB != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.DNPHCJEBIKB);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.HFMLNLGIGHB {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        };
        os.write_repeated_packed_uint32(9, &self.MFGONHJGIPP)?;
        if self.retcode != 0 {
            os.write_uint32(4, self.retcode)?;
        }
        if self.CPIOGJKFECH != 0 {
            os.write_uint32(15, self.CPIOGJKFECH)?;
        }
        if self.DNPHCJEBIKB != 0 {
            os.write_uint32(3, self.DNPHCJEBIKB)?;
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

    fn new() -> GetFightFestDataScRsp {
        GetFightFestDataScRsp::new()
    }

    fn clear(&mut self) {
        self.HFMLNLGIGHB.clear();
        self.MFGONHJGIPP.clear();
        self.retcode = 0;
        self.CPIOGJKFECH = 0;
        self.DNPHCJEBIKB = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetFightFestDataScRsp {
        static instance: GetFightFestDataScRsp = GetFightFestDataScRsp {
            HFMLNLGIGHB: ::std::vec::Vec::new(),
            MFGONHJGIPP: ::std::vec::Vec::new(),
            retcode: 0,
            CPIOGJKFECH: 0,
            DNPHCJEBIKB: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetFightFestDataScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetFightFestDataScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetFightFestDataScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetFightFestDataScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bGetFightFestDataScRsp.proto\x1a\x11IKLNILKPENA.proto\"\xc7\x01\n\
    \x15GetFightFestDataScRsp\x12.\n\x0bHFMLNLGIGHB\x18\n\x20\x03(\x0b2\x0c.\
    IKLNILKPENAR\x0bHFMLNLGIGHB\x12\x20\n\x0bMFGONHJGIPP\x18\t\x20\x03(\rR\
    \x0bMFGONHJGIPP\x12\x18\n\x07retcode\x18\x04\x20\x01(\rR\x07retcode\x12\
    \x20\n\x0bCPIOGJKFECH\x18\x0f\x20\x01(\rR\x0bCPIOGJKFECH\x12\x20\n\x0bDN\
    PHCJEBIKB\x18\x03\x20\x01(\rR\x0bDNPHCJEBIKBb\x06proto3\
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
            deps.push(super::IKLNILKPENA::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetFightFestDataScRsp::generated_message_descriptor_data());
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
