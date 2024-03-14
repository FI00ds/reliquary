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

//! Generated file from `BoardDataSync.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:BoardDataSync)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BoardDataSync {
    // message fields
    // @@protoc_insertion_point(field:BoardDataSync.unlocked_head_icon_list)
    pub unlocked_head_icon_list: ::std::vec::Vec<super::HeadIcon::HeadIcon>,
    // @@protoc_insertion_point(field:BoardDataSync.signature)
    pub signature: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:BoardDataSync.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BoardDataSync {
    fn default() -> &'a BoardDataSync {
        <BoardDataSync as ::protobuf::Message>::default_instance()
    }
}

impl BoardDataSync {
    pub fn new() -> BoardDataSync {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "unlocked_head_icon_list",
            |m: &BoardDataSync| { &m.unlocked_head_icon_list },
            |m: &mut BoardDataSync| { &mut m.unlocked_head_icon_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "signature",
            |m: &BoardDataSync| { &m.signature },
            |m: &mut BoardDataSync| { &mut m.signature },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BoardDataSync>(
            "BoardDataSync",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BoardDataSync {
    const NAME: &'static str = "BoardDataSync";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                42 => {
                    self.unlocked_head_icon_list.push(is.read_message()?);
                },
                82 => {
                    self.signature = is.read_string()?;
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
        for value in &self.unlocked_head_icon_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if !self.signature.is_empty() {
            my_size += ::protobuf::rt::string_size(10, &self.signature);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.unlocked_head_icon_list {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        if !self.signature.is_empty() {
            os.write_string(10, &self.signature)?;
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

    fn new() -> BoardDataSync {
        BoardDataSync::new()
    }

    fn clear(&mut self) {
        self.unlocked_head_icon_list.clear();
        self.signature.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BoardDataSync {
        static instance: BoardDataSync = BoardDataSync {
            unlocked_head_icon_list: ::std::vec::Vec::new(),
            signature: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BoardDataSync {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BoardDataSync").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BoardDataSync {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BoardDataSync {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13BoardDataSync.proto\x1a\x0eHeadIcon.proto\"o\n\rBoardDataSync\x12@\
    \n\x17unlocked_head_icon_list\x18\x05\x20\x03(\x0b2\t.HeadIconR\x14unloc\
    kedHeadIconList\x12\x1c\n\tsignature\x18\n\x20\x01(\tR\tsignatureB\x15\n\
    \x13emu.lunarcore.protob\x06proto3\
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
            deps.push(super::HeadIcon::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(BoardDataSync::generated_message_descriptor_data());
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
