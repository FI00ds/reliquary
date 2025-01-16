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

//! Generated file from `LobbyGetInfoScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:LobbyGetInfoScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LobbyGetInfoScRsp {
    // message fields
    // @@protoc_insertion_point(field:LobbyGetInfoScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:LobbyGetInfoScRsp.FDBOEOGNPFB)
    pub FDBOEOGNPFB: u64,
    // @@protoc_insertion_point(field:LobbyGetInfoScRsp.MGGMPKIGHBM)
    pub MGGMPKIGHBM: ::protobuf::EnumOrUnknown<super::FightGameMode::FightGameMode>,
    // @@protoc_insertion_point(field:LobbyGetInfoScRsp.FMJCBKLEHDO)
    pub FMJCBKLEHDO: u64,
    // @@protoc_insertion_point(field:LobbyGetInfoScRsp.LHIFALKBALM)
    pub LHIFALKBALM: ::std::vec::Vec<super::GDOHNMGABGE::GDOHNMGABGE>,
    // special fields
    // @@protoc_insertion_point(special_field:LobbyGetInfoScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LobbyGetInfoScRsp {
    fn default() -> &'a LobbyGetInfoScRsp {
        <LobbyGetInfoScRsp as ::protobuf::Message>::default_instance()
    }
}

impl LobbyGetInfoScRsp {
    pub fn new() -> LobbyGetInfoScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &LobbyGetInfoScRsp| { &m.retcode },
            |m: &mut LobbyGetInfoScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FDBOEOGNPFB",
            |m: &LobbyGetInfoScRsp| { &m.FDBOEOGNPFB },
            |m: &mut LobbyGetInfoScRsp| { &mut m.FDBOEOGNPFB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MGGMPKIGHBM",
            |m: &LobbyGetInfoScRsp| { &m.MGGMPKIGHBM },
            |m: &mut LobbyGetInfoScRsp| { &mut m.MGGMPKIGHBM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FMJCBKLEHDO",
            |m: &LobbyGetInfoScRsp| { &m.FMJCBKLEHDO },
            |m: &mut LobbyGetInfoScRsp| { &mut m.FMJCBKLEHDO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LHIFALKBALM",
            |m: &LobbyGetInfoScRsp| { &m.LHIFALKBALM },
            |m: &mut LobbyGetInfoScRsp| { &mut m.LHIFALKBALM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LobbyGetInfoScRsp>(
            "LobbyGetInfoScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LobbyGetInfoScRsp {
    const NAME: &'static str = "LobbyGetInfoScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.retcode = is.read_uint32()?;
                },
                56 => {
                    self.FDBOEOGNPFB = is.read_uint64()?;
                },
                80 => {
                    self.MGGMPKIGHBM = is.read_enum_or_unknown()?;
                },
                72 => {
                    self.FMJCBKLEHDO = is.read_uint64()?;
                },
                66 => {
                    self.LHIFALKBALM.push(is.read_message()?);
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
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.retcode);
        }
        if self.FDBOEOGNPFB != 0 {
            my_size += ::protobuf::rt::uint64_size(7, self.FDBOEOGNPFB);
        }
        if self.MGGMPKIGHBM != ::protobuf::EnumOrUnknown::new(super::FightGameMode::FightGameMode::FIGHT_GAME_MODE_NONE) {
            my_size += ::protobuf::rt::int32_size(10, self.MGGMPKIGHBM.value());
        }
        if self.FMJCBKLEHDO != 0 {
            my_size += ::protobuf::rt::uint64_size(9, self.FMJCBKLEHDO);
        }
        for value in &self.LHIFALKBALM {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.retcode != 0 {
            os.write_uint32(11, self.retcode)?;
        }
        if self.FDBOEOGNPFB != 0 {
            os.write_uint64(7, self.FDBOEOGNPFB)?;
        }
        if self.MGGMPKIGHBM != ::protobuf::EnumOrUnknown::new(super::FightGameMode::FightGameMode::FIGHT_GAME_MODE_NONE) {
            os.write_enum(10, ::protobuf::EnumOrUnknown::value(&self.MGGMPKIGHBM))?;
        }
        if self.FMJCBKLEHDO != 0 {
            os.write_uint64(9, self.FMJCBKLEHDO)?;
        }
        for v in &self.LHIFALKBALM {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
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

    fn new() -> LobbyGetInfoScRsp {
        LobbyGetInfoScRsp::new()
    }

    fn clear(&mut self) {
        self.retcode = 0;
        self.FDBOEOGNPFB = 0;
        self.MGGMPKIGHBM = ::protobuf::EnumOrUnknown::new(super::FightGameMode::FightGameMode::FIGHT_GAME_MODE_NONE);
        self.FMJCBKLEHDO = 0;
        self.LHIFALKBALM.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LobbyGetInfoScRsp {
        static instance: LobbyGetInfoScRsp = LobbyGetInfoScRsp {
            retcode: 0,
            FDBOEOGNPFB: 0,
            MGGMPKIGHBM: ::protobuf::EnumOrUnknown::from_i32(0),
            FMJCBKLEHDO: 0,
            LHIFALKBALM: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LobbyGetInfoScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LobbyGetInfoScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LobbyGetInfoScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LobbyGetInfoScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17LobbyGetInfoScRsp.proto\x1a\x13FightGameMode.proto\x1a\x11GDOHNMGA\
    BGE.proto\"\xd3\x01\n\x11LobbyGetInfoScRsp\x12\x18\n\x07retcode\x18\x0b\
    \x20\x01(\rR\x07retcode\x12\x20\n\x0bFDBOEOGNPFB\x18\x07\x20\x01(\x04R\
    \x0bFDBOEOGNPFB\x120\n\x0bMGGMPKIGHBM\x18\n\x20\x01(\x0e2\x0e.FightGameM\
    odeR\x0bMGGMPKIGHBM\x12\x20\n\x0bFMJCBKLEHDO\x18\t\x20\x01(\x04R\x0bFMJC\
    BKLEHDO\x12.\n\x0bLHIFALKBALM\x18\x08\x20\x03(\x0b2\x0c.GDOHNMGABGER\x0b\
    LHIFALKBALMb\x06proto3\
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
            deps.push(super::FightGameMode::file_descriptor().clone());
            deps.push(super::GDOHNMGABGE::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LobbyGetInfoScRsp::generated_message_descriptor_data());
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
