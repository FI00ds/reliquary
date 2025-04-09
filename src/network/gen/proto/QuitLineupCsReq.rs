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

//! Generated file from `QuitLineupCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:QuitLineupCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct QuitLineupCsReq {
    // message fields
    // @@protoc_insertion_point(field:QuitLineupCsReq.IHLEAMDIKKN)
    pub IHLEAMDIKKN: ::protobuf::EnumOrUnknown<super::AvatarType::AvatarType>,
    // @@protoc_insertion_point(field:QuitLineupCsReq.NJDMFPFKKIH)
    pub NJDMFPFKKIH: ::protobuf::EnumOrUnknown<super::ExtraLineupType::ExtraLineupType>,
    // @@protoc_insertion_point(field:QuitLineupCsReq.EGMAFIOOKJJ)
    pub EGMAFIOOKJJ: u32,
    // @@protoc_insertion_point(field:QuitLineupCsReq.PDONLOOBBCI)
    pub PDONLOOBBCI: u32,
    // @@protoc_insertion_point(field:QuitLineupCsReq.base_avatar_id)
    pub base_avatar_id: u32,
    // @@protoc_insertion_point(field:QuitLineupCsReq.HCDNLLHBBOK)
    pub HCDNLLHBBOK: bool,
    // special fields
    // @@protoc_insertion_point(special_field:QuitLineupCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a QuitLineupCsReq {
    fn default() -> &'a QuitLineupCsReq {
        <QuitLineupCsReq as ::protobuf::Message>::default_instance()
    }
}

impl QuitLineupCsReq {
    pub fn new() -> QuitLineupCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IHLEAMDIKKN",
            |m: &QuitLineupCsReq| { &m.IHLEAMDIKKN },
            |m: &mut QuitLineupCsReq| { &mut m.IHLEAMDIKKN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NJDMFPFKKIH",
            |m: &QuitLineupCsReq| { &m.NJDMFPFKKIH },
            |m: &mut QuitLineupCsReq| { &mut m.NJDMFPFKKIH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EGMAFIOOKJJ",
            |m: &QuitLineupCsReq| { &m.EGMAFIOOKJJ },
            |m: &mut QuitLineupCsReq| { &mut m.EGMAFIOOKJJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PDONLOOBBCI",
            |m: &QuitLineupCsReq| { &m.PDONLOOBBCI },
            |m: &mut QuitLineupCsReq| { &mut m.PDONLOOBBCI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "base_avatar_id",
            |m: &QuitLineupCsReq| { &m.base_avatar_id },
            |m: &mut QuitLineupCsReq| { &mut m.base_avatar_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HCDNLLHBBOK",
            |m: &QuitLineupCsReq| { &m.HCDNLLHBBOK },
            |m: &mut QuitLineupCsReq| { &mut m.HCDNLLHBBOK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<QuitLineupCsReq>(
            "QuitLineupCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for QuitLineupCsReq {
    const NAME: &'static str = "QuitLineupCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.IHLEAMDIKKN = is.read_enum_or_unknown()?;
                },
                80 => {
                    self.NJDMFPFKKIH = is.read_enum_or_unknown()?;
                },
                40 => {
                    self.EGMAFIOOKJJ = is.read_uint32()?;
                },
                16 => {
                    self.PDONLOOBBCI = is.read_uint32()?;
                },
                24 => {
                    self.base_avatar_id = is.read_uint32()?;
                },
                32 => {
                    self.HCDNLLHBBOK = is.read_bool()?;
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
        if self.IHLEAMDIKKN != ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(9, self.IHLEAMDIKKN.value());
        }
        if self.NJDMFPFKKIH != ::protobuf::EnumOrUnknown::new(super::ExtraLineupType::ExtraLineupType::LINEUP_NONE) {
            my_size += ::protobuf::rt::int32_size(10, self.NJDMFPFKKIH.value());
        }
        if self.EGMAFIOOKJJ != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.EGMAFIOOKJJ);
        }
        if self.PDONLOOBBCI != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.PDONLOOBBCI);
        }
        if self.base_avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.base_avatar_id);
        }
        if self.HCDNLLHBBOK != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IHLEAMDIKKN != ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE) {
            os.write_enum(9, ::protobuf::EnumOrUnknown::value(&self.IHLEAMDIKKN))?;
        }
        if self.NJDMFPFKKIH != ::protobuf::EnumOrUnknown::new(super::ExtraLineupType::ExtraLineupType::LINEUP_NONE) {
            os.write_enum(10, ::protobuf::EnumOrUnknown::value(&self.NJDMFPFKKIH))?;
        }
        if self.EGMAFIOOKJJ != 0 {
            os.write_uint32(5, self.EGMAFIOOKJJ)?;
        }
        if self.PDONLOOBBCI != 0 {
            os.write_uint32(2, self.PDONLOOBBCI)?;
        }
        if self.base_avatar_id != 0 {
            os.write_uint32(3, self.base_avatar_id)?;
        }
        if self.HCDNLLHBBOK != false {
            os.write_bool(4, self.HCDNLLHBBOK)?;
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

    fn new() -> QuitLineupCsReq {
        QuitLineupCsReq::new()
    }

    fn clear(&mut self) {
        self.IHLEAMDIKKN = ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE);
        self.NJDMFPFKKIH = ::protobuf::EnumOrUnknown::new(super::ExtraLineupType::ExtraLineupType::LINEUP_NONE);
        self.EGMAFIOOKJJ = 0;
        self.PDONLOOBBCI = 0;
        self.base_avatar_id = 0;
        self.HCDNLLHBBOK = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static QuitLineupCsReq {
        static instance: QuitLineupCsReq = QuitLineupCsReq {
            IHLEAMDIKKN: ::protobuf::EnumOrUnknown::from_i32(0),
            NJDMFPFKKIH: ::protobuf::EnumOrUnknown::from_i32(0),
            EGMAFIOOKJJ: 0,
            PDONLOOBBCI: 0,
            base_avatar_id: 0,
            HCDNLLHBBOK: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for QuitLineupCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("QuitLineupCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for QuitLineupCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QuitLineupCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15QuitLineupCsReq.proto\x1a\x10AvatarType.proto\x1a\x15ExtraLineupTy\
    pe.proto\"\x80\x02\n\x0fQuitLineupCsReq\x12-\n\x0bIHLEAMDIKKN\x18\t\x20\
    \x01(\x0e2\x0b.AvatarTypeR\x0bIHLEAMDIKKN\x122\n\x0bNJDMFPFKKIH\x18\n\
    \x20\x01(\x0e2\x10.ExtraLineupTypeR\x0bNJDMFPFKKIH\x12\x20\n\x0bEGMAFIOO\
    KJJ\x18\x05\x20\x01(\rR\x0bEGMAFIOOKJJ\x12\x20\n\x0bPDONLOOBBCI\x18\x02\
    \x20\x01(\rR\x0bPDONLOOBBCI\x12$\n\x0ebase_avatar_id\x18\x03\x20\x01(\rR\
    \x0cbaseAvatarId\x12\x20\n\x0bHCDNLLHBBOK\x18\x04\x20\x01(\x08R\x0bHCDNL\
    LHBBOKb\x06proto3\
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
            deps.push(super::AvatarType::file_descriptor().clone());
            deps.push(super::ExtraLineupType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(QuitLineupCsReq::generated_message_descriptor_data());
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
