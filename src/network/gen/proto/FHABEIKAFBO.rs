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

//! Generated file from `FHABEIKAFBO.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:FHABEIKAFBO)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FHABEIKAFBO {
    // message fields
    // @@protoc_insertion_point(field:FHABEIKAFBO.ELGANMDPMID)
    pub ELGANMDPMID: u32,
    // message oneof groups
    pub IJCLEPCJBOA: ::std::option::Option<fhabeikafbo::IJCLEPCJBOA>,
    // special fields
    // @@protoc_insertion_point(special_field:FHABEIKAFBO.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FHABEIKAFBO {
    fn default() -> &'a FHABEIKAFBO {
        <FHABEIKAFBO as ::protobuf::Message>::default_instance()
    }
}

impl FHABEIKAFBO {
    pub fn new() -> FHABEIKAFBO {
        ::std::default::Default::default()
    }

    // .GNIEJGNKKGG KFBPCFDHLHL = 1100;

    pub fn KFBPCFDHLHL(&self) -> &super::GNIEJGNKKGG::GNIEJGNKKGG {
        match self.IJCLEPCJBOA {
            ::std::option::Option::Some(fhabeikafbo::IJCLEPCJBOA::KFBPCFDHLHL(ref v)) => v,
            _ => <super::GNIEJGNKKGG::GNIEJGNKKGG as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_KFBPCFDHLHL(&mut self) {
        self.IJCLEPCJBOA = ::std::option::Option::None;
    }

    pub fn has_KFBPCFDHLHL(&self) -> bool {
        match self.IJCLEPCJBOA {
            ::std::option::Option::Some(fhabeikafbo::IJCLEPCJBOA::KFBPCFDHLHL(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_KFBPCFDHLHL(&mut self, v: super::GNIEJGNKKGG::GNIEJGNKKGG) {
        self.IJCLEPCJBOA = ::std::option::Option::Some(fhabeikafbo::IJCLEPCJBOA::KFBPCFDHLHL(v))
    }

    // Mutable pointer to the field.
    pub fn mut_KFBPCFDHLHL(&mut self) -> &mut super::GNIEJGNKKGG::GNIEJGNKKGG {
        if let ::std::option::Option::Some(fhabeikafbo::IJCLEPCJBOA::KFBPCFDHLHL(_)) = self.IJCLEPCJBOA {
        } else {
            self.IJCLEPCJBOA = ::std::option::Option::Some(fhabeikafbo::IJCLEPCJBOA::KFBPCFDHLHL(super::GNIEJGNKKGG::GNIEJGNKKGG::new()));
        }
        match self.IJCLEPCJBOA {
            ::std::option::Option::Some(fhabeikafbo::IJCLEPCJBOA::KFBPCFDHLHL(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_KFBPCFDHLHL(&mut self) -> super::GNIEJGNKKGG::GNIEJGNKKGG {
        if self.has_KFBPCFDHLHL() {
            match self.IJCLEPCJBOA.take() {
                ::std::option::Option::Some(fhabeikafbo::IJCLEPCJBOA::KFBPCFDHLHL(v)) => v,
                _ => panic!(),
            }
        } else {
            super::GNIEJGNKKGG::GNIEJGNKKGG::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ELGANMDPMID",
            |m: &FHABEIKAFBO| { &m.ELGANMDPMID },
            |m: &mut FHABEIKAFBO| { &mut m.ELGANMDPMID },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::GNIEJGNKKGG::GNIEJGNKKGG>(
            "KFBPCFDHLHL",
            FHABEIKAFBO::has_KFBPCFDHLHL,
            FHABEIKAFBO::KFBPCFDHLHL,
            FHABEIKAFBO::mut_KFBPCFDHLHL,
            FHABEIKAFBO::set_KFBPCFDHLHL,
        ));
        oneofs.push(fhabeikafbo::IJCLEPCJBOA::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FHABEIKAFBO>(
            "FHABEIKAFBO",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FHABEIKAFBO {
    const NAME: &'static str = "FHABEIKAFBO";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.ELGANMDPMID = is.read_uint32()?;
                },
                8802 => {
                    self.IJCLEPCJBOA = ::std::option::Option::Some(fhabeikafbo::IJCLEPCJBOA::KFBPCFDHLHL(is.read_message()?));
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
        if self.ELGANMDPMID != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.ELGANMDPMID);
        }
        if let ::std::option::Option::Some(ref v) = self.IJCLEPCJBOA {
            match v {
                &fhabeikafbo::IJCLEPCJBOA::KFBPCFDHLHL(ref v) => {
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
        if self.ELGANMDPMID != 0 {
            os.write_uint32(1, self.ELGANMDPMID)?;
        }
        if let ::std::option::Option::Some(ref v) = self.IJCLEPCJBOA {
            match v {
                &fhabeikafbo::IJCLEPCJBOA::KFBPCFDHLHL(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1100, v, os)?;
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

    fn new() -> FHABEIKAFBO {
        FHABEIKAFBO::new()
    }

    fn clear(&mut self) {
        self.ELGANMDPMID = 0;
        self.IJCLEPCJBOA = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FHABEIKAFBO {
        static instance: FHABEIKAFBO = FHABEIKAFBO {
            ELGANMDPMID: 0,
            IJCLEPCJBOA: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FHABEIKAFBO {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FHABEIKAFBO").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FHABEIKAFBO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FHABEIKAFBO {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `FHABEIKAFBO`
pub mod fhabeikafbo {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:FHABEIKAFBO.IJCLEPCJBOA)
    pub enum IJCLEPCJBOA {
        // @@protoc_insertion_point(oneof_field:FHABEIKAFBO.KFBPCFDHLHL)
        KFBPCFDHLHL(super::super::GNIEJGNKKGG::GNIEJGNKKGG),
    }

    impl ::protobuf::Oneof for IJCLEPCJBOA {
    }

    impl ::protobuf::OneofFull for IJCLEPCJBOA {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::FHABEIKAFBO as ::protobuf::MessageFull>::descriptor().oneof_by_name("IJCLEPCJBOA").unwrap()).clone()
        }
    }

    impl IJCLEPCJBOA {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<IJCLEPCJBOA>("IJCLEPCJBOA")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11FHABEIKAFBO.proto\x1a\x11GNIEJGNKKGG.proto\"q\n\x0bFHABEIKAFBO\x12\
    \x20\n\x0bELGANMDPMID\x18\x01\x20\x01(\rR\x0bELGANMDPMID\x121\n\x0bKFBPC\
    FDHLHL\x18\xcc\x08\x20\x01(\x0b2\x0c.GNIEJGNKKGGH\0R\x0bKFBPCFDHLHLB\r\n\
    \x0bIJCLEPCJBOAb\x06proto3\
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
            deps.push(super::GNIEJGNKKGG::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(FHABEIKAFBO::generated_message_descriptor_data());
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
