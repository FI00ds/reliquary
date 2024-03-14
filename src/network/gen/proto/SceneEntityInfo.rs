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

//! Generated file from `SceneEntityInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SceneEntityInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SceneEntityInfo {
    // message fields
    // @@protoc_insertion_point(field:SceneEntityInfo.entity_id)
    pub entity_id: u32,
    // @@protoc_insertion_point(field:SceneEntityInfo.group_id)
    pub group_id: u32,
    // @@protoc_insertion_point(field:SceneEntityInfo.motion)
    pub motion: ::protobuf::MessageField<super::MotionInfo::MotionInfo>,
    // @@protoc_insertion_point(field:SceneEntityInfo.inst_id)
    pub inst_id: u32,
    // @@protoc_insertion_point(field:SceneEntityInfo.actor)
    pub actor: ::protobuf::MessageField<super::SceneActorInfo::SceneActorInfo>,
    // @@protoc_insertion_point(field:SceneEntityInfo.npc_monster)
    pub npc_monster: ::protobuf::MessageField<super::SceneNpcMonsterInfo::SceneNpcMonsterInfo>,
    // @@protoc_insertion_point(field:SceneEntityInfo.npc)
    pub npc: ::protobuf::MessageField<super::SceneNpcInfo::SceneNpcInfo>,
    // @@protoc_insertion_point(field:SceneEntityInfo.prop)
    pub prop: ::protobuf::MessageField<super::ScenePropInfo::ScenePropInfo>,
    // @@protoc_insertion_point(field:SceneEntityInfo.summon_unit)
    pub summon_unit: ::protobuf::MessageField<super::SceneSummonUnitInfo::SceneSummonUnitInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:SceneEntityInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SceneEntityInfo {
    fn default() -> &'a SceneEntityInfo {
        <SceneEntityInfo as ::protobuf::Message>::default_instance()
    }
}

impl SceneEntityInfo {
    pub fn new() -> SceneEntityInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "entity_id",
            |m: &SceneEntityInfo| { &m.entity_id },
            |m: &mut SceneEntityInfo| { &mut m.entity_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "group_id",
            |m: &SceneEntityInfo| { &m.group_id },
            |m: &mut SceneEntityInfo| { &mut m.group_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MotionInfo::MotionInfo>(
            "motion",
            |m: &SceneEntityInfo| { &m.motion },
            |m: &mut SceneEntityInfo| { &mut m.motion },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "inst_id",
            |m: &SceneEntityInfo| { &m.inst_id },
            |m: &mut SceneEntityInfo| { &mut m.inst_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::SceneActorInfo::SceneActorInfo>(
            "actor",
            |m: &SceneEntityInfo| { &m.actor },
            |m: &mut SceneEntityInfo| { &mut m.actor },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::SceneNpcMonsterInfo::SceneNpcMonsterInfo>(
            "npc_monster",
            |m: &SceneEntityInfo| { &m.npc_monster },
            |m: &mut SceneEntityInfo| { &mut m.npc_monster },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::SceneNpcInfo::SceneNpcInfo>(
            "npc",
            |m: &SceneEntityInfo| { &m.npc },
            |m: &mut SceneEntityInfo| { &mut m.npc },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ScenePropInfo::ScenePropInfo>(
            "prop",
            |m: &SceneEntityInfo| { &m.prop },
            |m: &mut SceneEntityInfo| { &mut m.prop },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::SceneSummonUnitInfo::SceneSummonUnitInfo>(
            "summon_unit",
            |m: &SceneEntityInfo| { &m.summon_unit },
            |m: &mut SceneEntityInfo| { &mut m.summon_unit },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SceneEntityInfo>(
            "SceneEntityInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SceneEntityInfo {
    const NAME: &'static str = "SceneEntityInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.entity_id = is.read_uint32()?;
                },
                88 => {
                    self.group_id = is.read_uint32()?;
                },
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.motion)?;
                },
                32 => {
                    self.inst_id = is.read_uint32()?;
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.actor)?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.npc_monster)?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.npc)?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.prop)?;
                },
                98 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.summon_unit)?;
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
        if self.entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.entity_id);
        }
        if self.group_id != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.group_id);
        }
        if let Some(v) = self.motion.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.inst_id != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.inst_id);
        }
        if let Some(v) = self.actor.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.npc_monster.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.npc.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.prop.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.summon_unit.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.entity_id != 0 {
            os.write_uint32(6, self.entity_id)?;
        }
        if self.group_id != 0 {
            os.write_uint32(11, self.group_id)?;
        }
        if let Some(v) = self.motion.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        if self.inst_id != 0 {
            os.write_uint32(4, self.inst_id)?;
        }
        if let Some(v) = self.actor.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if let Some(v) = self.npc_monster.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if let Some(v) = self.npc.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        if let Some(v) = self.prop.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if let Some(v) = self.summon_unit.as_ref() {
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

    fn new() -> SceneEntityInfo {
        SceneEntityInfo::new()
    }

    fn clear(&mut self) {
        self.entity_id = 0;
        self.group_id = 0;
        self.motion.clear();
        self.inst_id = 0;
        self.actor.clear();
        self.npc_monster.clear();
        self.npc.clear();
        self.prop.clear();
        self.summon_unit.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SceneEntityInfo {
        static instance: SceneEntityInfo = SceneEntityInfo {
            entity_id: 0,
            group_id: 0,
            motion: ::protobuf::MessageField::none(),
            inst_id: 0,
            actor: ::protobuf::MessageField::none(),
            npc_monster: ::protobuf::MessageField::none(),
            npc: ::protobuf::MessageField::none(),
            prop: ::protobuf::MessageField::none(),
            summon_unit: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SceneEntityInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SceneEntityInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SceneEntityInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SceneEntityInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15SceneEntityInfo.proto\x1a\x10MotionInfo.proto\x1a\x19SceneSummonUn\
    itInfo.proto\x1a\x14SceneActorInfo.proto\x1a\x19SceneNpcMonsterInfo.prot\
    o\x1a\x12SceneNpcInfo.proto\x1a\x13ScenePropInfo.proto\"\xe1\x02\n\x0fSc\
    eneEntityInfo\x12\x1b\n\tentity_id\x18\x06\x20\x01(\rR\x08entityId\x12\
    \x19\n\x08group_id\x18\x0b\x20\x01(\rR\x07groupId\x12#\n\x06motion\x18\
    \x08\x20\x01(\x0b2\x0b.MotionInfoR\x06motion\x12\x17\n\x07inst_id\x18\
    \x04\x20\x01(\rR\x06instId\x12%\n\x05actor\x18\r\x20\x01(\x0b2\x0f.Scene\
    ActorInfoR\x05actor\x125\n\x0bnpc_monster\x18\t\x20\x01(\x0b2\x14.SceneN\
    pcMonsterInfoR\nnpcMonster\x12\x1f\n\x03npc\x18\x0e\x20\x01(\x0b2\r.Scen\
    eNpcInfoR\x03npc\x12\"\n\x04prop\x18\x05\x20\x01(\x0b2\x0e.ScenePropInfo\
    R\x04prop\x125\n\x0bsummon_unit\x18\x0c\x20\x01(\x0b2\x14.SceneSummonUni\
    tInfoR\nsummonUnitB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(6);
            deps.push(super::MotionInfo::file_descriptor().clone());
            deps.push(super::SceneSummonUnitInfo::file_descriptor().clone());
            deps.push(super::SceneActorInfo::file_descriptor().clone());
            deps.push(super::SceneNpcMonsterInfo::file_descriptor().clone());
            deps.push(super::SceneNpcInfo::file_descriptor().clone());
            deps.push(super::ScenePropInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SceneEntityInfo::generated_message_descriptor_data());
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
