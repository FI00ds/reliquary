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

//! Generated file from `PlayerDetailInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PlayerDetailInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PlayerDetailInfo {
    // message fields
    // @@protoc_insertion_point(field:PlayerDetailInfo.world_level)
    pub world_level: u32,
    // @@protoc_insertion_point(field:PlayerDetailInfo.signature)
    pub signature: ::std::string::String,
    // @@protoc_insertion_point(field:PlayerDetailInfo.platform_type)
    pub platform_type: ::protobuf::EnumOrUnknown<super::PlatformType::PlatformType>,
    // @@protoc_insertion_point(field:PlayerDetailInfo.level)
    pub level: u32,
    // @@protoc_insertion_point(field:PlayerDetailInfo.head_icon)
    pub head_icon: u32,
    // @@protoc_insertion_point(field:PlayerDetailInfo.uid)
    pub uid: u32,
    // @@protoc_insertion_point(field:PlayerDetailInfo.nickname)
    pub nickname: ::std::string::String,
    // @@protoc_insertion_point(field:PlayerDetailInfo.display_avatar_info)
    pub display_avatar_info: ::std::string::String,
    // @@protoc_insertion_point(field:PlayerDetailInfo.record_info)
    pub record_info: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:PlayerDetailInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PlayerDetailInfo {
    fn default() -> &'a PlayerDetailInfo {
        <PlayerDetailInfo as ::protobuf::Message>::default_instance()
    }
}

impl PlayerDetailInfo {
    pub fn new() -> PlayerDetailInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "world_level",
            |m: &PlayerDetailInfo| { &m.world_level },
            |m: &mut PlayerDetailInfo| { &mut m.world_level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "signature",
            |m: &PlayerDetailInfo| { &m.signature },
            |m: &mut PlayerDetailInfo| { &mut m.signature },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "platform_type",
            |m: &PlayerDetailInfo| { &m.platform_type },
            |m: &mut PlayerDetailInfo| { &mut m.platform_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &PlayerDetailInfo| { &m.level },
            |m: &mut PlayerDetailInfo| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "head_icon",
            |m: &PlayerDetailInfo| { &m.head_icon },
            |m: &mut PlayerDetailInfo| { &mut m.head_icon },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "uid",
            |m: &PlayerDetailInfo| { &m.uid },
            |m: &mut PlayerDetailInfo| { &mut m.uid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "nickname",
            |m: &PlayerDetailInfo| { &m.nickname },
            |m: &mut PlayerDetailInfo| { &mut m.nickname },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "display_avatar_info",
            |m: &PlayerDetailInfo| { &m.display_avatar_info },
            |m: &mut PlayerDetailInfo| { &mut m.display_avatar_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "record_info",
            |m: &PlayerDetailInfo| { &m.record_info },
            |m: &mut PlayerDetailInfo| { &mut m.record_info },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PlayerDetailInfo>(
            "PlayerDetailInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PlayerDetailInfo {
    const NAME: &'static str = "PlayerDetailInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                80 => {
                    self.world_level = is.read_uint32()?;
                },
                18 => {
                    self.signature = is.read_string()?;
                },
                32 => {
                    self.platform_type = is.read_enum_or_unknown()?;
                },
                72 => {
                    self.level = is.read_uint32()?;
                },
                56 => {
                    self.head_icon = is.read_uint32()?;
                },
                104 => {
                    self.uid = is.read_uint32()?;
                },
                50 => {
                    self.nickname = is.read_string()?;
                },
                114 => {
                    self.display_avatar_info = is.read_string()?;
                },
                122 => {
                    self.record_info = is.read_string()?;
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
        if self.world_level != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.world_level);
        }
        if !self.signature.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.signature);
        }
        if self.platform_type != ::protobuf::EnumOrUnknown::new(super::PlatformType::PlatformType::EDITOR) {
            my_size += ::protobuf::rt::int32_size(4, self.platform_type.value());
        }
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.level);
        }
        if self.head_icon != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.head_icon);
        }
        if self.uid != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.uid);
        }
        if !self.nickname.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.nickname);
        }
        if !self.display_avatar_info.is_empty() {
            my_size += ::protobuf::rt::string_size(14, &self.display_avatar_info);
        }
        if !self.record_info.is_empty() {
            my_size += ::protobuf::rt::string_size(15, &self.record_info);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.world_level != 0 {
            os.write_uint32(10, self.world_level)?;
        }
        if !self.signature.is_empty() {
            os.write_string(2, &self.signature)?;
        }
        if self.platform_type != ::protobuf::EnumOrUnknown::new(super::PlatformType::PlatformType::EDITOR) {
            os.write_enum(4, ::protobuf::EnumOrUnknown::value(&self.platform_type))?;
        }
        if self.level != 0 {
            os.write_uint32(9, self.level)?;
        }
        if self.head_icon != 0 {
            os.write_uint32(7, self.head_icon)?;
        }
        if self.uid != 0 {
            os.write_uint32(13, self.uid)?;
        }
        if !self.nickname.is_empty() {
            os.write_string(6, &self.nickname)?;
        }
        if !self.display_avatar_info.is_empty() {
            os.write_string(14, &self.display_avatar_info)?;
        }
        if !self.record_info.is_empty() {
            os.write_string(15, &self.record_info)?;
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

    fn new() -> PlayerDetailInfo {
        PlayerDetailInfo::new()
    }

    fn clear(&mut self) {
        self.world_level = 0;
        self.signature.clear();
        self.platform_type = ::protobuf::EnumOrUnknown::new(super::PlatformType::PlatformType::EDITOR);
        self.level = 0;
        self.head_icon = 0;
        self.uid = 0;
        self.nickname.clear();
        self.display_avatar_info.clear();
        self.record_info.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PlayerDetailInfo {
        static instance: PlayerDetailInfo = PlayerDetailInfo {
            world_level: 0,
            signature: ::std::string::String::new(),
            platform_type: ::protobuf::EnumOrUnknown::from_i32(0),
            level: 0,
            head_icon: 0,
            uid: 0,
            nickname: ::std::string::String::new(),
            display_avatar_info: ::std::string::String::new(),
            record_info: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PlayerDetailInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PlayerDetailInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PlayerDetailInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlayerDetailInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16PlayerDetailInfo.proto\x1a\x12PlatformType.proto\"\xb7\x02\n\x10Pl\
    ayerDetailInfo\x12\x1f\n\x0bworld_level\x18\n\x20\x01(\rR\nworldLevel\
    \x12\x1c\n\tsignature\x18\x02\x20\x01(\tR\tsignature\x122\n\rplatform_ty\
    pe\x18\x04\x20\x01(\x0e2\r.PlatformTypeR\x0cplatformType\x12\x14\n\x05le\
    vel\x18\t\x20\x01(\rR\x05level\x12\x1b\n\thead_icon\x18\x07\x20\x01(\rR\
    \x08headIcon\x12\x10\n\x03uid\x18\r\x20\x01(\rR\x03uid\x12\x1a\n\x08nick\
    name\x18\x06\x20\x01(\tR\x08nickname\x12.\n\x13display_avatar_info\x18\
    \x0e\x20\x01(\tR\x11displayAvatarInfo\x12\x1f\n\x0brecord_info\x18\x0f\
    \x20\x01(\tR\nrecordInfoB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            deps.push(super::PlatformType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PlayerDetailInfo::generated_message_descriptor_data());
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
