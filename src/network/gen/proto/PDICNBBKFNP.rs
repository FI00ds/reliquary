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

//! Generated file from `PDICNBBKFNP.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:PDICNBBKFNP)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PDICNBBKFNP {
    // message fields
    // @@protoc_insertion_point(field:PDICNBBKFNP.EODGCNAFIAC)
    pub EODGCNAFIAC: u32,
    // @@protoc_insertion_point(field:PDICNBBKFNP.exp)
    pub exp: u32,
    // @@protoc_insertion_point(field:PDICNBBKFNP.KLGHECCBHCG)
    pub KLGHECCBHCG: ::std::vec::Vec<super::CEENLALPDMK::CEENLALPDMK>,
    // @@protoc_insertion_point(field:PDICNBBKFNP.FJOCDKIFPPC)
    pub FJOCDKIFPPC: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:PDICNBBKFNP.CPIOGJKFECH)
    pub CPIOGJKFECH: u32,
    // @@protoc_insertion_point(field:PDICNBBKFNP.OOFHJAHFIDH)
    pub OOFHJAHFIDH: bool,
    // @@protoc_insertion_point(field:PDICNBBKFNP.DEHGHEDINIH)
    pub DEHGHEDINIH: bool,
    // @@protoc_insertion_point(field:PDICNBBKFNP.COKDNPEEMAG)
    pub COKDNPEEMAG: ::std::vec::Vec<super::IMGJIEBFGPF::IMGJIEBFGPF>,
    // @@protoc_insertion_point(field:PDICNBBKFNP.LCJNNDGKIDP)
    pub LCJNNDGKIDP: ::std::vec::Vec<super::ECMMJLLHPMD::ECMMJLLHPMD>,
    // @@protoc_insertion_point(field:PDICNBBKFNP.LMELJCIFBDF)
    pub LMELJCIFBDF: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:PDICNBBKFNP.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PDICNBBKFNP {
    fn default() -> &'a PDICNBBKFNP {
        <PDICNBBKFNP as ::protobuf::Message>::default_instance()
    }
}

impl PDICNBBKFNP {
    pub fn new() -> PDICNBBKFNP {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(10);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EODGCNAFIAC",
            |m: &PDICNBBKFNP| { &m.EODGCNAFIAC },
            |m: &mut PDICNBBKFNP| { &mut m.EODGCNAFIAC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "exp",
            |m: &PDICNBBKFNP| { &m.exp },
            |m: &mut PDICNBBKFNP| { &mut m.exp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KLGHECCBHCG",
            |m: &PDICNBBKFNP| { &m.KLGHECCBHCG },
            |m: &mut PDICNBBKFNP| { &mut m.KLGHECCBHCG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FJOCDKIFPPC",
            |m: &PDICNBBKFNP| { &m.FJOCDKIFPPC },
            |m: &mut PDICNBBKFNP| { &mut m.FJOCDKIFPPC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CPIOGJKFECH",
            |m: &PDICNBBKFNP| { &m.CPIOGJKFECH },
            |m: &mut PDICNBBKFNP| { &mut m.CPIOGJKFECH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OOFHJAHFIDH",
            |m: &PDICNBBKFNP| { &m.OOFHJAHFIDH },
            |m: &mut PDICNBBKFNP| { &mut m.OOFHJAHFIDH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DEHGHEDINIH",
            |m: &PDICNBBKFNP| { &m.DEHGHEDINIH },
            |m: &mut PDICNBBKFNP| { &mut m.DEHGHEDINIH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "COKDNPEEMAG",
            |m: &PDICNBBKFNP| { &m.COKDNPEEMAG },
            |m: &mut PDICNBBKFNP| { &mut m.COKDNPEEMAG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LCJNNDGKIDP",
            |m: &PDICNBBKFNP| { &m.LCJNNDGKIDP },
            |m: &mut PDICNBBKFNP| { &mut m.LCJNNDGKIDP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LMELJCIFBDF",
            |m: &PDICNBBKFNP| { &m.LMELJCIFBDF },
            |m: &mut PDICNBBKFNP| { &mut m.LMELJCIFBDF },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PDICNBBKFNP>(
            "PDICNBBKFNP",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PDICNBBKFNP {
    const NAME: &'static str = "PDICNBBKFNP";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.EODGCNAFIAC = is.read_uint32()?;
                },
                24 => {
                    self.exp = is.read_uint32()?;
                },
                58 => {
                    self.KLGHECCBHCG.push(is.read_message()?);
                },
                90 => {
                    is.read_repeated_packed_uint32_into(&mut self.FJOCDKIFPPC)?;
                },
                88 => {
                    self.FJOCDKIFPPC.push(is.read_uint32()?);
                },
                8 => {
                    self.CPIOGJKFECH = is.read_uint32()?;
                },
                96 => {
                    self.OOFHJAHFIDH = is.read_bool()?;
                },
                32 => {
                    self.DEHGHEDINIH = is.read_bool()?;
                },
                42 => {
                    self.COKDNPEEMAG.push(is.read_message()?);
                },
                18 => {
                    self.LCJNNDGKIDP.push(is.read_message()?);
                },
                82 => {
                    is.read_repeated_packed_uint32_into(&mut self.LMELJCIFBDF)?;
                },
                80 => {
                    self.LMELJCIFBDF.push(is.read_uint32()?);
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
        if self.EODGCNAFIAC != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.EODGCNAFIAC);
        }
        if self.exp != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.exp);
        }
        for value in &self.KLGHECCBHCG {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::vec_packed_uint32_size(11, &self.FJOCDKIFPPC);
        if self.CPIOGJKFECH != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.CPIOGJKFECH);
        }
        if self.OOFHJAHFIDH != false {
            my_size += 1 + 1;
        }
        if self.DEHGHEDINIH != false {
            my_size += 1 + 1;
        }
        for value in &self.COKDNPEEMAG {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.LCJNNDGKIDP {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::vec_packed_uint32_size(10, &self.LMELJCIFBDF);
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.EODGCNAFIAC != 0 {
            os.write_uint32(13, self.EODGCNAFIAC)?;
        }
        if self.exp != 0 {
            os.write_uint32(3, self.exp)?;
        }
        for v in &self.KLGHECCBHCG {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        };
        os.write_repeated_packed_uint32(11, &self.FJOCDKIFPPC)?;
        if self.CPIOGJKFECH != 0 {
            os.write_uint32(1, self.CPIOGJKFECH)?;
        }
        if self.OOFHJAHFIDH != false {
            os.write_bool(12, self.OOFHJAHFIDH)?;
        }
        if self.DEHGHEDINIH != false {
            os.write_bool(4, self.DEHGHEDINIH)?;
        }
        for v in &self.COKDNPEEMAG {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        for v in &self.LCJNNDGKIDP {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        os.write_repeated_packed_uint32(10, &self.LMELJCIFBDF)?;
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> PDICNBBKFNP {
        PDICNBBKFNP::new()
    }

    fn clear(&mut self) {
        self.EODGCNAFIAC = 0;
        self.exp = 0;
        self.KLGHECCBHCG.clear();
        self.FJOCDKIFPPC.clear();
        self.CPIOGJKFECH = 0;
        self.OOFHJAHFIDH = false;
        self.DEHGHEDINIH = false;
        self.COKDNPEEMAG.clear();
        self.LCJNNDGKIDP.clear();
        self.LMELJCIFBDF.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PDICNBBKFNP {
        static instance: PDICNBBKFNP = PDICNBBKFNP {
            EODGCNAFIAC: 0,
            exp: 0,
            KLGHECCBHCG: ::std::vec::Vec::new(),
            FJOCDKIFPPC: ::std::vec::Vec::new(),
            CPIOGJKFECH: 0,
            OOFHJAHFIDH: false,
            DEHGHEDINIH: false,
            COKDNPEEMAG: ::std::vec::Vec::new(),
            LCJNNDGKIDP: ::std::vec::Vec::new(),
            LMELJCIFBDF: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PDICNBBKFNP {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PDICNBBKFNP").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PDICNBBKFNP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PDICNBBKFNP {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PDICNBBKFNP.proto\x1a\x11CEENLALPDMK.proto\x1a\x11ECMMJLLHPMD.prot\
    o\x1a\x11IMGJIEBFGPF.proto\"\xfb\x02\n\x0bPDICNBBKFNP\x12\x20\n\x0bEODGC\
    NAFIAC\x18\r\x20\x01(\rR\x0bEODGCNAFIAC\x12\x10\n\x03exp\x18\x03\x20\x01\
    (\rR\x03exp\x12.\n\x0bKLGHECCBHCG\x18\x07\x20\x03(\x0b2\x0c.CEENLALPDMKR\
    \x0bKLGHECCBHCG\x12\x20\n\x0bFJOCDKIFPPC\x18\x0b\x20\x03(\rR\x0bFJOCDKIF\
    PPC\x12\x20\n\x0bCPIOGJKFECH\x18\x01\x20\x01(\rR\x0bCPIOGJKFECH\x12\x20\
    \n\x0bOOFHJAHFIDH\x18\x0c\x20\x01(\x08R\x0bOOFHJAHFIDH\x12\x20\n\x0bDEHG\
    HEDINIH\x18\x04\x20\x01(\x08R\x0bDEHGHEDINIH\x12.\n\x0bCOKDNPEEMAG\x18\
    \x05\x20\x03(\x0b2\x0c.IMGJIEBFGPFR\x0bCOKDNPEEMAG\x12.\n\x0bLCJNNDGKIDP\
    \x18\x02\x20\x03(\x0b2\x0c.ECMMJLLHPMDR\x0bLCJNNDGKIDP\x12\x20\n\x0bLMEL\
    JCIFBDF\x18\n\x20\x03(\rR\x0bLMELJCIFBDFb\x06proto3\
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
            deps.push(super::CEENLALPDMK::file_descriptor().clone());
            deps.push(super::ECMMJLLHPMD::file_descriptor().clone());
            deps.push(super::IMGJIEBFGPF::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PDICNBBKFNP::generated_message_descriptor_data());
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
