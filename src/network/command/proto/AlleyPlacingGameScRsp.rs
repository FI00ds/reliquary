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

//! Generated file from `AlleyPlacingGameScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:AlleyPlacingGameScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AlleyPlacingGameScRsp {
    // message fields
    // @@protoc_insertion_point(field:AlleyPlacingGameScRsp.KHIBBGPHDMB)
    pub KHIBBGPHDMB: u32,
    // @@protoc_insertion_point(field:AlleyPlacingGameScRsp.KFBOKLHDJDA)
    pub KFBOKLHDJDA: u32,
    // @@protoc_insertion_point(field:AlleyPlacingGameScRsp.ELBBANDDJCI)
    pub ELBBANDDJCI: u32,
    // @@protoc_insertion_point(field:AlleyPlacingGameScRsp.KOFICKLLJNI)
    pub KOFICKLLJNI: u32,
    // @@protoc_insertion_point(field:AlleyPlacingGameScRsp.PGLGBLKKIDA)
    pub PGLGBLKKIDA: u32,
    // @@protoc_insertion_point(field:AlleyPlacingGameScRsp.event_id)
    pub event_id: u32,
    // @@protoc_insertion_point(field:AlleyPlacingGameScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:AlleyPlacingGameScRsp.ILEGFKGCMOM)
    pub ILEGFKGCMOM: u32,
    // special fields
    // @@protoc_insertion_point(special_field:AlleyPlacingGameScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AlleyPlacingGameScRsp {
    fn default() -> &'a AlleyPlacingGameScRsp {
        <AlleyPlacingGameScRsp as ::protobuf::Message>::default_instance()
    }
}

impl AlleyPlacingGameScRsp {
    pub fn new() -> AlleyPlacingGameScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KHIBBGPHDMB",
            |m: &AlleyPlacingGameScRsp| { &m.KHIBBGPHDMB },
            |m: &mut AlleyPlacingGameScRsp| { &mut m.KHIBBGPHDMB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KFBOKLHDJDA",
            |m: &AlleyPlacingGameScRsp| { &m.KFBOKLHDJDA },
            |m: &mut AlleyPlacingGameScRsp| { &mut m.KFBOKLHDJDA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ELBBANDDJCI",
            |m: &AlleyPlacingGameScRsp| { &m.ELBBANDDJCI },
            |m: &mut AlleyPlacingGameScRsp| { &mut m.ELBBANDDJCI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KOFICKLLJNI",
            |m: &AlleyPlacingGameScRsp| { &m.KOFICKLLJNI },
            |m: &mut AlleyPlacingGameScRsp| { &mut m.KOFICKLLJNI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PGLGBLKKIDA",
            |m: &AlleyPlacingGameScRsp| { &m.PGLGBLKKIDA },
            |m: &mut AlleyPlacingGameScRsp| { &mut m.PGLGBLKKIDA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "event_id",
            |m: &AlleyPlacingGameScRsp| { &m.event_id },
            |m: &mut AlleyPlacingGameScRsp| { &mut m.event_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &AlleyPlacingGameScRsp| { &m.retcode },
            |m: &mut AlleyPlacingGameScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ILEGFKGCMOM",
            |m: &AlleyPlacingGameScRsp| { &m.ILEGFKGCMOM },
            |m: &mut AlleyPlacingGameScRsp| { &mut m.ILEGFKGCMOM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AlleyPlacingGameScRsp>(
            "AlleyPlacingGameScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AlleyPlacingGameScRsp {
    const NAME: &'static str = "AlleyPlacingGameScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.KHIBBGPHDMB = is.read_uint32()?;
                },
                16 => {
                    self.KFBOKLHDJDA = is.read_uint32()?;
                },
                88 => {
                    self.ELBBANDDJCI = is.read_uint32()?;
                },
                40 => {
                    self.KOFICKLLJNI = is.read_uint32()?;
                },
                112 => {
                    self.PGLGBLKKIDA = is.read_uint32()?;
                },
                80 => {
                    self.event_id = is.read_uint32()?;
                },
                96 => {
                    self.retcode = is.read_uint32()?;
                },
                104 => {
                    self.ILEGFKGCMOM = is.read_uint32()?;
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
        if self.KHIBBGPHDMB != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.KHIBBGPHDMB);
        }
        if self.KFBOKLHDJDA != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.KFBOKLHDJDA);
        }
        if self.ELBBANDDJCI != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.ELBBANDDJCI);
        }
        if self.KOFICKLLJNI != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.KOFICKLLJNI);
        }
        if self.PGLGBLKKIDA != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.PGLGBLKKIDA);
        }
        if self.event_id != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.event_id);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.retcode);
        }
        if self.ILEGFKGCMOM != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.ILEGFKGCMOM);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.KHIBBGPHDMB != 0 {
            os.write_uint32(8, self.KHIBBGPHDMB)?;
        }
        if self.KFBOKLHDJDA != 0 {
            os.write_uint32(2, self.KFBOKLHDJDA)?;
        }
        if self.ELBBANDDJCI != 0 {
            os.write_uint32(11, self.ELBBANDDJCI)?;
        }
        if self.KOFICKLLJNI != 0 {
            os.write_uint32(5, self.KOFICKLLJNI)?;
        }
        if self.PGLGBLKKIDA != 0 {
            os.write_uint32(14, self.PGLGBLKKIDA)?;
        }
        if self.event_id != 0 {
            os.write_uint32(10, self.event_id)?;
        }
        if self.retcode != 0 {
            os.write_uint32(12, self.retcode)?;
        }
        if self.ILEGFKGCMOM != 0 {
            os.write_uint32(13, self.ILEGFKGCMOM)?;
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

    fn new() -> AlleyPlacingGameScRsp {
        AlleyPlacingGameScRsp::new()
    }

    fn clear(&mut self) {
        self.KHIBBGPHDMB = 0;
        self.KFBOKLHDJDA = 0;
        self.ELBBANDDJCI = 0;
        self.KOFICKLLJNI = 0;
        self.PGLGBLKKIDA = 0;
        self.event_id = 0;
        self.retcode = 0;
        self.ILEGFKGCMOM = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AlleyPlacingGameScRsp {
        static instance: AlleyPlacingGameScRsp = AlleyPlacingGameScRsp {
            KHIBBGPHDMB: 0,
            KFBOKLHDJDA: 0,
            ELBBANDDJCI: 0,
            KOFICKLLJNI: 0,
            PGLGBLKKIDA: 0,
            event_id: 0,
            retcode: 0,
            ILEGFKGCMOM: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AlleyPlacingGameScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AlleyPlacingGameScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AlleyPlacingGameScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AlleyPlacingGameScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bAlleyPlacingGameScRsp.proto\"\x98\x02\n\x15AlleyPlacingGameScRsp\
    \x12\x20\n\x0bKHIBBGPHDMB\x18\x08\x20\x01(\rR\x0bKHIBBGPHDMB\x12\x20\n\
    \x0bKFBOKLHDJDA\x18\x02\x20\x01(\rR\x0bKFBOKLHDJDA\x12\x20\n\x0bELBBANDD\
    JCI\x18\x0b\x20\x01(\rR\x0bELBBANDDJCI\x12\x20\n\x0bKOFICKLLJNI\x18\x05\
    \x20\x01(\rR\x0bKOFICKLLJNI\x12\x20\n\x0bPGLGBLKKIDA\x18\x0e\x20\x01(\rR\
    \x0bPGLGBLKKIDA\x12\x19\n\x08event_id\x18\n\x20\x01(\rR\x07eventId\x12\
    \x18\n\x07retcode\x18\x0c\x20\x01(\rR\x07retcode\x12\x20\n\x0bILEGFKGCMO\
    M\x18\r\x20\x01(\rR\x0bILEGFKGCMOMb\x06proto3\
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
            messages.push(AlleyPlacingGameScRsp::generated_message_descriptor_data());
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
