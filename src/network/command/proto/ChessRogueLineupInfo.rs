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

//! Generated file from `ChessRogueLineupInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:ChessRogueLineupInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChessRogueLineupInfo {
    // message fields
    // @@protoc_insertion_point(field:ChessRogueLineupInfo.chess_avatar_list)
    pub chess_avatar_list: ::std::vec::Vec<super::ChessRogueLineupAvatarInfo::ChessRogueLineupAvatarInfo>,
    // @@protoc_insertion_point(field:ChessRogueLineupInfo.revive_info)
    pub revive_info: ::protobuf::MessageField<super::RogueAvatarReviveCost::RogueAvatarReviveCost>,
    // special fields
    // @@protoc_insertion_point(special_field:ChessRogueLineupInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChessRogueLineupInfo {
    fn default() -> &'a ChessRogueLineupInfo {
        <ChessRogueLineupInfo as ::protobuf::Message>::default_instance()
    }
}

impl ChessRogueLineupInfo {
    pub fn new() -> ChessRogueLineupInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "chess_avatar_list",
            |m: &ChessRogueLineupInfo| { &m.chess_avatar_list },
            |m: &mut ChessRogueLineupInfo| { &mut m.chess_avatar_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::RogueAvatarReviveCost::RogueAvatarReviveCost>(
            "revive_info",
            |m: &ChessRogueLineupInfo| { &m.revive_info },
            |m: &mut ChessRogueLineupInfo| { &mut m.revive_info },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChessRogueLineupInfo>(
            "ChessRogueLineupInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChessRogueLineupInfo {
    const NAME: &'static str = "ChessRogueLineupInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.chess_avatar_list.push(is.read_message()?);
                },
                98 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.revive_info)?;
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
        for value in &self.chess_avatar_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.revive_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.chess_avatar_list {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        if let Some(v) = self.revive_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
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

    fn new() -> ChessRogueLineupInfo {
        ChessRogueLineupInfo::new()
    }

    fn clear(&mut self) {
        self.chess_avatar_list.clear();
        self.revive_info.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChessRogueLineupInfo {
        static instance: ChessRogueLineupInfo = ChessRogueLineupInfo {
            chess_avatar_list: ::std::vec::Vec::new(),
            revive_info: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ChessRogueLineupInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChessRogueLineupInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChessRogueLineupInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChessRogueLineupInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aChessRogueLineupInfo.proto\x1a\x20ChessRogueLineupAvatarInfo.proto\
    \x1a\x1bRogueAvatarReviveCost.proto\"\x98\x01\n\x14ChessRogueLineupInfo\
    \x12G\n\x11chess_avatar_list\x18\x01\x20\x03(\x0b2\x1b.ChessRogueLineupA\
    vatarInfoR\x0fchessAvatarList\x127\n\x0brevive_info\x18\x0c\x20\x01(\x0b\
    2\x16.RogueAvatarReviveCostR\nreviveInfob\x06proto3\
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
            deps.push(super::ChessRogueLineupAvatarInfo::file_descriptor().clone());
            deps.push(super::RogueAvatarReviveCost::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ChessRogueLineupInfo::generated_message_descriptor_data());
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
