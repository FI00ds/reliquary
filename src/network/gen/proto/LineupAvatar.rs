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

//! Generated file from `LineupAvatar.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:LineupAvatar)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LineupAvatar {
    // message fields
    // @@protoc_insertion_point(field:LineupAvatar.avatar_type)
    pub avatar_type: ::protobuf::EnumOrUnknown<super::AvatarType::AvatarType>,
    // @@protoc_insertion_point(field:LineupAvatar.slot)
    pub slot: u32,
    // @@protoc_insertion_point(field:LineupAvatar.satiety)
    pub satiety: u32,
    // @@protoc_insertion_point(field:LineupAvatar.hp)
    pub hp: u32,
    // @@protoc_insertion_point(field:LineupAvatar.id)
    pub id: u32,
    // @@protoc_insertion_point(field:LineupAvatar.sp_bar)
    pub sp_bar: ::protobuf::MessageField<super::SpBarInfo::SpBarInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:LineupAvatar.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LineupAvatar {
    fn default() -> &'a LineupAvatar {
        <LineupAvatar as ::protobuf::Message>::default_instance()
    }
}

impl LineupAvatar {
    pub fn new() -> LineupAvatar {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_type",
            |m: &LineupAvatar| { &m.avatar_type },
            |m: &mut LineupAvatar| { &mut m.avatar_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "slot",
            |m: &LineupAvatar| { &m.slot },
            |m: &mut LineupAvatar| { &mut m.slot },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "satiety",
            |m: &LineupAvatar| { &m.satiety },
            |m: &mut LineupAvatar| { &mut m.satiety },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "hp",
            |m: &LineupAvatar| { &m.hp },
            |m: &mut LineupAvatar| { &mut m.hp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &LineupAvatar| { &m.id },
            |m: &mut LineupAvatar| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::SpBarInfo::SpBarInfo>(
            "sp_bar",
            |m: &LineupAvatar| { &m.sp_bar },
            |m: &mut LineupAvatar| { &mut m.sp_bar },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LineupAvatar>(
            "LineupAvatar",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LineupAvatar {
    const NAME: &'static str = "LineupAvatar";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.avatar_type = is.read_enum_or_unknown()?;
                },
                40 => {
                    self.slot = is.read_uint32()?;
                },
                120 => {
                    self.satiety = is.read_uint32()?;
                },
                48 => {
                    self.hp = is.read_uint32()?;
                },
                64 => {
                    self.id = is.read_uint32()?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.sp_bar)?;
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
        if self.avatar_type != ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(1, self.avatar_type.value());
        }
        if self.slot != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.slot);
        }
        if self.satiety != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.satiety);
        }
        if self.hp != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.hp);
        }
        if self.id != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.id);
        }
        if let Some(v) = self.sp_bar.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.avatar_type != ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.avatar_type))?;
        }
        if self.slot != 0 {
            os.write_uint32(5, self.slot)?;
        }
        if self.satiety != 0 {
            os.write_uint32(15, self.satiety)?;
        }
        if self.hp != 0 {
            os.write_uint32(6, self.hp)?;
        }
        if self.id != 0 {
            os.write_uint32(8, self.id)?;
        }
        if let Some(v) = self.sp_bar.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
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

    fn new() -> LineupAvatar {
        LineupAvatar::new()
    }

    fn clear(&mut self) {
        self.avatar_type = ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE);
        self.slot = 0;
        self.satiety = 0;
        self.hp = 0;
        self.id = 0;
        self.sp_bar.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LineupAvatar {
        static instance: LineupAvatar = LineupAvatar {
            avatar_type: ::protobuf::EnumOrUnknown::from_i32(0),
            slot: 0,
            satiety: 0,
            hp: 0,
            id: 0,
            sp_bar: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LineupAvatar {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LineupAvatar").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LineupAvatar {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LineupAvatar {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12LineupAvatar.proto\x1a\x10AvatarType.proto\x1a\x0fSpBarInfo.proto\
    \"\xad\x01\n\x0cLineupAvatar\x12,\n\x0bavatar_type\x18\x01\x20\x01(\x0e2\
    \x0b.AvatarTypeR\navatarType\x12\x12\n\x04slot\x18\x05\x20\x01(\rR\x04sl\
    ot\x12\x18\n\x07satiety\x18\x0f\x20\x01(\rR\x07satiety\x12\x0e\n\x02hp\
    \x18\x06\x20\x01(\rR\x02hp\x12\x0e\n\x02id\x18\x08\x20\x01(\rR\x02id\x12\
    !\n\x06sp_bar\x18\x0e\x20\x01(\x0b2\n.SpBarInfoR\x05spBarB\x15\n\x13emu.\
    lunarcore.protob\x06proto3\
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
            deps.push(super::AvatarType::file_descriptor().clone());
            deps.push(super::SpBarInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LineupAvatar::generated_message_descriptor_data());
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
