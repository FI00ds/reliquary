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

//! Generated file from `GetSceneMapInfoScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:GetSceneMapInfoScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetSceneMapInfoScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetSceneMapInfoScRsp.IGFIKGHLLNO)
    pub IGFIKGHLLNO: bool,
    // @@protoc_insertion_point(field:GetSceneMapInfoScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:GetSceneMapInfoScRsp.LOLCMPAOJBG)
    pub LOLCMPAOJBG: u32,
    // @@protoc_insertion_point(field:GetSceneMapInfoScRsp.PPGEBHKKIIN)
    pub PPGEBHKKIIN: u32,
    // @@protoc_insertion_point(field:GetSceneMapInfoScRsp.NINBEFLEEMP)
    pub NINBEFLEEMP: ::std::vec::Vec<super::HCMFFDCMMPK::HCMFFDCMMPK>,
    // special fields
    // @@protoc_insertion_point(special_field:GetSceneMapInfoScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetSceneMapInfoScRsp {
    fn default() -> &'a GetSceneMapInfoScRsp {
        <GetSceneMapInfoScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetSceneMapInfoScRsp {
    pub fn new() -> GetSceneMapInfoScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IGFIKGHLLNO",
            |m: &GetSceneMapInfoScRsp| { &m.IGFIKGHLLNO },
            |m: &mut GetSceneMapInfoScRsp| { &mut m.IGFIKGHLLNO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetSceneMapInfoScRsp| { &m.retcode },
            |m: &mut GetSceneMapInfoScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LOLCMPAOJBG",
            |m: &GetSceneMapInfoScRsp| { &m.LOLCMPAOJBG },
            |m: &mut GetSceneMapInfoScRsp| { &mut m.LOLCMPAOJBG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PPGEBHKKIIN",
            |m: &GetSceneMapInfoScRsp| { &m.PPGEBHKKIIN },
            |m: &mut GetSceneMapInfoScRsp| { &mut m.PPGEBHKKIIN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NINBEFLEEMP",
            |m: &GetSceneMapInfoScRsp| { &m.NINBEFLEEMP },
            |m: &mut GetSceneMapInfoScRsp| { &mut m.NINBEFLEEMP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetSceneMapInfoScRsp>(
            "GetSceneMapInfoScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetSceneMapInfoScRsp {
    const NAME: &'static str = "GetSceneMapInfoScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.IGFIKGHLLNO = is.read_bool()?;
                },
                24 => {
                    self.retcode = is.read_uint32()?;
                },
                104 => {
                    self.LOLCMPAOJBG = is.read_uint32()?;
                },
                64 => {
                    self.PPGEBHKKIIN = is.read_uint32()?;
                },
                82 => {
                    self.NINBEFLEEMP.push(is.read_message()?);
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
        if self.IGFIKGHLLNO != false {
            my_size += 1 + 1;
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.retcode);
        }
        if self.LOLCMPAOJBG != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.LOLCMPAOJBG);
        }
        if self.PPGEBHKKIIN != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.PPGEBHKKIIN);
        }
        for value in &self.NINBEFLEEMP {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IGFIKGHLLNO != false {
            os.write_bool(1, self.IGFIKGHLLNO)?;
        }
        if self.retcode != 0 {
            os.write_uint32(3, self.retcode)?;
        }
        if self.LOLCMPAOJBG != 0 {
            os.write_uint32(13, self.LOLCMPAOJBG)?;
        }
        if self.PPGEBHKKIIN != 0 {
            os.write_uint32(8, self.PPGEBHKKIIN)?;
        }
        for v in &self.NINBEFLEEMP {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> GetSceneMapInfoScRsp {
        GetSceneMapInfoScRsp::new()
    }

    fn clear(&mut self) {
        self.IGFIKGHLLNO = false;
        self.retcode = 0;
        self.LOLCMPAOJBG = 0;
        self.PPGEBHKKIIN = 0;
        self.NINBEFLEEMP.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetSceneMapInfoScRsp {
        static instance: GetSceneMapInfoScRsp = GetSceneMapInfoScRsp {
            IGFIKGHLLNO: false,
            retcode: 0,
            LOLCMPAOJBG: 0,
            PPGEBHKKIIN: 0,
            NINBEFLEEMP: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetSceneMapInfoScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetSceneMapInfoScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetSceneMapInfoScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetSceneMapInfoScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aGetSceneMapInfoScRsp.proto\x1a\x11HCMFFDCMMPK.proto\"\xc6\x01\n\
    \x14GetSceneMapInfoScRsp\x12\x20\n\x0bIGFIKGHLLNO\x18\x01\x20\x01(\x08R\
    \x0bIGFIKGHLLNO\x12\x18\n\x07retcode\x18\x03\x20\x01(\rR\x07retcode\x12\
    \x20\n\x0bLOLCMPAOJBG\x18\r\x20\x01(\rR\x0bLOLCMPAOJBG\x12\x20\n\x0bPPGE\
    BHKKIIN\x18\x08\x20\x01(\rR\x0bPPGEBHKKIIN\x12.\n\x0bNINBEFLEEMP\x18\n\
    \x20\x03(\x0b2\x0c.HCMFFDCMMPKR\x0bNINBEFLEEMPb\x06proto3\
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
            deps.push(super::HCMFFDCMMPK::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetSceneMapInfoScRsp::generated_message_descriptor_data());
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
