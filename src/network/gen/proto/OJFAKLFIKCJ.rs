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

//! Generated file from `OJFAKLFIKCJ.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:OJFAKLFIKCJ)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct OJFAKLFIKCJ {
    // message fields
    // @@protoc_insertion_point(field:OJFAKLFIKCJ.PFEANIAHFPC)
    pub PFEANIAHFPC: ::std::vec::Vec<super::FECADLCJFIC::FECADLCJFIC>,
    // @@protoc_insertion_point(field:OJFAKLFIKCJ.PJPMLCDHEBL)
    pub PJPMLCDHEBL: u64,
    // @@protoc_insertion_point(field:OJFAKLFIKCJ.PJBIPPDMCHE)
    pub PJBIPPDMCHE: u64,
    // @@protoc_insertion_point(field:OJFAKLFIKCJ.NKJPGKMLHNK)
    pub NKJPGKMLHNK: ::std::vec::Vec<super::LuckyKoiInfo::LuckyKoiInfo>,
    // @@protoc_insertion_point(field:OJFAKLFIKCJ.IIGMEOBDMJE)
    pub IIGMEOBDMJE: u32,
    // @@protoc_insertion_point(field:OJFAKLFIKCJ.OGCKDLKCABG)
    pub OGCKDLKCABG: u64,
    // special fields
    // @@protoc_insertion_point(special_field:OJFAKLFIKCJ.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a OJFAKLFIKCJ {
    fn default() -> &'a OJFAKLFIKCJ {
        <OJFAKLFIKCJ as ::protobuf::Message>::default_instance()
    }
}

impl OJFAKLFIKCJ {
    pub fn new() -> OJFAKLFIKCJ {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PFEANIAHFPC",
            |m: &OJFAKLFIKCJ| { &m.PFEANIAHFPC },
            |m: &mut OJFAKLFIKCJ| { &mut m.PFEANIAHFPC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PJPMLCDHEBL",
            |m: &OJFAKLFIKCJ| { &m.PJPMLCDHEBL },
            |m: &mut OJFAKLFIKCJ| { &mut m.PJPMLCDHEBL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PJBIPPDMCHE",
            |m: &OJFAKLFIKCJ| { &m.PJBIPPDMCHE },
            |m: &mut OJFAKLFIKCJ| { &mut m.PJBIPPDMCHE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NKJPGKMLHNK",
            |m: &OJFAKLFIKCJ| { &m.NKJPGKMLHNK },
            |m: &mut OJFAKLFIKCJ| { &mut m.NKJPGKMLHNK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IIGMEOBDMJE",
            |m: &OJFAKLFIKCJ| { &m.IIGMEOBDMJE },
            |m: &mut OJFAKLFIKCJ| { &mut m.IIGMEOBDMJE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OGCKDLKCABG",
            |m: &OJFAKLFIKCJ| { &m.OGCKDLKCABG },
            |m: &mut OJFAKLFIKCJ| { &mut m.OGCKDLKCABG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<OJFAKLFIKCJ>(
            "OJFAKLFIKCJ",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for OJFAKLFIKCJ {
    const NAME: &'static str = "OJFAKLFIKCJ";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                122 => {
                    self.PFEANIAHFPC.push(is.read_message()?);
                },
                96 => {
                    self.PJPMLCDHEBL = is.read_uint64()?;
                },
                72 => {
                    self.PJBIPPDMCHE = is.read_uint64()?;
                },
                90 => {
                    self.NKJPGKMLHNK.push(is.read_message()?);
                },
                112 => {
                    self.IIGMEOBDMJE = is.read_uint32()?;
                },
                40 => {
                    self.OGCKDLKCABG = is.read_uint64()?;
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
        for value in &self.PFEANIAHFPC {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.PJPMLCDHEBL != 0 {
            my_size += ::protobuf::rt::uint64_size(12, self.PJPMLCDHEBL);
        }
        if self.PJBIPPDMCHE != 0 {
            my_size += ::protobuf::rt::uint64_size(9, self.PJBIPPDMCHE);
        }
        for value in &self.NKJPGKMLHNK {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.IIGMEOBDMJE != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.IIGMEOBDMJE);
        }
        if self.OGCKDLKCABG != 0 {
            my_size += ::protobuf::rt::uint64_size(5, self.OGCKDLKCABG);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.PFEANIAHFPC {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        };
        if self.PJPMLCDHEBL != 0 {
            os.write_uint64(12, self.PJPMLCDHEBL)?;
        }
        if self.PJBIPPDMCHE != 0 {
            os.write_uint64(9, self.PJBIPPDMCHE)?;
        }
        for v in &self.NKJPGKMLHNK {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        if self.IIGMEOBDMJE != 0 {
            os.write_uint32(14, self.IIGMEOBDMJE)?;
        }
        if self.OGCKDLKCABG != 0 {
            os.write_uint64(5, self.OGCKDLKCABG)?;
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

    fn new() -> OJFAKLFIKCJ {
        OJFAKLFIKCJ::new()
    }

    fn clear(&mut self) {
        self.PFEANIAHFPC.clear();
        self.PJPMLCDHEBL = 0;
        self.PJBIPPDMCHE = 0;
        self.NKJPGKMLHNK.clear();
        self.IIGMEOBDMJE = 0;
        self.OGCKDLKCABG = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static OJFAKLFIKCJ {
        static instance: OJFAKLFIKCJ = OJFAKLFIKCJ {
            PFEANIAHFPC: ::std::vec::Vec::new(),
            PJPMLCDHEBL: 0,
            PJBIPPDMCHE: 0,
            NKJPGKMLHNK: ::std::vec::Vec::new(),
            IIGMEOBDMJE: 0,
            OGCKDLKCABG: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for OJFAKLFIKCJ {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("OJFAKLFIKCJ").unwrap()).clone()
    }
}

impl ::std::fmt::Display for OJFAKLFIKCJ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OJFAKLFIKCJ {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11OJFAKLFIKCJ.proto\x1a\x11FECADLCJFIC.proto\x1a\x12LuckyKoiInfo.pro\
    to\"\xf6\x01\n\x0bOJFAKLFIKCJ\x12.\n\x0bPFEANIAHFPC\x18\x0f\x20\x03(\x0b\
    2\x0c.FECADLCJFICR\x0bPFEANIAHFPC\x12\x20\n\x0bPJPMLCDHEBL\x18\x0c\x20\
    \x01(\x04R\x0bPJPMLCDHEBL\x12\x20\n\x0bPJBIPPDMCHE\x18\t\x20\x01(\x04R\
    \x0bPJBIPPDMCHE\x12/\n\x0bNKJPGKMLHNK\x18\x0b\x20\x03(\x0b2\r.LuckyKoiIn\
    foR\x0bNKJPGKMLHNK\x12\x20\n\x0bIIGMEOBDMJE\x18\x0e\x20\x01(\rR\x0bIIGME\
    OBDMJE\x12\x20\n\x0bOGCKDLKCABG\x18\x05\x20\x01(\x04R\x0bOGCKDLKCABGb\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::FECADLCJFIC::file_descriptor().clone());
            deps.push(super::LuckyKoiInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(OJFAKLFIKCJ::generated_message_descriptor_data());
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
