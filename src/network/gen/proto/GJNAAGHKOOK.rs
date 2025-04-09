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

//! Generated file from `GJNAAGHKOOK.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:GJNAAGHKOOK)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GJNAAGHKOOK {
    // message fields
    // @@protoc_insertion_point(field:GJNAAGHKOOK.uid)
    pub uid: u32,
    // @@protoc_insertion_point(field:GJNAAGHKOOK.FBJPBDIJPFK)
    pub FBJPBDIJPFK: u32,
    // @@protoc_insertion_point(field:GJNAAGHKOOK.MNCIHJHGNMJ)
    pub MNCIHJHGNMJ: u32,
    // @@protoc_insertion_point(field:GJNAAGHKOOK.IFJFCEJJBPE)
    pub IFJFCEJJBPE: u32,
    // @@protoc_insertion_point(field:GJNAAGHKOOK.LAHPGLCDIJK)
    pub LAHPGLCDIJK: i64,
    // @@protoc_insertion_point(field:GJNAAGHKOOK.IACOCNEGAJO)
    pub IACOCNEGAJO: u32,
    // @@protoc_insertion_point(field:GJNAAGHKOOK.PJDCNAHGAHC)
    pub PJDCNAHGAHC: u32,
    // @@protoc_insertion_point(field:GJNAAGHKOOK.FELGGJMHONO)
    pub FELGGJMHONO: ::protobuf::EnumOrUnknown<super::PunkLordShareType::PunkLordShareType>,
    // @@protoc_insertion_point(field:GJNAAGHKOOK.PPBOCECKCAH)
    pub PPBOCECKCAH: bool,
    // special fields
    // @@protoc_insertion_point(special_field:GJNAAGHKOOK.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GJNAAGHKOOK {
    fn default() -> &'a GJNAAGHKOOK {
        <GJNAAGHKOOK as ::protobuf::Message>::default_instance()
    }
}

impl GJNAAGHKOOK {
    pub fn new() -> GJNAAGHKOOK {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "uid",
            |m: &GJNAAGHKOOK| { &m.uid },
            |m: &mut GJNAAGHKOOK| { &mut m.uid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FBJPBDIJPFK",
            |m: &GJNAAGHKOOK| { &m.FBJPBDIJPFK },
            |m: &mut GJNAAGHKOOK| { &mut m.FBJPBDIJPFK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MNCIHJHGNMJ",
            |m: &GJNAAGHKOOK| { &m.MNCIHJHGNMJ },
            |m: &mut GJNAAGHKOOK| { &mut m.MNCIHJHGNMJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IFJFCEJJBPE",
            |m: &GJNAAGHKOOK| { &m.IFJFCEJJBPE },
            |m: &mut GJNAAGHKOOK| { &mut m.IFJFCEJJBPE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LAHPGLCDIJK",
            |m: &GJNAAGHKOOK| { &m.LAHPGLCDIJK },
            |m: &mut GJNAAGHKOOK| { &mut m.LAHPGLCDIJK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IACOCNEGAJO",
            |m: &GJNAAGHKOOK| { &m.IACOCNEGAJO },
            |m: &mut GJNAAGHKOOK| { &mut m.IACOCNEGAJO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PJDCNAHGAHC",
            |m: &GJNAAGHKOOK| { &m.PJDCNAHGAHC },
            |m: &mut GJNAAGHKOOK| { &mut m.PJDCNAHGAHC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FELGGJMHONO",
            |m: &GJNAAGHKOOK| { &m.FELGGJMHONO },
            |m: &mut GJNAAGHKOOK| { &mut m.FELGGJMHONO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PPBOCECKCAH",
            |m: &GJNAAGHKOOK| { &m.PPBOCECKCAH },
            |m: &mut GJNAAGHKOOK| { &mut m.PPBOCECKCAH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GJNAAGHKOOK>(
            "GJNAAGHKOOK",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GJNAAGHKOOK {
    const NAME: &'static str = "GJNAAGHKOOK";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.uid = is.read_uint32()?;
                },
                16 => {
                    self.FBJPBDIJPFK = is.read_uint32()?;
                },
                24 => {
                    self.MNCIHJHGNMJ = is.read_uint32()?;
                },
                32 => {
                    self.IFJFCEJJBPE = is.read_uint32()?;
                },
                40 => {
                    self.LAHPGLCDIJK = is.read_int64()?;
                },
                48 => {
                    self.IACOCNEGAJO = is.read_uint32()?;
                },
                56 => {
                    self.PJDCNAHGAHC = is.read_uint32()?;
                },
                64 => {
                    self.FELGGJMHONO = is.read_enum_or_unknown()?;
                },
                72 => {
                    self.PPBOCECKCAH = is.read_bool()?;
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
        if self.uid != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.uid);
        }
        if self.FBJPBDIJPFK != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.FBJPBDIJPFK);
        }
        if self.MNCIHJHGNMJ != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.MNCIHJHGNMJ);
        }
        if self.IFJFCEJJBPE != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.IFJFCEJJBPE);
        }
        if self.LAHPGLCDIJK != 0 {
            my_size += ::protobuf::rt::int64_size(5, self.LAHPGLCDIJK);
        }
        if self.IACOCNEGAJO != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.IACOCNEGAJO);
        }
        if self.PJDCNAHGAHC != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.PJDCNAHGAHC);
        }
        if self.FELGGJMHONO != ::protobuf::EnumOrUnknown::new(super::PunkLordShareType::PunkLordShareType::PUNK_LORD_SHARE_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(8, self.FELGGJMHONO.value());
        }
        if self.PPBOCECKCAH != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.uid != 0 {
            os.write_uint32(1, self.uid)?;
        }
        if self.FBJPBDIJPFK != 0 {
            os.write_uint32(2, self.FBJPBDIJPFK)?;
        }
        if self.MNCIHJHGNMJ != 0 {
            os.write_uint32(3, self.MNCIHJHGNMJ)?;
        }
        if self.IFJFCEJJBPE != 0 {
            os.write_uint32(4, self.IFJFCEJJBPE)?;
        }
        if self.LAHPGLCDIJK != 0 {
            os.write_int64(5, self.LAHPGLCDIJK)?;
        }
        if self.IACOCNEGAJO != 0 {
            os.write_uint32(6, self.IACOCNEGAJO)?;
        }
        if self.PJDCNAHGAHC != 0 {
            os.write_uint32(7, self.PJDCNAHGAHC)?;
        }
        if self.FELGGJMHONO != ::protobuf::EnumOrUnknown::new(super::PunkLordShareType::PunkLordShareType::PUNK_LORD_SHARE_TYPE_NONE) {
            os.write_enum(8, ::protobuf::EnumOrUnknown::value(&self.FELGGJMHONO))?;
        }
        if self.PPBOCECKCAH != false {
            os.write_bool(9, self.PPBOCECKCAH)?;
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

    fn new() -> GJNAAGHKOOK {
        GJNAAGHKOOK::new()
    }

    fn clear(&mut self) {
        self.uid = 0;
        self.FBJPBDIJPFK = 0;
        self.MNCIHJHGNMJ = 0;
        self.IFJFCEJJBPE = 0;
        self.LAHPGLCDIJK = 0;
        self.IACOCNEGAJO = 0;
        self.PJDCNAHGAHC = 0;
        self.FELGGJMHONO = ::protobuf::EnumOrUnknown::new(super::PunkLordShareType::PunkLordShareType::PUNK_LORD_SHARE_TYPE_NONE);
        self.PPBOCECKCAH = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GJNAAGHKOOK {
        static instance: GJNAAGHKOOK = GJNAAGHKOOK {
            uid: 0,
            FBJPBDIJPFK: 0,
            MNCIHJHGNMJ: 0,
            IFJFCEJJBPE: 0,
            LAHPGLCDIJK: 0,
            IACOCNEGAJO: 0,
            PJDCNAHGAHC: 0,
            FELGGJMHONO: ::protobuf::EnumOrUnknown::from_i32(0),
            PPBOCECKCAH: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GJNAAGHKOOK {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GJNAAGHKOOK").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GJNAAGHKOOK {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GJNAAGHKOOK {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11GJNAAGHKOOK.proto\x1a\x17PunkLordShareType.proto\"\xc3\x02\n\x0bGJ\
    NAAGHKOOK\x12\x10\n\x03uid\x18\x01\x20\x01(\rR\x03uid\x12\x20\n\x0bFBJPB\
    DIJPFK\x18\x02\x20\x01(\rR\x0bFBJPBDIJPFK\x12\x20\n\x0bMNCIHJHGNMJ\x18\
    \x03\x20\x01(\rR\x0bMNCIHJHGNMJ\x12\x20\n\x0bIFJFCEJJBPE\x18\x04\x20\x01\
    (\rR\x0bIFJFCEJJBPE\x12\x20\n\x0bLAHPGLCDIJK\x18\x05\x20\x01(\x03R\x0bLA\
    HPGLCDIJK\x12\x20\n\x0bIACOCNEGAJO\x18\x06\x20\x01(\rR\x0bIACOCNEGAJO\
    \x12\x20\n\x0bPJDCNAHGAHC\x18\x07\x20\x01(\rR\x0bPJDCNAHGAHC\x124\n\x0bF\
    ELGGJMHONO\x18\x08\x20\x01(\x0e2\x12.PunkLordShareTypeR\x0bFELGGJMHONO\
    \x12\x20\n\x0bPPBOCECKCAH\x18\t\x20\x01(\x08R\x0bPPBOCECKCAHb\x06proto3\
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
            deps.push(super::PunkLordShareType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GJNAAGHKOOK::generated_message_descriptor_data());
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
