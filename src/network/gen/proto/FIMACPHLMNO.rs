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

//! Generated file from `FIMACPHLMNO.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:FIMACPHLMNO)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FIMACPHLMNO {
    // message oneof groups
    pub KFELKJLDKEH: ::std::option::Option<fimacphlmno::KFELKJLDKEH>,
    // special fields
    // @@protoc_insertion_point(special_field:FIMACPHLMNO.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FIMACPHLMNO {
    fn default() -> &'a FIMACPHLMNO {
        <FIMACPHLMNO as ::protobuf::Message>::default_instance()
    }
}

impl FIMACPHLMNO {
    pub fn new() -> FIMACPHLMNO {
        ::std::default::Default::default()
    }

    // int64 KBEFCMIIIIN = 13;

    pub fn KBEFCMIIIIN(&self) -> i64 {
        match self.KFELKJLDKEH {
            ::std::option::Option::Some(fimacphlmno::KFELKJLDKEH::KBEFCMIIIIN(v)) => v,
            _ => 0,
        }
    }

    pub fn clear_KBEFCMIIIIN(&mut self) {
        self.KFELKJLDKEH = ::std::option::Option::None;
    }

    pub fn has_KBEFCMIIIIN(&self) -> bool {
        match self.KFELKJLDKEH {
            ::std::option::Option::Some(fimacphlmno::KFELKJLDKEH::KBEFCMIIIIN(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_KBEFCMIIIIN(&mut self, v: i64) {
        self.KFELKJLDKEH = ::std::option::Option::Some(fimacphlmno::KFELKJLDKEH::KBEFCMIIIIN(v))
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "KBEFCMIIIIN",
            FIMACPHLMNO::has_KBEFCMIIIIN,
            FIMACPHLMNO::KBEFCMIIIIN,
            FIMACPHLMNO::set_KBEFCMIIIIN,
        ));
        oneofs.push(fimacphlmno::KFELKJLDKEH::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FIMACPHLMNO>(
            "FIMACPHLMNO",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FIMACPHLMNO {
    const NAME: &'static str = "FIMACPHLMNO";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.KFELKJLDKEH = ::std::option::Option::Some(fimacphlmno::KFELKJLDKEH::KBEFCMIIIIN(is.read_int64()?));
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
                &fimacphlmno::KFELKJLDKEH::KBEFCMIIIIN(v) => {
                    my_size += ::protobuf::rt::int64_size(13, v);
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
                &fimacphlmno::KFELKJLDKEH::KBEFCMIIIIN(v) => {
                    os.write_int64(13, v)?;
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

    fn new() -> FIMACPHLMNO {
        FIMACPHLMNO::new()
    }

    fn clear(&mut self) {
        self.KFELKJLDKEH = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FIMACPHLMNO {
        static instance: FIMACPHLMNO = FIMACPHLMNO {
            KFELKJLDKEH: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FIMACPHLMNO {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FIMACPHLMNO").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FIMACPHLMNO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FIMACPHLMNO {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `FIMACPHLMNO`
pub mod fimacphlmno {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:FIMACPHLMNO.KFELKJLDKEH)
    pub enum KFELKJLDKEH {
        // @@protoc_insertion_point(oneof_field:FIMACPHLMNO.KBEFCMIIIIN)
        KBEFCMIIIIN(i64),
    }

    impl ::protobuf::Oneof for KFELKJLDKEH {
    }

    impl ::protobuf::OneofFull for KFELKJLDKEH {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::FIMACPHLMNO as ::protobuf::MessageFull>::descriptor().oneof_by_name("KFELKJLDKEH").unwrap()).clone()
        }
    }

    impl KFELKJLDKEH {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<KFELKJLDKEH>("KFELKJLDKEH")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11FIMACPHLMNO.proto\"@\n\x0bFIMACPHLMNO\x12\"\n\x0bKBEFCMIIIIN\x18\r\
    \x20\x01(\x03H\0R\x0bKBEFCMIIIINB\r\n\x0bKFELKJLDKEHb\x06proto3\
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
            messages.push(FIMACPHLMNO::generated_message_descriptor_data());
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
