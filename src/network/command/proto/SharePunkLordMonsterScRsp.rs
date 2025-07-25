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

//! Generated file from `SharePunkLordMonsterScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:SharePunkLordMonsterScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SharePunkLordMonsterScRsp {
    // message fields
    // @@protoc_insertion_point(field:SharePunkLordMonsterScRsp.uid)
    pub uid: u32,
    // @@protoc_insertion_point(field:SharePunkLordMonsterScRsp.monster_id)
    pub monster_id: u32,
    // @@protoc_insertion_point(field:SharePunkLordMonsterScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:SharePunkLordMonsterScRsp.share_type)
    pub share_type: ::protobuf::EnumOrUnknown<super::PunkLordShareType::PunkLordShareType>,
    // special fields
    // @@protoc_insertion_point(special_field:SharePunkLordMonsterScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SharePunkLordMonsterScRsp {
    fn default() -> &'a SharePunkLordMonsterScRsp {
        <SharePunkLordMonsterScRsp as ::protobuf::Message>::default_instance()
    }
}

impl SharePunkLordMonsterScRsp {
    pub fn new() -> SharePunkLordMonsterScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "uid",
            |m: &SharePunkLordMonsterScRsp| { &m.uid },
            |m: &mut SharePunkLordMonsterScRsp| { &mut m.uid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "monster_id",
            |m: &SharePunkLordMonsterScRsp| { &m.monster_id },
            |m: &mut SharePunkLordMonsterScRsp| { &mut m.monster_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &SharePunkLordMonsterScRsp| { &m.retcode },
            |m: &mut SharePunkLordMonsterScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "share_type",
            |m: &SharePunkLordMonsterScRsp| { &m.share_type },
            |m: &mut SharePunkLordMonsterScRsp| { &mut m.share_type },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SharePunkLordMonsterScRsp>(
            "SharePunkLordMonsterScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SharePunkLordMonsterScRsp {
    const NAME: &'static str = "SharePunkLordMonsterScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                112 => {
                    self.uid = is.read_uint32()?;
                },
                48 => {
                    self.monster_id = is.read_uint32()?;
                },
                88 => {
                    self.retcode = is.read_uint32()?;
                },
                24 => {
                    self.share_type = is.read_enum_or_unknown()?;
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
        if self.uid != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.uid);
        }
        if self.monster_id != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.monster_id);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.retcode);
        }
        if self.share_type != ::protobuf::EnumOrUnknown::new(super::PunkLordShareType::PunkLordShareType::PUNK_LORD_SHARE_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(3, self.share_type.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.uid != 0 {
            os.write_uint32(14, self.uid)?;
        }
        if self.monster_id != 0 {
            os.write_uint32(6, self.monster_id)?;
        }
        if self.retcode != 0 {
            os.write_uint32(11, self.retcode)?;
        }
        if self.share_type != ::protobuf::EnumOrUnknown::new(super::PunkLordShareType::PunkLordShareType::PUNK_LORD_SHARE_TYPE_NONE) {
            os.write_enum(3, ::protobuf::EnumOrUnknown::value(&self.share_type))?;
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

    fn new() -> SharePunkLordMonsterScRsp {
        SharePunkLordMonsterScRsp::new()
    }

    fn clear(&mut self) {
        self.uid = 0;
        self.monster_id = 0;
        self.retcode = 0;
        self.share_type = ::protobuf::EnumOrUnknown::new(super::PunkLordShareType::PunkLordShareType::PUNK_LORD_SHARE_TYPE_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SharePunkLordMonsterScRsp {
        static instance: SharePunkLordMonsterScRsp = SharePunkLordMonsterScRsp {
            uid: 0,
            monster_id: 0,
            retcode: 0,
            share_type: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SharePunkLordMonsterScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SharePunkLordMonsterScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SharePunkLordMonsterScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SharePunkLordMonsterScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1fSharePunkLordMonsterScRsp.proto\x1a\x17PunkLordShareType.proto\"\
    \x99\x01\n\x19SharePunkLordMonsterScRsp\x12\x10\n\x03uid\x18\x0e\x20\x01\
    (\rR\x03uid\x12\x1d\n\nmonster_id\x18\x06\x20\x01(\rR\tmonsterId\x12\x18\
    \n\x07retcode\x18\x0b\x20\x01(\rR\x07retcode\x121\n\nshare_type\x18\x03\
    \x20\x01(\x0e2\x12.PunkLordShareTypeR\tshareTypeb\x06proto3\
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
            deps.push(super::PunkLordShareType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SharePunkLordMonsterScRsp::generated_message_descriptor_data());
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
