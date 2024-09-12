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

//! Generated file from `Challenge.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:Challenge)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Challenge {
    // message fields
    // @@protoc_insertion_point(field:Challenge.stars)
    pub stars: u32,
    // @@protoc_insertion_point(field:Challenge.ext_info)
    pub ext_info: ::protobuf::MessageField<super::ChallengeExtInfo::ChallengeExtInfo>,
    // @@protoc_insertion_point(field:Challenge.taken_reward)
    pub taken_reward: u32,
    // @@protoc_insertion_point(field:Challenge.challenge_id)
    pub challenge_id: u32,
    // @@protoc_insertion_point(field:Challenge.score)
    pub score: u32,
    // special fields
    // @@protoc_insertion_point(special_field:Challenge.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Challenge {
    fn default() -> &'a Challenge {
        <Challenge as ::protobuf::Message>::default_instance()
    }
}

impl Challenge {
    pub fn new() -> Challenge {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "stars",
            |m: &Challenge| { &m.stars },
            |m: &mut Challenge| { &mut m.stars },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ChallengeExtInfo::ChallengeExtInfo>(
            "ext_info",
            |m: &Challenge| { &m.ext_info },
            |m: &mut Challenge| { &mut m.ext_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "taken_reward",
            |m: &Challenge| { &m.taken_reward },
            |m: &mut Challenge| { &mut m.taken_reward },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "challenge_id",
            |m: &Challenge| { &m.challenge_id },
            |m: &mut Challenge| { &mut m.challenge_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "score",
            |m: &Challenge| { &m.score },
            |m: &mut Challenge| { &mut m.score },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Challenge>(
            "Challenge",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Challenge {
    const NAME: &'static str = "Challenge";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.stars = is.read_uint32()?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ext_info)?;
                },
                48 => {
                    self.taken_reward = is.read_uint32()?;
                },
                80 => {
                    self.challenge_id = is.read_uint32()?;
                },
                56 => {
                    self.score = is.read_uint32()?;
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
        if self.stars != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.stars);
        }
        if let Some(v) = self.ext_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.taken_reward != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.taken_reward);
        }
        if self.challenge_id != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.challenge_id);
        }
        if self.score != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.score);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.stars != 0 {
            os.write_uint32(1, self.stars)?;
        }
        if let Some(v) = self.ext_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if self.taken_reward != 0 {
            os.write_uint32(6, self.taken_reward)?;
        }
        if self.challenge_id != 0 {
            os.write_uint32(10, self.challenge_id)?;
        }
        if self.score != 0 {
            os.write_uint32(7, self.score)?;
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

    fn new() -> Challenge {
        Challenge::new()
    }

    fn clear(&mut self) {
        self.stars = 0;
        self.ext_info.clear();
        self.taken_reward = 0;
        self.challenge_id = 0;
        self.score = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Challenge {
        static instance: Challenge = Challenge {
            stars: 0,
            ext_info: ::protobuf::MessageField::none(),
            taken_reward: 0,
            challenge_id: 0,
            score: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Challenge {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Challenge").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Challenge {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Challenge {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fChallenge.proto\x1a\x16ChallengeExtInfo.proto\"\xab\x01\n\tChallen\
    ge\x12\x14\n\x05stars\x18\x01\x20\x01(\rR\x05stars\x12,\n\x08ext_info\
    \x18\x05\x20\x01(\x0b2\x11.ChallengeExtInfoR\x07extInfo\x12!\n\x0ctaken_\
    reward\x18\x06\x20\x01(\rR\x0btakenReward\x12!\n\x0cchallenge_id\x18\n\
    \x20\x01(\rR\x0bchallengeId\x12\x14\n\x05score\x18\x07\x20\x01(\rR\x05sc\
    oreB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            deps.push(super::ChallengeExtInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Challenge::generated_message_descriptor_data());
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
