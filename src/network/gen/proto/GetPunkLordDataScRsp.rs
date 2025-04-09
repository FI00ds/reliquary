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

//! Generated file from `GetPunkLordDataScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:GetPunkLordDataScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetPunkLordDataScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetPunkLordDataScRsp.GBJODJCOLGA)
    pub GBJODJCOLGA: u32,
    // @@protoc_insertion_point(field:GetPunkLordDataScRsp.GNLMKKHAEKM)
    pub GNLMKKHAEKM: u32,
    // @@protoc_insertion_point(field:GetPunkLordDataScRsp.BEDJDEANCOJ)
    pub BEDJDEANCOJ: i64,
    // @@protoc_insertion_point(field:GetPunkLordDataScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:GetPunkLordDataScRsp.BDPBDGBKDOO)
    pub BDPBDGBKDOO: u32,
    // @@protoc_insertion_point(field:GetPunkLordDataScRsp.PPNKPNBIIEN)
    pub PPNKPNBIIEN: u32,
    // @@protoc_insertion_point(field:GetPunkLordDataScRsp.EAHBIKFALLF)
    pub EAHBIKFALLF: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetPunkLordDataScRsp.IADCOHODGJN)
    pub IADCOHODGJN: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GetPunkLordDataScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetPunkLordDataScRsp {
    fn default() -> &'a GetPunkLordDataScRsp {
        <GetPunkLordDataScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetPunkLordDataScRsp {
    pub fn new() -> GetPunkLordDataScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GBJODJCOLGA",
            |m: &GetPunkLordDataScRsp| { &m.GBJODJCOLGA },
            |m: &mut GetPunkLordDataScRsp| { &mut m.GBJODJCOLGA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GNLMKKHAEKM",
            |m: &GetPunkLordDataScRsp| { &m.GNLMKKHAEKM },
            |m: &mut GetPunkLordDataScRsp| { &mut m.GNLMKKHAEKM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BEDJDEANCOJ",
            |m: &GetPunkLordDataScRsp| { &m.BEDJDEANCOJ },
            |m: &mut GetPunkLordDataScRsp| { &mut m.BEDJDEANCOJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetPunkLordDataScRsp| { &m.retcode },
            |m: &mut GetPunkLordDataScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BDPBDGBKDOO",
            |m: &GetPunkLordDataScRsp| { &m.BDPBDGBKDOO },
            |m: &mut GetPunkLordDataScRsp| { &mut m.BDPBDGBKDOO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PPNKPNBIIEN",
            |m: &GetPunkLordDataScRsp| { &m.PPNKPNBIIEN },
            |m: &mut GetPunkLordDataScRsp| { &mut m.PPNKPNBIIEN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EAHBIKFALLF",
            |m: &GetPunkLordDataScRsp| { &m.EAHBIKFALLF },
            |m: &mut GetPunkLordDataScRsp| { &mut m.EAHBIKFALLF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IADCOHODGJN",
            |m: &GetPunkLordDataScRsp| { &m.IADCOHODGJN },
            |m: &mut GetPunkLordDataScRsp| { &mut m.IADCOHODGJN },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetPunkLordDataScRsp>(
            "GetPunkLordDataScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetPunkLordDataScRsp {
    const NAME: &'static str = "GetPunkLordDataScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.GBJODJCOLGA = is.read_uint32()?;
                },
                48 => {
                    self.GNLMKKHAEKM = is.read_uint32()?;
                },
                16 => {
                    self.BEDJDEANCOJ = is.read_int64()?;
                },
                72 => {
                    self.retcode = is.read_uint32()?;
                },
                88 => {
                    self.BDPBDGBKDOO = is.read_uint32()?;
                },
                80 => {
                    self.PPNKPNBIIEN = is.read_uint32()?;
                },
                122 => {
                    is.read_repeated_packed_uint32_into(&mut self.EAHBIKFALLF)?;
                },
                120 => {
                    self.EAHBIKFALLF.push(is.read_uint32()?);
                },
                56 => {
                    self.IADCOHODGJN = is.read_uint32()?;
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
        if self.GBJODJCOLGA != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.GBJODJCOLGA);
        }
        if self.GNLMKKHAEKM != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.GNLMKKHAEKM);
        }
        if self.BEDJDEANCOJ != 0 {
            my_size += ::protobuf::rt::int64_size(2, self.BEDJDEANCOJ);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.retcode);
        }
        if self.BDPBDGBKDOO != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.BDPBDGBKDOO);
        }
        if self.PPNKPNBIIEN != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.PPNKPNBIIEN);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(15, &self.EAHBIKFALLF);
        if self.IADCOHODGJN != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.IADCOHODGJN);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.GBJODJCOLGA != 0 {
            os.write_uint32(1, self.GBJODJCOLGA)?;
        }
        if self.GNLMKKHAEKM != 0 {
            os.write_uint32(6, self.GNLMKKHAEKM)?;
        }
        if self.BEDJDEANCOJ != 0 {
            os.write_int64(2, self.BEDJDEANCOJ)?;
        }
        if self.retcode != 0 {
            os.write_uint32(9, self.retcode)?;
        }
        if self.BDPBDGBKDOO != 0 {
            os.write_uint32(11, self.BDPBDGBKDOO)?;
        }
        if self.PPNKPNBIIEN != 0 {
            os.write_uint32(10, self.PPNKPNBIIEN)?;
        }
        os.write_repeated_packed_uint32(15, &self.EAHBIKFALLF)?;
        if self.IADCOHODGJN != 0 {
            os.write_uint32(7, self.IADCOHODGJN)?;
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

    fn new() -> GetPunkLordDataScRsp {
        GetPunkLordDataScRsp::new()
    }

    fn clear(&mut self) {
        self.GBJODJCOLGA = 0;
        self.GNLMKKHAEKM = 0;
        self.BEDJDEANCOJ = 0;
        self.retcode = 0;
        self.BDPBDGBKDOO = 0;
        self.PPNKPNBIIEN = 0;
        self.EAHBIKFALLF.clear();
        self.IADCOHODGJN = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetPunkLordDataScRsp {
        static instance: GetPunkLordDataScRsp = GetPunkLordDataScRsp {
            GBJODJCOLGA: 0,
            GNLMKKHAEKM: 0,
            BEDJDEANCOJ: 0,
            retcode: 0,
            BDPBDGBKDOO: 0,
            PPNKPNBIIEN: 0,
            EAHBIKFALLF: ::std::vec::Vec::new(),
            IADCOHODGJN: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetPunkLordDataScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetPunkLordDataScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetPunkLordDataScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetPunkLordDataScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aGetPunkLordDataScRsp.proto\"\x9e\x02\n\x14GetPunkLordDataScRsp\x12\
    \x20\n\x0bGBJODJCOLGA\x18\x01\x20\x01(\rR\x0bGBJODJCOLGA\x12\x20\n\x0bGN\
    LMKKHAEKM\x18\x06\x20\x01(\rR\x0bGNLMKKHAEKM\x12\x20\n\x0bBEDJDEANCOJ\
    \x18\x02\x20\x01(\x03R\x0bBEDJDEANCOJ\x12\x18\n\x07retcode\x18\t\x20\x01\
    (\rR\x07retcode\x12\x20\n\x0bBDPBDGBKDOO\x18\x0b\x20\x01(\rR\x0bBDPBDGBK\
    DOO\x12\x20\n\x0bPPNKPNBIIEN\x18\n\x20\x01(\rR\x0bPPNKPNBIIEN\x12\x20\n\
    \x0bEAHBIKFALLF\x18\x0f\x20\x03(\rR\x0bEAHBIKFALLF\x12\x20\n\x0bIADCOHOD\
    GJN\x18\x07\x20\x01(\rR\x0bIADCOHODGJNb\x06proto3\
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
            messages.push(GetPunkLordDataScRsp::generated_message_descriptor_data());
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
