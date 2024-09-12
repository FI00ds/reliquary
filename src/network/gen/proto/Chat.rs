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

//! Generated file from `Chat.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:Chat)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Chat {
    // message fields
    // @@protoc_insertion_point(field:Chat.emote)
    pub emote: u32,
    // @@protoc_insertion_point(field:Chat.msg_type)
    pub msg_type: ::protobuf::EnumOrUnknown<super::MsgType::MsgType>,
    // @@protoc_insertion_point(field:Chat.sender_uid)
    pub sender_uid: u32,
    // @@protoc_insertion_point(field:Chat.sent_time)
    pub sent_time: u64,
    // @@protoc_insertion_point(field:Chat.text)
    pub text: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:Chat.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Chat {
    fn default() -> &'a Chat {
        <Chat as ::protobuf::Message>::default_instance()
    }
}

impl Chat {
    pub fn new() -> Chat {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "emote",
            |m: &Chat| { &m.emote },
            |m: &mut Chat| { &mut m.emote },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "msg_type",
            |m: &Chat| { &m.msg_type },
            |m: &mut Chat| { &mut m.msg_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "sender_uid",
            |m: &Chat| { &m.sender_uid },
            |m: &mut Chat| { &mut m.sender_uid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "sent_time",
            |m: &Chat| { &m.sent_time },
            |m: &mut Chat| { &mut m.sent_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "text",
            |m: &Chat| { &m.text },
            |m: &mut Chat| { &mut m.text },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Chat>(
            "Chat",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Chat {
    const NAME: &'static str = "Chat";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.emote = is.read_uint32()?;
                },
                80 => {
                    self.msg_type = is.read_enum_or_unknown()?;
                },
                16 => {
                    self.sender_uid = is.read_uint32()?;
                },
                96 => {
                    self.sent_time = is.read_uint64()?;
                },
                90 => {
                    self.text = is.read_string()?;
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
        if self.emote != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.emote);
        }
        if self.msg_type != ::protobuf::EnumOrUnknown::new(super::MsgType::MsgType::MSG_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(10, self.msg_type.value());
        }
        if self.sender_uid != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.sender_uid);
        }
        if self.sent_time != 0 {
            my_size += ::protobuf::rt::uint64_size(12, self.sent_time);
        }
        if !self.text.is_empty() {
            my_size += ::protobuf::rt::string_size(11, &self.text);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.emote != 0 {
            os.write_uint32(3, self.emote)?;
        }
        if self.msg_type != ::protobuf::EnumOrUnknown::new(super::MsgType::MsgType::MSG_TYPE_NONE) {
            os.write_enum(10, ::protobuf::EnumOrUnknown::value(&self.msg_type))?;
        }
        if self.sender_uid != 0 {
            os.write_uint32(2, self.sender_uid)?;
        }
        if self.sent_time != 0 {
            os.write_uint64(12, self.sent_time)?;
        }
        if !self.text.is_empty() {
            os.write_string(11, &self.text)?;
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

    fn new() -> Chat {
        Chat::new()
    }

    fn clear(&mut self) {
        self.emote = 0;
        self.msg_type = ::protobuf::EnumOrUnknown::new(super::MsgType::MsgType::MSG_TYPE_NONE);
        self.sender_uid = 0;
        self.sent_time = 0;
        self.text.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Chat {
        static instance: Chat = Chat {
            emote: 0,
            msg_type: ::protobuf::EnumOrUnknown::from_i32(0),
            sender_uid: 0,
            sent_time: 0,
            text: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Chat {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Chat").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Chat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Chat {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\nChat.proto\x1a\rMsgType.proto\"\x91\x01\n\x04Chat\x12\x14\n\x05emote\
    \x18\x03\x20\x01(\rR\x05emote\x12#\n\x08msg_type\x18\n\x20\x01(\x0e2\x08\
    .MsgTypeR\x07msgType\x12\x1d\n\nsender_uid\x18\x02\x20\x01(\rR\tsenderUi\
    d\x12\x1b\n\tsent_time\x18\x0c\x20\x01(\x04R\x08sentTime\x12\x12\n\x04te\
    xt\x18\x0b\x20\x01(\tR\x04textB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            deps.push(super::MsgType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Chat::generated_message_descriptor_data());
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
