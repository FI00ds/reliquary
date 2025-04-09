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

//! Generated file from `BKDCJANPNBP.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:BKDCJANPNBP)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BKDCJANPNBP {
    // message fields
    // @@protoc_insertion_point(field:BKDCJANPNBP.LBNBDEKPPFN)
    pub LBNBDEKPPFN: ::protobuf::EnumOrUnknown<super::JEIDMGKAJJP::JEIDMGKAJJP>,
    // @@protoc_insertion_point(field:BKDCJANPNBP.ANPLLAOBFJI)
    pub ANPLLAOBFJI: u32,
    // @@protoc_insertion_point(field:BKDCJANPNBP.uid)
    pub uid: u32,
    // @@protoc_insertion_point(field:BKDCJANPNBP.LIAKONIIOMK)
    pub LIAKONIIOMK: u32,
    // @@protoc_insertion_point(field:BKDCJANPNBP.LCEODDKLMGB)
    pub LCEODDKLMGB: ::std::string::String,
    // @@protoc_insertion_point(field:BKDCJANPNBP.DMCCBECKHML)
    pub DMCCBECKHML: ::std::string::String,
    // @@protoc_insertion_point(field:BKDCJANPNBP.JFGAEKJJPIE)
    pub JFGAEKJJPIE: bool,
    // @@protoc_insertion_point(field:BKDCJANPNBP.GMALCPNOHBF)
    pub GMALCPNOHBF: ::std::string::String,
    // @@protoc_insertion_point(field:BKDCJANPNBP.CGGANANCAOM)
    pub CGGANANCAOM: ::std::vec::Vec<super::AssistSimpleInfo::AssistSimpleInfo>,
    // @@protoc_insertion_point(field:BKDCJANPNBP.PADMHPNGPNO)
    pub PADMHPNGPNO: u32,
    // @@protoc_insertion_point(field:BKDCJANPNBP.KOGNFNCMKFF)
    pub KOGNFNCMKFF: i64,
    // @@protoc_insertion_point(field:BKDCJANPNBP.level)
    pub level: u32,
    // @@protoc_insertion_point(field:BKDCJANPNBP.NEFBPLJIMMG)
    pub NEFBPLJIMMG: ::protobuf::EnumOrUnknown<super::NKJFOGEMLDE::NKJFOGEMLDE>,
    // @@protoc_insertion_point(field:BKDCJANPNBP.AKCEJFCFBAN)
    pub AKCEJFCFBAN: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:BKDCJANPNBP.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BKDCJANPNBP {
    fn default() -> &'a BKDCJANPNBP {
        <BKDCJANPNBP as ::protobuf::Message>::default_instance()
    }
}

impl BKDCJANPNBP {
    pub fn new() -> BKDCJANPNBP {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(14);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LBNBDEKPPFN",
            |m: &BKDCJANPNBP| { &m.LBNBDEKPPFN },
            |m: &mut BKDCJANPNBP| { &mut m.LBNBDEKPPFN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ANPLLAOBFJI",
            |m: &BKDCJANPNBP| { &m.ANPLLAOBFJI },
            |m: &mut BKDCJANPNBP| { &mut m.ANPLLAOBFJI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "uid",
            |m: &BKDCJANPNBP| { &m.uid },
            |m: &mut BKDCJANPNBP| { &mut m.uid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LIAKONIIOMK",
            |m: &BKDCJANPNBP| { &m.LIAKONIIOMK },
            |m: &mut BKDCJANPNBP| { &mut m.LIAKONIIOMK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LCEODDKLMGB",
            |m: &BKDCJANPNBP| { &m.LCEODDKLMGB },
            |m: &mut BKDCJANPNBP| { &mut m.LCEODDKLMGB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DMCCBECKHML",
            |m: &BKDCJANPNBP| { &m.DMCCBECKHML },
            |m: &mut BKDCJANPNBP| { &mut m.DMCCBECKHML },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JFGAEKJJPIE",
            |m: &BKDCJANPNBP| { &m.JFGAEKJJPIE },
            |m: &mut BKDCJANPNBP| { &mut m.JFGAEKJJPIE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GMALCPNOHBF",
            |m: &BKDCJANPNBP| { &m.GMALCPNOHBF },
            |m: &mut BKDCJANPNBP| { &mut m.GMALCPNOHBF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CGGANANCAOM",
            |m: &BKDCJANPNBP| { &m.CGGANANCAOM },
            |m: &mut BKDCJANPNBP| { &mut m.CGGANANCAOM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PADMHPNGPNO",
            |m: &BKDCJANPNBP| { &m.PADMHPNGPNO },
            |m: &mut BKDCJANPNBP| { &mut m.PADMHPNGPNO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KOGNFNCMKFF",
            |m: &BKDCJANPNBP| { &m.KOGNFNCMKFF },
            |m: &mut BKDCJANPNBP| { &mut m.KOGNFNCMKFF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &BKDCJANPNBP| { &m.level },
            |m: &mut BKDCJANPNBP| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NEFBPLJIMMG",
            |m: &BKDCJANPNBP| { &m.NEFBPLJIMMG },
            |m: &mut BKDCJANPNBP| { &mut m.NEFBPLJIMMG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AKCEJFCFBAN",
            |m: &BKDCJANPNBP| { &m.AKCEJFCFBAN },
            |m: &mut BKDCJANPNBP| { &mut m.AKCEJFCFBAN },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BKDCJANPNBP>(
            "BKDCJANPNBP",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BKDCJANPNBP {
    const NAME: &'static str = "BKDCJANPNBP";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.LBNBDEKPPFN = is.read_enum_or_unknown()?;
                },
                112 => {
                    self.ANPLLAOBFJI = is.read_uint32()?;
                },
                80 => {
                    self.uid = is.read_uint32()?;
                },
                16 => {
                    self.LIAKONIIOMK = is.read_uint32()?;
                },
                122 => {
                    self.LCEODDKLMGB = is.read_string()?;
                },
                50 => {
                    self.DMCCBECKHML = is.read_string()?;
                },
                56 => {
                    self.JFGAEKJJPIE = is.read_bool()?;
                },
                106 => {
                    self.GMALCPNOHBF = is.read_string()?;
                },
                42 => {
                    self.CGGANANCAOM.push(is.read_message()?);
                },
                64 => {
                    self.PADMHPNGPNO = is.read_uint32()?;
                },
                88 => {
                    self.KOGNFNCMKFF = is.read_int64()?;
                },
                24 => {
                    self.level = is.read_uint32()?;
                },
                96 => {
                    self.NEFBPLJIMMG = is.read_enum_or_unknown()?;
                },
                34 => {
                    self.AKCEJFCFBAN = is.read_string()?;
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
        if self.LBNBDEKPPFN != ::protobuf::EnumOrUnknown::new(super::JEIDMGKAJJP::JEIDMGKAJJP::EDITOR) {
            my_size += ::protobuf::rt::int32_size(9, self.LBNBDEKPPFN.value());
        }
        if self.ANPLLAOBFJI != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.ANPLLAOBFJI);
        }
        if self.uid != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.uid);
        }
        if self.LIAKONIIOMK != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.LIAKONIIOMK);
        }
        if !self.LCEODDKLMGB.is_empty() {
            my_size += ::protobuf::rt::string_size(15, &self.LCEODDKLMGB);
        }
        if !self.DMCCBECKHML.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.DMCCBECKHML);
        }
        if self.JFGAEKJJPIE != false {
            my_size += 1 + 1;
        }
        if !self.GMALCPNOHBF.is_empty() {
            my_size += ::protobuf::rt::string_size(13, &self.GMALCPNOHBF);
        }
        for value in &self.CGGANANCAOM {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.PADMHPNGPNO != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.PADMHPNGPNO);
        }
        if self.KOGNFNCMKFF != 0 {
            my_size += ::protobuf::rt::int64_size(11, self.KOGNFNCMKFF);
        }
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.level);
        }
        if self.NEFBPLJIMMG != ::protobuf::EnumOrUnknown::new(super::NKJFOGEMLDE::NKJFOGEMLDE::FRIEND_ONLINE_STATUS_OFFLINE) {
            my_size += ::protobuf::rt::int32_size(12, self.NEFBPLJIMMG.value());
        }
        if !self.AKCEJFCFBAN.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.AKCEJFCFBAN);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.LBNBDEKPPFN != ::protobuf::EnumOrUnknown::new(super::JEIDMGKAJJP::JEIDMGKAJJP::EDITOR) {
            os.write_enum(9, ::protobuf::EnumOrUnknown::value(&self.LBNBDEKPPFN))?;
        }
        if self.ANPLLAOBFJI != 0 {
            os.write_uint32(14, self.ANPLLAOBFJI)?;
        }
        if self.uid != 0 {
            os.write_uint32(10, self.uid)?;
        }
        if self.LIAKONIIOMK != 0 {
            os.write_uint32(2, self.LIAKONIIOMK)?;
        }
        if !self.LCEODDKLMGB.is_empty() {
            os.write_string(15, &self.LCEODDKLMGB)?;
        }
        if !self.DMCCBECKHML.is_empty() {
            os.write_string(6, &self.DMCCBECKHML)?;
        }
        if self.JFGAEKJJPIE != false {
            os.write_bool(7, self.JFGAEKJJPIE)?;
        }
        if !self.GMALCPNOHBF.is_empty() {
            os.write_string(13, &self.GMALCPNOHBF)?;
        }
        for v in &self.CGGANANCAOM {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        if self.PADMHPNGPNO != 0 {
            os.write_uint32(8, self.PADMHPNGPNO)?;
        }
        if self.KOGNFNCMKFF != 0 {
            os.write_int64(11, self.KOGNFNCMKFF)?;
        }
        if self.level != 0 {
            os.write_uint32(3, self.level)?;
        }
        if self.NEFBPLJIMMG != ::protobuf::EnumOrUnknown::new(super::NKJFOGEMLDE::NKJFOGEMLDE::FRIEND_ONLINE_STATUS_OFFLINE) {
            os.write_enum(12, ::protobuf::EnumOrUnknown::value(&self.NEFBPLJIMMG))?;
        }
        if !self.AKCEJFCFBAN.is_empty() {
            os.write_string(4, &self.AKCEJFCFBAN)?;
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

    fn new() -> BKDCJANPNBP {
        BKDCJANPNBP::new()
    }

    fn clear(&mut self) {
        self.LBNBDEKPPFN = ::protobuf::EnumOrUnknown::new(super::JEIDMGKAJJP::JEIDMGKAJJP::EDITOR);
        self.ANPLLAOBFJI = 0;
        self.uid = 0;
        self.LIAKONIIOMK = 0;
        self.LCEODDKLMGB.clear();
        self.DMCCBECKHML.clear();
        self.JFGAEKJJPIE = false;
        self.GMALCPNOHBF.clear();
        self.CGGANANCAOM.clear();
        self.PADMHPNGPNO = 0;
        self.KOGNFNCMKFF = 0;
        self.level = 0;
        self.NEFBPLJIMMG = ::protobuf::EnumOrUnknown::new(super::NKJFOGEMLDE::NKJFOGEMLDE::FRIEND_ONLINE_STATUS_OFFLINE);
        self.AKCEJFCFBAN.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BKDCJANPNBP {
        static instance: BKDCJANPNBP = BKDCJANPNBP {
            LBNBDEKPPFN: ::protobuf::EnumOrUnknown::from_i32(0),
            ANPLLAOBFJI: 0,
            uid: 0,
            LIAKONIIOMK: 0,
            LCEODDKLMGB: ::std::string::String::new(),
            DMCCBECKHML: ::std::string::String::new(),
            JFGAEKJJPIE: false,
            GMALCPNOHBF: ::std::string::String::new(),
            CGGANANCAOM: ::std::vec::Vec::new(),
            PADMHPNGPNO: 0,
            KOGNFNCMKFF: 0,
            level: 0,
            NEFBPLJIMMG: ::protobuf::EnumOrUnknown::from_i32(0),
            AKCEJFCFBAN: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BKDCJANPNBP {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BKDCJANPNBP").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BKDCJANPNBP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BKDCJANPNBP {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11BKDCJANPNBP.proto\x1a\x16AssistSimpleInfo.proto\x1a\x11JEIDMGKAJJP\
    .proto\x1a\x11NKJFOGEMLDE.proto\"\xfc\x03\n\x0bBKDCJANPNBP\x12.\n\x0bLBN\
    BDEKPPFN\x18\t\x20\x01(\x0e2\x0c.JEIDMGKAJJPR\x0bLBNBDEKPPFN\x12\x20\n\
    \x0bANPLLAOBFJI\x18\x0e\x20\x01(\rR\x0bANPLLAOBFJI\x12\x10\n\x03uid\x18\
    \n\x20\x01(\rR\x03uid\x12\x20\n\x0bLIAKONIIOMK\x18\x02\x20\x01(\rR\x0bLI\
    AKONIIOMK\x12\x20\n\x0bLCEODDKLMGB\x18\x0f\x20\x01(\tR\x0bLCEODDKLMGB\
    \x12\x20\n\x0bDMCCBECKHML\x18\x06\x20\x01(\tR\x0bDMCCBECKHML\x12\x20\n\
    \x0bJFGAEKJJPIE\x18\x07\x20\x01(\x08R\x0bJFGAEKJJPIE\x12\x20\n\x0bGMALCP\
    NOHBF\x18\r\x20\x01(\tR\x0bGMALCPNOHBF\x123\n\x0bCGGANANCAOM\x18\x05\x20\
    \x03(\x0b2\x11.AssistSimpleInfoR\x0bCGGANANCAOM\x12\x20\n\x0bPADMHPNGPNO\
    \x18\x08\x20\x01(\rR\x0bPADMHPNGPNO\x12\x20\n\x0bKOGNFNCMKFF\x18\x0b\x20\
    \x01(\x03R\x0bKOGNFNCMKFF\x12\x14\n\x05level\x18\x03\x20\x01(\rR\x05leve\
    l\x12.\n\x0bNEFBPLJIMMG\x18\x0c\x20\x01(\x0e2\x0c.NKJFOGEMLDER\x0bNEFBPL\
    JIMMG\x12\x20\n\x0bAKCEJFCFBAN\x18\x04\x20\x01(\tR\x0bAKCEJFCFBANb\x06pr\
    oto3\
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
            deps.push(super::AssistSimpleInfo::file_descriptor().clone());
            deps.push(super::JEIDMGKAJJP::file_descriptor().clone());
            deps.push(super::NKJFOGEMLDE::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(BKDCJANPNBP::generated_message_descriptor_data());
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
