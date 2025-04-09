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

//! Generated file from `GetFriendChallengeDetailScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:GetFriendChallengeDetailScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetFriendChallengeDetailScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetFriendChallengeDetailScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:GetFriendChallengeDetailScRsp.CFAAFJJAADP)
    pub CFAAFJJAADP: u32,
    // @@protoc_insertion_point(field:GetFriendChallengeDetailScRsp.uid)
    pub uid: u32,
    // @@protoc_insertion_point(field:GetFriendChallengeDetailScRsp.CCGDMOOLHHB)
    pub CCGDMOOLHHB: ::std::vec::Vec<super::DisplayAvatarDetailInfo::DisplayAvatarDetailInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:GetFriendChallengeDetailScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetFriendChallengeDetailScRsp {
    fn default() -> &'a GetFriendChallengeDetailScRsp {
        <GetFriendChallengeDetailScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetFriendChallengeDetailScRsp {
    pub fn new() -> GetFriendChallengeDetailScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetFriendChallengeDetailScRsp| { &m.retcode },
            |m: &mut GetFriendChallengeDetailScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CFAAFJJAADP",
            |m: &GetFriendChallengeDetailScRsp| { &m.CFAAFJJAADP },
            |m: &mut GetFriendChallengeDetailScRsp| { &mut m.CFAAFJJAADP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "uid",
            |m: &GetFriendChallengeDetailScRsp| { &m.uid },
            |m: &mut GetFriendChallengeDetailScRsp| { &mut m.uid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CCGDMOOLHHB",
            |m: &GetFriendChallengeDetailScRsp| { &m.CCGDMOOLHHB },
            |m: &mut GetFriendChallengeDetailScRsp| { &mut m.CCGDMOOLHHB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetFriendChallengeDetailScRsp>(
            "GetFriendChallengeDetailScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetFriendChallengeDetailScRsp {
    const NAME: &'static str = "GetFriendChallengeDetailScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                40 => {
                    self.retcode = is.read_uint32()?;
                },
                64 => {
                    self.CFAAFJJAADP = is.read_uint32()?;
                },
                72 => {
                    self.uid = is.read_uint32()?;
                },
                50 => {
                    self.CCGDMOOLHHB.push(is.read_message()?);
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
            my_size += ::protobuf::rt::uint32_size(5, self.retcode);
        }
        if self.CFAAFJJAADP != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.CFAAFJJAADP);
        }
        if self.uid != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.uid);
        }
        for value in &self.CCGDMOOLHHB {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.retcode != 0 {
            os.write_uint32(5, self.retcode)?;
        }
        if self.CFAAFJJAADP != 0 {
            os.write_uint32(8, self.CFAAFJJAADP)?;
        }
        if self.uid != 0 {
            os.write_uint32(9, self.uid)?;
        }
        for v in &self.CCGDMOOLHHB {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
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

    fn new() -> GetFriendChallengeDetailScRsp {
        GetFriendChallengeDetailScRsp::new()
    }

    fn clear(&mut self) {
        self.retcode = 0;
        self.CFAAFJJAADP = 0;
        self.uid = 0;
        self.CCGDMOOLHHB.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetFriendChallengeDetailScRsp {
        static instance: GetFriendChallengeDetailScRsp = GetFriendChallengeDetailScRsp {
            retcode: 0,
            CFAAFJJAADP: 0,
            uid: 0,
            CCGDMOOLHHB: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetFriendChallengeDetailScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetFriendChallengeDetailScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetFriendChallengeDetailScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetFriendChallengeDetailScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n#GetFriendChallengeDetailScRsp.proto\x1a\x1dDisplayAvatarDetailInfo.pr\
    oto\"\xa9\x01\n\x1dGetFriendChallengeDetailScRsp\x12\x18\n\x07retcode\
    \x18\x05\x20\x01(\rR\x07retcode\x12\x20\n\x0bCFAAFJJAADP\x18\x08\x20\x01\
    (\rR\x0bCFAAFJJAADP\x12\x10\n\x03uid\x18\t\x20\x01(\rR\x03uid\x12:\n\x0b\
    CCGDMOOLHHB\x18\x06\x20\x03(\x0b2\x18.DisplayAvatarDetailInfoR\x0bCCGDMO\
    OLHHBb\x06proto3\
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
            deps.push(super::DisplayAvatarDetailInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetFriendChallengeDetailScRsp::generated_message_descriptor_data());
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
