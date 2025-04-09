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

//! Generated file from `FHEPDKKHFCF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:FHEPDKKHFCF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FHEPDKKHFCF {
    // message oneof groups
    pub KFELKJLDKEH: ::std::option::Option<fhepdkkhfcf::KFELKJLDKEH>,
    // special fields
    // @@protoc_insertion_point(special_field:FHEPDKKHFCF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FHEPDKKHFCF {
    fn default() -> &'a FHEPDKKHFCF {
        <FHEPDKKHFCF as ::protobuf::Message>::default_instance()
    }
}

impl FHEPDKKHFCF {
    pub fn new() -> FHEPDKKHFCF {
        ::std::default::Default::default()
    }

    // .CKJCFDJKDFG LACHNCFKAKO = 9;

    pub fn LACHNCFKAKO(&self) -> &super::CKJCFDJKDFG::CKJCFDJKDFG {
        match self.KFELKJLDKEH {
            ::std::option::Option::Some(fhepdkkhfcf::KFELKJLDKEH::LACHNCFKAKO(ref v)) => v,
            _ => <super::CKJCFDJKDFG::CKJCFDJKDFG as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_LACHNCFKAKO(&mut self) {
        self.KFELKJLDKEH = ::std::option::Option::None;
    }

    pub fn has_LACHNCFKAKO(&self) -> bool {
        match self.KFELKJLDKEH {
            ::std::option::Option::Some(fhepdkkhfcf::KFELKJLDKEH::LACHNCFKAKO(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_LACHNCFKAKO(&mut self, v: super::CKJCFDJKDFG::CKJCFDJKDFG) {
        self.KFELKJLDKEH = ::std::option::Option::Some(fhepdkkhfcf::KFELKJLDKEH::LACHNCFKAKO(v))
    }

    // Mutable pointer to the field.
    pub fn mut_LACHNCFKAKO(&mut self) -> &mut super::CKJCFDJKDFG::CKJCFDJKDFG {
        if let ::std::option::Option::Some(fhepdkkhfcf::KFELKJLDKEH::LACHNCFKAKO(_)) = self.KFELKJLDKEH {
        } else {
            self.KFELKJLDKEH = ::std::option::Option::Some(fhepdkkhfcf::KFELKJLDKEH::LACHNCFKAKO(super::CKJCFDJKDFG::CKJCFDJKDFG::new()));
        }
        match self.KFELKJLDKEH {
            ::std::option::Option::Some(fhepdkkhfcf::KFELKJLDKEH::LACHNCFKAKO(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_LACHNCFKAKO(&mut self) -> super::CKJCFDJKDFG::CKJCFDJKDFG {
        if self.has_LACHNCFKAKO() {
            match self.KFELKJLDKEH.take() {
                ::std::option::Option::Some(fhepdkkhfcf::KFELKJLDKEH::LACHNCFKAKO(v)) => v,
                _ => panic!(),
            }
        } else {
            super::CKJCFDJKDFG::CKJCFDJKDFG::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::CKJCFDJKDFG::CKJCFDJKDFG>(
            "LACHNCFKAKO",
            FHEPDKKHFCF::has_LACHNCFKAKO,
            FHEPDKKHFCF::LACHNCFKAKO,
            FHEPDKKHFCF::mut_LACHNCFKAKO,
            FHEPDKKHFCF::set_LACHNCFKAKO,
        ));
        oneofs.push(fhepdkkhfcf::KFELKJLDKEH::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FHEPDKKHFCF>(
            "FHEPDKKHFCF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FHEPDKKHFCF {
    const NAME: &'static str = "FHEPDKKHFCF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                74 => {
                    self.KFELKJLDKEH = ::std::option::Option::Some(fhepdkkhfcf::KFELKJLDKEH::LACHNCFKAKO(is.read_message()?));
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
                &fhepdkkhfcf::KFELKJLDKEH::LACHNCFKAKO(ref v) => {
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
                &fhepdkkhfcf::KFELKJLDKEH::LACHNCFKAKO(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
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

    fn new() -> FHEPDKKHFCF {
        FHEPDKKHFCF::new()
    }

    fn clear(&mut self) {
        self.KFELKJLDKEH = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FHEPDKKHFCF {
        static instance: FHEPDKKHFCF = FHEPDKKHFCF {
            KFELKJLDKEH: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FHEPDKKHFCF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FHEPDKKHFCF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FHEPDKKHFCF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FHEPDKKHFCF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `FHEPDKKHFCF`
pub mod fhepdkkhfcf {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:FHEPDKKHFCF.KFELKJLDKEH)
    pub enum KFELKJLDKEH {
        // @@protoc_insertion_point(oneof_field:FHEPDKKHFCF.LACHNCFKAKO)
        LACHNCFKAKO(super::super::CKJCFDJKDFG::CKJCFDJKDFG),
    }

    impl ::protobuf::Oneof for KFELKJLDKEH {
    }

    impl ::protobuf::OneofFull for KFELKJLDKEH {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::FHEPDKKHFCF as ::protobuf::MessageFull>::descriptor().oneof_by_name("KFELKJLDKEH").unwrap()).clone()
        }
    }

    impl KFELKJLDKEH {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<KFELKJLDKEH>("KFELKJLDKEH")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11FHEPDKKHFCF.proto\x1a\x11CKJCFDJKDFG.proto\"N\n\x0bFHEPDKKHFCF\x12\
    0\n\x0bLACHNCFKAKO\x18\t\x20\x01(\x0b2\x0c.CKJCFDJKDFGH\0R\x0bLACHNCFKAK\
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
            deps.push(super::CKJCFDJKDFG::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(FHEPDKKHFCF::generated_message_descriptor_data());
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
