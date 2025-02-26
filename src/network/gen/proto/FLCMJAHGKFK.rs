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

//! Generated file from `FLCMJAHGKFK.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:FLCMJAHGKFK)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FLCMJAHGKFK {
    // message fields
    // @@protoc_insertion_point(field:FLCMJAHGKFK.LCEODDKLMGB)
    pub LCEODDKLMGB: ::std::string::String,
    // @@protoc_insertion_point(field:FLCMJAHGKFK.KMONAGFELPG)
    pub KMONAGFELPG: u32,
    // @@protoc_insertion_point(field:FLCMJAHGKFK.GMALCPNOHBF)
    pub GMALCPNOHBF: ::std::string::String,
    // @@protoc_insertion_point(field:FLCMJAHGKFK.NPPPHGFENPH)
    pub NPPPHGFENPH: u32,
    // @@protoc_insertion_point(field:FLCMJAHGKFK.LBNBDEKPPFN)
    pub LBNBDEKPPFN: ::protobuf::EnumOrUnknown<super::JEIDMGKAJJP::JEIDMGKAJJP>,
    // @@protoc_insertion_point(field:FLCMJAHGKFK.IODLGIKKHJI)
    pub IODLGIKKHJI: ::std::string::String,
    // @@protoc_insertion_point(field:FLCMJAHGKFK.LIAKONIIOMK)
    pub LIAKONIIOMK: u32,
    // @@protoc_insertion_point(field:FLCMJAHGKFK.level)
    pub level: u32,
    // special fields
    // @@protoc_insertion_point(special_field:FLCMJAHGKFK.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FLCMJAHGKFK {
    fn default() -> &'a FLCMJAHGKFK {
        <FLCMJAHGKFK as ::protobuf::Message>::default_instance()
    }
}

impl FLCMJAHGKFK {
    pub fn new() -> FLCMJAHGKFK {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LCEODDKLMGB",
            |m: &FLCMJAHGKFK| { &m.LCEODDKLMGB },
            |m: &mut FLCMJAHGKFK| { &mut m.LCEODDKLMGB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KMONAGFELPG",
            |m: &FLCMJAHGKFK| { &m.KMONAGFELPG },
            |m: &mut FLCMJAHGKFK| { &mut m.KMONAGFELPG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GMALCPNOHBF",
            |m: &FLCMJAHGKFK| { &m.GMALCPNOHBF },
            |m: &mut FLCMJAHGKFK| { &mut m.GMALCPNOHBF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NPPPHGFENPH",
            |m: &FLCMJAHGKFK| { &m.NPPPHGFENPH },
            |m: &mut FLCMJAHGKFK| { &mut m.NPPPHGFENPH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LBNBDEKPPFN",
            |m: &FLCMJAHGKFK| { &m.LBNBDEKPPFN },
            |m: &mut FLCMJAHGKFK| { &mut m.LBNBDEKPPFN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IODLGIKKHJI",
            |m: &FLCMJAHGKFK| { &m.IODLGIKKHJI },
            |m: &mut FLCMJAHGKFK| { &mut m.IODLGIKKHJI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LIAKONIIOMK",
            |m: &FLCMJAHGKFK| { &m.LIAKONIIOMK },
            |m: &mut FLCMJAHGKFK| { &mut m.LIAKONIIOMK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &FLCMJAHGKFK| { &m.level },
            |m: &mut FLCMJAHGKFK| { &mut m.level },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FLCMJAHGKFK>(
            "FLCMJAHGKFK",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FLCMJAHGKFK {
    const NAME: &'static str = "FLCMJAHGKFK";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                50 => {
                    self.LCEODDKLMGB = is.read_string()?;
                },
                104 => {
                    self.KMONAGFELPG = is.read_uint32()?;
                },
                74 => {
                    self.GMALCPNOHBF = is.read_string()?;
                },
                112 => {
                    self.NPPPHGFENPH = is.read_uint32()?;
                },
                120 => {
                    self.LBNBDEKPPFN = is.read_enum_or_unknown()?;
                },
                58 => {
                    self.IODLGIKKHJI = is.read_string()?;
                },
                80 => {
                    self.LIAKONIIOMK = is.read_uint32()?;
                },
                40 => {
                    self.level = is.read_uint32()?;
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
        if !self.LCEODDKLMGB.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.LCEODDKLMGB);
        }
        if self.KMONAGFELPG != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.KMONAGFELPG);
        }
        if !self.GMALCPNOHBF.is_empty() {
            my_size += ::protobuf::rt::string_size(9, &self.GMALCPNOHBF);
        }
        if self.NPPPHGFENPH != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.NPPPHGFENPH);
        }
        if self.LBNBDEKPPFN != ::protobuf::EnumOrUnknown::new(super::JEIDMGKAJJP::JEIDMGKAJJP::EDITOR) {
            my_size += ::protobuf::rt::int32_size(15, self.LBNBDEKPPFN.value());
        }
        if !self.IODLGIKKHJI.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.IODLGIKKHJI);
        }
        if self.LIAKONIIOMK != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.LIAKONIIOMK);
        }
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.level);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.LCEODDKLMGB.is_empty() {
            os.write_string(6, &self.LCEODDKLMGB)?;
        }
        if self.KMONAGFELPG != 0 {
            os.write_uint32(13, self.KMONAGFELPG)?;
        }
        if !self.GMALCPNOHBF.is_empty() {
            os.write_string(9, &self.GMALCPNOHBF)?;
        }
        if self.NPPPHGFENPH != 0 {
            os.write_uint32(14, self.NPPPHGFENPH)?;
        }
        if self.LBNBDEKPPFN != ::protobuf::EnumOrUnknown::new(super::JEIDMGKAJJP::JEIDMGKAJJP::EDITOR) {
            os.write_enum(15, ::protobuf::EnumOrUnknown::value(&self.LBNBDEKPPFN))?;
        }
        if !self.IODLGIKKHJI.is_empty() {
            os.write_string(7, &self.IODLGIKKHJI)?;
        }
        if self.LIAKONIIOMK != 0 {
            os.write_uint32(10, self.LIAKONIIOMK)?;
        }
        if self.level != 0 {
            os.write_uint32(5, self.level)?;
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

    fn new() -> FLCMJAHGKFK {
        FLCMJAHGKFK::new()
    }

    fn clear(&mut self) {
        self.LCEODDKLMGB.clear();
        self.KMONAGFELPG = 0;
        self.GMALCPNOHBF.clear();
        self.NPPPHGFENPH = 0;
        self.LBNBDEKPPFN = ::protobuf::EnumOrUnknown::new(super::JEIDMGKAJJP::JEIDMGKAJJP::EDITOR);
        self.IODLGIKKHJI.clear();
        self.LIAKONIIOMK = 0;
        self.level = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FLCMJAHGKFK {
        static instance: FLCMJAHGKFK = FLCMJAHGKFK {
            LCEODDKLMGB: ::std::string::String::new(),
            KMONAGFELPG: 0,
            GMALCPNOHBF: ::std::string::String::new(),
            NPPPHGFENPH: 0,
            LBNBDEKPPFN: ::protobuf::EnumOrUnknown::from_i32(0),
            IODLGIKKHJI: ::std::string::String::new(),
            LIAKONIIOMK: 0,
            level: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FLCMJAHGKFK {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FLCMJAHGKFK").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FLCMJAHGKFK {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FLCMJAHGKFK {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11FLCMJAHGKFK.proto\x1a\x11JEIDMGKAJJP.proto\"\x9f\x02\n\x0bFLCMJAHG\
    KFK\x12\x20\n\x0bLCEODDKLMGB\x18\x06\x20\x01(\tR\x0bLCEODDKLMGB\x12\x20\
    \n\x0bKMONAGFELPG\x18\r\x20\x01(\rR\x0bKMONAGFELPG\x12\x20\n\x0bGMALCPNO\
    HBF\x18\t\x20\x01(\tR\x0bGMALCPNOHBF\x12\x20\n\x0bNPPPHGFENPH\x18\x0e\
    \x20\x01(\rR\x0bNPPPHGFENPH\x12.\n\x0bLBNBDEKPPFN\x18\x0f\x20\x01(\x0e2\
    \x0c.JEIDMGKAJJPR\x0bLBNBDEKPPFN\x12\x20\n\x0bIODLGIKKHJI\x18\x07\x20\
    \x01(\tR\x0bIODLGIKKHJI\x12\x20\n\x0bLIAKONIIOMK\x18\n\x20\x01(\rR\x0bLI\
    AKONIIOMK\x12\x14\n\x05level\x18\x05\x20\x01(\rR\x05levelb\x06proto3\
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
            deps.push(super::JEIDMGKAJJP::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(FLCMJAHGKFK::generated_message_descriptor_data());
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
