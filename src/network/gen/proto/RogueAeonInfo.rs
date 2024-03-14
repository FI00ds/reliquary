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

//! Generated file from `RogueAeonInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RogueAeonInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueAeonInfo {
    // message fields
    // @@protoc_insertion_point(field:RogueAeonInfo.aeon_id_list)
    pub aeon_id_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:RogueAeonInfo.is_unlocked)
    pub is_unlocked: bool,
    // @@protoc_insertion_point(field:RogueAeonInfo.unlock_aeon_num)
    pub unlock_aeon_num: u32,
    // @@protoc_insertion_point(field:RogueAeonInfo.JPEBPGIEGPO)
    pub JPEBPGIEGPO: u32,
    // special fields
    // @@protoc_insertion_point(special_field:RogueAeonInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueAeonInfo {
    fn default() -> &'a RogueAeonInfo {
        <RogueAeonInfo as ::protobuf::Message>::default_instance()
    }
}

impl RogueAeonInfo {
    pub fn new() -> RogueAeonInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "aeon_id_list",
            |m: &RogueAeonInfo| { &m.aeon_id_list },
            |m: &mut RogueAeonInfo| { &mut m.aeon_id_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_unlocked",
            |m: &RogueAeonInfo| { &m.is_unlocked },
            |m: &mut RogueAeonInfo| { &mut m.is_unlocked },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "unlock_aeon_num",
            |m: &RogueAeonInfo| { &m.unlock_aeon_num },
            |m: &mut RogueAeonInfo| { &mut m.unlock_aeon_num },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JPEBPGIEGPO",
            |m: &RogueAeonInfo| { &m.JPEBPGIEGPO },
            |m: &mut RogueAeonInfo| { &mut m.JPEBPGIEGPO },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueAeonInfo>(
            "RogueAeonInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueAeonInfo {
    const NAME: &'static str = "RogueAeonInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                66 => {
                    is.read_repeated_packed_uint32_into(&mut self.aeon_id_list)?;
                },
                64 => {
                    self.aeon_id_list.push(is.read_uint32()?);
                },
                8 => {
                    self.is_unlocked = is.read_bool()?;
                },
                120 => {
                    self.unlock_aeon_num = is.read_uint32()?;
                },
                112 => {
                    self.JPEBPGIEGPO = is.read_uint32()?;
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
        for value in &self.aeon_id_list {
            my_size += ::protobuf::rt::uint32_size(8, *value);
        };
        if self.is_unlocked != false {
            my_size += 1 + 1;
        }
        if self.unlock_aeon_num != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.unlock_aeon_num);
        }
        if self.JPEBPGIEGPO != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.JPEBPGIEGPO);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.aeon_id_list {
            os.write_uint32(8, *v)?;
        };
        if self.is_unlocked != false {
            os.write_bool(1, self.is_unlocked)?;
        }
        if self.unlock_aeon_num != 0 {
            os.write_uint32(15, self.unlock_aeon_num)?;
        }
        if self.JPEBPGIEGPO != 0 {
            os.write_uint32(14, self.JPEBPGIEGPO)?;
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

    fn new() -> RogueAeonInfo {
        RogueAeonInfo::new()
    }

    fn clear(&mut self) {
        self.aeon_id_list.clear();
        self.is_unlocked = false;
        self.unlock_aeon_num = 0;
        self.JPEBPGIEGPO = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueAeonInfo {
        static instance: RogueAeonInfo = RogueAeonInfo {
            aeon_id_list: ::std::vec::Vec::new(),
            is_unlocked: false,
            unlock_aeon_num: 0,
            JPEBPGIEGPO: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RogueAeonInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueAeonInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueAeonInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueAeonInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13RogueAeonInfo.proto\"\x9c\x01\n\rRogueAeonInfo\x12\x20\n\x0caeon_i\
    d_list\x18\x08\x20\x03(\rR\naeonIdList\x12\x1f\n\x0bis_unlocked\x18\x01\
    \x20\x01(\x08R\nisUnlocked\x12&\n\x0funlock_aeon_num\x18\x0f\x20\x01(\rR\
    \runlockAeonNum\x12\x20\n\x0bJPEBPGIEGPO\x18\x0e\x20\x01(\rR\x0bJPEBPGIE\
    GPOB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RogueAeonInfo::generated_message_descriptor_data());
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
