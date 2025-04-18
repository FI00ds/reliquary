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

//! Generated file from `HeartDialScriptChangeScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:HeartDialScriptChangeScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HeartDialScriptChangeScNotify {
    // message fields
    // @@protoc_insertion_point(field:HeartDialScriptChangeScNotify.OCMOEJIDLAM)
    pub OCMOEJIDLAM: ::std::vec::Vec<super::MMEINFMDJFG::MMEINFMDJFG>,
    // @@protoc_insertion_point(field:HeartDialScriptChangeScNotify.NFEOJAAJMKE)
    pub NFEOJAAJMKE: ::protobuf::EnumOrUnknown<super::OOEHGMEMKOI::OOEHGMEMKOI>,
    // @@protoc_insertion_point(field:HeartDialScriptChangeScNotify.PKFLGDFDMOI)
    pub PKFLGDFDMOI: ::std::vec::Vec<super::MAIABOOMMNN::MAIABOOMMNN>,
    // @@protoc_insertion_point(field:HeartDialScriptChangeScNotify.NGLHCMBGHIO)
    pub NGLHCMBGHIO: ::std::vec::Vec<super::OICENKLJICG::OICENKLJICG>,
    // special fields
    // @@protoc_insertion_point(special_field:HeartDialScriptChangeScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HeartDialScriptChangeScNotify {
    fn default() -> &'a HeartDialScriptChangeScNotify {
        <HeartDialScriptChangeScNotify as ::protobuf::Message>::default_instance()
    }
}

impl HeartDialScriptChangeScNotify {
    pub fn new() -> HeartDialScriptChangeScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "OCMOEJIDLAM",
            |m: &HeartDialScriptChangeScNotify| { &m.OCMOEJIDLAM },
            |m: &mut HeartDialScriptChangeScNotify| { &mut m.OCMOEJIDLAM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NFEOJAAJMKE",
            |m: &HeartDialScriptChangeScNotify| { &m.NFEOJAAJMKE },
            |m: &mut HeartDialScriptChangeScNotify| { &mut m.NFEOJAAJMKE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PKFLGDFDMOI",
            |m: &HeartDialScriptChangeScNotify| { &m.PKFLGDFDMOI },
            |m: &mut HeartDialScriptChangeScNotify| { &mut m.PKFLGDFDMOI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NGLHCMBGHIO",
            |m: &HeartDialScriptChangeScNotify| { &m.NGLHCMBGHIO },
            |m: &mut HeartDialScriptChangeScNotify| { &mut m.NGLHCMBGHIO },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HeartDialScriptChangeScNotify>(
            "HeartDialScriptChangeScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HeartDialScriptChangeScNotify {
    const NAME: &'static str = "HeartDialScriptChangeScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                50 => {
                    self.OCMOEJIDLAM.push(is.read_message()?);
                },
                8 => {
                    self.NFEOJAAJMKE = is.read_enum_or_unknown()?;
                },
                122 => {
                    self.PKFLGDFDMOI.push(is.read_message()?);
                },
                18 => {
                    self.NGLHCMBGHIO.push(is.read_message()?);
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
        for value in &self.OCMOEJIDLAM {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.NFEOJAAJMKE != ::protobuf::EnumOrUnknown::new(super::OOEHGMEMKOI::OOEHGMEMKOI::HEART_DIAL_UNLOCK_STATUS_LOCK) {
            my_size += ::protobuf::rt::int32_size(1, self.NFEOJAAJMKE.value());
        }
        for value in &self.PKFLGDFDMOI {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.NGLHCMBGHIO {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.OCMOEJIDLAM {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        };
        if self.NFEOJAAJMKE != ::protobuf::EnumOrUnknown::new(super::OOEHGMEMKOI::OOEHGMEMKOI::HEART_DIAL_UNLOCK_STATUS_LOCK) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.NFEOJAAJMKE))?;
        }
        for v in &self.PKFLGDFDMOI {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        };
        for v in &self.NGLHCMBGHIO {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
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

    fn new() -> HeartDialScriptChangeScNotify {
        HeartDialScriptChangeScNotify::new()
    }

    fn clear(&mut self) {
        self.OCMOEJIDLAM.clear();
        self.NFEOJAAJMKE = ::protobuf::EnumOrUnknown::new(super::OOEHGMEMKOI::OOEHGMEMKOI::HEART_DIAL_UNLOCK_STATUS_LOCK);
        self.PKFLGDFDMOI.clear();
        self.NGLHCMBGHIO.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HeartDialScriptChangeScNotify {
        static instance: HeartDialScriptChangeScNotify = HeartDialScriptChangeScNotify {
            OCMOEJIDLAM: ::std::vec::Vec::new(),
            NFEOJAAJMKE: ::protobuf::EnumOrUnknown::from_i32(0),
            PKFLGDFDMOI: ::std::vec::Vec::new(),
            NGLHCMBGHIO: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HeartDialScriptChangeScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HeartDialScriptChangeScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HeartDialScriptChangeScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HeartDialScriptChangeScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n#HeartDialScriptChangeScNotify.proto\x1a\x11MAIABOOMMNN.proto\x1a\x11M\
    MEINFMDJFG.proto\x1a\x11OICENKLJICG.proto\x1a\x11OOEHGMEMKOI.proto\"\xdf\
    \x01\n\x1dHeartDialScriptChangeScNotify\x12.\n\x0bOCMOEJIDLAM\x18\x06\
    \x20\x03(\x0b2\x0c.MMEINFMDJFGR\x0bOCMOEJIDLAM\x12.\n\x0bNFEOJAAJMKE\x18\
    \x01\x20\x01(\x0e2\x0c.OOEHGMEMKOIR\x0bNFEOJAAJMKE\x12.\n\x0bPKFLGDFDMOI\
    \x18\x0f\x20\x03(\x0b2\x0c.MAIABOOMMNNR\x0bPKFLGDFDMOI\x12.\n\x0bNGLHCMB\
    GHIO\x18\x02\x20\x03(\x0b2\x0c.OICENKLJICGR\x0bNGLHCMBGHIOb\x06proto3\
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
            deps.push(super::MAIABOOMMNN::file_descriptor().clone());
            deps.push(super::MMEINFMDJFG::file_descriptor().clone());
            deps.push(super::OICENKLJICG::file_descriptor().clone());
            deps.push(super::OOEHGMEMKOI::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(HeartDialScriptChangeScNotify::generated_message_descriptor_data());
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
