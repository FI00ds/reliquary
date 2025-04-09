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

//! Generated file from `BKBILPDKOIL.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:BKBILPDKOIL)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BKBILPDKOIL {
    // message fields
    // @@protoc_insertion_point(field:BKBILPDKOIL.LHJPKMDMNMP)
    pub LHJPKMDMNMP: u32,
    // @@protoc_insertion_point(field:BKBILPDKOIL.HPJJDCJHHOA)
    pub HPJJDCJHHOA: u64,
    // @@protoc_insertion_point(field:BKBILPDKOIL.HJEJFNFAMPN)
    pub HJEJFNFAMPN: i64,
    // @@protoc_insertion_point(field:BKBILPDKOIL.CABEHKOFLPG)
    pub CABEHKOFLPG: bool,
    // @@protoc_insertion_point(field:BKBILPDKOIL.JPACOBGBDBG)
    pub JPACOBGBDBG: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:BKBILPDKOIL.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BKBILPDKOIL {
    fn default() -> &'a BKBILPDKOIL {
        <BKBILPDKOIL as ::protobuf::Message>::default_instance()
    }
}

impl BKBILPDKOIL {
    pub fn new() -> BKBILPDKOIL {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LHJPKMDMNMP",
            |m: &BKBILPDKOIL| { &m.LHJPKMDMNMP },
            |m: &mut BKBILPDKOIL| { &mut m.LHJPKMDMNMP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HPJJDCJHHOA",
            |m: &BKBILPDKOIL| { &m.HPJJDCJHHOA },
            |m: &mut BKBILPDKOIL| { &mut m.HPJJDCJHHOA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HJEJFNFAMPN",
            |m: &BKBILPDKOIL| { &m.HJEJFNFAMPN },
            |m: &mut BKBILPDKOIL| { &mut m.HJEJFNFAMPN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CABEHKOFLPG",
            |m: &BKBILPDKOIL| { &m.CABEHKOFLPG },
            |m: &mut BKBILPDKOIL| { &mut m.CABEHKOFLPG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JPACOBGBDBG",
            |m: &BKBILPDKOIL| { &m.JPACOBGBDBG },
            |m: &mut BKBILPDKOIL| { &mut m.JPACOBGBDBG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BKBILPDKOIL>(
            "BKBILPDKOIL",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BKBILPDKOIL {
    const NAME: &'static str = "BKBILPDKOIL";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.LHJPKMDMNMP = is.read_uint32()?;
                },
                88 => {
                    self.HPJJDCJHHOA = is.read_uint64()?;
                },
                8 => {
                    self.HJEJFNFAMPN = is.read_int64()?;
                },
                24 => {
                    self.CABEHKOFLPG = is.read_bool()?;
                },
                74 => {
                    is.read_repeated_packed_uint32_into(&mut self.JPACOBGBDBG)?;
                },
                72 => {
                    self.JPACOBGBDBG.push(is.read_uint32()?);
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
        if self.LHJPKMDMNMP != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.LHJPKMDMNMP);
        }
        if self.HPJJDCJHHOA != 0 {
            my_size += ::protobuf::rt::uint64_size(11, self.HPJJDCJHHOA);
        }
        if self.HJEJFNFAMPN != 0 {
            my_size += ::protobuf::rt::int64_size(1, self.HJEJFNFAMPN);
        }
        if self.CABEHKOFLPG != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(9, &self.JPACOBGBDBG);
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.LHJPKMDMNMP != 0 {
            os.write_uint32(15, self.LHJPKMDMNMP)?;
        }
        if self.HPJJDCJHHOA != 0 {
            os.write_uint64(11, self.HPJJDCJHHOA)?;
        }
        if self.HJEJFNFAMPN != 0 {
            os.write_int64(1, self.HJEJFNFAMPN)?;
        }
        if self.CABEHKOFLPG != false {
            os.write_bool(3, self.CABEHKOFLPG)?;
        }
        os.write_repeated_packed_uint32(9, &self.JPACOBGBDBG)?;
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> BKBILPDKOIL {
        BKBILPDKOIL::new()
    }

    fn clear(&mut self) {
        self.LHJPKMDMNMP = 0;
        self.HPJJDCJHHOA = 0;
        self.HJEJFNFAMPN = 0;
        self.CABEHKOFLPG = false;
        self.JPACOBGBDBG.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BKBILPDKOIL {
        static instance: BKBILPDKOIL = BKBILPDKOIL {
            LHJPKMDMNMP: 0,
            HPJJDCJHHOA: 0,
            HJEJFNFAMPN: 0,
            CABEHKOFLPG: false,
            JPACOBGBDBG: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BKBILPDKOIL {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BKBILPDKOIL").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BKBILPDKOIL {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BKBILPDKOIL {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11BKBILPDKOIL.proto\"\xb7\x01\n\x0bBKBILPDKOIL\x12\x20\n\x0bLHJPKMDM\
    NMP\x18\x0f\x20\x01(\rR\x0bLHJPKMDMNMP\x12\x20\n\x0bHPJJDCJHHOA\x18\x0b\
    \x20\x01(\x04R\x0bHPJJDCJHHOA\x12\x20\n\x0bHJEJFNFAMPN\x18\x01\x20\x01(\
    \x03R\x0bHJEJFNFAMPN\x12\x20\n\x0bCABEHKOFLPG\x18\x03\x20\x01(\x08R\x0bC\
    ABEHKOFLPG\x12\x20\n\x0bJPACOBGBDBG\x18\t\x20\x03(\rR\x0bJPACOBGBDBGb\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(BKBILPDKOIL::generated_message_descriptor_data());
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
