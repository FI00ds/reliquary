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

//! Generated file from `GetAetherDivideChallengeInfoScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GetAetherDivideChallengeInfoScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetAetherDivideChallengeInfoScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetAetherDivideChallengeInfoScRsp.BFIBKFEBCNA)
    pub BFIBKFEBCNA: u32,
    // @@protoc_insertion_point(field:GetAetherDivideChallengeInfoScRsp.CMMMJEKGOJP)
    pub CMMMJEKGOJP: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetAetherDivideChallengeInfoScRsp.CIFLENFECFC)
    pub CIFLENFECFC: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetAetherDivideChallengeInfoScRsp.retcode)
    pub retcode: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GetAetherDivideChallengeInfoScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetAetherDivideChallengeInfoScRsp {
    fn default() -> &'a GetAetherDivideChallengeInfoScRsp {
        <GetAetherDivideChallengeInfoScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetAetherDivideChallengeInfoScRsp {
    pub fn new() -> GetAetherDivideChallengeInfoScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BFIBKFEBCNA",
            |m: &GetAetherDivideChallengeInfoScRsp| { &m.BFIBKFEBCNA },
            |m: &mut GetAetherDivideChallengeInfoScRsp| { &mut m.BFIBKFEBCNA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CMMMJEKGOJP",
            |m: &GetAetherDivideChallengeInfoScRsp| { &m.CMMMJEKGOJP },
            |m: &mut GetAetherDivideChallengeInfoScRsp| { &mut m.CMMMJEKGOJP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CIFLENFECFC",
            |m: &GetAetherDivideChallengeInfoScRsp| { &m.CIFLENFECFC },
            |m: &mut GetAetherDivideChallengeInfoScRsp| { &mut m.CIFLENFECFC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetAetherDivideChallengeInfoScRsp| { &m.retcode },
            |m: &mut GetAetherDivideChallengeInfoScRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetAetherDivideChallengeInfoScRsp>(
            "GetAetherDivideChallengeInfoScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetAetherDivideChallengeInfoScRsp {
    const NAME: &'static str = "GetAetherDivideChallengeInfoScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.BFIBKFEBCNA = is.read_uint32()?;
                },
                66 => {
                    is.read_repeated_packed_uint32_into(&mut self.CMMMJEKGOJP)?;
                },
                64 => {
                    self.CMMMJEKGOJP.push(is.read_uint32()?);
                },
                34 => {
                    is.read_repeated_packed_uint32_into(&mut self.CIFLENFECFC)?;
                },
                32 => {
                    self.CIFLENFECFC.push(is.read_uint32()?);
                },
                72 => {
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
        if self.BFIBKFEBCNA != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.BFIBKFEBCNA);
        }
        for value in &self.CMMMJEKGOJP {
            my_size += ::protobuf::rt::uint32_size(8, *value);
        };
        for value in &self.CIFLENFECFC {
            my_size += ::protobuf::rt::uint32_size(4, *value);
        };
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.BFIBKFEBCNA != 0 {
            os.write_uint32(13, self.BFIBKFEBCNA)?;
        }
        for v in &self.CMMMJEKGOJP {
            os.write_uint32(8, *v)?;
        };
        for v in &self.CIFLENFECFC {
            os.write_uint32(4, *v)?;
        };
        if self.retcode != 0 {
            os.write_uint32(9, self.retcode)?;
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

    fn new() -> GetAetherDivideChallengeInfoScRsp {
        GetAetherDivideChallengeInfoScRsp::new()
    }

    fn clear(&mut self) {
        self.BFIBKFEBCNA = 0;
        self.CMMMJEKGOJP.clear();
        self.CIFLENFECFC.clear();
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetAetherDivideChallengeInfoScRsp {
        static instance: GetAetherDivideChallengeInfoScRsp = GetAetherDivideChallengeInfoScRsp {
            BFIBKFEBCNA: 0,
            CMMMJEKGOJP: ::std::vec::Vec::new(),
            CIFLENFECFC: ::std::vec::Vec::new(),
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetAetherDivideChallengeInfoScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetAetherDivideChallengeInfoScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetAetherDivideChallengeInfoScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetAetherDivideChallengeInfoScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'GetAetherDivideChallengeInfoScRsp.proto\"\xa3\x01\n!GetAetherDivideCh\
    allengeInfoScRsp\x12\x20\n\x0bBFIBKFEBCNA\x18\r\x20\x01(\rR\x0bBFIBKFEBC\
    NA\x12\x20\n\x0bCMMMJEKGOJP\x18\x08\x20\x03(\rR\x0bCMMMJEKGOJP\x12\x20\n\
    \x0bCIFLENFECFC\x18\x04\x20\x03(\rR\x0bCIFLENFECFC\x12\x18\n\x07retcode\
    \x18\t\x20\x01(\rR\x07retcodeb\x06proto3\
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
            messages.push(GetAetherDivideChallengeInfoScRsp::generated_message_descriptor_data());
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
