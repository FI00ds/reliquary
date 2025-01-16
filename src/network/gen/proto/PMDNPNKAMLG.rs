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

//! Generated file from `PMDNPNKAMLG.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PMDNPNKAMLG)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PMDNPNKAMLG {
    // message fields
    // @@protoc_insertion_point(field:PMDNPNKAMLG.DDGFFPGNCED)
    pub DDGFFPGNCED: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:PMDNPNKAMLG.KBALPDHMBNP)
    pub KBALPDHMBNP: u32,
    // @@protoc_insertion_point(field:PMDNPNKAMLG.KILOLIKCLPD)
    pub KILOLIKCLPD: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:PMDNPNKAMLG.ACCPEHFPKAB)
    pub ACCPEHFPKAB: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:PMDNPNKAMLG.BGOGKAMJLMB)
    pub BGOGKAMJLMB: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:PMDNPNKAMLG.HIFACPPEMOD)
    pub HIFACPPEMOD: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:PMDNPNKAMLG.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PMDNPNKAMLG {
    fn default() -> &'a PMDNPNKAMLG {
        <PMDNPNKAMLG as ::protobuf::Message>::default_instance()
    }
}

impl PMDNPNKAMLG {
    pub fn new() -> PMDNPNKAMLG {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "DDGFFPGNCED",
            |m: &PMDNPNKAMLG| { &m.DDGFFPGNCED },
            |m: &mut PMDNPNKAMLG| { &mut m.DDGFFPGNCED },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KBALPDHMBNP",
            |m: &PMDNPNKAMLG| { &m.KBALPDHMBNP },
            |m: &mut PMDNPNKAMLG| { &mut m.KBALPDHMBNP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KILOLIKCLPD",
            |m: &PMDNPNKAMLG| { &m.KILOLIKCLPD },
            |m: &mut PMDNPNKAMLG| { &mut m.KILOLIKCLPD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "ACCPEHFPKAB",
            |m: &PMDNPNKAMLG| { &m.ACCPEHFPKAB },
            |m: &mut PMDNPNKAMLG| { &mut m.ACCPEHFPKAB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "BGOGKAMJLMB",
            |m: &PMDNPNKAMLG| { &m.BGOGKAMJLMB },
            |m: &mut PMDNPNKAMLG| { &mut m.BGOGKAMJLMB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HIFACPPEMOD",
            |m: &PMDNPNKAMLG| { &m.HIFACPPEMOD },
            |m: &mut PMDNPNKAMLG| { &mut m.HIFACPPEMOD },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PMDNPNKAMLG>(
            "PMDNPNKAMLG",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PMDNPNKAMLG {
    const NAME: &'static str = "PMDNPNKAMLG";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                82 => {
                    is.read_repeated_packed_uint32_into(&mut self.DDGFFPGNCED)?;
                },
                80 => {
                    self.DDGFFPGNCED.push(is.read_uint32()?);
                },
                104 => {
                    self.KBALPDHMBNP = is.read_uint32()?;
                },
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.KILOLIKCLPD)?;
                },
                8 => {
                    self.KILOLIKCLPD.push(is.read_uint32()?);
                },
                74 => {
                    is.read_repeated_packed_uint32_into(&mut self.ACCPEHFPKAB)?;
                },
                72 => {
                    self.ACCPEHFPKAB.push(is.read_uint32()?);
                },
                98 => {
                    is.read_repeated_packed_uint32_into(&mut self.BGOGKAMJLMB)?;
                },
                96 => {
                    self.BGOGKAMJLMB.push(is.read_uint32()?);
                },
                34 => {
                    is.read_repeated_packed_uint32_into(&mut self.HIFACPPEMOD)?;
                },
                32 => {
                    self.HIFACPPEMOD.push(is.read_uint32()?);
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
        for value in &self.DDGFFPGNCED {
            my_size += ::protobuf::rt::uint32_size(10, *value);
        };
        if self.KBALPDHMBNP != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.KBALPDHMBNP);
        }
        for value in &self.KILOLIKCLPD {
            my_size += ::protobuf::rt::uint32_size(1, *value);
        };
        for value in &self.ACCPEHFPKAB {
            my_size += ::protobuf::rt::uint32_size(9, *value);
        };
        for value in &self.BGOGKAMJLMB {
            my_size += ::protobuf::rt::uint32_size(12, *value);
        };
        for value in &self.HIFACPPEMOD {
            my_size += ::protobuf::rt::uint32_size(4, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.DDGFFPGNCED {
            os.write_uint32(10, *v)?;
        };
        if self.KBALPDHMBNP != 0 {
            os.write_uint32(13, self.KBALPDHMBNP)?;
        }
        for v in &self.KILOLIKCLPD {
            os.write_uint32(1, *v)?;
        };
        for v in &self.ACCPEHFPKAB {
            os.write_uint32(9, *v)?;
        };
        for v in &self.BGOGKAMJLMB {
            os.write_uint32(12, *v)?;
        };
        for v in &self.HIFACPPEMOD {
            os.write_uint32(4, *v)?;
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

    fn new() -> PMDNPNKAMLG {
        PMDNPNKAMLG::new()
    }

    fn clear(&mut self) {
        self.DDGFFPGNCED.clear();
        self.KBALPDHMBNP = 0;
        self.KILOLIKCLPD.clear();
        self.ACCPEHFPKAB.clear();
        self.BGOGKAMJLMB.clear();
        self.HIFACPPEMOD.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PMDNPNKAMLG {
        static instance: PMDNPNKAMLG = PMDNPNKAMLG {
            DDGFFPGNCED: ::std::vec::Vec::new(),
            KBALPDHMBNP: 0,
            KILOLIKCLPD: ::std::vec::Vec::new(),
            ACCPEHFPKAB: ::std::vec::Vec::new(),
            BGOGKAMJLMB: ::std::vec::Vec::new(),
            HIFACPPEMOD: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PMDNPNKAMLG {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PMDNPNKAMLG").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PMDNPNKAMLG {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PMDNPNKAMLG {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PMDNPNKAMLG.proto\"\xd9\x01\n\x0bPMDNPNKAMLG\x12\x20\n\x0bDDGFFPGN\
    CED\x18\n\x20\x03(\rR\x0bDDGFFPGNCED\x12\x20\n\x0bKBALPDHMBNP\x18\r\x20\
    \x01(\rR\x0bKBALPDHMBNP\x12\x20\n\x0bKILOLIKCLPD\x18\x01\x20\x03(\rR\x0b\
    KILOLIKCLPD\x12\x20\n\x0bACCPEHFPKAB\x18\t\x20\x03(\rR\x0bACCPEHFPKAB\
    \x12\x20\n\x0bBGOGKAMJLMB\x18\x0c\x20\x03(\rR\x0bBGOGKAMJLMB\x12\x20\n\
    \x0bHIFACPPEMOD\x18\x04\x20\x03(\rR\x0bHIFACPPEMODb\x06proto3\
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
            messages.push(PMDNPNKAMLG::generated_message_descriptor_data());
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
