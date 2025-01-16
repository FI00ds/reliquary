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

//! Generated file from `EPEFCDLMNLO.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EPEFCDLMNLO)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EPEFCDLMNLO {
    // message fields
    // @@protoc_insertion_point(field:EPEFCDLMNLO.KHJONOJKHOI)
    pub KHJONOJKHOI: bool,
    // @@protoc_insertion_point(field:EPEFCDLMNLO.PPAEDKGDKDA)
    pub PPAEDKGDKDA: u32,
    // @@protoc_insertion_point(field:EPEFCDLMNLO.HCMKAKBEHDN)
    pub HCMKAKBEHDN: u32,
    // @@protoc_insertion_point(field:EPEFCDLMNLO.CFNJJEJIGOK)
    pub CFNJJEJIGOK: u32,
    // @@protoc_insertion_point(field:EPEFCDLMNLO.OLMMBFOFGEF)
    pub OLMMBFOFGEF: bool,
    // @@protoc_insertion_point(field:EPEFCDLMNLO.CIEGHGBOIEO)
    pub CIEGHGBOIEO: ::protobuf::MessageField<super::EJPNIIHEKDD::EJPNIIHEKDD>,
    // special fields
    // @@protoc_insertion_point(special_field:EPEFCDLMNLO.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EPEFCDLMNLO {
    fn default() -> &'a EPEFCDLMNLO {
        <EPEFCDLMNLO as ::protobuf::Message>::default_instance()
    }
}

impl EPEFCDLMNLO {
    pub fn new() -> EPEFCDLMNLO {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KHJONOJKHOI",
            |m: &EPEFCDLMNLO| { &m.KHJONOJKHOI },
            |m: &mut EPEFCDLMNLO| { &mut m.KHJONOJKHOI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PPAEDKGDKDA",
            |m: &EPEFCDLMNLO| { &m.PPAEDKGDKDA },
            |m: &mut EPEFCDLMNLO| { &mut m.PPAEDKGDKDA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HCMKAKBEHDN",
            |m: &EPEFCDLMNLO| { &m.HCMKAKBEHDN },
            |m: &mut EPEFCDLMNLO| { &mut m.HCMKAKBEHDN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CFNJJEJIGOK",
            |m: &EPEFCDLMNLO| { &m.CFNJJEJIGOK },
            |m: &mut EPEFCDLMNLO| { &mut m.CFNJJEJIGOK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OLMMBFOFGEF",
            |m: &EPEFCDLMNLO| { &m.OLMMBFOFGEF },
            |m: &mut EPEFCDLMNLO| { &mut m.OLMMBFOFGEF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::EJPNIIHEKDD::EJPNIIHEKDD>(
            "CIEGHGBOIEO",
            |m: &EPEFCDLMNLO| { &m.CIEGHGBOIEO },
            |m: &mut EPEFCDLMNLO| { &mut m.CIEGHGBOIEO },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EPEFCDLMNLO>(
            "EPEFCDLMNLO",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EPEFCDLMNLO {
    const NAME: &'static str = "EPEFCDLMNLO";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.KHJONOJKHOI = is.read_bool()?;
                },
                56 => {
                    self.PPAEDKGDKDA = is.read_uint32()?;
                },
                64 => {
                    self.HCMKAKBEHDN = is.read_uint32()?;
                },
                80 => {
                    self.CFNJJEJIGOK = is.read_uint32()?;
                },
                88 => {
                    self.OLMMBFOFGEF = is.read_bool()?;
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.CIEGHGBOIEO)?;
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
        if self.KHJONOJKHOI != false {
            my_size += 1 + 1;
        }
        if self.PPAEDKGDKDA != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.PPAEDKGDKDA);
        }
        if self.HCMKAKBEHDN != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.HCMKAKBEHDN);
        }
        if self.CFNJJEJIGOK != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.CFNJJEJIGOK);
        }
        if self.OLMMBFOFGEF != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.CIEGHGBOIEO.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.KHJONOJKHOI != false {
            os.write_bool(1, self.KHJONOJKHOI)?;
        }
        if self.PPAEDKGDKDA != 0 {
            os.write_uint32(7, self.PPAEDKGDKDA)?;
        }
        if self.HCMKAKBEHDN != 0 {
            os.write_uint32(8, self.HCMKAKBEHDN)?;
        }
        if self.CFNJJEJIGOK != 0 {
            os.write_uint32(10, self.CFNJJEJIGOK)?;
        }
        if self.OLMMBFOFGEF != false {
            os.write_bool(11, self.OLMMBFOFGEF)?;
        }
        if let Some(v) = self.CIEGHGBOIEO.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
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

    fn new() -> EPEFCDLMNLO {
        EPEFCDLMNLO::new()
    }

    fn clear(&mut self) {
        self.KHJONOJKHOI = false;
        self.PPAEDKGDKDA = 0;
        self.HCMKAKBEHDN = 0;
        self.CFNJJEJIGOK = 0;
        self.OLMMBFOFGEF = false;
        self.CIEGHGBOIEO.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EPEFCDLMNLO {
        static instance: EPEFCDLMNLO = EPEFCDLMNLO {
            KHJONOJKHOI: false,
            PPAEDKGDKDA: 0,
            HCMKAKBEHDN: 0,
            CFNJJEJIGOK: 0,
            OLMMBFOFGEF: false,
            CIEGHGBOIEO: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EPEFCDLMNLO {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EPEFCDLMNLO").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EPEFCDLMNLO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EPEFCDLMNLO {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11EPEFCDLMNLO.proto\x1a\x11EJPNIIHEKDD.proto\"\xe7\x01\n\x0bEPEFCDLM\
    NLO\x12\x20\n\x0bKHJONOJKHOI\x18\x01\x20\x01(\x08R\x0bKHJONOJKHOI\x12\
    \x20\n\x0bPPAEDKGDKDA\x18\x07\x20\x01(\rR\x0bPPAEDKGDKDA\x12\x20\n\x0bHC\
    MKAKBEHDN\x18\x08\x20\x01(\rR\x0bHCMKAKBEHDN\x12\x20\n\x0bCFNJJEJIGOK\
    \x18\n\x20\x01(\rR\x0bCFNJJEJIGOK\x12\x20\n\x0bOLMMBFOFGEF\x18\x0b\x20\
    \x01(\x08R\x0bOLMMBFOFGEF\x12.\n\x0bCIEGHGBOIEO\x18\r\x20\x01(\x0b2\x0c.\
    EJPNIIHEKDDR\x0bCIEGHGBOIEOb\x06proto3\
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
            deps.push(super::EJPNIIHEKDD::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EPEFCDLMNLO::generated_message_descriptor_data());
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
