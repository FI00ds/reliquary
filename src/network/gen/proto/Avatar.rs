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

//! Generated file from `Avatar.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:Avatar)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Avatar {
    // message fields
    // @@protoc_insertion_point(field:Avatar.is_marked)
    pub is_marked: bool,
    // @@protoc_insertion_point(field:Avatar.base_avatar_id)
    pub base_avatar_id: u32,
    // @@protoc_insertion_point(field:Avatar.avatar_skilltree_list)
    pub avatar_skilltree_list: ::std::vec::Vec<super::AvatarSkillTree::AvatarSkillTree>,
    // @@protoc_insertion_point(field:Avatar.equipment_unique_id)
    pub equipment_unique_id: u32,
    // @@protoc_insertion_point(field:Avatar.equip_relic_list)
    pub equip_relic_list: ::std::vec::Vec<super::EquipRelic::EquipRelic>,
    // @@protoc_insertion_point(field:Avatar.rank)
    pub rank: u32,
    // @@protoc_insertion_point(field:Avatar.first_met_timestamp)
    pub first_met_timestamp: u64,
    // @@protoc_insertion_point(field:Avatar.promotion)
    pub promotion: u32,
    // @@protoc_insertion_point(field:Avatar.taken_rewards)
    pub taken_rewards: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:Avatar.dressed_skin_id)
    pub dressed_skin_id: u32,
    // @@protoc_insertion_point(field:Avatar.level)
    pub level: u32,
    // @@protoc_insertion_point(field:Avatar.exp)
    pub exp: u32,
    // special fields
    // @@protoc_insertion_point(special_field:Avatar.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Avatar {
    fn default() -> &'a Avatar {
        <Avatar as ::protobuf::Message>::default_instance()
    }
}

impl Avatar {
    pub fn new() -> Avatar {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(12);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_marked",
            |m: &Avatar| { &m.is_marked },
            |m: &mut Avatar| { &mut m.is_marked },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "base_avatar_id",
            |m: &Avatar| { &m.base_avatar_id },
            |m: &mut Avatar| { &mut m.base_avatar_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "avatar_skilltree_list",
            |m: &Avatar| { &m.avatar_skilltree_list },
            |m: &mut Avatar| { &mut m.avatar_skilltree_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "equipment_unique_id",
            |m: &Avatar| { &m.equipment_unique_id },
            |m: &mut Avatar| { &mut m.equipment_unique_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "equip_relic_list",
            |m: &Avatar| { &m.equip_relic_list },
            |m: &mut Avatar| { &mut m.equip_relic_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "rank",
            |m: &Avatar| { &m.rank },
            |m: &mut Avatar| { &mut m.rank },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "first_met_timestamp",
            |m: &Avatar| { &m.first_met_timestamp },
            |m: &mut Avatar| { &mut m.first_met_timestamp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "promotion",
            |m: &Avatar| { &m.promotion },
            |m: &mut Avatar| { &mut m.promotion },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "taken_rewards",
            |m: &Avatar| { &m.taken_rewards },
            |m: &mut Avatar| { &mut m.taken_rewards },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "dressed_skin_id",
            |m: &Avatar| { &m.dressed_skin_id },
            |m: &mut Avatar| { &mut m.dressed_skin_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &Avatar| { &m.level },
            |m: &mut Avatar| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "exp",
            |m: &Avatar| { &m.exp },
            |m: &mut Avatar| { &mut m.exp },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Avatar>(
            "Avatar",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Avatar {
    const NAME: &'static str = "Avatar";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.is_marked = is.read_bool()?;
                },
                64 => {
                    self.base_avatar_id = is.read_uint32()?;
                },
                114 => {
                    self.avatar_skilltree_list.push(is.read_message()?);
                },
                88 => {
                    self.equipment_unique_id = is.read_uint32()?;
                },
                58 => {
                    self.equip_relic_list.push(is.read_message()?);
                },
                8 => {
                    self.rank = is.read_uint32()?;
                },
                72 => {
                    self.first_met_timestamp = is.read_uint64()?;
                },
                48 => {
                    self.promotion = is.read_uint32()?;
                },
                106 => {
                    is.read_repeated_packed_uint32_into(&mut self.taken_rewards)?;
                },
                104 => {
                    self.taken_rewards.push(is.read_uint32()?);
                },
                80 => {
                    self.dressed_skin_id = is.read_uint32()?;
                },
                24 => {
                    self.level = is.read_uint32()?;
                },
                32 => {
                    self.exp = is.read_uint32()?;
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
        if self.is_marked != false {
            my_size += 1 + 1;
        }
        if self.base_avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.base_avatar_id);
        }
        for value in &self.avatar_skilltree_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.equipment_unique_id != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.equipment_unique_id);
        }
        for value in &self.equip_relic_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.rank != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.rank);
        }
        if self.first_met_timestamp != 0 {
            my_size += ::protobuf::rt::uint64_size(9, self.first_met_timestamp);
        }
        if self.promotion != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.promotion);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(13, &self.taken_rewards);
        if self.dressed_skin_id != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.dressed_skin_id);
        }
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.level);
        }
        if self.exp != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.exp);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.is_marked != false {
            os.write_bool(2, self.is_marked)?;
        }
        if self.base_avatar_id != 0 {
            os.write_uint32(8, self.base_avatar_id)?;
        }
        for v in &self.avatar_skilltree_list {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        };
        if self.equipment_unique_id != 0 {
            os.write_uint32(11, self.equipment_unique_id)?;
        }
        for v in &self.equip_relic_list {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        };
        if self.rank != 0 {
            os.write_uint32(1, self.rank)?;
        }
        if self.first_met_timestamp != 0 {
            os.write_uint64(9, self.first_met_timestamp)?;
        }
        if self.promotion != 0 {
            os.write_uint32(6, self.promotion)?;
        }
        os.write_repeated_packed_uint32(13, &self.taken_rewards)?;
        if self.dressed_skin_id != 0 {
            os.write_uint32(10, self.dressed_skin_id)?;
        }
        if self.level != 0 {
            os.write_uint32(3, self.level)?;
        }
        if self.exp != 0 {
            os.write_uint32(4, self.exp)?;
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

    fn new() -> Avatar {
        Avatar::new()
    }

    fn clear(&mut self) {
        self.is_marked = false;
        self.base_avatar_id = 0;
        self.avatar_skilltree_list.clear();
        self.equipment_unique_id = 0;
        self.equip_relic_list.clear();
        self.rank = 0;
        self.first_met_timestamp = 0;
        self.promotion = 0;
        self.taken_rewards.clear();
        self.dressed_skin_id = 0;
        self.level = 0;
        self.exp = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Avatar {
        static instance: Avatar = Avatar {
            is_marked: false,
            base_avatar_id: 0,
            avatar_skilltree_list: ::std::vec::Vec::new(),
            equipment_unique_id: 0,
            equip_relic_list: ::std::vec::Vec::new(),
            rank: 0,
            first_met_timestamp: 0,
            promotion: 0,
            taken_rewards: ::std::vec::Vec::new(),
            dressed_skin_id: 0,
            level: 0,
            exp: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Avatar {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Avatar").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Avatar {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Avatar {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0cAvatar.proto\x1a\x15AvatarSkillTree.proto\x1a\x10EquipRelic.proto\
    \"\xcf\x03\n\x06Avatar\x12\x1b\n\tis_marked\x18\x02\x20\x01(\x08R\x08isM\
    arked\x12$\n\x0ebase_avatar_id\x18\x08\x20\x01(\rR\x0cbaseAvatarId\x12D\
    \n\x15avatar_skilltree_list\x18\x0e\x20\x03(\x0b2\x10.AvatarSkillTreeR\
    \x13avatarSkilltreeList\x12.\n\x13equipment_unique_id\x18\x0b\x20\x01(\r\
    R\x11equipmentUniqueId\x125\n\x10equip_relic_list\x18\x07\x20\x03(\x0b2\
    \x0b.EquipRelicR\x0eequipRelicList\x12\x12\n\x04rank\x18\x01\x20\x01(\rR\
    \x04rank\x12.\n\x13first_met_timestamp\x18\t\x20\x01(\x04R\x11firstMetTi\
    mestamp\x12\x1c\n\tpromotion\x18\x06\x20\x01(\rR\tpromotion\x12#\n\rtake\
    n_rewards\x18\r\x20\x03(\rR\x0ctakenRewards\x12&\n\x0fdressed_skin_id\
    \x18\n\x20\x01(\rR\rdressedSkinId\x12\x14\n\x05level\x18\x03\x20\x01(\rR\
    \x05level\x12\x10\n\x03exp\x18\x04\x20\x01(\rR\x03expb\x06proto3\
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
            deps.push(super::AvatarSkillTree::file_descriptor().clone());
            deps.push(super::EquipRelic::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Avatar::generated_message_descriptor_data());
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
