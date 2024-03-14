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

//! Generated file from `ChessRogueVirtualItemInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ChessRogueVirtualItemInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChessRogueVirtualItemInfo {
    // message fields
    // @@protoc_insertion_point(field:ChessRogueVirtualItemInfo.buff_info)
    pub buff_info: ::protobuf::MessageField<super::ChessRogueBuffInfo::ChessRogueBuffInfo>,
    // @@protoc_insertion_point(field:ChessRogueVirtualItemInfo.miracle_info)
    pub miracle_info: ::protobuf::MessageField<super::ChessRogueMiracleInfo::ChessRogueMiracleInfo>,
    // @@protoc_insertion_point(field:ChessRogueVirtualItemInfo.item_info)
    pub item_info: ::protobuf::MessageField<super::ChessRogueItemInfo::ChessRogueItemInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:ChessRogueVirtualItemInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChessRogueVirtualItemInfo {
    fn default() -> &'a ChessRogueVirtualItemInfo {
        <ChessRogueVirtualItemInfo as ::protobuf::Message>::default_instance()
    }
}

impl ChessRogueVirtualItemInfo {
    pub fn new() -> ChessRogueVirtualItemInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ChessRogueBuffInfo::ChessRogueBuffInfo>(
            "buff_info",
            |m: &ChessRogueVirtualItemInfo| { &m.buff_info },
            |m: &mut ChessRogueVirtualItemInfo| { &mut m.buff_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ChessRogueMiracleInfo::ChessRogueMiracleInfo>(
            "miracle_info",
            |m: &ChessRogueVirtualItemInfo| { &m.miracle_info },
            |m: &mut ChessRogueVirtualItemInfo| { &mut m.miracle_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ChessRogueItemInfo::ChessRogueItemInfo>(
            "item_info",
            |m: &ChessRogueVirtualItemInfo| { &m.item_info },
            |m: &mut ChessRogueVirtualItemInfo| { &mut m.item_info },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChessRogueVirtualItemInfo>(
            "ChessRogueVirtualItemInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChessRogueVirtualItemInfo {
    const NAME: &'static str = "ChessRogueVirtualItemInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.buff_info)?;
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.miracle_info)?;
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.item_info)?;
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
        if let Some(v) = self.buff_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.miracle_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.item_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.buff_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if let Some(v) = self.miracle_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if let Some(v) = self.item_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
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

    fn new() -> ChessRogueVirtualItemInfo {
        ChessRogueVirtualItemInfo::new()
    }

    fn clear(&mut self) {
        self.buff_info.clear();
        self.miracle_info.clear();
        self.item_info.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChessRogueVirtualItemInfo {
        static instance: ChessRogueVirtualItemInfo = ChessRogueVirtualItemInfo {
            buff_info: ::protobuf::MessageField::none(),
            miracle_info: ::protobuf::MessageField::none(),
            item_info: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ChessRogueVirtualItemInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChessRogueVirtualItemInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChessRogueVirtualItemInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChessRogueVirtualItemInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1fChessRogueVirtualItemInfo.proto\x1a\x18ChessRogueBuffInfo.proto\
    \x1a\x1bChessRogueMiracleInfo.proto\x1a\x18ChessRogueItemInfo.proto\"\
    \xba\x01\n\x19ChessRogueVirtualItemInfo\x120\n\tbuff_info\x18\t\x20\x01(\
    \x0b2\x13.ChessRogueBuffInfoR\x08buffInfo\x129\n\x0cmiracle_info\x18\r\
    \x20\x01(\x0b2\x16.ChessRogueMiracleInfoR\x0bmiracleInfo\x120\n\titem_in\
    fo\x18\x06\x20\x01(\x0b2\x13.ChessRogueItemInfoR\x08itemInfoB\x15\n\x13e\
    mu.lunarcore.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::ChessRogueBuffInfo::file_descriptor().clone());
            deps.push(super::ChessRogueMiracleInfo::file_descriptor().clone());
            deps.push(super::ChessRogueItemInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ChessRogueVirtualItemInfo::generated_message_descriptor_data());
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
