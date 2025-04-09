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

//! Generated file from `GetRogueShopMiracleInfoScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:GetRogueShopMiracleInfoScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetRogueShopMiracleInfoScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetRogueShopMiracleInfoScRsp.AEFHKANBFNC)
    pub AEFHKANBFNC: i32,
    // @@protoc_insertion_point(field:GetRogueShopMiracleInfoScRsp.IHJHCCFMIFD)
    pub IHJHCCFMIFD: ::protobuf::MessageField<super::KNAPAIOFJIE::KNAPAIOFJIE>,
    // @@protoc_insertion_point(field:GetRogueShopMiracleInfoScRsp.EFOJOCFGIDJ)
    pub EFOJOCFGIDJ: i32,
    // @@protoc_insertion_point(field:GetRogueShopMiracleInfoScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:GetRogueShopMiracleInfoScRsp.EEPPKMPAJOH)
    pub EEPPKMPAJOH: ::protobuf::MessageField<super::NNJOLKJLPJG::NNJOLKJLPJG>,
    // special fields
    // @@protoc_insertion_point(special_field:GetRogueShopMiracleInfoScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetRogueShopMiracleInfoScRsp {
    fn default() -> &'a GetRogueShopMiracleInfoScRsp {
        <GetRogueShopMiracleInfoScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetRogueShopMiracleInfoScRsp {
    pub fn new() -> GetRogueShopMiracleInfoScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AEFHKANBFNC",
            |m: &GetRogueShopMiracleInfoScRsp| { &m.AEFHKANBFNC },
            |m: &mut GetRogueShopMiracleInfoScRsp| { &mut m.AEFHKANBFNC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::KNAPAIOFJIE::KNAPAIOFJIE>(
            "IHJHCCFMIFD",
            |m: &GetRogueShopMiracleInfoScRsp| { &m.IHJHCCFMIFD },
            |m: &mut GetRogueShopMiracleInfoScRsp| { &mut m.IHJHCCFMIFD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EFOJOCFGIDJ",
            |m: &GetRogueShopMiracleInfoScRsp| { &m.EFOJOCFGIDJ },
            |m: &mut GetRogueShopMiracleInfoScRsp| { &mut m.EFOJOCFGIDJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetRogueShopMiracleInfoScRsp| { &m.retcode },
            |m: &mut GetRogueShopMiracleInfoScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::NNJOLKJLPJG::NNJOLKJLPJG>(
            "EEPPKMPAJOH",
            |m: &GetRogueShopMiracleInfoScRsp| { &m.EEPPKMPAJOH },
            |m: &mut GetRogueShopMiracleInfoScRsp| { &mut m.EEPPKMPAJOH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetRogueShopMiracleInfoScRsp>(
            "GetRogueShopMiracleInfoScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetRogueShopMiracleInfoScRsp {
    const NAME: &'static str = "GetRogueShopMiracleInfoScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.AEFHKANBFNC = is.read_int32()?;
                },
                98 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.IHJHCCFMIFD)?;
                },
                40 => {
                    self.EFOJOCFGIDJ = is.read_int32()?;
                },
                112 => {
                    self.retcode = is.read_uint32()?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.EEPPKMPAJOH)?;
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
        if self.AEFHKANBFNC != 0 {
            my_size += ::protobuf::rt::int32_size(8, self.AEFHKANBFNC);
        }
        if let Some(v) = self.IHJHCCFMIFD.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.EFOJOCFGIDJ != 0 {
            my_size += ::protobuf::rt::int32_size(5, self.EFOJOCFGIDJ);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.retcode);
        }
        if let Some(v) = self.EEPPKMPAJOH.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.AEFHKANBFNC != 0 {
            os.write_int32(8, self.AEFHKANBFNC)?;
        }
        if let Some(v) = self.IHJHCCFMIFD.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        }
        if self.EFOJOCFGIDJ != 0 {
            os.write_int32(5, self.EFOJOCFGIDJ)?;
        }
        if self.retcode != 0 {
            os.write_uint32(14, self.retcode)?;
        }
        if let Some(v) = self.EEPPKMPAJOH.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
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

    fn new() -> GetRogueShopMiracleInfoScRsp {
        GetRogueShopMiracleInfoScRsp::new()
    }

    fn clear(&mut self) {
        self.AEFHKANBFNC = 0;
        self.IHJHCCFMIFD.clear();
        self.EFOJOCFGIDJ = 0;
        self.retcode = 0;
        self.EEPPKMPAJOH.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetRogueShopMiracleInfoScRsp {
        static instance: GetRogueShopMiracleInfoScRsp = GetRogueShopMiracleInfoScRsp {
            AEFHKANBFNC: 0,
            IHJHCCFMIFD: ::protobuf::MessageField::none(),
            EFOJOCFGIDJ: 0,
            retcode: 0,
            EEPPKMPAJOH: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetRogueShopMiracleInfoScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetRogueShopMiracleInfoScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetRogueShopMiracleInfoScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetRogueShopMiracleInfoScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"GetRogueShopMiracleInfoScRsp.proto\x1a\x11KNAPAIOFJIE.proto\x1a\x11N\
    NJOLKJLPJG.proto\"\xdc\x01\n\x1cGetRogueShopMiracleInfoScRsp\x12\x20\n\
    \x0bAEFHKANBFNC\x18\x08\x20\x01(\x05R\x0bAEFHKANBFNC\x12.\n\x0bIHJHCCFMI\
    FD\x18\x0c\x20\x01(\x0b2\x0c.KNAPAIOFJIER\x0bIHJHCCFMIFD\x12\x20\n\x0bEF\
    OJOCFGIDJ\x18\x05\x20\x01(\x05R\x0bEFOJOCFGIDJ\x12\x18\n\x07retcode\x18\
    \x0e\x20\x01(\rR\x07retcode\x12.\n\x0bEEPPKMPAJOH\x18\t\x20\x01(\x0b2\
    \x0c.NNJOLKJLPJGR\x0bEEPPKMPAJOHb\x06proto3\
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
            deps.push(super::KNAPAIOFJIE::file_descriptor().clone());
            deps.push(super::NNJOLKJLPJG::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetRogueShopMiracleInfoScRsp::generated_message_descriptor_data());
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
