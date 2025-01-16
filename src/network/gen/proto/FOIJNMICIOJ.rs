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

//! Generated file from `FOIJNMICIOJ.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:FOIJNMICIOJ)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FOIJNMICIOJ {
    // message fields
    // @@protoc_insertion_point(field:FOIJNMICIOJ.JHGNCIBOPBE)
    pub JHGNCIBOPBE: u32,
    // @@protoc_insertion_point(field:FOIJNMICIOJ.HCMKKIIIGME)
    pub HCMKKIIIGME: bool,
    // @@protoc_insertion_point(field:FOIJNMICIOJ.DPLCBCGFDHO)
    pub DPLCBCGFDHO: u32,
    // special fields
    // @@protoc_insertion_point(special_field:FOIJNMICIOJ.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FOIJNMICIOJ {
    fn default() -> &'a FOIJNMICIOJ {
        <FOIJNMICIOJ as ::protobuf::Message>::default_instance()
    }
}

impl FOIJNMICIOJ {
    pub fn new() -> FOIJNMICIOJ {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JHGNCIBOPBE",
            |m: &FOIJNMICIOJ| { &m.JHGNCIBOPBE },
            |m: &mut FOIJNMICIOJ| { &mut m.JHGNCIBOPBE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HCMKKIIIGME",
            |m: &FOIJNMICIOJ| { &m.HCMKKIIIGME },
            |m: &mut FOIJNMICIOJ| { &mut m.HCMKKIIIGME },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DPLCBCGFDHO",
            |m: &FOIJNMICIOJ| { &m.DPLCBCGFDHO },
            |m: &mut FOIJNMICIOJ| { &mut m.DPLCBCGFDHO },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FOIJNMICIOJ>(
            "FOIJNMICIOJ",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FOIJNMICIOJ {
    const NAME: &'static str = "FOIJNMICIOJ";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.JHGNCIBOPBE = is.read_uint32()?;
                },
                8 => {
                    self.HCMKKIIIGME = is.read_bool()?;
                },
                120 => {
                    self.DPLCBCGFDHO = is.read_uint32()?;
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
        if self.JHGNCIBOPBE != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.JHGNCIBOPBE);
        }
        if self.HCMKKIIIGME != false {
            my_size += 1 + 1;
        }
        if self.DPLCBCGFDHO != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.DPLCBCGFDHO);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.JHGNCIBOPBE != 0 {
            os.write_uint32(8, self.JHGNCIBOPBE)?;
        }
        if self.HCMKKIIIGME != false {
            os.write_bool(1, self.HCMKKIIIGME)?;
        }
        if self.DPLCBCGFDHO != 0 {
            os.write_uint32(15, self.DPLCBCGFDHO)?;
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

    fn new() -> FOIJNMICIOJ {
        FOIJNMICIOJ::new()
    }

    fn clear(&mut self) {
        self.JHGNCIBOPBE = 0;
        self.HCMKKIIIGME = false;
        self.DPLCBCGFDHO = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FOIJNMICIOJ {
        static instance: FOIJNMICIOJ = FOIJNMICIOJ {
            JHGNCIBOPBE: 0,
            HCMKKIIIGME: false,
            DPLCBCGFDHO: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FOIJNMICIOJ {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FOIJNMICIOJ").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FOIJNMICIOJ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FOIJNMICIOJ {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11FOIJNMICIOJ.proto\"s\n\x0bFOIJNMICIOJ\x12\x20\n\x0bJHGNCIBOPBE\x18\
    \x08\x20\x01(\rR\x0bJHGNCIBOPBE\x12\x20\n\x0bHCMKKIIIGME\x18\x01\x20\x01\
    (\x08R\x0bHCMKKIIIGME\x12\x20\n\x0bDPLCBCGFDHO\x18\x0f\x20\x01(\rR\x0bDP\
    LCBCGFDHOb\x06proto3\
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
            messages.push(FOIJNMICIOJ::generated_message_descriptor_data());
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
