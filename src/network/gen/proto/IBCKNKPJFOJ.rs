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

//! Generated file from `IBCKNKPJFOJ.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:IBCKNKPJFOJ)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct IBCKNKPJFOJ {
    // message fields
    // @@protoc_insertion_point(field:IBCKNKPJFOJ.FAGNLGFJAIF)
    pub FAGNLGFJAIF: u32,
    // @@protoc_insertion_point(field:IBCKNKPJFOJ.PGIMNCFHJEA)
    pub PGIMNCFHJEA: ::std::vec::Vec<super::RogueUnlockProgress::RogueUnlockProgress>,
    // @@protoc_insertion_point(field:IBCKNKPJFOJ.PBLFLJNHMIL)
    pub PBLFLJNHMIL: ::protobuf::EnumOrUnknown<super::RogueTalentStatus::RogueTalentStatus>,
    // special fields
    // @@protoc_insertion_point(special_field:IBCKNKPJFOJ.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a IBCKNKPJFOJ {
    fn default() -> &'a IBCKNKPJFOJ {
        <IBCKNKPJFOJ as ::protobuf::Message>::default_instance()
    }
}

impl IBCKNKPJFOJ {
    pub fn new() -> IBCKNKPJFOJ {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FAGNLGFJAIF",
            |m: &IBCKNKPJFOJ| { &m.FAGNLGFJAIF },
            |m: &mut IBCKNKPJFOJ| { &mut m.FAGNLGFJAIF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PGIMNCFHJEA",
            |m: &IBCKNKPJFOJ| { &m.PGIMNCFHJEA },
            |m: &mut IBCKNKPJFOJ| { &mut m.PGIMNCFHJEA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PBLFLJNHMIL",
            |m: &IBCKNKPJFOJ| { &m.PBLFLJNHMIL },
            |m: &mut IBCKNKPJFOJ| { &mut m.PBLFLJNHMIL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<IBCKNKPJFOJ>(
            "IBCKNKPJFOJ",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for IBCKNKPJFOJ {
    const NAME: &'static str = "IBCKNKPJFOJ";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                112 => {
                    self.FAGNLGFJAIF = is.read_uint32()?;
                },
                74 => {
                    self.PGIMNCFHJEA.push(is.read_message()?);
                },
                32 => {
                    self.PBLFLJNHMIL = is.read_enum_or_unknown()?;
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
        if self.FAGNLGFJAIF != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.FAGNLGFJAIF);
        }
        for value in &self.PGIMNCFHJEA {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.PBLFLJNHMIL != ::protobuf::EnumOrUnknown::new(super::RogueTalentStatus::RogueTalentStatus::ROGUE_TALENT_STATUS_LOCK) {
            my_size += ::protobuf::rt::int32_size(4, self.PBLFLJNHMIL.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.FAGNLGFJAIF != 0 {
            os.write_uint32(14, self.FAGNLGFJAIF)?;
        }
        for v in &self.PGIMNCFHJEA {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        };
        if self.PBLFLJNHMIL != ::protobuf::EnumOrUnknown::new(super::RogueTalentStatus::RogueTalentStatus::ROGUE_TALENT_STATUS_LOCK) {
            os.write_enum(4, ::protobuf::EnumOrUnknown::value(&self.PBLFLJNHMIL))?;
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

    fn new() -> IBCKNKPJFOJ {
        IBCKNKPJFOJ::new()
    }

    fn clear(&mut self) {
        self.FAGNLGFJAIF = 0;
        self.PGIMNCFHJEA.clear();
        self.PBLFLJNHMIL = ::protobuf::EnumOrUnknown::new(super::RogueTalentStatus::RogueTalentStatus::ROGUE_TALENT_STATUS_LOCK);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static IBCKNKPJFOJ {
        static instance: IBCKNKPJFOJ = IBCKNKPJFOJ {
            FAGNLGFJAIF: 0,
            PGIMNCFHJEA: ::std::vec::Vec::new(),
            PBLFLJNHMIL: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for IBCKNKPJFOJ {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("IBCKNKPJFOJ").unwrap()).clone()
    }
}

impl ::std::fmt::Display for IBCKNKPJFOJ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IBCKNKPJFOJ {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11IBCKNKPJFOJ.proto\x1a\x17RogueTalentStatus.proto\x1a\x19RogueUnloc\
    kProgress.proto\"\x9d\x01\n\x0bIBCKNKPJFOJ\x12\x20\n\x0bFAGNLGFJAIF\x18\
    \x0e\x20\x01(\rR\x0bFAGNLGFJAIF\x126\n\x0bPGIMNCFHJEA\x18\t\x20\x03(\x0b\
    2\x14.RogueUnlockProgressR\x0bPGIMNCFHJEA\x124\n\x0bPBLFLJNHMIL\x18\x04\
    \x20\x01(\x0e2\x12.RogueTalentStatusR\x0bPBLFLJNHMILb\x06proto3\
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
            deps.push(super::RogueTalentStatus::file_descriptor().clone());
            deps.push(super::RogueUnlockProgress::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(IBCKNKPJFOJ::generated_message_descriptor_data());
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
