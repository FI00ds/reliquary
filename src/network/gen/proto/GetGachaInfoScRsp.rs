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

//! Generated file from `GetGachaInfoScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GetGachaInfoScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetGachaInfoScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetGachaInfoScRsp.AKHOAANIFFN)
    pub AKHOAANIFFN: u32,
    // @@protoc_insertion_point(field:GetGachaInfoScRsp.ELGFKKDKOGM)
    pub ELGFKKDKOGM: ::std::vec::Vec<super::HBIOKHLMONF::HBIOKHLMONF>,
    // @@protoc_insertion_point(field:GetGachaInfoScRsp.EJOFFJNOMLA)
    pub EJOFFJNOMLA: u32,
    // @@protoc_insertion_point(field:GetGachaInfoScRsp.OFDKDOIDAHB)
    pub OFDKDOIDAHB: u32,
    // @@protoc_insertion_point(field:GetGachaInfoScRsp.NDJAGNBNBGI)
    pub NDJAGNBNBGI: u32,
    // @@protoc_insertion_point(field:GetGachaInfoScRsp.retcode)
    pub retcode: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GetGachaInfoScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetGachaInfoScRsp {
    fn default() -> &'a GetGachaInfoScRsp {
        <GetGachaInfoScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetGachaInfoScRsp {
    pub fn new() -> GetGachaInfoScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AKHOAANIFFN",
            |m: &GetGachaInfoScRsp| { &m.AKHOAANIFFN },
            |m: &mut GetGachaInfoScRsp| { &mut m.AKHOAANIFFN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "ELGFKKDKOGM",
            |m: &GetGachaInfoScRsp| { &m.ELGFKKDKOGM },
            |m: &mut GetGachaInfoScRsp| { &mut m.ELGFKKDKOGM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EJOFFJNOMLA",
            |m: &GetGachaInfoScRsp| { &m.EJOFFJNOMLA },
            |m: &mut GetGachaInfoScRsp| { &mut m.EJOFFJNOMLA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OFDKDOIDAHB",
            |m: &GetGachaInfoScRsp| { &m.OFDKDOIDAHB },
            |m: &mut GetGachaInfoScRsp| { &mut m.OFDKDOIDAHB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NDJAGNBNBGI",
            |m: &GetGachaInfoScRsp| { &m.NDJAGNBNBGI },
            |m: &mut GetGachaInfoScRsp| { &mut m.NDJAGNBNBGI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetGachaInfoScRsp| { &m.retcode },
            |m: &mut GetGachaInfoScRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetGachaInfoScRsp>(
            "GetGachaInfoScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetGachaInfoScRsp {
    const NAME: &'static str = "GetGachaInfoScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.AKHOAANIFFN = is.read_uint32()?;
                },
                66 => {
                    self.ELGFKKDKOGM.push(is.read_message()?);
                },
                104 => {
                    self.EJOFFJNOMLA = is.read_uint32()?;
                },
                24 => {
                    self.OFDKDOIDAHB = is.read_uint32()?;
                },
                40 => {
                    self.NDJAGNBNBGI = is.read_uint32()?;
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
        if self.AKHOAANIFFN != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.AKHOAANIFFN);
        }
        for value in &self.ELGFKKDKOGM {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.EJOFFJNOMLA != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.EJOFFJNOMLA);
        }
        if self.OFDKDOIDAHB != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.OFDKDOIDAHB);
        }
        if self.NDJAGNBNBGI != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.NDJAGNBNBGI);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.AKHOAANIFFN != 0 {
            os.write_uint32(11, self.AKHOAANIFFN)?;
        }
        for v in &self.ELGFKKDKOGM {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        if self.EJOFFJNOMLA != 0 {
            os.write_uint32(13, self.EJOFFJNOMLA)?;
        }
        if self.OFDKDOIDAHB != 0 {
            os.write_uint32(3, self.OFDKDOIDAHB)?;
        }
        if self.NDJAGNBNBGI != 0 {
            os.write_uint32(5, self.NDJAGNBNBGI)?;
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

    fn new() -> GetGachaInfoScRsp {
        GetGachaInfoScRsp::new()
    }

    fn clear(&mut self) {
        self.AKHOAANIFFN = 0;
        self.ELGFKKDKOGM.clear();
        self.EJOFFJNOMLA = 0;
        self.OFDKDOIDAHB = 0;
        self.NDJAGNBNBGI = 0;
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetGachaInfoScRsp {
        static instance: GetGachaInfoScRsp = GetGachaInfoScRsp {
            AKHOAANIFFN: 0,
            ELGFKKDKOGM: ::std::vec::Vec::new(),
            EJOFFJNOMLA: 0,
            OFDKDOIDAHB: 0,
            NDJAGNBNBGI: 0,
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetGachaInfoScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetGachaInfoScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetGachaInfoScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetGachaInfoScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17GetGachaInfoScRsp.proto\x1a\x11HBIOKHLMONF.proto\"\xe5\x01\n\x11Ge\
    tGachaInfoScRsp\x12\x20\n\x0bAKHOAANIFFN\x18\x0b\x20\x01(\rR\x0bAKHOAANI\
    FFN\x12.\n\x0bELGFKKDKOGM\x18\x08\x20\x03(\x0b2\x0c.HBIOKHLMONFR\x0bELGF\
    KKDKOGM\x12\x20\n\x0bEJOFFJNOMLA\x18\r\x20\x01(\rR\x0bEJOFFJNOMLA\x12\
    \x20\n\x0bOFDKDOIDAHB\x18\x03\x20\x01(\rR\x0bOFDKDOIDAHB\x12\x20\n\x0bND\
    JAGNBNBGI\x18\x05\x20\x01(\rR\x0bNDJAGNBNBGI\x12\x18\n\x07retcode\x18\
    \x06\x20\x01(\rR\x07retcodeb\x06proto3\
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
            deps.push(super::HBIOKHLMONF::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetGachaInfoScRsp::generated_message_descriptor_data());
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
