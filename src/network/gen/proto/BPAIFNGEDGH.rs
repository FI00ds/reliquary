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

//! Generated file from `BPAIFNGEDGH.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:BPAIFNGEDGH)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BPAIFNGEDGH {
    // message fields
    // @@protoc_insertion_point(field:BPAIFNGEDGH.BHPGJCICMJM)
    pub BHPGJCICMJM: u32,
    // @@protoc_insertion_point(field:BPAIFNGEDGH.OGLDNEFKNDO)
    pub OGLDNEFKNDO: u32,
    // @@protoc_insertion_point(field:BPAIFNGEDGH.MNFJEIININL)
    pub MNFJEIININL: ::std::vec::Vec<super::OIIKGFIPMFG::OIIKGFIPMFG>,
    // @@protoc_insertion_point(field:BPAIFNGEDGH.PPJBGNBMKPM)
    pub PPJBGNBMKPM: u32,
    // @@protoc_insertion_point(field:BPAIFNGEDGH.PBLFLJNHMIL)
    pub PBLFLJNHMIL: ::protobuf::EnumOrUnknown<super::JLCBBKKGOEJ::JLCBBKKGOEJ>,
    // special fields
    // @@protoc_insertion_point(special_field:BPAIFNGEDGH.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BPAIFNGEDGH {
    fn default() -> &'a BPAIFNGEDGH {
        <BPAIFNGEDGH as ::protobuf::Message>::default_instance()
    }
}

impl BPAIFNGEDGH {
    pub fn new() -> BPAIFNGEDGH {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BHPGJCICMJM",
            |m: &BPAIFNGEDGH| { &m.BHPGJCICMJM },
            |m: &mut BPAIFNGEDGH| { &mut m.BHPGJCICMJM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OGLDNEFKNDO",
            |m: &BPAIFNGEDGH| { &m.OGLDNEFKNDO },
            |m: &mut BPAIFNGEDGH| { &mut m.OGLDNEFKNDO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MNFJEIININL",
            |m: &BPAIFNGEDGH| { &m.MNFJEIININL },
            |m: &mut BPAIFNGEDGH| { &mut m.MNFJEIININL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PPJBGNBMKPM",
            |m: &BPAIFNGEDGH| { &m.PPJBGNBMKPM },
            |m: &mut BPAIFNGEDGH| { &mut m.PPJBGNBMKPM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PBLFLJNHMIL",
            |m: &BPAIFNGEDGH| { &m.PBLFLJNHMIL },
            |m: &mut BPAIFNGEDGH| { &mut m.PBLFLJNHMIL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BPAIFNGEDGH>(
            "BPAIFNGEDGH",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BPAIFNGEDGH {
    const NAME: &'static str = "BPAIFNGEDGH";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.BHPGJCICMJM = is.read_uint32()?;
                },
                64 => {
                    self.OGLDNEFKNDO = is.read_uint32()?;
                },
                34 => {
                    self.MNFJEIININL.push(is.read_message()?);
                },
                8 => {
                    self.PPJBGNBMKPM = is.read_uint32()?;
                },
                96 => {
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
        if self.BHPGJCICMJM != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.BHPGJCICMJM);
        }
        if self.OGLDNEFKNDO != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.OGLDNEFKNDO);
        }
        for value in &self.MNFJEIININL {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.PPJBGNBMKPM != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.PPJBGNBMKPM);
        }
        if self.PBLFLJNHMIL != ::protobuf::EnumOrUnknown::new(super::JLCBBKKGOEJ::JLCBBKKGOEJ::ROGUE_MAGIC_LAYER_STATUS_NONE) {
            my_size += ::protobuf::rt::int32_size(12, self.PBLFLJNHMIL.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.BHPGJCICMJM != 0 {
            os.write_uint32(9, self.BHPGJCICMJM)?;
        }
        if self.OGLDNEFKNDO != 0 {
            os.write_uint32(8, self.OGLDNEFKNDO)?;
        }
        for v in &self.MNFJEIININL {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        if self.PPJBGNBMKPM != 0 {
            os.write_uint32(1, self.PPJBGNBMKPM)?;
        }
        if self.PBLFLJNHMIL != ::protobuf::EnumOrUnknown::new(super::JLCBBKKGOEJ::JLCBBKKGOEJ::ROGUE_MAGIC_LAYER_STATUS_NONE) {
            os.write_enum(12, ::protobuf::EnumOrUnknown::value(&self.PBLFLJNHMIL))?;
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

    fn new() -> BPAIFNGEDGH {
        BPAIFNGEDGH::new()
    }

    fn clear(&mut self) {
        self.BHPGJCICMJM = 0;
        self.OGLDNEFKNDO = 0;
        self.MNFJEIININL.clear();
        self.PPJBGNBMKPM = 0;
        self.PBLFLJNHMIL = ::protobuf::EnumOrUnknown::new(super::JLCBBKKGOEJ::JLCBBKKGOEJ::ROGUE_MAGIC_LAYER_STATUS_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BPAIFNGEDGH {
        static instance: BPAIFNGEDGH = BPAIFNGEDGH {
            BHPGJCICMJM: 0,
            OGLDNEFKNDO: 0,
            MNFJEIININL: ::std::vec::Vec::new(),
            PPJBGNBMKPM: 0,
            PBLFLJNHMIL: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BPAIFNGEDGH {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BPAIFNGEDGH").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BPAIFNGEDGH {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BPAIFNGEDGH {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11BPAIFNGEDGH.proto\x1a\x11JLCBBKKGOEJ.proto\x1a\x11OIIKGFIPMFG.prot\
    o\"\xd3\x01\n\x0bBPAIFNGEDGH\x12\x20\n\x0bBHPGJCICMJM\x18\t\x20\x01(\rR\
    \x0bBHPGJCICMJM\x12\x20\n\x0bOGLDNEFKNDO\x18\x08\x20\x01(\rR\x0bOGLDNEFK\
    NDO\x12.\n\x0bMNFJEIININL\x18\x04\x20\x03(\x0b2\x0c.OIIKGFIPMFGR\x0bMNFJ\
    EIININL\x12\x20\n\x0bPPJBGNBMKPM\x18\x01\x20\x01(\rR\x0bPPJBGNBMKPM\x12.\
    \n\x0bPBLFLJNHMIL\x18\x0c\x20\x01(\x0e2\x0c.JLCBBKKGOEJR\x0bPBLFLJNHMILb\
    \x06proto3\
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
            deps.push(super::JLCBBKKGOEJ::file_descriptor().clone());
            deps.push(super::OIIKGFIPMFG::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(BPAIFNGEDGH::generated_message_descriptor_data());
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
