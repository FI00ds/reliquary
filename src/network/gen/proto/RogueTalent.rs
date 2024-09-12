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

//! Generated file from `RogueTalent.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RogueTalent)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueTalent {
    // message fields
    // @@protoc_insertion_point(field:RogueTalent.status)
    pub status: ::protobuf::EnumOrUnknown<super::RogueTalentStatus::RogueTalentStatus>,
    // @@protoc_insertion_point(field:RogueTalent.unlock_progress_list)
    pub unlock_progress_list: ::std::vec::Vec<super::RogueUnlockProgress::RogueUnlockProgress>,
    // @@protoc_insertion_point(field:RogueTalent.talent_id)
    pub talent_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:RogueTalent.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueTalent {
    fn default() -> &'a RogueTalent {
        <RogueTalent as ::protobuf::Message>::default_instance()
    }
}

impl RogueTalent {
    pub fn new() -> RogueTalent {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "status",
            |m: &RogueTalent| { &m.status },
            |m: &mut RogueTalent| { &mut m.status },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "unlock_progress_list",
            |m: &RogueTalent| { &m.unlock_progress_list },
            |m: &mut RogueTalent| { &mut m.unlock_progress_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "talent_id",
            |m: &RogueTalent| { &m.talent_id },
            |m: &mut RogueTalent| { &mut m.talent_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueTalent>(
            "RogueTalent",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueTalent {
    const NAME: &'static str = "RogueTalent";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.status = is.read_enum_or_unknown()?;
                },
                50 => {
                    self.unlock_progress_list.push(is.read_message()?);
                },
                40 => {
                    self.talent_id = is.read_uint32()?;
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
        if self.status != ::protobuf::EnumOrUnknown::new(super::RogueTalentStatus::RogueTalentStatus::ROGUE_TALENT_STATUS_LOCK) {
            my_size += ::protobuf::rt::int32_size(15, self.status.value());
        }
        for value in &self.unlock_progress_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.talent_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.talent_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.status != ::protobuf::EnumOrUnknown::new(super::RogueTalentStatus::RogueTalentStatus::ROGUE_TALENT_STATUS_LOCK) {
            os.write_enum(15, ::protobuf::EnumOrUnknown::value(&self.status))?;
        }
        for v in &self.unlock_progress_list {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        };
        if self.talent_id != 0 {
            os.write_uint32(5, self.talent_id)?;
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

    fn new() -> RogueTalent {
        RogueTalent::new()
    }

    fn clear(&mut self) {
        self.status = ::protobuf::EnumOrUnknown::new(super::RogueTalentStatus::RogueTalentStatus::ROGUE_TALENT_STATUS_LOCK);
        self.unlock_progress_list.clear();
        self.talent_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueTalent {
        static instance: RogueTalent = RogueTalent {
            status: ::protobuf::EnumOrUnknown::from_i32(0),
            unlock_progress_list: ::std::vec::Vec::new(),
            talent_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RogueTalent {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueTalent").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueTalent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueTalent {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11RogueTalent.proto\x1a\x17RogueTalentStatus.proto\x1a\x19RogueUnloc\
    kProgress.proto\"\x9e\x01\n\x0bRogueTalent\x12*\n\x06status\x18\x0f\x20\
    \x01(\x0e2\x12.RogueTalentStatusR\x06status\x12F\n\x14unlock_progress_li\
    st\x18\x06\x20\x03(\x0b2\x14.RogueUnlockProgressR\x12unlockProgressList\
    \x12\x1b\n\ttalent_id\x18\x05\x20\x01(\rR\x08talentIdB\x15\n\x13emu.luna\
    rcore.protob\x06proto3\
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
            messages.push(RogueTalent::generated_message_descriptor_data());
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
