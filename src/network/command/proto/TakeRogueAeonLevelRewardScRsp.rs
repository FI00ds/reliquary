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

//! Generated file from `TakeRogueAeonLevelRewardScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:TakeRogueAeonLevelRewardScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TakeRogueAeonLevelRewardScRsp {
    // message fields
    // @@protoc_insertion_point(field:TakeRogueAeonLevelRewardScRsp.level)
    pub level: u32,
    // @@protoc_insertion_point(field:TakeRogueAeonLevelRewardScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:TakeRogueAeonLevelRewardScRsp.reward)
    pub reward: ::protobuf::MessageField<super::ItemList::ItemList>,
    // @@protoc_insertion_point(field:TakeRogueAeonLevelRewardScRsp.aeon_id)
    pub aeon_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:TakeRogueAeonLevelRewardScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TakeRogueAeonLevelRewardScRsp {
    fn default() -> &'a TakeRogueAeonLevelRewardScRsp {
        <TakeRogueAeonLevelRewardScRsp as ::protobuf::Message>::default_instance()
    }
}

impl TakeRogueAeonLevelRewardScRsp {
    pub fn new() -> TakeRogueAeonLevelRewardScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &TakeRogueAeonLevelRewardScRsp| { &m.level },
            |m: &mut TakeRogueAeonLevelRewardScRsp| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &TakeRogueAeonLevelRewardScRsp| { &m.retcode },
            |m: &mut TakeRogueAeonLevelRewardScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemList::ItemList>(
            "reward",
            |m: &TakeRogueAeonLevelRewardScRsp| { &m.reward },
            |m: &mut TakeRogueAeonLevelRewardScRsp| { &mut m.reward },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "aeon_id",
            |m: &TakeRogueAeonLevelRewardScRsp| { &m.aeon_id },
            |m: &mut TakeRogueAeonLevelRewardScRsp| { &mut m.aeon_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TakeRogueAeonLevelRewardScRsp>(
            "TakeRogueAeonLevelRewardScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TakeRogueAeonLevelRewardScRsp {
    const NAME: &'static str = "TakeRogueAeonLevelRewardScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                112 => {
                    self.level = is.read_uint32()?;
                },
                8 => {
                    self.retcode = is.read_uint32()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.reward)?;
                },
                88 => {
                    self.aeon_id = is.read_uint32()?;
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
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.level);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.retcode);
        }
        if let Some(v) = self.reward.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.aeon_id != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.aeon_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.level != 0 {
            os.write_uint32(14, self.level)?;
        }
        if self.retcode != 0 {
            os.write_uint32(1, self.retcode)?;
        }
        if let Some(v) = self.reward.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if self.aeon_id != 0 {
            os.write_uint32(11, self.aeon_id)?;
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

    fn new() -> TakeRogueAeonLevelRewardScRsp {
        TakeRogueAeonLevelRewardScRsp::new()
    }

    fn clear(&mut self) {
        self.level = 0;
        self.retcode = 0;
        self.reward.clear();
        self.aeon_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TakeRogueAeonLevelRewardScRsp {
        static instance: TakeRogueAeonLevelRewardScRsp = TakeRogueAeonLevelRewardScRsp {
            level: 0,
            retcode: 0,
            reward: ::protobuf::MessageField::none(),
            aeon_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TakeRogueAeonLevelRewardScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TakeRogueAeonLevelRewardScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TakeRogueAeonLevelRewardScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TakeRogueAeonLevelRewardScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n#TakeRogueAeonLevelRewardScRsp.proto\x1a\x0eItemList.proto\"\x8b\x01\n\
    \x1dTakeRogueAeonLevelRewardScRsp\x12\x14\n\x05level\x18\x0e\x20\x01(\rR\
    \x05level\x12\x18\n\x07retcode\x18\x01\x20\x01(\rR\x07retcode\x12!\n\x06\
    reward\x18\x02\x20\x01(\x0b2\t.ItemListR\x06reward\x12\x17\n\x07aeon_id\
    \x18\x0b\x20\x01(\rR\x06aeonIdb\x06proto3\
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
            deps.push(super::ItemList::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(TakeRogueAeonLevelRewardScRsp::generated_message_descriptor_data());
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
