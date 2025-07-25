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

//! Generated file from `CurChallenge.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:CurChallenge)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CurChallenge {
    // message fields
    // @@protoc_insertion_point(field:CurChallenge.score_id)
    pub score_id: u32,
    // @@protoc_insertion_point(field:CurChallenge.stage_info)
    pub stage_info: ::protobuf::MessageField<super::ChallengeCurBuffInfo::ChallengeCurBuffInfo>,
    // @@protoc_insertion_point(field:CurChallenge.score_two)
    pub score_two: u32,
    // @@protoc_insertion_point(field:CurChallenge.dead_avatar_num)
    pub dead_avatar_num: u32,
    // @@protoc_insertion_point(field:CurChallenge.extra_lineup_type)
    pub extra_lineup_type: ::protobuf::EnumOrUnknown<super::ExtraLineupType::ExtraLineupType>,
    // @@protoc_insertion_point(field:CurChallenge.challenge_id)
    pub challenge_id: u32,
    // @@protoc_insertion_point(field:CurChallenge.round_count)
    pub round_count: u32,
    // @@protoc_insertion_point(field:CurChallenge.status)
    pub status: ::protobuf::EnumOrUnknown<super::ChallengeStatus::ChallengeStatus>,
    // @@protoc_insertion_point(field:CurChallenge.kill_monster_list)
    pub kill_monster_list: ::std::vec::Vec<super::KillMonster::KillMonster>,
    // special fields
    // @@protoc_insertion_point(special_field:CurChallenge.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CurChallenge {
    fn default() -> &'a CurChallenge {
        <CurChallenge as ::protobuf::Message>::default_instance()
    }
}

impl CurChallenge {
    pub fn new() -> CurChallenge {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "score_id",
            |m: &CurChallenge| { &m.score_id },
            |m: &mut CurChallenge| { &mut m.score_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ChallengeCurBuffInfo::ChallengeCurBuffInfo>(
            "stage_info",
            |m: &CurChallenge| { &m.stage_info },
            |m: &mut CurChallenge| { &mut m.stage_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "score_two",
            |m: &CurChallenge| { &m.score_two },
            |m: &mut CurChallenge| { &mut m.score_two },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "dead_avatar_num",
            |m: &CurChallenge| { &m.dead_avatar_num },
            |m: &mut CurChallenge| { &mut m.dead_avatar_num },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "extra_lineup_type",
            |m: &CurChallenge| { &m.extra_lineup_type },
            |m: &mut CurChallenge| { &mut m.extra_lineup_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "challenge_id",
            |m: &CurChallenge| { &m.challenge_id },
            |m: &mut CurChallenge| { &mut m.challenge_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "round_count",
            |m: &CurChallenge| { &m.round_count },
            |m: &mut CurChallenge| { &mut m.round_count },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "status",
            |m: &CurChallenge| { &m.status },
            |m: &mut CurChallenge| { &mut m.status },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "kill_monster_list",
            |m: &CurChallenge| { &m.kill_monster_list },
            |m: &mut CurChallenge| { &mut m.kill_monster_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CurChallenge>(
            "CurChallenge",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CurChallenge {
    const NAME: &'static str = "CurChallenge";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.score_id = is.read_uint32()?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.stage_info)?;
                },
                8 => {
                    self.score_two = is.read_uint32()?;
                },
                56 => {
                    self.dead_avatar_num = is.read_uint32()?;
                },
                16 => {
                    self.extra_lineup_type = is.read_enum_or_unknown()?;
                },
                112 => {
                    self.challenge_id = is.read_uint32()?;
                },
                80 => {
                    self.round_count = is.read_uint32()?;
                },
                120 => {
                    self.status = is.read_enum_or_unknown()?;
                },
                98 => {
                    self.kill_monster_list.push(is.read_message()?);
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
        if self.score_id != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.score_id);
        }
        if let Some(v) = self.stage_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.score_two != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.score_two);
        }
        if self.dead_avatar_num != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.dead_avatar_num);
        }
        if self.extra_lineup_type != ::protobuf::EnumOrUnknown::new(super::ExtraLineupType::ExtraLineupType::LINEUP_NONE) {
            my_size += ::protobuf::rt::int32_size(2, self.extra_lineup_type.value());
        }
        if self.challenge_id != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.challenge_id);
        }
        if self.round_count != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.round_count);
        }
        if self.status != ::protobuf::EnumOrUnknown::new(super::ChallengeStatus::ChallengeStatus::CHALLENGE_UNKNOWN) {
            my_size += ::protobuf::rt::int32_size(15, self.status.value());
        }
        for value in &self.kill_monster_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.score_id != 0 {
            os.write_uint32(8, self.score_id)?;
        }
        if let Some(v) = self.stage_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if self.score_two != 0 {
            os.write_uint32(1, self.score_two)?;
        }
        if self.dead_avatar_num != 0 {
            os.write_uint32(7, self.dead_avatar_num)?;
        }
        if self.extra_lineup_type != ::protobuf::EnumOrUnknown::new(super::ExtraLineupType::ExtraLineupType::LINEUP_NONE) {
            os.write_enum(2, ::protobuf::EnumOrUnknown::value(&self.extra_lineup_type))?;
        }
        if self.challenge_id != 0 {
            os.write_uint32(14, self.challenge_id)?;
        }
        if self.round_count != 0 {
            os.write_uint32(10, self.round_count)?;
        }
        if self.status != ::protobuf::EnumOrUnknown::new(super::ChallengeStatus::ChallengeStatus::CHALLENGE_UNKNOWN) {
            os.write_enum(15, ::protobuf::EnumOrUnknown::value(&self.status))?;
        }
        for v in &self.kill_monster_list {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CurChallenge {
        CurChallenge::new()
    }

    fn clear(&mut self) {
        self.score_id = 0;
        self.stage_info.clear();
        self.score_two = 0;
        self.dead_avatar_num = 0;
        self.extra_lineup_type = ::protobuf::EnumOrUnknown::new(super::ExtraLineupType::ExtraLineupType::LINEUP_NONE);
        self.challenge_id = 0;
        self.round_count = 0;
        self.status = ::protobuf::EnumOrUnknown::new(super::ChallengeStatus::ChallengeStatus::CHALLENGE_UNKNOWN);
        self.kill_monster_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CurChallenge {
        static instance: CurChallenge = CurChallenge {
            score_id: 0,
            stage_info: ::protobuf::MessageField::none(),
            score_two: 0,
            dead_avatar_num: 0,
            extra_lineup_type: ::protobuf::EnumOrUnknown::from_i32(0),
            challenge_id: 0,
            round_count: 0,
            status: ::protobuf::EnumOrUnknown::from_i32(0),
            kill_monster_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CurChallenge {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CurChallenge").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CurChallenge {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CurChallenge {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12CurChallenge.proto\x1a\x1aChallengeCurBuffInfo.proto\x1a\x15Challe\
    ngeStatus.proto\x1a\x15ExtraLineupType.proto\x1a\x11KillMonster.proto\"\
    \x8a\x03\n\x0cCurChallenge\x12\x19\n\x08score_id\x18\x08\x20\x01(\rR\x07\
    scoreId\x124\n\nstage_info\x18\t\x20\x01(\x0b2\x15.ChallengeCurBuffInfoR\
    \tstageInfo\x12\x1b\n\tscore_two\x18\x01\x20\x01(\rR\x08scoreTwo\x12&\n\
    \x0fdead_avatar_num\x18\x07\x20\x01(\rR\rdeadAvatarNum\x12<\n\x11extra_l\
    ineup_type\x18\x02\x20\x01(\x0e2\x10.ExtraLineupTypeR\x0fextraLineupType\
    \x12!\n\x0cchallenge_id\x18\x0e\x20\x01(\rR\x0bchallengeId\x12\x1f\n\x0b\
    round_count\x18\n\x20\x01(\rR\nroundCount\x12(\n\x06status\x18\x0f\x20\
    \x01(\x0e2\x10.ChallengeStatusR\x06status\x128\n\x11kill_monster_list\
    \x18\x0c\x20\x03(\x0b2\x0c.KillMonsterR\x0fkillMonsterListb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::ChallengeCurBuffInfo::file_descriptor().clone());
            deps.push(super::ChallengeStatus::file_descriptor().clone());
            deps.push(super::ExtraLineupType::file_descriptor().clone());
            deps.push(super::KillMonster::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CurChallenge::generated_message_descriptor_data());
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
