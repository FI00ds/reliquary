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

//! Generated file from `HKMLALBDPGO.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:HKMLALBDPGO)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HKMLALBDPGO {
    // message fields
    // @@protoc_insertion_point(field:HKMLALBDPGO.FAHIHDJFOHM)
    pub FAHIHDJFOHM: u32,
    // @@protoc_insertion_point(field:HKMLALBDPGO.BOONPDEOBLA)
    pub BOONPDEOBLA: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:HKMLALBDPGO.FJKGKAEKBKJ)
    pub FJKGKAEKBKJ: bool,
    // @@protoc_insertion_point(field:HKMLALBDPGO.LHCBBGIMMDG)
    pub LHCBBGIMMDG: u32,
    // special fields
    // @@protoc_insertion_point(special_field:HKMLALBDPGO.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HKMLALBDPGO {
    fn default() -> &'a HKMLALBDPGO {
        <HKMLALBDPGO as ::protobuf::Message>::default_instance()
    }
}

impl HKMLALBDPGO {
    pub fn new() -> HKMLALBDPGO {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FAHIHDJFOHM",
            |m: &HKMLALBDPGO| { &m.FAHIHDJFOHM },
            |m: &mut HKMLALBDPGO| { &mut m.FAHIHDJFOHM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "BOONPDEOBLA",
            |m: &HKMLALBDPGO| { &m.BOONPDEOBLA },
            |m: &mut HKMLALBDPGO| { &mut m.BOONPDEOBLA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FJKGKAEKBKJ",
            |m: &HKMLALBDPGO| { &m.FJKGKAEKBKJ },
            |m: &mut HKMLALBDPGO| { &mut m.FJKGKAEKBKJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LHCBBGIMMDG",
            |m: &HKMLALBDPGO| { &m.LHCBBGIMMDG },
            |m: &mut HKMLALBDPGO| { &mut m.LHCBBGIMMDG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HKMLALBDPGO>(
            "HKMLALBDPGO",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HKMLALBDPGO {
    const NAME: &'static str = "HKMLALBDPGO";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                112 => {
                    self.FAHIHDJFOHM = is.read_uint32()?;
                },
                98 => {
                    is.read_repeated_packed_uint32_into(&mut self.BOONPDEOBLA)?;
                },
                96 => {
                    self.BOONPDEOBLA.push(is.read_uint32()?);
                },
                80 => {
                    self.FJKGKAEKBKJ = is.read_bool()?;
                },
                56 => {
                    self.LHCBBGIMMDG = is.read_uint32()?;
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
        if self.FAHIHDJFOHM != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.FAHIHDJFOHM);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(12, &self.BOONPDEOBLA);
        if self.FJKGKAEKBKJ != false {
            my_size += 1 + 1;
        }
        if self.LHCBBGIMMDG != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.LHCBBGIMMDG);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.FAHIHDJFOHM != 0 {
            os.write_uint32(14, self.FAHIHDJFOHM)?;
        }
        os.write_repeated_packed_uint32(12, &self.BOONPDEOBLA)?;
        if self.FJKGKAEKBKJ != false {
            os.write_bool(10, self.FJKGKAEKBKJ)?;
        }
        if self.LHCBBGIMMDG != 0 {
            os.write_uint32(7, self.LHCBBGIMMDG)?;
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

    fn new() -> HKMLALBDPGO {
        HKMLALBDPGO::new()
    }

    fn clear(&mut self) {
        self.FAHIHDJFOHM = 0;
        self.BOONPDEOBLA.clear();
        self.FJKGKAEKBKJ = false;
        self.LHCBBGIMMDG = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HKMLALBDPGO {
        static instance: HKMLALBDPGO = HKMLALBDPGO {
            FAHIHDJFOHM: 0,
            BOONPDEOBLA: ::std::vec::Vec::new(),
            FJKGKAEKBKJ: false,
            LHCBBGIMMDG: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HKMLALBDPGO {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HKMLALBDPGO").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HKMLALBDPGO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HKMLALBDPGO {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11HKMLALBDPGO.proto\"\x95\x01\n\x0bHKMLALBDPGO\x12\x20\n\x0bFAHIHDJF\
    OHM\x18\x0e\x20\x01(\rR\x0bFAHIHDJFOHM\x12\x20\n\x0bBOONPDEOBLA\x18\x0c\
    \x20\x03(\rR\x0bBOONPDEOBLA\x12\x20\n\x0bFJKGKAEKBKJ\x18\n\x20\x01(\x08R\
    \x0bFJKGKAEKBKJ\x12\x20\n\x0bLHCBBGIMMDG\x18\x07\x20\x01(\rR\x0bLHCBBGIM\
    MDGb\x06proto3\
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
            messages.push(HKMLALBDPGO::generated_message_descriptor_data());
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
