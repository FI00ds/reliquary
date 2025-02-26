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

//! Generated file from `RogueMagicLevelInfoUpdateScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:RogueMagicLevelInfoUpdateScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueMagicLevelInfoUpdateScNotify {
    // message fields
    // @@protoc_insertion_point(field:RogueMagicLevelInfoUpdateScNotify.ACGBELAIGBO)
    pub ACGBELAIGBO: u32,
    // @@protoc_insertion_point(field:RogueMagicLevelInfoUpdateScNotify.ALIFPIHNMEK)
    pub ALIFPIHNMEK: ::protobuf::EnumOrUnknown<super::JMPPHGIGFFI::JMPPHGIGFFI>,
    // @@protoc_insertion_point(field:RogueMagicLevelInfoUpdateScNotify.LFCDODFMHHN)
    pub LFCDODFMHHN: u32,
    // @@protoc_insertion_point(field:RogueMagicLevelInfoUpdateScNotify.PBLFLJNHMIL)
    pub PBLFLJNHMIL: ::protobuf::EnumOrUnknown<super::BEHFNMKDOMK::BEHFNMKDOMK>,
    // @@protoc_insertion_point(field:RogueMagicLevelInfoUpdateScNotify.AHOOAFGDEHF)
    pub AHOOAFGDEHF: ::std::vec::Vec<super::BPAIFNGEDGH::BPAIFNGEDGH>,
    // @@protoc_insertion_point(field:RogueMagicLevelInfoUpdateScNotify.BGOKHEIBNKL)
    pub BGOKHEIBNKL: u32,
    // special fields
    // @@protoc_insertion_point(special_field:RogueMagicLevelInfoUpdateScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueMagicLevelInfoUpdateScNotify {
    fn default() -> &'a RogueMagicLevelInfoUpdateScNotify {
        <RogueMagicLevelInfoUpdateScNotify as ::protobuf::Message>::default_instance()
    }
}

impl RogueMagicLevelInfoUpdateScNotify {
    pub fn new() -> RogueMagicLevelInfoUpdateScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ACGBELAIGBO",
            |m: &RogueMagicLevelInfoUpdateScNotify| { &m.ACGBELAIGBO },
            |m: &mut RogueMagicLevelInfoUpdateScNotify| { &mut m.ACGBELAIGBO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ALIFPIHNMEK",
            |m: &RogueMagicLevelInfoUpdateScNotify| { &m.ALIFPIHNMEK },
            |m: &mut RogueMagicLevelInfoUpdateScNotify| { &mut m.ALIFPIHNMEK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LFCDODFMHHN",
            |m: &RogueMagicLevelInfoUpdateScNotify| { &m.LFCDODFMHHN },
            |m: &mut RogueMagicLevelInfoUpdateScNotify| { &mut m.LFCDODFMHHN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PBLFLJNHMIL",
            |m: &RogueMagicLevelInfoUpdateScNotify| { &m.PBLFLJNHMIL },
            |m: &mut RogueMagicLevelInfoUpdateScNotify| { &mut m.PBLFLJNHMIL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "AHOOAFGDEHF",
            |m: &RogueMagicLevelInfoUpdateScNotify| { &m.AHOOAFGDEHF },
            |m: &mut RogueMagicLevelInfoUpdateScNotify| { &mut m.AHOOAFGDEHF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BGOKHEIBNKL",
            |m: &RogueMagicLevelInfoUpdateScNotify| { &m.BGOKHEIBNKL },
            |m: &mut RogueMagicLevelInfoUpdateScNotify| { &mut m.BGOKHEIBNKL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueMagicLevelInfoUpdateScNotify>(
            "RogueMagicLevelInfoUpdateScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueMagicLevelInfoUpdateScNotify {
    const NAME: &'static str = "RogueMagicLevelInfoUpdateScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.ACGBELAIGBO = is.read_uint32()?;
                },
                96 => {
                    self.ALIFPIHNMEK = is.read_enum_or_unknown()?;
                },
                120 => {
                    self.LFCDODFMHHN = is.read_uint32()?;
                },
                16 => {
                    self.PBLFLJNHMIL = is.read_enum_or_unknown()?;
                },
                34 => {
                    self.AHOOAFGDEHF.push(is.read_message()?);
                },
                8 => {
                    self.BGOKHEIBNKL = is.read_uint32()?;
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
        if self.ACGBELAIGBO != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.ACGBELAIGBO);
        }
        if self.ALIFPIHNMEK != ::protobuf::EnumOrUnknown::new(super::JMPPHGIGFFI::JMPPHGIGFFI::ROGUE_MAGIC_SETTLE_REASON_NONE) {
            my_size += ::protobuf::rt::int32_size(12, self.ALIFPIHNMEK.value());
        }
        if self.LFCDODFMHHN != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.LFCDODFMHHN);
        }
        if self.PBLFLJNHMIL != ::protobuf::EnumOrUnknown::new(super::BEHFNMKDOMK::BEHFNMKDOMK::ROGUE_MAGIC_LEVEL_STATUS_NONE) {
            my_size += ::protobuf::rt::int32_size(2, self.PBLFLJNHMIL.value());
        }
        for value in &self.AHOOAFGDEHF {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.BGOKHEIBNKL != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.BGOKHEIBNKL);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.ACGBELAIGBO != 0 {
            os.write_uint32(8, self.ACGBELAIGBO)?;
        }
        if self.ALIFPIHNMEK != ::protobuf::EnumOrUnknown::new(super::JMPPHGIGFFI::JMPPHGIGFFI::ROGUE_MAGIC_SETTLE_REASON_NONE) {
            os.write_enum(12, ::protobuf::EnumOrUnknown::value(&self.ALIFPIHNMEK))?;
        }
        if self.LFCDODFMHHN != 0 {
            os.write_uint32(15, self.LFCDODFMHHN)?;
        }
        if self.PBLFLJNHMIL != ::protobuf::EnumOrUnknown::new(super::BEHFNMKDOMK::BEHFNMKDOMK::ROGUE_MAGIC_LEVEL_STATUS_NONE) {
            os.write_enum(2, ::protobuf::EnumOrUnknown::value(&self.PBLFLJNHMIL))?;
        }
        for v in &self.AHOOAFGDEHF {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        if self.BGOKHEIBNKL != 0 {
            os.write_uint32(1, self.BGOKHEIBNKL)?;
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

    fn new() -> RogueMagicLevelInfoUpdateScNotify {
        RogueMagicLevelInfoUpdateScNotify::new()
    }

    fn clear(&mut self) {
        self.ACGBELAIGBO = 0;
        self.ALIFPIHNMEK = ::protobuf::EnumOrUnknown::new(super::JMPPHGIGFFI::JMPPHGIGFFI::ROGUE_MAGIC_SETTLE_REASON_NONE);
        self.LFCDODFMHHN = 0;
        self.PBLFLJNHMIL = ::protobuf::EnumOrUnknown::new(super::BEHFNMKDOMK::BEHFNMKDOMK::ROGUE_MAGIC_LEVEL_STATUS_NONE);
        self.AHOOAFGDEHF.clear();
        self.BGOKHEIBNKL = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueMagicLevelInfoUpdateScNotify {
        static instance: RogueMagicLevelInfoUpdateScNotify = RogueMagicLevelInfoUpdateScNotify {
            ACGBELAIGBO: 0,
            ALIFPIHNMEK: ::protobuf::EnumOrUnknown::from_i32(0),
            LFCDODFMHHN: 0,
            PBLFLJNHMIL: ::protobuf::EnumOrUnknown::from_i32(0),
            AHOOAFGDEHF: ::std::vec::Vec::new(),
            BGOKHEIBNKL: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RogueMagicLevelInfoUpdateScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueMagicLevelInfoUpdateScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueMagicLevelInfoUpdateScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueMagicLevelInfoUpdateScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'RogueMagicLevelInfoUpdateScNotify.proto\x1a\x11BEHFNMKDOMK.proto\x1a\
    \x11BPAIFNGEDGH.proto\x1a\x11JMPPHGIGFFI.proto\"\x99\x02\n!RogueMagicLev\
    elInfoUpdateScNotify\x12\x20\n\x0bACGBELAIGBO\x18\x08\x20\x01(\rR\x0bACG\
    BELAIGBO\x12.\n\x0bALIFPIHNMEK\x18\x0c\x20\x01(\x0e2\x0c.JMPPHGIGFFIR\
    \x0bALIFPIHNMEK\x12\x20\n\x0bLFCDODFMHHN\x18\x0f\x20\x01(\rR\x0bLFCDODFM\
    HHN\x12.\n\x0bPBLFLJNHMIL\x18\x02\x20\x01(\x0e2\x0c.BEHFNMKDOMKR\x0bPBLF\
    LJNHMIL\x12.\n\x0bAHOOAFGDEHF\x18\x04\x20\x03(\x0b2\x0c.BPAIFNGEDGHR\x0b\
    AHOOAFGDEHF\x12\x20\n\x0bBGOKHEIBNKL\x18\x01\x20\x01(\rR\x0bBGOKHEIBNKLb\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::BEHFNMKDOMK::file_descriptor().clone());
            deps.push(super::BPAIFNGEDGH::file_descriptor().clone());
            deps.push(super::JMPPHGIGFFI::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RogueMagicLevelInfoUpdateScNotify::generated_message_descriptor_data());
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
