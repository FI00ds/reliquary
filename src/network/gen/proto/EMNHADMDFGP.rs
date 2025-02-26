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

//! Generated file from `EMNHADMDFGP.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:EMNHADMDFGP)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EMNHADMDFGP {
    // message fields
    // @@protoc_insertion_point(field:EMNHADMDFGP.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:EMNHADMDFGP.DNPOBGICFCJ)
    pub DNPOBGICFCJ: ::std::string::String,
    // @@protoc_insertion_point(field:EMNHADMDFGP.ICBMAPMKPPG)
    pub ICBMAPMKPPG: ::std::string::String,
    // @@protoc_insertion_point(field:EMNHADMDFGP.CJAFOALDKBM)
    pub CJAFOALDKBM: ::std::vec::Vec<super::PHIMJDOFHBH::PHIMJDOFHBH>,
    // @@protoc_insertion_point(field:EMNHADMDFGP.AEECMAGMGAD)
    pub AEECMAGMGAD: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:EMNHADMDFGP.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EMNHADMDFGP {
    fn default() -> &'a EMNHADMDFGP {
        <EMNHADMDFGP as ::protobuf::Message>::default_instance()
    }
}

impl EMNHADMDFGP {
    pub fn new() -> EMNHADMDFGP {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &EMNHADMDFGP| { &m.retcode },
            |m: &mut EMNHADMDFGP| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DNPOBGICFCJ",
            |m: &EMNHADMDFGP| { &m.DNPOBGICFCJ },
            |m: &mut EMNHADMDFGP| { &mut m.DNPOBGICFCJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ICBMAPMKPPG",
            |m: &EMNHADMDFGP| { &m.ICBMAPMKPPG },
            |m: &mut EMNHADMDFGP| { &mut m.ICBMAPMKPPG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CJAFOALDKBM",
            |m: &EMNHADMDFGP| { &m.CJAFOALDKBM },
            |m: &mut EMNHADMDFGP| { &mut m.CJAFOALDKBM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AEECMAGMGAD",
            |m: &EMNHADMDFGP| { &m.AEECMAGMGAD },
            |m: &mut EMNHADMDFGP| { &mut m.AEECMAGMGAD },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EMNHADMDFGP>(
            "EMNHADMDFGP",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EMNHADMDFGP {
    const NAME: &'static str = "EMNHADMDFGP";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.retcode = is.read_uint32()?;
                },
                18 => {
                    self.DNPOBGICFCJ = is.read_string()?;
                },
                26 => {
                    self.ICBMAPMKPPG = is.read_string()?;
                },
                34 => {
                    self.CJAFOALDKBM.push(is.read_message()?);
                },
                42 => {
                    self.AEECMAGMGAD = is.read_string()?;
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
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.retcode);
        }
        if !self.DNPOBGICFCJ.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.DNPOBGICFCJ);
        }
        if !self.ICBMAPMKPPG.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.ICBMAPMKPPG);
        }
        for value in &self.CJAFOALDKBM {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if !self.AEECMAGMGAD.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.AEECMAGMGAD);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.retcode != 0 {
            os.write_uint32(1, self.retcode)?;
        }
        if !self.DNPOBGICFCJ.is_empty() {
            os.write_string(2, &self.DNPOBGICFCJ)?;
        }
        if !self.ICBMAPMKPPG.is_empty() {
            os.write_string(3, &self.ICBMAPMKPPG)?;
        }
        for v in &self.CJAFOALDKBM {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        if !self.AEECMAGMGAD.is_empty() {
            os.write_string(5, &self.AEECMAGMGAD)?;
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

    fn new() -> EMNHADMDFGP {
        EMNHADMDFGP::new()
    }

    fn clear(&mut self) {
        self.retcode = 0;
        self.DNPOBGICFCJ.clear();
        self.ICBMAPMKPPG.clear();
        self.CJAFOALDKBM.clear();
        self.AEECMAGMGAD.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EMNHADMDFGP {
        static instance: EMNHADMDFGP = EMNHADMDFGP {
            retcode: 0,
            DNPOBGICFCJ: ::std::string::String::new(),
            ICBMAPMKPPG: ::std::string::String::new(),
            CJAFOALDKBM: ::std::vec::Vec::new(),
            AEECMAGMGAD: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EMNHADMDFGP {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EMNHADMDFGP").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EMNHADMDFGP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EMNHADMDFGP {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11EMNHADMDFGP.proto\x1a\x11PHIMJDOFHBH.proto\"\xbd\x01\n\x0bEMNHADMD\
    FGP\x12\x18\n\x07retcode\x18\x01\x20\x01(\rR\x07retcode\x12\x20\n\x0bDNP\
    OBGICFCJ\x18\x02\x20\x01(\tR\x0bDNPOBGICFCJ\x12\x20\n\x0bICBMAPMKPPG\x18\
    \x03\x20\x01(\tR\x0bICBMAPMKPPG\x12.\n\x0bCJAFOALDKBM\x18\x04\x20\x03(\
    \x0b2\x0c.PHIMJDOFHBHR\x0bCJAFOALDKBM\x12\x20\n\x0bAEECMAGMGAD\x18\x05\
    \x20\x01(\tR\x0bAEECMAGMGADb\x06proto3\
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
            deps.push(super::PHIMJDOFHBH::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EMNHADMDFGP::generated_message_descriptor_data());
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
