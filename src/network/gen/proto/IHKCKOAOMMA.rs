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

//! Generated file from `IHKCKOAOMMA.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:IHKCKOAOMMA)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct IHKCKOAOMMA {
    // message fields
    // @@protoc_insertion_point(field:IHKCKOAOMMA.NNOGGMBOOKF)
    pub NNOGGMBOOKF: u32,
    // @@protoc_insertion_point(field:IHKCKOAOMMA.OLDKAMACFMD)
    pub OLDKAMACFMD: ::protobuf::EnumOrUnknown<super::LKOILBGBKEM::LKOILBGBKEM>,
    // @@protoc_insertion_point(field:IHKCKOAOMMA.MFDJNOFJNDC)
    pub MFDJNOFJNDC: bool,
    // @@protoc_insertion_point(field:IHKCKOAOMMA.KECHADNGNEG)
    pub KECHADNGNEG: ::std::vec::Vec<super::NCIOGBELNBB::NCIOGBELNBB>,
    // @@protoc_insertion_point(field:IHKCKOAOMMA.IOPEEMNLIDM)
    pub IOPEEMNLIDM: ::protobuf::EnumOrUnknown<super::RogueTournLevelStatus::RogueTournLevelStatus>,
    // special fields
    // @@protoc_insertion_point(special_field:IHKCKOAOMMA.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a IHKCKOAOMMA {
    fn default() -> &'a IHKCKOAOMMA {
        <IHKCKOAOMMA as ::protobuf::Message>::default_instance()
    }
}

impl IHKCKOAOMMA {
    pub fn new() -> IHKCKOAOMMA {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NNOGGMBOOKF",
            |m: &IHKCKOAOMMA| { &m.NNOGGMBOOKF },
            |m: &mut IHKCKOAOMMA| { &mut m.NNOGGMBOOKF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OLDKAMACFMD",
            |m: &IHKCKOAOMMA| { &m.OLDKAMACFMD },
            |m: &mut IHKCKOAOMMA| { &mut m.OLDKAMACFMD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MFDJNOFJNDC",
            |m: &IHKCKOAOMMA| { &m.MFDJNOFJNDC },
            |m: &mut IHKCKOAOMMA| { &mut m.MFDJNOFJNDC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KECHADNGNEG",
            |m: &IHKCKOAOMMA| { &m.KECHADNGNEG },
            |m: &mut IHKCKOAOMMA| { &mut m.KECHADNGNEG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IOPEEMNLIDM",
            |m: &IHKCKOAOMMA| { &m.IOPEEMNLIDM },
            |m: &mut IHKCKOAOMMA| { &mut m.IOPEEMNLIDM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<IHKCKOAOMMA>(
            "IHKCKOAOMMA",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for IHKCKOAOMMA {
    const NAME: &'static str = "IHKCKOAOMMA";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.NNOGGMBOOKF = is.read_uint32()?;
                },
                40 => {
                    self.OLDKAMACFMD = is.read_enum_or_unknown()?;
                },
                112 => {
                    self.MFDJNOFJNDC = is.read_bool()?;
                },
                82 => {
                    self.KECHADNGNEG.push(is.read_message()?);
                },
                96 => {
                    self.IOPEEMNLIDM = is.read_enum_or_unknown()?;
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
        if self.NNOGGMBOOKF != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.NNOGGMBOOKF);
        }
        if self.OLDKAMACFMD != ::protobuf::EnumOrUnknown::new(super::LKOILBGBKEM::LKOILBGBKEM::ROGUE_TOURN_SETTLE_REASON_NONE) {
            my_size += ::protobuf::rt::int32_size(5, self.OLDKAMACFMD.value());
        }
        if self.MFDJNOFJNDC != false {
            my_size += 1 + 1;
        }
        for value in &self.KECHADNGNEG {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.IOPEEMNLIDM != ::protobuf::EnumOrUnknown::new(super::RogueTournLevelStatus::RogueTournLevelStatus::ROGUE_TOURN_LEVEL_STATUS_NONE) {
            my_size += ::protobuf::rt::int32_size(12, self.IOPEEMNLIDM.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.NNOGGMBOOKF != 0 {
            os.write_uint32(15, self.NNOGGMBOOKF)?;
        }
        if self.OLDKAMACFMD != ::protobuf::EnumOrUnknown::new(super::LKOILBGBKEM::LKOILBGBKEM::ROGUE_TOURN_SETTLE_REASON_NONE) {
            os.write_enum(5, ::protobuf::EnumOrUnknown::value(&self.OLDKAMACFMD))?;
        }
        if self.MFDJNOFJNDC != false {
            os.write_bool(14, self.MFDJNOFJNDC)?;
        }
        for v in &self.KECHADNGNEG {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        };
        if self.IOPEEMNLIDM != ::protobuf::EnumOrUnknown::new(super::RogueTournLevelStatus::RogueTournLevelStatus::ROGUE_TOURN_LEVEL_STATUS_NONE) {
            os.write_enum(12, ::protobuf::EnumOrUnknown::value(&self.IOPEEMNLIDM))?;
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

    fn new() -> IHKCKOAOMMA {
        IHKCKOAOMMA::new()
    }

    fn clear(&mut self) {
        self.NNOGGMBOOKF = 0;
        self.OLDKAMACFMD = ::protobuf::EnumOrUnknown::new(super::LKOILBGBKEM::LKOILBGBKEM::ROGUE_TOURN_SETTLE_REASON_NONE);
        self.MFDJNOFJNDC = false;
        self.KECHADNGNEG.clear();
        self.IOPEEMNLIDM = ::protobuf::EnumOrUnknown::new(super::RogueTournLevelStatus::RogueTournLevelStatus::ROGUE_TOURN_LEVEL_STATUS_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static IHKCKOAOMMA {
        static instance: IHKCKOAOMMA = IHKCKOAOMMA {
            NNOGGMBOOKF: 0,
            OLDKAMACFMD: ::protobuf::EnumOrUnknown::from_i32(0),
            MFDJNOFJNDC: false,
            KECHADNGNEG: ::std::vec::Vec::new(),
            IOPEEMNLIDM: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for IHKCKOAOMMA {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("IHKCKOAOMMA").unwrap()).clone()
    }
}

impl ::std::fmt::Display for IHKCKOAOMMA {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IHKCKOAOMMA {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11IHKCKOAOMMA.proto\x1a\x11LKOILBGBKEM.proto\x1a\x11NCIOGBELNBB.prot\
    o\x1a\x1bRogueTournLevelStatus.proto\"\xeb\x01\n\x0bIHKCKOAOMMA\x12\x20\
    \n\x0bNNOGGMBOOKF\x18\x0f\x20\x01(\rR\x0bNNOGGMBOOKF\x12.\n\x0bOLDKAMACF\
    MD\x18\x05\x20\x01(\x0e2\x0c.LKOILBGBKEMR\x0bOLDKAMACFMD\x12\x20\n\x0bMF\
    DJNOFJNDC\x18\x0e\x20\x01(\x08R\x0bMFDJNOFJNDC\x12.\n\x0bKECHADNGNEG\x18\
    \n\x20\x03(\x0b2\x0c.NCIOGBELNBBR\x0bKECHADNGNEG\x128\n\x0bIOPEEMNLIDM\
    \x18\x0c\x20\x01(\x0e2\x16.RogueTournLevelStatusR\x0bIOPEEMNLIDMb\x06pro\
    to3\
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
            deps.push(super::LKOILBGBKEM::file_descriptor().clone());
            deps.push(super::NCIOGBELNBB::file_descriptor().clone());
            deps.push(super::RogueTournLevelStatus::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(IHKCKOAOMMA::generated_message_descriptor_data());
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
