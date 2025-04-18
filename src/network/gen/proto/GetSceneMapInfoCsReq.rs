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

//! Generated file from `GetSceneMapInfoCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:GetSceneMapInfoCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetSceneMapInfoCsReq {
    // message fields
    // @@protoc_insertion_point(field:GetSceneMapInfoCsReq.IGFIKGHLLNO)
    pub IGFIKGHLLNO: bool,
    // @@protoc_insertion_point(field:GetSceneMapInfoCsReq.LOLCMPAOJBG)
    pub LOLCMPAOJBG: u32,
    // @@protoc_insertion_point(field:GetSceneMapInfoCsReq.PPGEBHKKIIN)
    pub PPGEBHKKIIN: u32,
    // @@protoc_insertion_point(field:GetSceneMapInfoCsReq.MGGLDEJMDHL)
    pub MGGLDEJMDHL: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetSceneMapInfoCsReq.PICBDIMILIE)
    pub PICBDIMILIE: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:GetSceneMapInfoCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetSceneMapInfoCsReq {
    fn default() -> &'a GetSceneMapInfoCsReq {
        <GetSceneMapInfoCsReq as ::protobuf::Message>::default_instance()
    }
}

impl GetSceneMapInfoCsReq {
    pub fn new() -> GetSceneMapInfoCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IGFIKGHLLNO",
            |m: &GetSceneMapInfoCsReq| { &m.IGFIKGHLLNO },
            |m: &mut GetSceneMapInfoCsReq| { &mut m.IGFIKGHLLNO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LOLCMPAOJBG",
            |m: &GetSceneMapInfoCsReq| { &m.LOLCMPAOJBG },
            |m: &mut GetSceneMapInfoCsReq| { &mut m.LOLCMPAOJBG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PPGEBHKKIIN",
            |m: &GetSceneMapInfoCsReq| { &m.PPGEBHKKIIN },
            |m: &mut GetSceneMapInfoCsReq| { &mut m.PPGEBHKKIIN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MGGLDEJMDHL",
            |m: &GetSceneMapInfoCsReq| { &m.MGGLDEJMDHL },
            |m: &mut GetSceneMapInfoCsReq| { &mut m.MGGLDEJMDHL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PICBDIMILIE",
            |m: &GetSceneMapInfoCsReq| { &m.PICBDIMILIE },
            |m: &mut GetSceneMapInfoCsReq| { &mut m.PICBDIMILIE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetSceneMapInfoCsReq>(
            "GetSceneMapInfoCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetSceneMapInfoCsReq {
    const NAME: &'static str = "GetSceneMapInfoCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.IGFIKGHLLNO = is.read_bool()?;
                },
                16 => {
                    self.LOLCMPAOJBG = is.read_uint32()?;
                },
                80 => {
                    self.PPGEBHKKIIN = is.read_uint32()?;
                },
                34 => {
                    is.read_repeated_packed_uint32_into(&mut self.MGGLDEJMDHL)?;
                },
                32 => {
                    self.MGGLDEJMDHL.push(is.read_uint32()?);
                },
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.PICBDIMILIE)?;
                },
                8 => {
                    self.PICBDIMILIE.push(is.read_uint32()?);
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
        if self.LOLCMPAOJBG != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.LOLCMPAOJBG);
        }
        if self.PPGEBHKKIIN != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.PPGEBHKKIIN);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(4, &self.MGGLDEJMDHL);
        my_size += ::protobuf::rt::vec_packed_uint32_size(1, &self.PICBDIMILIE);
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IGFIKGHLLNO != false {
            os.write_bool(3, self.IGFIKGHLLNO)?;
        }
        if self.LOLCMPAOJBG != 0 {
            os.write_uint32(2, self.LOLCMPAOJBG)?;
        }
        if self.PPGEBHKKIIN != 0 {
            os.write_uint32(10, self.PPGEBHKKIIN)?;
        }
        os.write_repeated_packed_uint32(4, &self.MGGLDEJMDHL)?;
        os.write_repeated_packed_uint32(1, &self.PICBDIMILIE)?;
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> GetSceneMapInfoCsReq {
        GetSceneMapInfoCsReq::new()
    }

    fn clear(&mut self) {
        self.IGFIKGHLLNO = false;
        self.LOLCMPAOJBG = 0;
        self.PPGEBHKKIIN = 0;
        self.MGGLDEJMDHL.clear();
        self.PICBDIMILIE.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetSceneMapInfoCsReq {
        static instance: GetSceneMapInfoCsReq = GetSceneMapInfoCsReq {
            IGFIKGHLLNO: false,
            LOLCMPAOJBG: 0,
            PPGEBHKKIIN: 0,
            MGGLDEJMDHL: ::std::vec::Vec::new(),
            PICBDIMILIE: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetSceneMapInfoCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetSceneMapInfoCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetSceneMapInfoCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetSceneMapInfoCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aGetSceneMapInfoCsReq.proto\"\xc0\x01\n\x14GetSceneMapInfoCsReq\x12\
    \x20\n\x0bIGFIKGHLLNO\x18\x03\x20\x01(\x08R\x0bIGFIKGHLLNO\x12\x20\n\x0b\
    LOLCMPAOJBG\x18\x02\x20\x01(\rR\x0bLOLCMPAOJBG\x12\x20\n\x0bPPGEBHKKIIN\
    \x18\n\x20\x01(\rR\x0bPPGEBHKKIIN\x12\x20\n\x0bMGGLDEJMDHL\x18\x04\x20\
    \x03(\rR\x0bMGGLDEJMDHL\x12\x20\n\x0bPICBDIMILIE\x18\x01\x20\x03(\rR\x0b\
    PICBDIMILIEb\x06proto3\
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
            messages.push(GetSceneMapInfoCsReq::generated_message_descriptor_data());
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
