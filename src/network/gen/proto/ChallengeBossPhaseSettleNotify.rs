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

//! Generated file from `ChallengeBossPhaseSettleNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ChallengeBossPhaseSettleNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChallengeBossPhaseSettleNotify {
    // message fields
    // @@protoc_insertion_point(field:ChallengeBossPhaseSettleNotify.challenge_id)
    pub challenge_id: u32,
    // @@protoc_insertion_point(field:ChallengeBossPhaseSettleNotify.COKFJJOLMDI)
    pub COKFJJOLMDI: u32,
    // @@protoc_insertion_point(field:ChallengeBossPhaseSettleNotify.score_two)
    pub score_two: u32,
    // @@protoc_insertion_point(field:ChallengeBossPhaseSettleNotify.unkbool1)
    pub unkbool1: bool,
    // @@protoc_insertion_point(field:ChallengeBossPhaseSettleNotify.is_win)
    pub is_win: bool,
    // @@protoc_insertion_point(field:ChallengeBossPhaseSettleNotify.BIPEPADLEIA)
    pub BIPEPADLEIA: ::std::vec::Vec<super::BattleTarget::BattleTarget>,
    // @@protoc_insertion_point(field:ChallengeBossPhaseSettleNotify.stars)
    pub stars: u32,
    // @@protoc_insertion_point(field:ChallengeBossPhaseSettleNotify.challenge_score)
    pub challenge_score: u32,
    // @@protoc_insertion_point(field:ChallengeBossPhaseSettleNotify.phase)
    pub phase: u32,
    // @@protoc_insertion_point(field:ChallengeBossPhaseSettleNotify.unkbool2)
    pub unkbool2: bool,
    // special fields
    // @@protoc_insertion_point(special_field:ChallengeBossPhaseSettleNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChallengeBossPhaseSettleNotify {
    fn default() -> &'a ChallengeBossPhaseSettleNotify {
        <ChallengeBossPhaseSettleNotify as ::protobuf::Message>::default_instance()
    }
}

impl ChallengeBossPhaseSettleNotify {
    pub fn new() -> ChallengeBossPhaseSettleNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(10);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "challenge_id",
            |m: &ChallengeBossPhaseSettleNotify| { &m.challenge_id },
            |m: &mut ChallengeBossPhaseSettleNotify| { &mut m.challenge_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "COKFJJOLMDI",
            |m: &ChallengeBossPhaseSettleNotify| { &m.COKFJJOLMDI },
            |m: &mut ChallengeBossPhaseSettleNotify| { &mut m.COKFJJOLMDI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "score_two",
            |m: &ChallengeBossPhaseSettleNotify| { &m.score_two },
            |m: &mut ChallengeBossPhaseSettleNotify| { &mut m.score_two },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "unkbool1",
            |m: &ChallengeBossPhaseSettleNotify| { &m.unkbool1 },
            |m: &mut ChallengeBossPhaseSettleNotify| { &mut m.unkbool1 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_win",
            |m: &ChallengeBossPhaseSettleNotify| { &m.is_win },
            |m: &mut ChallengeBossPhaseSettleNotify| { &mut m.is_win },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "BIPEPADLEIA",
            |m: &ChallengeBossPhaseSettleNotify| { &m.BIPEPADLEIA },
            |m: &mut ChallengeBossPhaseSettleNotify| { &mut m.BIPEPADLEIA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "stars",
            |m: &ChallengeBossPhaseSettleNotify| { &m.stars },
            |m: &mut ChallengeBossPhaseSettleNotify| { &mut m.stars },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "challenge_score",
            |m: &ChallengeBossPhaseSettleNotify| { &m.challenge_score },
            |m: &mut ChallengeBossPhaseSettleNotify| { &mut m.challenge_score },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "phase",
            |m: &ChallengeBossPhaseSettleNotify| { &m.phase },
            |m: &mut ChallengeBossPhaseSettleNotify| { &mut m.phase },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "unkbool2",
            |m: &ChallengeBossPhaseSettleNotify| { &m.unkbool2 },
            |m: &mut ChallengeBossPhaseSettleNotify| { &mut m.unkbool2 },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChallengeBossPhaseSettleNotify>(
            "ChallengeBossPhaseSettleNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChallengeBossPhaseSettleNotify {
    const NAME: &'static str = "ChallengeBossPhaseSettleNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.challenge_id = is.read_uint32()?;
                },
                96 => {
                    self.COKFJJOLMDI = is.read_uint32()?;
                },
                16 => {
                    self.score_two = is.read_uint32()?;
                },
                56 => {
                    self.unkbool1 = is.read_bool()?;
                },
                88 => {
                    self.is_win = is.read_bool()?;
                },
                10 => {
                    self.BIPEPADLEIA.push(is.read_message()?);
                },
                32 => {
                    self.stars = is.read_uint32()?;
                },
                24 => {
                    self.challenge_score = is.read_uint32()?;
                },
                112 => {
                    self.phase = is.read_uint32()?;
                },
                120 => {
                    self.unkbool2 = is.read_bool()?;
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
        if self.challenge_id != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.challenge_id);
        }
        if self.COKFJJOLMDI != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.COKFJJOLMDI);
        }
        if self.score_two != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.score_two);
        }
        if self.unkbool1 != false {
            my_size += 1 + 1;
        }
        if self.is_win != false {
            my_size += 1 + 1;
        }
        for value in &self.BIPEPADLEIA {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.stars != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.stars);
        }
        if self.challenge_score != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.challenge_score);
        }
        if self.phase != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.phase);
        }
        if self.unkbool2 != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.challenge_id != 0 {
            os.write_uint32(13, self.challenge_id)?;
        }
        if self.COKFJJOLMDI != 0 {
            os.write_uint32(12, self.COKFJJOLMDI)?;
        }
        if self.score_two != 0 {
            os.write_uint32(2, self.score_two)?;
        }
        if self.unkbool1 != false {
            os.write_bool(7, self.unkbool1)?;
        }
        if self.is_win != false {
            os.write_bool(11, self.is_win)?;
        }
        for v in &self.BIPEPADLEIA {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        if self.stars != 0 {
            os.write_uint32(4, self.stars)?;
        }
        if self.challenge_score != 0 {
            os.write_uint32(3, self.challenge_score)?;
        }
        if self.phase != 0 {
            os.write_uint32(14, self.phase)?;
        }
        if self.unkbool2 != false {
            os.write_bool(15, self.unkbool2)?;
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

    fn new() -> ChallengeBossPhaseSettleNotify {
        ChallengeBossPhaseSettleNotify::new()
    }

    fn clear(&mut self) {
        self.challenge_id = 0;
        self.COKFJJOLMDI = 0;
        self.score_two = 0;
        self.unkbool1 = false;
        self.is_win = false;
        self.BIPEPADLEIA.clear();
        self.stars = 0;
        self.challenge_score = 0;
        self.phase = 0;
        self.unkbool2 = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChallengeBossPhaseSettleNotify {
        static instance: ChallengeBossPhaseSettleNotify = ChallengeBossPhaseSettleNotify {
            challenge_id: 0,
            COKFJJOLMDI: 0,
            score_two: 0,
            unkbool1: false,
            is_win: false,
            BIPEPADLEIA: ::std::vec::Vec::new(),
            stars: 0,
            challenge_score: 0,
            phase: 0,
            unkbool2: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ChallengeBossPhaseSettleNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChallengeBossPhaseSettleNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChallengeBossPhaseSettleNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChallengeBossPhaseSettleNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n$ChallengeBossPhaseSettleNotify.proto\x1a\x12BattleTarget.proto\"\xd7\
    \x02\n\x1eChallengeBossPhaseSettleNotify\x12!\n\x0cchallenge_id\x18\r\
    \x20\x01(\rR\x0bchallengeId\x12\x20\n\x0bCOKFJJOLMDI\x18\x0c\x20\x01(\rR\
    \x0bCOKFJJOLMDI\x12\x1b\n\tscore_two\x18\x02\x20\x01(\rR\x08scoreTwo\x12\
    \x1a\n\x08unkbool1\x18\x07\x20\x01(\x08R\x08unkbool1\x12\x15\n\x06is_win\
    \x18\x0b\x20\x01(\x08R\x05isWin\x12/\n\x0bBIPEPADLEIA\x18\x01\x20\x03(\
    \x0b2\r.BattleTargetR\x0bBIPEPADLEIA\x12\x14\n\x05stars\x18\x04\x20\x01(\
    \rR\x05stars\x12'\n\x0fchallenge_score\x18\x03\x20\x01(\rR\x0echallengeS\
    core\x12\x14\n\x05phase\x18\x0e\x20\x01(\rR\x05phase\x12\x1a\n\x08unkboo\
    l2\x18\x0f\x20\x01(\x08R\x08unkbool2B\x15\n\x13emu.lunarcore.protob\x06p\
    roto3\
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
            deps.push(super::BattleTarget::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ChallengeBossPhaseSettleNotify::generated_message_descriptor_data());
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
