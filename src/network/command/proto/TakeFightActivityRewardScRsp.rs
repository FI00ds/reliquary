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

//! Generated file from `TakeFightActivityRewardScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:TakeFightActivityRewardScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TakeFightActivityRewardScRsp {
    // message fields
    // @@protoc_insertion_point(field:TakeFightActivityRewardScRsp.NEDFIBONLKB)
    pub NEDFIBONLKB: u32,
    // @@protoc_insertion_point(field:TakeFightActivityRewardScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:TakeFightActivityRewardScRsp.reward)
    pub reward: ::protobuf::MessageField<super::ItemList::ItemList>,
    // @@protoc_insertion_point(field:TakeFightActivityRewardScRsp.group_id)
    pub group_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:TakeFightActivityRewardScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TakeFightActivityRewardScRsp {
    fn default() -> &'a TakeFightActivityRewardScRsp {
        <TakeFightActivityRewardScRsp as ::protobuf::Message>::default_instance()
    }
}

impl TakeFightActivityRewardScRsp {
    pub fn new() -> TakeFightActivityRewardScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NEDFIBONLKB",
            |m: &TakeFightActivityRewardScRsp| { &m.NEDFIBONLKB },
            |m: &mut TakeFightActivityRewardScRsp| { &mut m.NEDFIBONLKB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &TakeFightActivityRewardScRsp| { &m.retcode },
            |m: &mut TakeFightActivityRewardScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemList::ItemList>(
            "reward",
            |m: &TakeFightActivityRewardScRsp| { &m.reward },
            |m: &mut TakeFightActivityRewardScRsp| { &mut m.reward },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "group_id",
            |m: &TakeFightActivityRewardScRsp| { &m.group_id },
            |m: &mut TakeFightActivityRewardScRsp| { &mut m.group_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TakeFightActivityRewardScRsp>(
            "TakeFightActivityRewardScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TakeFightActivityRewardScRsp {
    const NAME: &'static str = "TakeFightActivityRewardScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.NEDFIBONLKB = is.read_uint32()?;
                },
                48 => {
                    self.retcode = is.read_uint32()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.reward)?;
                },
                40 => {
                    self.group_id = is.read_uint32()?;
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
        if self.NEDFIBONLKB != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.NEDFIBONLKB);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.retcode);
        }
        if let Some(v) = self.reward.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.group_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.group_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.NEDFIBONLKB != 0 {
            os.write_uint32(8, self.NEDFIBONLKB)?;
        }
        if self.retcode != 0 {
            os.write_uint32(6, self.retcode)?;
        }
        if let Some(v) = self.reward.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if self.group_id != 0 {
            os.write_uint32(5, self.group_id)?;
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

    fn new() -> TakeFightActivityRewardScRsp {
        TakeFightActivityRewardScRsp::new()
    }

    fn clear(&mut self) {
        self.NEDFIBONLKB = 0;
        self.retcode = 0;
        self.reward.clear();
        self.group_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TakeFightActivityRewardScRsp {
        static instance: TakeFightActivityRewardScRsp = TakeFightActivityRewardScRsp {
            NEDFIBONLKB: 0,
            retcode: 0,
            reward: ::protobuf::MessageField::none(),
            group_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TakeFightActivityRewardScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TakeFightActivityRewardScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TakeFightActivityRewardScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TakeFightActivityRewardScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"TakeFightActivityRewardScRsp.proto\x1a\x0eItemList.proto\"\x98\x01\n\
    \x1cTakeFightActivityRewardScRsp\x12\x20\n\x0bNEDFIBONLKB\x18\x08\x20\
    \x01(\rR\x0bNEDFIBONLKB\x12\x18\n\x07retcode\x18\x06\x20\x01(\rR\x07retc\
    ode\x12!\n\x06reward\x18\x02\x20\x01(\x0b2\t.ItemListR\x06reward\x12\x19\
    \n\x08group_id\x18\x05\x20\x01(\rR\x07groupIdb\x06proto3\
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
            messages.push(TakeFightActivityRewardScRsp::generated_message_descriptor_data());
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
