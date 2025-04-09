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

//! Generated file from `GLADGJAKPLC.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:GLADGJAKPLC)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GLADGJAKPLC {
    // message fields
    // @@protoc_insertion_point(field:GLADGJAKPLC.MDBEDJLDPGM)
    pub MDBEDJLDPGM: u32,
    // @@protoc_insertion_point(field:GLADGJAKPLC.HNLFGDMCJDF)
    pub HNLFGDMCJDF: ::protobuf::EnumOrUnknown<super::RogueRoomStatus::RogueRoomStatus>,
    // @@protoc_insertion_point(field:GLADGJAKPLC.JJLOGICPOAO)
    pub JJLOGICPOAO: u32,
    // @@protoc_insertion_point(field:GLADGJAKPLC.BEEEBOIOJIF)
    pub BEEEBOIOJIF: ::protobuf::EnumOrUnknown<super::RogueRoomStatus::RogueRoomStatus>,
    // @@protoc_insertion_point(field:GLADGJAKPLC.IMIMGFAAGHM)
    pub IMIMGFAAGHM: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GLADGJAKPLC.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GLADGJAKPLC {
    fn default() -> &'a GLADGJAKPLC {
        <GLADGJAKPLC as ::protobuf::Message>::default_instance()
    }
}

impl GLADGJAKPLC {
    pub fn new() -> GLADGJAKPLC {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MDBEDJLDPGM",
            |m: &GLADGJAKPLC| { &m.MDBEDJLDPGM },
            |m: &mut GLADGJAKPLC| { &mut m.MDBEDJLDPGM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HNLFGDMCJDF",
            |m: &GLADGJAKPLC| { &m.HNLFGDMCJDF },
            |m: &mut GLADGJAKPLC| { &mut m.HNLFGDMCJDF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JJLOGICPOAO",
            |m: &GLADGJAKPLC| { &m.JJLOGICPOAO },
            |m: &mut GLADGJAKPLC| { &mut m.JJLOGICPOAO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BEEEBOIOJIF",
            |m: &GLADGJAKPLC| { &m.BEEEBOIOJIF },
            |m: &mut GLADGJAKPLC| { &mut m.BEEEBOIOJIF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IMIMGFAAGHM",
            |m: &GLADGJAKPLC| { &m.IMIMGFAAGHM },
            |m: &mut GLADGJAKPLC| { &mut m.IMIMGFAAGHM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GLADGJAKPLC>(
            "GLADGJAKPLC",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GLADGJAKPLC {
    const NAME: &'static str = "GLADGJAKPLC";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.MDBEDJLDPGM = is.read_uint32()?;
                },
                24 => {
                    self.HNLFGDMCJDF = is.read_enum_or_unknown()?;
                },
                16 => {
                    self.JJLOGICPOAO = is.read_uint32()?;
                },
                112 => {
                    self.BEEEBOIOJIF = is.read_enum_or_unknown()?;
                },
                8 => {
                    self.IMIMGFAAGHM = is.read_uint32()?;
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
        if self.MDBEDJLDPGM != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.MDBEDJLDPGM);
        }
        if self.HNLFGDMCJDF != ::protobuf::EnumOrUnknown::new(super::RogueRoomStatus::RogueRoomStatus::ROGUE_ROOM_STATUS_NONE) {
            my_size += ::protobuf::rt::int32_size(3, self.HNLFGDMCJDF.value());
        }
        if self.JJLOGICPOAO != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.JJLOGICPOAO);
        }
        if self.BEEEBOIOJIF != ::protobuf::EnumOrUnknown::new(super::RogueRoomStatus::RogueRoomStatus::ROGUE_ROOM_STATUS_NONE) {
            my_size += ::protobuf::rt::int32_size(14, self.BEEEBOIOJIF.value());
        }
        if self.IMIMGFAAGHM != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.IMIMGFAAGHM);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.MDBEDJLDPGM != 0 {
            os.write_uint32(7, self.MDBEDJLDPGM)?;
        }
        if self.HNLFGDMCJDF != ::protobuf::EnumOrUnknown::new(super::RogueRoomStatus::RogueRoomStatus::ROGUE_ROOM_STATUS_NONE) {
            os.write_enum(3, ::protobuf::EnumOrUnknown::value(&self.HNLFGDMCJDF))?;
        }
        if self.JJLOGICPOAO != 0 {
            os.write_uint32(2, self.JJLOGICPOAO)?;
        }
        if self.BEEEBOIOJIF != ::protobuf::EnumOrUnknown::new(super::RogueRoomStatus::RogueRoomStatus::ROGUE_ROOM_STATUS_NONE) {
            os.write_enum(14, ::protobuf::EnumOrUnknown::value(&self.BEEEBOIOJIF))?;
        }
        if self.IMIMGFAAGHM != 0 {
            os.write_uint32(1, self.IMIMGFAAGHM)?;
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

    fn new() -> GLADGJAKPLC {
        GLADGJAKPLC::new()
    }

    fn clear(&mut self) {
        self.MDBEDJLDPGM = 0;
        self.HNLFGDMCJDF = ::protobuf::EnumOrUnknown::new(super::RogueRoomStatus::RogueRoomStatus::ROGUE_ROOM_STATUS_NONE);
        self.JJLOGICPOAO = 0;
        self.BEEEBOIOJIF = ::protobuf::EnumOrUnknown::new(super::RogueRoomStatus::RogueRoomStatus::ROGUE_ROOM_STATUS_NONE);
        self.IMIMGFAAGHM = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GLADGJAKPLC {
        static instance: GLADGJAKPLC = GLADGJAKPLC {
            MDBEDJLDPGM: 0,
            HNLFGDMCJDF: ::protobuf::EnumOrUnknown::from_i32(0),
            JJLOGICPOAO: 0,
            BEEEBOIOJIF: ::protobuf::EnumOrUnknown::from_i32(0),
            IMIMGFAAGHM: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GLADGJAKPLC {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GLADGJAKPLC").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GLADGJAKPLC {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GLADGJAKPLC {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11GLADGJAKPLC.proto\x1a\x15RogueRoomStatus.proto\"\xdb\x01\n\x0bGLAD\
    GJAKPLC\x12\x20\n\x0bMDBEDJLDPGM\x18\x07\x20\x01(\rR\x0bMDBEDJLDPGM\x122\
    \n\x0bHNLFGDMCJDF\x18\x03\x20\x01(\x0e2\x10.RogueRoomStatusR\x0bHNLFGDMC\
    JDF\x12\x20\n\x0bJJLOGICPOAO\x18\x02\x20\x01(\rR\x0bJJLOGICPOAO\x122\n\
    \x0bBEEEBOIOJIF\x18\x0e\x20\x01(\x0e2\x10.RogueRoomStatusR\x0bBEEEBOIOJI\
    F\x12\x20\n\x0bIMIMGFAAGHM\x18\x01\x20\x01(\rR\x0bIMIMGFAAGHMb\x06proto3\
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
            deps.push(super::RogueRoomStatus::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GLADGJAKPLC::generated_message_descriptor_data());
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
