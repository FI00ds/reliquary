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

//! Generated file from `LBAOGIBPJOP.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:LBAOGIBPJOP)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LBAOGIBPJOP {
    // message fields
    // @@protoc_insertion_point(field:LBAOGIBPJOP.JIJHAAIHNCN)
    pub JIJHAAIHNCN: u32,
    // @@protoc_insertion_point(field:LBAOGIBPJOP.KHCMOFPFOAK)
    pub KHCMOFPFOAK: u32,
    // @@protoc_insertion_point(field:LBAOGIBPJOP.EIGABCKEDNP)
    pub EIGABCKEDNP: u32,
    // @@protoc_insertion_point(field:LBAOGIBPJOP.JJEFBPKPKBK)
    pub JJEFBPKPKBK: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:LBAOGIBPJOP.DKPNENBHELH)
    pub DKPNENBHELH: u32,
    // @@protoc_insertion_point(field:LBAOGIBPJOP.JIKEIGBCABB)
    pub JIKEIGBCABB: bool,
    // @@protoc_insertion_point(field:LBAOGIBPJOP.MODDKLNDAMK)
    pub MODDKLNDAMK: u32,
    // @@protoc_insertion_point(field:LBAOGIBPJOP.ACJCPHIFMLN)
    pub ACJCPHIFMLN: u32,
    // @@protoc_insertion_point(field:LBAOGIBPJOP.HJEFOLKGAEI)
    pub HJEFOLKGAEI: ::protobuf::EnumOrUnknown<super::JOMKPEGEFMP::JOMKPEGEFMP>,
    // @@protoc_insertion_point(field:LBAOGIBPJOP.LIPJDJPMOKB)
    pub LIPJDJPMOKB: ::std::vec::Vec<super::CBBDIOMIFHD::CBBDIOMIFHD>,
    // @@protoc_insertion_point(field:LBAOGIBPJOP.LIKHCLPMHJK)
    pub LIKHCLPMHJK: bool,
    // @@protoc_insertion_point(field:LBAOGIBPJOP.PCMGAGAHBLK)
    pub PCMGAGAHBLK: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:LBAOGIBPJOP.PBFAIOJJGNL)
    pub PBFAIOJJGNL: u32,
    // @@protoc_insertion_point(field:LBAOGIBPJOP.BBDOPLEKIAC)
    pub BBDOPLEKIAC: u32,
    // @@protoc_insertion_point(field:LBAOGIBPJOP.NLIBKABFGCC)
    pub NLIBKABFGCC: u32,
    // @@protoc_insertion_point(field:LBAOGIBPJOP.CEIFKJIEAJE)
    pub CEIFKJIEAJE: ::protobuf::EnumOrUnknown<super::JOMKPEGEFMP::JOMKPEGEFMP>,
    // special fields
    // @@protoc_insertion_point(special_field:LBAOGIBPJOP.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LBAOGIBPJOP {
    fn default() -> &'a LBAOGIBPJOP {
        <LBAOGIBPJOP as ::protobuf::Message>::default_instance()
    }
}

impl LBAOGIBPJOP {
    pub fn new() -> LBAOGIBPJOP {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(16);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JIJHAAIHNCN",
            |m: &LBAOGIBPJOP| { &m.JIJHAAIHNCN },
            |m: &mut LBAOGIBPJOP| { &mut m.JIJHAAIHNCN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KHCMOFPFOAK",
            |m: &LBAOGIBPJOP| { &m.KHCMOFPFOAK },
            |m: &mut LBAOGIBPJOP| { &mut m.KHCMOFPFOAK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EIGABCKEDNP",
            |m: &LBAOGIBPJOP| { &m.EIGABCKEDNP },
            |m: &mut LBAOGIBPJOP| { &mut m.EIGABCKEDNP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JJEFBPKPKBK",
            |m: &LBAOGIBPJOP| { &m.JJEFBPKPKBK },
            |m: &mut LBAOGIBPJOP| { &mut m.JJEFBPKPKBK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DKPNENBHELH",
            |m: &LBAOGIBPJOP| { &m.DKPNENBHELH },
            |m: &mut LBAOGIBPJOP| { &mut m.DKPNENBHELH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JIKEIGBCABB",
            |m: &LBAOGIBPJOP| { &m.JIKEIGBCABB },
            |m: &mut LBAOGIBPJOP| { &mut m.JIKEIGBCABB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MODDKLNDAMK",
            |m: &LBAOGIBPJOP| { &m.MODDKLNDAMK },
            |m: &mut LBAOGIBPJOP| { &mut m.MODDKLNDAMK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ACJCPHIFMLN",
            |m: &LBAOGIBPJOP| { &m.ACJCPHIFMLN },
            |m: &mut LBAOGIBPJOP| { &mut m.ACJCPHIFMLN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HJEFOLKGAEI",
            |m: &LBAOGIBPJOP| { &m.HJEFOLKGAEI },
            |m: &mut LBAOGIBPJOP| { &mut m.HJEFOLKGAEI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LIPJDJPMOKB",
            |m: &LBAOGIBPJOP| { &m.LIPJDJPMOKB },
            |m: &mut LBAOGIBPJOP| { &mut m.LIPJDJPMOKB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LIKHCLPMHJK",
            |m: &LBAOGIBPJOP| { &m.LIKHCLPMHJK },
            |m: &mut LBAOGIBPJOP| { &mut m.LIKHCLPMHJK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PCMGAGAHBLK",
            |m: &LBAOGIBPJOP| { &m.PCMGAGAHBLK },
            |m: &mut LBAOGIBPJOP| { &mut m.PCMGAGAHBLK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PBFAIOJJGNL",
            |m: &LBAOGIBPJOP| { &m.PBFAIOJJGNL },
            |m: &mut LBAOGIBPJOP| { &mut m.PBFAIOJJGNL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BBDOPLEKIAC",
            |m: &LBAOGIBPJOP| { &m.BBDOPLEKIAC },
            |m: &mut LBAOGIBPJOP| { &mut m.BBDOPLEKIAC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NLIBKABFGCC",
            |m: &LBAOGIBPJOP| { &m.NLIBKABFGCC },
            |m: &mut LBAOGIBPJOP| { &mut m.NLIBKABFGCC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CEIFKJIEAJE",
            |m: &LBAOGIBPJOP| { &m.CEIFKJIEAJE },
            |m: &mut LBAOGIBPJOP| { &mut m.CEIFKJIEAJE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LBAOGIBPJOP>(
            "LBAOGIBPJOP",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LBAOGIBPJOP {
    const NAME: &'static str = "LBAOGIBPJOP";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.JIJHAAIHNCN = is.read_uint32()?;
                },
                56 => {
                    self.KHCMOFPFOAK = is.read_uint32()?;
                },
                24 => {
                    self.EIGABCKEDNP = is.read_uint32()?;
                },
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.JJEFBPKPKBK)?;
                },
                8 => {
                    self.JJEFBPKPKBK.push(is.read_uint32()?);
                },
                120 => {
                    self.DKPNENBHELH = is.read_uint32()?;
                },
                64 => {
                    self.JIKEIGBCABB = is.read_bool()?;
                },
                96 => {
                    self.MODDKLNDAMK = is.read_uint32()?;
                },
                16 => {
                    self.ACJCPHIFMLN = is.read_uint32()?;
                },
                112 => {
                    self.HJEFOLKGAEI = is.read_enum_or_unknown()?;
                },
                11578 => {
                    self.LIPJDJPMOKB.push(is.read_message()?);
                },
                80 => {
                    self.LIKHCLPMHJK = is.read_bool()?;
                },
                106 => {
                    is.read_repeated_packed_uint32_into(&mut self.PCMGAGAHBLK)?;
                },
                104 => {
                    self.PCMGAGAHBLK.push(is.read_uint32()?);
                },
                72 => {
                    self.PBFAIOJJGNL = is.read_uint32()?;
                },
                48 => {
                    self.BBDOPLEKIAC = is.read_uint32()?;
                },
                40 => {
                    self.NLIBKABFGCC = is.read_uint32()?;
                },
                32 => {
                    self.CEIFKJIEAJE = is.read_enum_or_unknown()?;
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
        if self.JIJHAAIHNCN != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.JIJHAAIHNCN);
        }
        if self.KHCMOFPFOAK != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.KHCMOFPFOAK);
        }
        if self.EIGABCKEDNP != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.EIGABCKEDNP);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(1, &self.JJEFBPKPKBK);
        if self.DKPNENBHELH != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.DKPNENBHELH);
        }
        if self.JIKEIGBCABB != false {
            my_size += 1 + 1;
        }
        if self.MODDKLNDAMK != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.MODDKLNDAMK);
        }
        if self.ACJCPHIFMLN != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.ACJCPHIFMLN);
        }
        if self.HJEFOLKGAEI != ::protobuf::EnumOrUnknown::new(super::JOMKPEGEFMP::JOMKPEGEFMP::MARBLE_TEAM_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(14, self.HJEFOLKGAEI.value());
        }
        for value in &self.LIPJDJPMOKB {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.LIKHCLPMHJK != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(13, &self.PCMGAGAHBLK);
        if self.PBFAIOJJGNL != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.PBFAIOJJGNL);
        }
        if self.BBDOPLEKIAC != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.BBDOPLEKIAC);
        }
        if self.NLIBKABFGCC != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.NLIBKABFGCC);
        }
        if self.CEIFKJIEAJE != ::protobuf::EnumOrUnknown::new(super::JOMKPEGEFMP::JOMKPEGEFMP::MARBLE_TEAM_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(4, self.CEIFKJIEAJE.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.JIJHAAIHNCN != 0 {
            os.write_uint32(11, self.JIJHAAIHNCN)?;
        }
        if self.KHCMOFPFOAK != 0 {
            os.write_uint32(7, self.KHCMOFPFOAK)?;
        }
        if self.EIGABCKEDNP != 0 {
            os.write_uint32(3, self.EIGABCKEDNP)?;
        }
        os.write_repeated_packed_uint32(1, &self.JJEFBPKPKBK)?;
        if self.DKPNENBHELH != 0 {
            os.write_uint32(15, self.DKPNENBHELH)?;
        }
        if self.JIKEIGBCABB != false {
            os.write_bool(8, self.JIKEIGBCABB)?;
        }
        if self.MODDKLNDAMK != 0 {
            os.write_uint32(12, self.MODDKLNDAMK)?;
        }
        if self.ACJCPHIFMLN != 0 {
            os.write_uint32(2, self.ACJCPHIFMLN)?;
        }
        if self.HJEFOLKGAEI != ::protobuf::EnumOrUnknown::new(super::JOMKPEGEFMP::JOMKPEGEFMP::MARBLE_TEAM_TYPE_NONE) {
            os.write_enum(14, ::protobuf::EnumOrUnknown::value(&self.HJEFOLKGAEI))?;
        }
        for v in &self.LIPJDJPMOKB {
            ::protobuf::rt::write_message_field_with_cached_size(1447, v, os)?;
        };
        if self.LIKHCLPMHJK != false {
            os.write_bool(10, self.LIKHCLPMHJK)?;
        }
        os.write_repeated_packed_uint32(13, &self.PCMGAGAHBLK)?;
        if self.PBFAIOJJGNL != 0 {
            os.write_uint32(9, self.PBFAIOJJGNL)?;
        }
        if self.BBDOPLEKIAC != 0 {
            os.write_uint32(6, self.BBDOPLEKIAC)?;
        }
        if self.NLIBKABFGCC != 0 {
            os.write_uint32(5, self.NLIBKABFGCC)?;
        }
        if self.CEIFKJIEAJE != ::protobuf::EnumOrUnknown::new(super::JOMKPEGEFMP::JOMKPEGEFMP::MARBLE_TEAM_TYPE_NONE) {
            os.write_enum(4, ::protobuf::EnumOrUnknown::value(&self.CEIFKJIEAJE))?;
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

    fn new() -> LBAOGIBPJOP {
        LBAOGIBPJOP::new()
    }

    fn clear(&mut self) {
        self.JIJHAAIHNCN = 0;
        self.KHCMOFPFOAK = 0;
        self.EIGABCKEDNP = 0;
        self.JJEFBPKPKBK.clear();
        self.DKPNENBHELH = 0;
        self.JIKEIGBCABB = false;
        self.MODDKLNDAMK = 0;
        self.ACJCPHIFMLN = 0;
        self.HJEFOLKGAEI = ::protobuf::EnumOrUnknown::new(super::JOMKPEGEFMP::JOMKPEGEFMP::MARBLE_TEAM_TYPE_NONE);
        self.LIPJDJPMOKB.clear();
        self.LIKHCLPMHJK = false;
        self.PCMGAGAHBLK.clear();
        self.PBFAIOJJGNL = 0;
        self.BBDOPLEKIAC = 0;
        self.NLIBKABFGCC = 0;
        self.CEIFKJIEAJE = ::protobuf::EnumOrUnknown::new(super::JOMKPEGEFMP::JOMKPEGEFMP::MARBLE_TEAM_TYPE_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LBAOGIBPJOP {
        static instance: LBAOGIBPJOP = LBAOGIBPJOP {
            JIJHAAIHNCN: 0,
            KHCMOFPFOAK: 0,
            EIGABCKEDNP: 0,
            JJEFBPKPKBK: ::std::vec::Vec::new(),
            DKPNENBHELH: 0,
            JIKEIGBCABB: false,
            MODDKLNDAMK: 0,
            ACJCPHIFMLN: 0,
            HJEFOLKGAEI: ::protobuf::EnumOrUnknown::from_i32(0),
            LIPJDJPMOKB: ::std::vec::Vec::new(),
            LIKHCLPMHJK: false,
            PCMGAGAHBLK: ::std::vec::Vec::new(),
            PBFAIOJJGNL: 0,
            BBDOPLEKIAC: 0,
            NLIBKABFGCC: 0,
            CEIFKJIEAJE: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LBAOGIBPJOP {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LBAOGIBPJOP").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LBAOGIBPJOP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LBAOGIBPJOP {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LBAOGIBPJOP.proto\x1a\x11CBBDIOMIFHD.proto\x1a\x11JOMKPEGEFMP.prot\
    o\"\xd8\x04\n\x0bLBAOGIBPJOP\x12\x20\n\x0bJIJHAAIHNCN\x18\x0b\x20\x01(\r\
    R\x0bJIJHAAIHNCN\x12\x20\n\x0bKHCMOFPFOAK\x18\x07\x20\x01(\rR\x0bKHCMOFP\
    FOAK\x12\x20\n\x0bEIGABCKEDNP\x18\x03\x20\x01(\rR\x0bEIGABCKEDNP\x12\x20\
    \n\x0bJJEFBPKPKBK\x18\x01\x20\x03(\rR\x0bJJEFBPKPKBK\x12\x20\n\x0bDKPNEN\
    BHELH\x18\x0f\x20\x01(\rR\x0bDKPNENBHELH\x12\x20\n\x0bJIKEIGBCABB\x18\
    \x08\x20\x01(\x08R\x0bJIKEIGBCABB\x12\x20\n\x0bMODDKLNDAMK\x18\x0c\x20\
    \x01(\rR\x0bMODDKLNDAMK\x12\x20\n\x0bACJCPHIFMLN\x18\x02\x20\x01(\rR\x0b\
    ACJCPHIFMLN\x12.\n\x0bHJEFOLKGAEI\x18\x0e\x20\x01(\x0e2\x0c.JOMKPEGEFMPR\
    \x0bHJEFOLKGAEI\x12/\n\x0bLIPJDJPMOKB\x18\xa7\x0b\x20\x03(\x0b2\x0c.CBBD\
    IOMIFHDR\x0bLIPJDJPMOKB\x12\x20\n\x0bLIKHCLPMHJK\x18\n\x20\x01(\x08R\x0b\
    LIKHCLPMHJK\x12\x20\n\x0bPCMGAGAHBLK\x18\r\x20\x03(\rR\x0bPCMGAGAHBLK\
    \x12\x20\n\x0bPBFAIOJJGNL\x18\t\x20\x01(\rR\x0bPBFAIOJJGNL\x12\x20\n\x0b\
    BBDOPLEKIAC\x18\x06\x20\x01(\rR\x0bBBDOPLEKIAC\x12\x20\n\x0bNLIBKABFGCC\
    \x18\x05\x20\x01(\rR\x0bNLIBKABFGCC\x12.\n\x0bCEIFKJIEAJE\x18\x04\x20\
    \x01(\x0e2\x0c.JOMKPEGEFMPR\x0bCEIFKJIEAJEb\x06proto3\
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
            deps.push(super::CBBDIOMIFHD::file_descriptor().clone());
            deps.push(super::JOMKPEGEFMP::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LBAOGIBPJOP::generated_message_descriptor_data());
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
