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

//! Generated file from `MMOIBACBPKA.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:MMOIBACBPKA)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MMOIBACBPKA {
    // message fields
    // @@protoc_insertion_point(field:MMOIBACBPKA.value)
    pub value: u32,
    // @@protoc_insertion_point(field:MMOIBACBPKA.PJGBFKNJPNO)
    pub PJGBFKNJPNO: ::protobuf::EnumOrUnknown<super::HDIJJMDPILE::HDIJJMDPILE>,
    // special fields
    // @@protoc_insertion_point(special_field:MMOIBACBPKA.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MMOIBACBPKA {
    fn default() -> &'a MMOIBACBPKA {
        <MMOIBACBPKA as ::protobuf::Message>::default_instance()
    }
}

impl MMOIBACBPKA {
    pub fn new() -> MMOIBACBPKA {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "value",
            |m: &MMOIBACBPKA| { &m.value },
            |m: &mut MMOIBACBPKA| { &mut m.value },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PJGBFKNJPNO",
            |m: &MMOIBACBPKA| { &m.PJGBFKNJPNO },
            |m: &mut MMOIBACBPKA| { &mut m.PJGBFKNJPNO },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MMOIBACBPKA>(
            "MMOIBACBPKA",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MMOIBACBPKA {
    const NAME: &'static str = "MMOIBACBPKA";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                112 => {
                    self.value = is.read_uint32()?;
                },
                40 => {
                    self.PJGBFKNJPNO = is.read_enum_or_unknown()?;
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
        if self.value != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.value);
        }
        if self.PJGBFKNJPNO != ::protobuf::EnumOrUnknown::new(super::HDIJJMDPILE::HDIJJMDPILE::SWORD_TRAINING_STATUS_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(5, self.PJGBFKNJPNO.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.value != 0 {
            os.write_uint32(14, self.value)?;
        }
        if self.PJGBFKNJPNO != ::protobuf::EnumOrUnknown::new(super::HDIJJMDPILE::HDIJJMDPILE::SWORD_TRAINING_STATUS_TYPE_NONE) {
            os.write_enum(5, ::protobuf::EnumOrUnknown::value(&self.PJGBFKNJPNO))?;
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

    fn new() -> MMOIBACBPKA {
        MMOIBACBPKA::new()
    }

    fn clear(&mut self) {
        self.value = 0;
        self.PJGBFKNJPNO = ::protobuf::EnumOrUnknown::new(super::HDIJJMDPILE::HDIJJMDPILE::SWORD_TRAINING_STATUS_TYPE_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MMOIBACBPKA {
        static instance: MMOIBACBPKA = MMOIBACBPKA {
            value: 0,
            PJGBFKNJPNO: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MMOIBACBPKA {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MMOIBACBPKA").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MMOIBACBPKA {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MMOIBACBPKA {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11MMOIBACBPKA.proto\x1a\x11HDIJJMDPILE.proto\"S\n\x0bMMOIBACBPKA\x12\
    \x14\n\x05value\x18\x0e\x20\x01(\rR\x05value\x12.\n\x0bPJGBFKNJPNO\x18\
    \x05\x20\x01(\x0e2\x0c.HDIJJMDPILER\x0bPJGBFKNJPNOb\x06proto3\
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
            deps.push(super::HDIJJMDPILE::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MMOIBACBPKA::generated_message_descriptor_data());
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
