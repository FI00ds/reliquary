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

//! Generated file from `CDIMEMFJJFP.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:CDIMEMFJJFP)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CDIMEMFJJFP {
    // message fields
    // @@protoc_insertion_point(field:CDIMEMFJJFP.LMMGODPHJNE)
    pub LMMGODPHJNE: u32,
    // @@protoc_insertion_point(field:CDIMEMFJJFP.PGJCCGNBBPI)
    pub PGJCCGNBBPI: u32,
    // @@protoc_insertion_point(field:CDIMEMFJJFP.NNBHKCJCPIO)
    pub NNBHKCJCPIO: u32,
    // @@protoc_insertion_point(field:CDIMEMFJJFP.KHBNJGPPHOA)
    pub KHBNJGPPHOA: u32,
    // @@protoc_insertion_point(field:CDIMEMFJJFP.AGDCEBLFGKH)
    pub AGDCEBLFGKH: u32,
    // @@protoc_insertion_point(field:CDIMEMFJJFP.DNCPBBLIOPL)
    pub DNCPBBLIOPL: u32,
    // @@protoc_insertion_point(field:CDIMEMFJJFP.LILIFGBAFKN)
    pub LILIFGBAFKN: u32,
    // @@protoc_insertion_point(field:CDIMEMFJJFP.MMIIJHOHOGE)
    pub MMIIJHOHOGE: u32,
    // @@protoc_insertion_point(field:CDIMEMFJJFP.IOKFIKHHANG)
    pub IOKFIKHHANG: u32,
    // @@protoc_insertion_point(field:CDIMEMFJJFP.CILKFJBLEJG)
    pub CILKFJBLEJG: u32,
    // special fields
    // @@protoc_insertion_point(special_field:CDIMEMFJJFP.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CDIMEMFJJFP {
    fn default() -> &'a CDIMEMFJJFP {
        <CDIMEMFJJFP as ::protobuf::Message>::default_instance()
    }
}

impl CDIMEMFJJFP {
    pub fn new() -> CDIMEMFJJFP {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(10);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LMMGODPHJNE",
            |m: &CDIMEMFJJFP| { &m.LMMGODPHJNE },
            |m: &mut CDIMEMFJJFP| { &mut m.LMMGODPHJNE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PGJCCGNBBPI",
            |m: &CDIMEMFJJFP| { &m.PGJCCGNBBPI },
            |m: &mut CDIMEMFJJFP| { &mut m.PGJCCGNBBPI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NNBHKCJCPIO",
            |m: &CDIMEMFJJFP| { &m.NNBHKCJCPIO },
            |m: &mut CDIMEMFJJFP| { &mut m.NNBHKCJCPIO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KHBNJGPPHOA",
            |m: &CDIMEMFJJFP| { &m.KHBNJGPPHOA },
            |m: &mut CDIMEMFJJFP| { &mut m.KHBNJGPPHOA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AGDCEBLFGKH",
            |m: &CDIMEMFJJFP| { &m.AGDCEBLFGKH },
            |m: &mut CDIMEMFJJFP| { &mut m.AGDCEBLFGKH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DNCPBBLIOPL",
            |m: &CDIMEMFJJFP| { &m.DNCPBBLIOPL },
            |m: &mut CDIMEMFJJFP| { &mut m.DNCPBBLIOPL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LILIFGBAFKN",
            |m: &CDIMEMFJJFP| { &m.LILIFGBAFKN },
            |m: &mut CDIMEMFJJFP| { &mut m.LILIFGBAFKN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MMIIJHOHOGE",
            |m: &CDIMEMFJJFP| { &m.MMIIJHOHOGE },
            |m: &mut CDIMEMFJJFP| { &mut m.MMIIJHOHOGE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IOKFIKHHANG",
            |m: &CDIMEMFJJFP| { &m.IOKFIKHHANG },
            |m: &mut CDIMEMFJJFP| { &mut m.IOKFIKHHANG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CILKFJBLEJG",
            |m: &CDIMEMFJJFP| { &m.CILKFJBLEJG },
            |m: &mut CDIMEMFJJFP| { &mut m.CILKFJBLEJG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CDIMEMFJJFP>(
            "CDIMEMFJJFP",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CDIMEMFJJFP {
    const NAME: &'static str = "CDIMEMFJJFP";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.LMMGODPHJNE = is.read_uint32()?;
                },
                16 => {
                    self.PGJCCGNBBPI = is.read_uint32()?;
                },
                24 => {
                    self.NNBHKCJCPIO = is.read_uint32()?;
                },
                32 => {
                    self.KHBNJGPPHOA = is.read_uint32()?;
                },
                40 => {
                    self.AGDCEBLFGKH = is.read_uint32()?;
                },
                48 => {
                    self.DNCPBBLIOPL = is.read_uint32()?;
                },
                56 => {
                    self.LILIFGBAFKN = is.read_uint32()?;
                },
                64 => {
                    self.MMIIJHOHOGE = is.read_uint32()?;
                },
                72 => {
                    self.IOKFIKHHANG = is.read_uint32()?;
                },
                80 => {
                    self.CILKFJBLEJG = is.read_uint32()?;
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
        if self.LMMGODPHJNE != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.LMMGODPHJNE);
        }
        if self.PGJCCGNBBPI != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.PGJCCGNBBPI);
        }
        if self.NNBHKCJCPIO != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.NNBHKCJCPIO);
        }
        if self.KHBNJGPPHOA != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.KHBNJGPPHOA);
        }
        if self.AGDCEBLFGKH != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.AGDCEBLFGKH);
        }
        if self.DNCPBBLIOPL != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.DNCPBBLIOPL);
        }
        if self.LILIFGBAFKN != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.LILIFGBAFKN);
        }
        if self.MMIIJHOHOGE != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.MMIIJHOHOGE);
        }
        if self.IOKFIKHHANG != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.IOKFIKHHANG);
        }
        if self.CILKFJBLEJG != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.CILKFJBLEJG);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.LMMGODPHJNE != 0 {
            os.write_uint32(1, self.LMMGODPHJNE)?;
        }
        if self.PGJCCGNBBPI != 0 {
            os.write_uint32(2, self.PGJCCGNBBPI)?;
        }
        if self.NNBHKCJCPIO != 0 {
            os.write_uint32(3, self.NNBHKCJCPIO)?;
        }
        if self.KHBNJGPPHOA != 0 {
            os.write_uint32(4, self.KHBNJGPPHOA)?;
        }
        if self.AGDCEBLFGKH != 0 {
            os.write_uint32(5, self.AGDCEBLFGKH)?;
        }
        if self.DNCPBBLIOPL != 0 {
            os.write_uint32(6, self.DNCPBBLIOPL)?;
        }
        if self.LILIFGBAFKN != 0 {
            os.write_uint32(7, self.LILIFGBAFKN)?;
        }
        if self.MMIIJHOHOGE != 0 {
            os.write_uint32(8, self.MMIIJHOHOGE)?;
        }
        if self.IOKFIKHHANG != 0 {
            os.write_uint32(9, self.IOKFIKHHANG)?;
        }
        if self.CILKFJBLEJG != 0 {
            os.write_uint32(10, self.CILKFJBLEJG)?;
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

    fn new() -> CDIMEMFJJFP {
        CDIMEMFJJFP::new()
    }

    fn clear(&mut self) {
        self.LMMGODPHJNE = 0;
        self.PGJCCGNBBPI = 0;
        self.NNBHKCJCPIO = 0;
        self.KHBNJGPPHOA = 0;
        self.AGDCEBLFGKH = 0;
        self.DNCPBBLIOPL = 0;
        self.LILIFGBAFKN = 0;
        self.MMIIJHOHOGE = 0;
        self.IOKFIKHHANG = 0;
        self.CILKFJBLEJG = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CDIMEMFJJFP {
        static instance: CDIMEMFJJFP = CDIMEMFJJFP {
            LMMGODPHJNE: 0,
            PGJCCGNBBPI: 0,
            NNBHKCJCPIO: 0,
            KHBNJGPPHOA: 0,
            AGDCEBLFGKH: 0,
            DNCPBBLIOPL: 0,
            LILIFGBAFKN: 0,
            MMIIJHOHOGE: 0,
            IOKFIKHHANG: 0,
            CILKFJBLEJG: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CDIMEMFJJFP {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CDIMEMFJJFP").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CDIMEMFJJFP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDIMEMFJJFP {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CDIMEMFJJFP.proto\"\xe1\x02\n\x0bCDIMEMFJJFP\x12\x20\n\x0bLMMGODPH\
    JNE\x18\x01\x20\x01(\rR\x0bLMMGODPHJNE\x12\x20\n\x0bPGJCCGNBBPI\x18\x02\
    \x20\x01(\rR\x0bPGJCCGNBBPI\x12\x20\n\x0bNNBHKCJCPIO\x18\x03\x20\x01(\rR\
    \x0bNNBHKCJCPIO\x12\x20\n\x0bKHBNJGPPHOA\x18\x04\x20\x01(\rR\x0bKHBNJGPP\
    HOA\x12\x20\n\x0bAGDCEBLFGKH\x18\x05\x20\x01(\rR\x0bAGDCEBLFGKH\x12\x20\
    \n\x0bDNCPBBLIOPL\x18\x06\x20\x01(\rR\x0bDNCPBBLIOPL\x12\x20\n\x0bLILIFG\
    BAFKN\x18\x07\x20\x01(\rR\x0bLILIFGBAFKN\x12\x20\n\x0bMMIIJHOHOGE\x18\
    \x08\x20\x01(\rR\x0bMMIIJHOHOGE\x12\x20\n\x0bIOKFIKHHANG\x18\t\x20\x01(\
    \rR\x0bIOKFIKHHANG\x12\x20\n\x0bCILKFJBLEJG\x18\n\x20\x01(\rR\x0bCILKFJB\
    LEJGb\x06proto3\
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
            messages.push(CDIMEMFJJFP::generated_message_descriptor_data());
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
