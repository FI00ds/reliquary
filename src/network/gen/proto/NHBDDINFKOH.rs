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

//! Generated file from `NHBDDINFKOH.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:NHBDDINFKOH)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NHBDDINFKOH {
    // message fields
    // @@protoc_insertion_point(field:NHBDDINFKOH.KBCDECDNEFM)
    pub KBCDECDNEFM: u32,
    // @@protoc_insertion_point(field:NHBDDINFKOH.IEAGBPEMFLG)
    pub IEAGBPEMFLG: i32,
    // @@protoc_insertion_point(field:NHBDDINFKOH.LLJAEGOBHMP)
    pub LLJAEGOBHMP: i32,
    // @@protoc_insertion_point(field:NHBDDINFKOH.unique_id)
    pub unique_id: u64,
    // @@protoc_insertion_point(field:NHBDDINFKOH.FILDLBJOMLD)
    pub FILDLBJOMLD: u32,
    // @@protoc_insertion_point(field:NHBDDINFKOH.IIMOPLCFHAH)
    pub IIMOPLCFHAH: u32,
    // special fields
    // @@protoc_insertion_point(special_field:NHBDDINFKOH.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NHBDDINFKOH {
    fn default() -> &'a NHBDDINFKOH {
        <NHBDDINFKOH as ::protobuf::Message>::default_instance()
    }
}

impl NHBDDINFKOH {
    pub fn new() -> NHBDDINFKOH {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KBCDECDNEFM",
            |m: &NHBDDINFKOH| { &m.KBCDECDNEFM },
            |m: &mut NHBDDINFKOH| { &mut m.KBCDECDNEFM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IEAGBPEMFLG",
            |m: &NHBDDINFKOH| { &m.IEAGBPEMFLG },
            |m: &mut NHBDDINFKOH| { &mut m.IEAGBPEMFLG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LLJAEGOBHMP",
            |m: &NHBDDINFKOH| { &m.LLJAEGOBHMP },
            |m: &mut NHBDDINFKOH| { &mut m.LLJAEGOBHMP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "unique_id",
            |m: &NHBDDINFKOH| { &m.unique_id },
            |m: &mut NHBDDINFKOH| { &mut m.unique_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FILDLBJOMLD",
            |m: &NHBDDINFKOH| { &m.FILDLBJOMLD },
            |m: &mut NHBDDINFKOH| { &mut m.FILDLBJOMLD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IIMOPLCFHAH",
            |m: &NHBDDINFKOH| { &m.IIMOPLCFHAH },
            |m: &mut NHBDDINFKOH| { &mut m.IIMOPLCFHAH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NHBDDINFKOH>(
            "NHBDDINFKOH",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NHBDDINFKOH {
    const NAME: &'static str = "NHBDDINFKOH";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.KBCDECDNEFM = is.read_uint32()?;
                },
                8 => {
                    self.IEAGBPEMFLG = is.read_int32()?;
                },
                104 => {
                    self.LLJAEGOBHMP = is.read_int32()?;
                },
                96 => {
                    self.unique_id = is.read_uint64()?;
                },
                72 => {
                    self.FILDLBJOMLD = is.read_uint32()?;
                },
                48 => {
                    self.IIMOPLCFHAH = is.read_uint32()?;
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
        if self.KBCDECDNEFM != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.KBCDECDNEFM);
        }
        if self.IEAGBPEMFLG != 0 {
            my_size += ::protobuf::rt::int32_size(1, self.IEAGBPEMFLG);
        }
        if self.LLJAEGOBHMP != 0 {
            my_size += ::protobuf::rt::int32_size(13, self.LLJAEGOBHMP);
        }
        if self.unique_id != 0 {
            my_size += ::protobuf::rt::uint64_size(12, self.unique_id);
        }
        if self.FILDLBJOMLD != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.FILDLBJOMLD);
        }
        if self.IIMOPLCFHAH != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.IIMOPLCFHAH);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.KBCDECDNEFM != 0 {
            os.write_uint32(8, self.KBCDECDNEFM)?;
        }
        if self.IEAGBPEMFLG != 0 {
            os.write_int32(1, self.IEAGBPEMFLG)?;
        }
        if self.LLJAEGOBHMP != 0 {
            os.write_int32(13, self.LLJAEGOBHMP)?;
        }
        if self.unique_id != 0 {
            os.write_uint64(12, self.unique_id)?;
        }
        if self.FILDLBJOMLD != 0 {
            os.write_uint32(9, self.FILDLBJOMLD)?;
        }
        if self.IIMOPLCFHAH != 0 {
            os.write_uint32(6, self.IIMOPLCFHAH)?;
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

    fn new() -> NHBDDINFKOH {
        NHBDDINFKOH::new()
    }

    fn clear(&mut self) {
        self.KBCDECDNEFM = 0;
        self.IEAGBPEMFLG = 0;
        self.LLJAEGOBHMP = 0;
        self.unique_id = 0;
        self.FILDLBJOMLD = 0;
        self.IIMOPLCFHAH = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NHBDDINFKOH {
        static instance: NHBDDINFKOH = NHBDDINFKOH {
            KBCDECDNEFM: 0,
            IEAGBPEMFLG: 0,
            LLJAEGOBHMP: 0,
            unique_id: 0,
            FILDLBJOMLD: 0,
            IIMOPLCFHAH: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for NHBDDINFKOH {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NHBDDINFKOH").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NHBDDINFKOH {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NHBDDINFKOH {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11NHBDDINFKOH.proto\"\xd4\x01\n\x0bNHBDDINFKOH\x12\x20\n\x0bKBCDECDN\
    EFM\x18\x08\x20\x01(\rR\x0bKBCDECDNEFM\x12\x20\n\x0bIEAGBPEMFLG\x18\x01\
    \x20\x01(\x05R\x0bIEAGBPEMFLG\x12\x20\n\x0bLLJAEGOBHMP\x18\r\x20\x01(\
    \x05R\x0bLLJAEGOBHMP\x12\x1b\n\tunique_id\x18\x0c\x20\x01(\x04R\x08uniqu\
    eId\x12\x20\n\x0bFILDLBJOMLD\x18\t\x20\x01(\rR\x0bFILDLBJOMLD\x12\x20\n\
    \x0bIIMOPLCFHAH\x18\x06\x20\x01(\rR\x0bIIMOPLCFHAHb\x06proto3\
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
            messages.push(NHBDDINFKOH::generated_message_descriptor_data());
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
