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

//! Generated file from `EOIAPEFEDGE.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EOIAPEFEDGE)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EOIAPEFEDGE {
    // message fields
    // @@protoc_insertion_point(field:EOIAPEFEDGE.JFGJGCEONBL)
    pub JFGJGCEONBL: i32,
    // @@protoc_insertion_point(field:EOIAPEFEDGE.BKIJGEBHAIA)
    pub BKIJGEBHAIA: i32,
    // @@protoc_insertion_point(field:EOIAPEFEDGE.NHIGINHAHPK)
    pub NHIGINHAHPK: i32,
    // special fields
    // @@protoc_insertion_point(special_field:EOIAPEFEDGE.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EOIAPEFEDGE {
    fn default() -> &'a EOIAPEFEDGE {
        <EOIAPEFEDGE as ::protobuf::Message>::default_instance()
    }
}

impl EOIAPEFEDGE {
    pub fn new() -> EOIAPEFEDGE {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JFGJGCEONBL",
            |m: &EOIAPEFEDGE| { &m.JFGJGCEONBL },
            |m: &mut EOIAPEFEDGE| { &mut m.JFGJGCEONBL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BKIJGEBHAIA",
            |m: &EOIAPEFEDGE| { &m.BKIJGEBHAIA },
            |m: &mut EOIAPEFEDGE| { &mut m.BKIJGEBHAIA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NHIGINHAHPK",
            |m: &EOIAPEFEDGE| { &m.NHIGINHAHPK },
            |m: &mut EOIAPEFEDGE| { &mut m.NHIGINHAHPK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EOIAPEFEDGE>(
            "EOIAPEFEDGE",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EOIAPEFEDGE {
    const NAME: &'static str = "EOIAPEFEDGE";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.JFGJGCEONBL = is.read_int32()?;
                },
                96 => {
                    self.BKIJGEBHAIA = is.read_int32()?;
                },
                24 => {
                    self.NHIGINHAHPK = is.read_int32()?;
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
        if self.JFGJGCEONBL != 0 {
            my_size += ::protobuf::rt::int32_size(15, self.JFGJGCEONBL);
        }
        if self.BKIJGEBHAIA != 0 {
            my_size += ::protobuf::rt::int32_size(12, self.BKIJGEBHAIA);
        }
        if self.NHIGINHAHPK != 0 {
            my_size += ::protobuf::rt::int32_size(3, self.NHIGINHAHPK);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.JFGJGCEONBL != 0 {
            os.write_int32(15, self.JFGJGCEONBL)?;
        }
        if self.BKIJGEBHAIA != 0 {
            os.write_int32(12, self.BKIJGEBHAIA)?;
        }
        if self.NHIGINHAHPK != 0 {
            os.write_int32(3, self.NHIGINHAHPK)?;
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

    fn new() -> EOIAPEFEDGE {
        EOIAPEFEDGE::new()
    }

    fn clear(&mut self) {
        self.JFGJGCEONBL = 0;
        self.BKIJGEBHAIA = 0;
        self.NHIGINHAHPK = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EOIAPEFEDGE {
        static instance: EOIAPEFEDGE = EOIAPEFEDGE {
            JFGJGCEONBL: 0,
            BKIJGEBHAIA: 0,
            NHIGINHAHPK: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EOIAPEFEDGE {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EOIAPEFEDGE").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EOIAPEFEDGE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EOIAPEFEDGE {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11EOIAPEFEDGE.proto\"s\n\x0bEOIAPEFEDGE\x12\x20\n\x0bJFGJGCEONBL\x18\
    \x0f\x20\x01(\x05R\x0bJFGJGCEONBL\x12\x20\n\x0bBKIJGEBHAIA\x18\x0c\x20\
    \x01(\x05R\x0bBKIJGEBHAIA\x12\x20\n\x0bNHIGINHAHPK\x18\x03\x20\x01(\x05R\
    \x0bNHIGINHAHPKb\x06proto3\
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
            messages.push(EOIAPEFEDGE::generated_message_descriptor_data());
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
