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

//! Generated file from `KDOEJMHBBGI.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:KDOEJMHBBGI)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct KDOEJMHBBGI {
    // message fields
    // @@protoc_insertion_point(field:KDOEJMHBBGI.DBGFAODBEFC)
    pub DBGFAODBEFC: ::protobuf::MessageField<super::OEDDOIJLGFG::OEDDOIJLGFG>,
    // @@protoc_insertion_point(field:KDOEJMHBBGI.LOPBAJPAEMI)
    pub LOPBAJPAEMI: ::std::vec::Vec<super::CFCAJKFEPAO::CFCAJKFEPAO>,
    // special fields
    // @@protoc_insertion_point(special_field:KDOEJMHBBGI.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a KDOEJMHBBGI {
    fn default() -> &'a KDOEJMHBBGI {
        <KDOEJMHBBGI as ::protobuf::Message>::default_instance()
    }
}

impl KDOEJMHBBGI {
    pub fn new() -> KDOEJMHBBGI {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::OEDDOIJLGFG::OEDDOIJLGFG>(
            "DBGFAODBEFC",
            |m: &KDOEJMHBBGI| { &m.DBGFAODBEFC },
            |m: &mut KDOEJMHBBGI| { &mut m.DBGFAODBEFC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LOPBAJPAEMI",
            |m: &KDOEJMHBBGI| { &m.LOPBAJPAEMI },
            |m: &mut KDOEJMHBBGI| { &mut m.LOPBAJPAEMI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<KDOEJMHBBGI>(
            "KDOEJMHBBGI",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for KDOEJMHBBGI {
    const NAME: &'static str = "KDOEJMHBBGI";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.DBGFAODBEFC)?;
                },
                82 => {
                    self.LOPBAJPAEMI.push(is.read_message()?);
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
        if let Some(v) = self.DBGFAODBEFC.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.LOPBAJPAEMI {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.DBGFAODBEFC.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        for v in &self.LOPBAJPAEMI {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> KDOEJMHBBGI {
        KDOEJMHBBGI::new()
    }

    fn clear(&mut self) {
        self.DBGFAODBEFC.clear();
        self.LOPBAJPAEMI.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static KDOEJMHBBGI {
        static instance: KDOEJMHBBGI = KDOEJMHBBGI {
            DBGFAODBEFC: ::protobuf::MessageField::none(),
            LOPBAJPAEMI: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for KDOEJMHBBGI {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("KDOEJMHBBGI").unwrap()).clone()
    }
}

impl ::std::fmt::Display for KDOEJMHBBGI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KDOEJMHBBGI {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11KDOEJMHBBGI.proto\x1a\x11CFCAJKFEPAO.proto\x1a\x11OEDDOIJLGFG.prot\
    o\"m\n\x0bKDOEJMHBBGI\x12.\n\x0bDBGFAODBEFC\x18\x0f\x20\x01(\x0b2\x0c.OE\
    DDOIJLGFGR\x0bDBGFAODBEFC\x12.\n\x0bLOPBAJPAEMI\x18\n\x20\x03(\x0b2\x0c.\
    CFCAJKFEPAOR\x0bLOPBAJPAEMIb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::CFCAJKFEPAO::file_descriptor().clone());
            deps.push(super::OEDDOIJLGFG::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(KDOEJMHBBGI::generated_message_descriptor_data());
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
