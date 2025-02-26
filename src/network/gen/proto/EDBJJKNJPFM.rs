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

//! Generated file from `EDBJJKNJPFM.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:EDBJJKNJPFM)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EDBJJKNJPFM {
    // message oneof groups
    pub KFELKJLDKEH: ::std::option::Option<edbjjknjpfm::KFELKJLDKEH>,
    // special fields
    // @@protoc_insertion_point(special_field:EDBJJKNJPFM.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EDBJJKNJPFM {
    fn default() -> &'a EDBJJKNJPFM {
        <EDBJJKNJPFM as ::protobuf::Message>::default_instance()
    }
}

impl EDBJJKNJPFM {
    pub fn new() -> EDBJJKNJPFM {
        ::std::default::Default::default()
    }

    // .EEHIJPMFIIN LODDNCNPANO = 10;

    pub fn LODDNCNPANO(&self) -> &super::EEHIJPMFIIN::EEHIJPMFIIN {
        match self.KFELKJLDKEH {
            ::std::option::Option::Some(edbjjknjpfm::KFELKJLDKEH::LODDNCNPANO(ref v)) => v,
            _ => <super::EEHIJPMFIIN::EEHIJPMFIIN as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_LODDNCNPANO(&mut self) {
        self.KFELKJLDKEH = ::std::option::Option::None;
    }

    pub fn has_LODDNCNPANO(&self) -> bool {
        match self.KFELKJLDKEH {
            ::std::option::Option::Some(edbjjknjpfm::KFELKJLDKEH::LODDNCNPANO(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_LODDNCNPANO(&mut self, v: super::EEHIJPMFIIN::EEHIJPMFIIN) {
        self.KFELKJLDKEH = ::std::option::Option::Some(edbjjknjpfm::KFELKJLDKEH::LODDNCNPANO(v))
    }

    // Mutable pointer to the field.
    pub fn mut_LODDNCNPANO(&mut self) -> &mut super::EEHIJPMFIIN::EEHIJPMFIIN {
        if let ::std::option::Option::Some(edbjjknjpfm::KFELKJLDKEH::LODDNCNPANO(_)) = self.KFELKJLDKEH {
        } else {
            self.KFELKJLDKEH = ::std::option::Option::Some(edbjjknjpfm::KFELKJLDKEH::LODDNCNPANO(super::EEHIJPMFIIN::EEHIJPMFIIN::new()));
        }
        match self.KFELKJLDKEH {
            ::std::option::Option::Some(edbjjknjpfm::KFELKJLDKEH::LODDNCNPANO(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_LODDNCNPANO(&mut self) -> super::EEHIJPMFIIN::EEHIJPMFIIN {
        if self.has_LODDNCNPANO() {
            match self.KFELKJLDKEH.take() {
                ::std::option::Option::Some(edbjjknjpfm::KFELKJLDKEH::LODDNCNPANO(v)) => v,
                _ => panic!(),
            }
        } else {
            super::EEHIJPMFIIN::EEHIJPMFIIN::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::EEHIJPMFIIN::EEHIJPMFIIN>(
            "LODDNCNPANO",
            EDBJJKNJPFM::has_LODDNCNPANO,
            EDBJJKNJPFM::LODDNCNPANO,
            EDBJJKNJPFM::mut_LODDNCNPANO,
            EDBJJKNJPFM::set_LODDNCNPANO,
        ));
        oneofs.push(edbjjknjpfm::KFELKJLDKEH::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EDBJJKNJPFM>(
            "EDBJJKNJPFM",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EDBJJKNJPFM {
    const NAME: &'static str = "EDBJJKNJPFM";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                82 => {
                    self.KFELKJLDKEH = ::std::option::Option::Some(edbjjknjpfm::KFELKJLDKEH::LODDNCNPANO(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.KFELKJLDKEH {
            match v {
                &edbjjknjpfm::KFELKJLDKEH::LODDNCNPANO(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let ::std::option::Option::Some(ref v) = self.KFELKJLDKEH {
            match v {
                &edbjjknjpfm::KFELKJLDKEH::LODDNCNPANO(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
                },
            };
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

    fn new() -> EDBJJKNJPFM {
        EDBJJKNJPFM::new()
    }

    fn clear(&mut self) {
        self.KFELKJLDKEH = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EDBJJKNJPFM {
        static instance: EDBJJKNJPFM = EDBJJKNJPFM {
            KFELKJLDKEH: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EDBJJKNJPFM {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EDBJJKNJPFM").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EDBJJKNJPFM {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EDBJJKNJPFM {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `EDBJJKNJPFM`
pub mod edbjjknjpfm {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:EDBJJKNJPFM.KFELKJLDKEH)
    pub enum KFELKJLDKEH {
        // @@protoc_insertion_point(oneof_field:EDBJJKNJPFM.LODDNCNPANO)
        LODDNCNPANO(super::super::EEHIJPMFIIN::EEHIJPMFIIN),
    }

    impl ::protobuf::Oneof for KFELKJLDKEH {
    }

    impl ::protobuf::OneofFull for KFELKJLDKEH {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::EDBJJKNJPFM as ::protobuf::MessageFull>::descriptor().oneof_by_name("KFELKJLDKEH").unwrap()).clone()
        }
    }

    impl KFELKJLDKEH {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<KFELKJLDKEH>("KFELKJLDKEH")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11EDBJJKNJPFM.proto\x1a\x11EEHIJPMFIIN.proto\"N\n\x0bEDBJJKNJPFM\x12\
    0\n\x0bLODDNCNPANO\x18\n\x20\x01(\x0b2\x0c.EEHIJPMFIINH\0R\x0bLODDNCNPAN\
    OB\r\n\x0bKFELKJLDKEHb\x06proto3\
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
            deps.push(super::EEHIJPMFIIN::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EDBJJKNJPFM::generated_message_descriptor_data());
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
