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

//! Generated file from `CJAKIBDIMMJ.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:CJAKIBDIMMJ)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CJAKIBDIMMJ {
    // message oneof groups
    pub PMFBBCEBACD: ::std::option::Option<cjakibdimmj::PMFBBCEBACD>,
    // special fields
    // @@protoc_insertion_point(special_field:CJAKIBDIMMJ.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CJAKIBDIMMJ {
    fn default() -> &'a CJAKIBDIMMJ {
        <CJAKIBDIMMJ as ::protobuf::Message>::default_instance()
    }
}

impl CJAKIBDIMMJ {
    pub fn new() -> CJAKIBDIMMJ {
        ::std::default::Default::default()
    }

    // bool IAHOPFNPFLN = 392;

    pub fn IAHOPFNPFLN(&self) -> bool {
        match self.PMFBBCEBACD {
            ::std::option::Option::Some(cjakibdimmj::PMFBBCEBACD::IAHOPFNPFLN(v)) => v,
            _ => false,
        }
    }

    pub fn clear_IAHOPFNPFLN(&mut self) {
        self.PMFBBCEBACD = ::std::option::Option::None;
    }

    pub fn has_IAHOPFNPFLN(&self) -> bool {
        match self.PMFBBCEBACD {
            ::std::option::Option::Some(cjakibdimmj::PMFBBCEBACD::IAHOPFNPFLN(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_IAHOPFNPFLN(&mut self, v: bool) {
        self.PMFBBCEBACD = ::std::option::Option::Some(cjakibdimmj::PMFBBCEBACD::IAHOPFNPFLN(v))
    }

    // .MENPBGGOGMC AMALJFOKCBA = 1558;

    pub fn AMALJFOKCBA(&self) -> &super::MENPBGGOGMC::MENPBGGOGMC {
        match self.PMFBBCEBACD {
            ::std::option::Option::Some(cjakibdimmj::PMFBBCEBACD::AMALJFOKCBA(ref v)) => v,
            _ => <super::MENPBGGOGMC::MENPBGGOGMC as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_AMALJFOKCBA(&mut self) {
        self.PMFBBCEBACD = ::std::option::Option::None;
    }

    pub fn has_AMALJFOKCBA(&self) -> bool {
        match self.PMFBBCEBACD {
            ::std::option::Option::Some(cjakibdimmj::PMFBBCEBACD::AMALJFOKCBA(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_AMALJFOKCBA(&mut self, v: super::MENPBGGOGMC::MENPBGGOGMC) {
        self.PMFBBCEBACD = ::std::option::Option::Some(cjakibdimmj::PMFBBCEBACD::AMALJFOKCBA(v))
    }

    // Mutable pointer to the field.
    pub fn mut_AMALJFOKCBA(&mut self) -> &mut super::MENPBGGOGMC::MENPBGGOGMC {
        if let ::std::option::Option::Some(cjakibdimmj::PMFBBCEBACD::AMALJFOKCBA(_)) = self.PMFBBCEBACD {
        } else {
            self.PMFBBCEBACD = ::std::option::Option::Some(cjakibdimmj::PMFBBCEBACD::AMALJFOKCBA(super::MENPBGGOGMC::MENPBGGOGMC::new()));
        }
        match self.PMFBBCEBACD {
            ::std::option::Option::Some(cjakibdimmj::PMFBBCEBACD::AMALJFOKCBA(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_AMALJFOKCBA(&mut self) -> super::MENPBGGOGMC::MENPBGGOGMC {
        if self.has_AMALJFOKCBA() {
            match self.PMFBBCEBACD.take() {
                ::std::option::Option::Some(cjakibdimmj::PMFBBCEBACD::AMALJFOKCBA(v)) => v,
                _ => panic!(),
            }
        } else {
            super::MENPBGGOGMC::MENPBGGOGMC::new()
        }
    }

    // .MENPBGGOGMC OECBELGDLFP = 393;

    pub fn OECBELGDLFP(&self) -> &super::MENPBGGOGMC::MENPBGGOGMC {
        match self.PMFBBCEBACD {
            ::std::option::Option::Some(cjakibdimmj::PMFBBCEBACD::OECBELGDLFP(ref v)) => v,
            _ => <super::MENPBGGOGMC::MENPBGGOGMC as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_OECBELGDLFP(&mut self) {
        self.PMFBBCEBACD = ::std::option::Option::None;
    }

    pub fn has_OECBELGDLFP(&self) -> bool {
        match self.PMFBBCEBACD {
            ::std::option::Option::Some(cjakibdimmj::PMFBBCEBACD::OECBELGDLFP(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_OECBELGDLFP(&mut self, v: super::MENPBGGOGMC::MENPBGGOGMC) {
        self.PMFBBCEBACD = ::std::option::Option::Some(cjakibdimmj::PMFBBCEBACD::OECBELGDLFP(v))
    }

    // Mutable pointer to the field.
    pub fn mut_OECBELGDLFP(&mut self) -> &mut super::MENPBGGOGMC::MENPBGGOGMC {
        if let ::std::option::Option::Some(cjakibdimmj::PMFBBCEBACD::OECBELGDLFP(_)) = self.PMFBBCEBACD {
        } else {
            self.PMFBBCEBACD = ::std::option::Option::Some(cjakibdimmj::PMFBBCEBACD::OECBELGDLFP(super::MENPBGGOGMC::MENPBGGOGMC::new()));
        }
        match self.PMFBBCEBACD {
            ::std::option::Option::Some(cjakibdimmj::PMFBBCEBACD::OECBELGDLFP(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_OECBELGDLFP(&mut self) -> super::MENPBGGOGMC::MENPBGGOGMC {
        if self.has_OECBELGDLFP() {
            match self.PMFBBCEBACD.take() {
                ::std::option::Option::Some(cjakibdimmj::PMFBBCEBACD::OECBELGDLFP(v)) => v,
                _ => panic!(),
            }
        } else {
            super::MENPBGGOGMC::MENPBGGOGMC::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "IAHOPFNPFLN",
            CJAKIBDIMMJ::has_IAHOPFNPFLN,
            CJAKIBDIMMJ::IAHOPFNPFLN,
            CJAKIBDIMMJ::set_IAHOPFNPFLN,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::MENPBGGOGMC::MENPBGGOGMC>(
            "AMALJFOKCBA",
            CJAKIBDIMMJ::has_AMALJFOKCBA,
            CJAKIBDIMMJ::AMALJFOKCBA,
            CJAKIBDIMMJ::mut_AMALJFOKCBA,
            CJAKIBDIMMJ::set_AMALJFOKCBA,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::MENPBGGOGMC::MENPBGGOGMC>(
            "OECBELGDLFP",
            CJAKIBDIMMJ::has_OECBELGDLFP,
            CJAKIBDIMMJ::OECBELGDLFP,
            CJAKIBDIMMJ::mut_OECBELGDLFP,
            CJAKIBDIMMJ::set_OECBELGDLFP,
        ));
        oneofs.push(cjakibdimmj::PMFBBCEBACD::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CJAKIBDIMMJ>(
            "CJAKIBDIMMJ",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CJAKIBDIMMJ {
    const NAME: &'static str = "CJAKIBDIMMJ";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                3136 => {
                    self.PMFBBCEBACD = ::std::option::Option::Some(cjakibdimmj::PMFBBCEBACD::IAHOPFNPFLN(is.read_bool()?));
                },
                12466 => {
                    self.PMFBBCEBACD = ::std::option::Option::Some(cjakibdimmj::PMFBBCEBACD::AMALJFOKCBA(is.read_message()?));
                },
                3146 => {
                    self.PMFBBCEBACD = ::std::option::Option::Some(cjakibdimmj::PMFBBCEBACD::OECBELGDLFP(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.PMFBBCEBACD {
            match v {
                &cjakibdimmj::PMFBBCEBACD::IAHOPFNPFLN(v) => {
                    my_size += 2 + 1;
                },
                &cjakibdimmj::PMFBBCEBACD::AMALJFOKCBA(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &cjakibdimmj::PMFBBCEBACD::OECBELGDLFP(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let ::std::option::Option::Some(ref v) = self.PMFBBCEBACD {
            match v {
                &cjakibdimmj::PMFBBCEBACD::IAHOPFNPFLN(v) => {
                    os.write_bool(392, v)?;
                },
                &cjakibdimmj::PMFBBCEBACD::AMALJFOKCBA(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1558, v, os)?;
                },
                &cjakibdimmj::PMFBBCEBACD::OECBELGDLFP(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(393, v, os)?;
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

    fn new() -> CJAKIBDIMMJ {
        CJAKIBDIMMJ::new()
    }

    fn clear(&mut self) {
        self.PMFBBCEBACD = ::std::option::Option::None;
        self.PMFBBCEBACD = ::std::option::Option::None;
        self.PMFBBCEBACD = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CJAKIBDIMMJ {
        static instance: CJAKIBDIMMJ = CJAKIBDIMMJ {
            PMFBBCEBACD: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CJAKIBDIMMJ {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CJAKIBDIMMJ").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CJAKIBDIMMJ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CJAKIBDIMMJ {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `CJAKIBDIMMJ`
pub mod cjakibdimmj {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:CJAKIBDIMMJ.PMFBBCEBACD)
    pub enum PMFBBCEBACD {
        // @@protoc_insertion_point(oneof_field:CJAKIBDIMMJ.IAHOPFNPFLN)
        IAHOPFNPFLN(bool),
        // @@protoc_insertion_point(oneof_field:CJAKIBDIMMJ.AMALJFOKCBA)
        AMALJFOKCBA(super::super::MENPBGGOGMC::MENPBGGOGMC),
        // @@protoc_insertion_point(oneof_field:CJAKIBDIMMJ.OECBELGDLFP)
        OECBELGDLFP(super::super::MENPBGGOGMC::MENPBGGOGMC),
    }

    impl ::protobuf::Oneof for PMFBBCEBACD {
    }

    impl ::protobuf::OneofFull for PMFBBCEBACD {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::CJAKIBDIMMJ as ::protobuf::MessageFull>::descriptor().oneof_by_name("PMFBBCEBACD").unwrap()).clone()
        }
    }

    impl PMFBBCEBACD {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<PMFBBCEBACD>("PMFBBCEBACD")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CJAKIBDIMMJ.proto\x1a\x11MENPBGGOGMC.proto\"\xa7\x01\n\x0bCJAKIBDI\
    MMJ\x12#\n\x0bIAHOPFNPFLN\x18\x88\x03\x20\x01(\x08H\0R\x0bIAHOPFNPFLN\
    \x121\n\x0bAMALJFOKCBA\x18\x96\x0c\x20\x01(\x0b2\x0c.MENPBGGOGMCH\0R\x0b\
    AMALJFOKCBA\x121\n\x0bOECBELGDLFP\x18\x89\x03\x20\x01(\x0b2\x0c.MENPBGGO\
    GMCH\0R\x0bOECBELGDLFPB\r\n\x0bPMFBBCEBACDb\x06proto3\
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
            deps.push(super::MENPBGGOGMC::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CJAKIBDIMMJ::generated_message_descriptor_data());
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
