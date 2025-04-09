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

//! Generated file from `GetRogueShopFormulaInfoScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:GetRogueShopFormulaInfoScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetRogueShopFormulaInfoScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetRogueShopFormulaInfoScRsp.CKAANMDDKCJ)
    pub CKAANMDDKCJ: ::protobuf::MessageField<super::GLPPDLECCLI::GLPPDLECCLI>,
    // @@protoc_insertion_point(field:GetRogueShopFormulaInfoScRsp.AEFHKANBFNC)
    pub AEFHKANBFNC: i32,
    // @@protoc_insertion_point(field:GetRogueShopFormulaInfoScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:GetRogueShopFormulaInfoScRsp.IHJHCCFMIFD)
    pub IHJHCCFMIFD: ::protobuf::MessageField<super::KNAPAIOFJIE::KNAPAIOFJIE>,
    // @@protoc_insertion_point(field:GetRogueShopFormulaInfoScRsp.EFOJOCFGIDJ)
    pub EFOJOCFGIDJ: i32,
    // special fields
    // @@protoc_insertion_point(special_field:GetRogueShopFormulaInfoScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetRogueShopFormulaInfoScRsp {
    fn default() -> &'a GetRogueShopFormulaInfoScRsp {
        <GetRogueShopFormulaInfoScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetRogueShopFormulaInfoScRsp {
    pub fn new() -> GetRogueShopFormulaInfoScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::GLPPDLECCLI::GLPPDLECCLI>(
            "CKAANMDDKCJ",
            |m: &GetRogueShopFormulaInfoScRsp| { &m.CKAANMDDKCJ },
            |m: &mut GetRogueShopFormulaInfoScRsp| { &mut m.CKAANMDDKCJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AEFHKANBFNC",
            |m: &GetRogueShopFormulaInfoScRsp| { &m.AEFHKANBFNC },
            |m: &mut GetRogueShopFormulaInfoScRsp| { &mut m.AEFHKANBFNC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetRogueShopFormulaInfoScRsp| { &m.retcode },
            |m: &mut GetRogueShopFormulaInfoScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::KNAPAIOFJIE::KNAPAIOFJIE>(
            "IHJHCCFMIFD",
            |m: &GetRogueShopFormulaInfoScRsp| { &m.IHJHCCFMIFD },
            |m: &mut GetRogueShopFormulaInfoScRsp| { &mut m.IHJHCCFMIFD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EFOJOCFGIDJ",
            |m: &GetRogueShopFormulaInfoScRsp| { &m.EFOJOCFGIDJ },
            |m: &mut GetRogueShopFormulaInfoScRsp| { &mut m.EFOJOCFGIDJ },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetRogueShopFormulaInfoScRsp>(
            "GetRogueShopFormulaInfoScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetRogueShopFormulaInfoScRsp {
    const NAME: &'static str = "GetRogueShopFormulaInfoScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.CKAANMDDKCJ)?;
                },
                96 => {
                    self.AEFHKANBFNC = is.read_int32()?;
                },
                64 => {
                    self.retcode = is.read_uint32()?;
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.IHJHCCFMIFD)?;
                },
                56 => {
                    self.EFOJOCFGIDJ = is.read_int32()?;
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
        if let Some(v) = self.CKAANMDDKCJ.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.AEFHKANBFNC != 0 {
            my_size += ::protobuf::rt::int32_size(12, self.AEFHKANBFNC);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.retcode);
        }
        if let Some(v) = self.IHJHCCFMIFD.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.EFOJOCFGIDJ != 0 {
            my_size += ::protobuf::rt::int32_size(7, self.EFOJOCFGIDJ);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.CKAANMDDKCJ.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        if self.AEFHKANBFNC != 0 {
            os.write_int32(12, self.AEFHKANBFNC)?;
        }
        if self.retcode != 0 {
            os.write_uint32(8, self.retcode)?;
        }
        if let Some(v) = self.IHJHCCFMIFD.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if self.EFOJOCFGIDJ != 0 {
            os.write_int32(7, self.EFOJOCFGIDJ)?;
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

    fn new() -> GetRogueShopFormulaInfoScRsp {
        GetRogueShopFormulaInfoScRsp::new()
    }

    fn clear(&mut self) {
        self.CKAANMDDKCJ.clear();
        self.AEFHKANBFNC = 0;
        self.retcode = 0;
        self.IHJHCCFMIFD.clear();
        self.EFOJOCFGIDJ = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetRogueShopFormulaInfoScRsp {
        static instance: GetRogueShopFormulaInfoScRsp = GetRogueShopFormulaInfoScRsp {
            CKAANMDDKCJ: ::protobuf::MessageField::none(),
            AEFHKANBFNC: 0,
            retcode: 0,
            IHJHCCFMIFD: ::protobuf::MessageField::none(),
            EFOJOCFGIDJ: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetRogueShopFormulaInfoScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetRogueShopFormulaInfoScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetRogueShopFormulaInfoScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetRogueShopFormulaInfoScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"GetRogueShopFormulaInfoScRsp.proto\x1a\x11GLPPDLECCLI.proto\x1a\x11K\
    NAPAIOFJIE.proto\"\xdc\x01\n\x1cGetRogueShopFormulaInfoScRsp\x12.\n\x0bC\
    KAANMDDKCJ\x18\x0e\x20\x01(\x0b2\x0c.GLPPDLECCLIR\x0bCKAANMDDKCJ\x12\x20\
    \n\x0bAEFHKANBFNC\x18\x0c\x20\x01(\x05R\x0bAEFHKANBFNC\x12\x18\n\x07retc\
    ode\x18\x08\x20\x01(\rR\x07retcode\x12.\n\x0bIHJHCCFMIFD\x18\r\x20\x01(\
    \x0b2\x0c.KNAPAIOFJIER\x0bIHJHCCFMIFD\x12\x20\n\x0bEFOJOCFGIDJ\x18\x07\
    \x20\x01(\x05R\x0bEFOJOCFGIDJb\x06proto3\
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
            deps.push(super::GLPPDLECCLI::file_descriptor().clone());
            deps.push(super::KNAPAIOFJIE::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetRogueShopFormulaInfoScRsp::generated_message_descriptor_data());
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
