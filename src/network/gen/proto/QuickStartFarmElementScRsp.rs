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

//! Generated file from `QuickStartFarmElementScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:QuickStartFarmElementScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct QuickStartFarmElementScRsp {
    // message fields
    // @@protoc_insertion_point(field:QuickStartFarmElementScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:QuickStartFarmElementScRsp.BANFFJDIPIE)
    pub BANFFJDIPIE: ::protobuf::MessageField<super::CMBHDGKGPGP::CMBHDGKGPGP>,
    // @@protoc_insertion_point(field:QuickStartFarmElementScRsp.IFJFCEJJBPE)
    pub IFJFCEJJBPE: u32,
    // @@protoc_insertion_point(field:QuickStartFarmElementScRsp.JDANOKNHNHL)
    pub JDANOKNHNHL: u32,
    // special fields
    // @@protoc_insertion_point(special_field:QuickStartFarmElementScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a QuickStartFarmElementScRsp {
    fn default() -> &'a QuickStartFarmElementScRsp {
        <QuickStartFarmElementScRsp as ::protobuf::Message>::default_instance()
    }
}

impl QuickStartFarmElementScRsp {
    pub fn new() -> QuickStartFarmElementScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &QuickStartFarmElementScRsp| { &m.retcode },
            |m: &mut QuickStartFarmElementScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::CMBHDGKGPGP::CMBHDGKGPGP>(
            "BANFFJDIPIE",
            |m: &QuickStartFarmElementScRsp| { &m.BANFFJDIPIE },
            |m: &mut QuickStartFarmElementScRsp| { &mut m.BANFFJDIPIE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IFJFCEJJBPE",
            |m: &QuickStartFarmElementScRsp| { &m.IFJFCEJJBPE },
            |m: &mut QuickStartFarmElementScRsp| { &mut m.IFJFCEJJBPE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JDANOKNHNHL",
            |m: &QuickStartFarmElementScRsp| { &m.JDANOKNHNHL },
            |m: &mut QuickStartFarmElementScRsp| { &mut m.JDANOKNHNHL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<QuickStartFarmElementScRsp>(
            "QuickStartFarmElementScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for QuickStartFarmElementScRsp {
    const NAME: &'static str = "QuickStartFarmElementScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.retcode = is.read_uint32()?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.BANFFJDIPIE)?;
                },
                48 => {
                    self.IFJFCEJJBPE = is.read_uint32()?;
                },
                112 => {
                    self.JDANOKNHNHL = is.read_uint32()?;
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
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.retcode);
        }
        if let Some(v) = self.BANFFJDIPIE.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.IFJFCEJJBPE != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.IFJFCEJJBPE);
        }
        if self.JDANOKNHNHL != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.JDANOKNHNHL);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.retcode != 0 {
            os.write_uint32(1, self.retcode)?;
        }
        if let Some(v) = self.BANFFJDIPIE.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if self.IFJFCEJJBPE != 0 {
            os.write_uint32(6, self.IFJFCEJJBPE)?;
        }
        if self.JDANOKNHNHL != 0 {
            os.write_uint32(14, self.JDANOKNHNHL)?;
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

    fn new() -> QuickStartFarmElementScRsp {
        QuickStartFarmElementScRsp::new()
    }

    fn clear(&mut self) {
        self.retcode = 0;
        self.BANFFJDIPIE.clear();
        self.IFJFCEJJBPE = 0;
        self.JDANOKNHNHL = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static QuickStartFarmElementScRsp {
        static instance: QuickStartFarmElementScRsp = QuickStartFarmElementScRsp {
            retcode: 0,
            BANFFJDIPIE: ::protobuf::MessageField::none(),
            IFJFCEJJBPE: 0,
            JDANOKNHNHL: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for QuickStartFarmElementScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("QuickStartFarmElementScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for QuickStartFarmElementScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QuickStartFarmElementScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20QuickStartFarmElementScRsp.proto\x1a\x11CMBHDGKGPGP.proto\"\xaa\
    \x01\n\x1aQuickStartFarmElementScRsp\x12\x18\n\x07retcode\x18\x01\x20\
    \x01(\rR\x07retcode\x12.\n\x0bBANFFJDIPIE\x18\x05\x20\x01(\x0b2\x0c.CMBH\
    DGKGPGPR\x0bBANFFJDIPIE\x12\x20\n\x0bIFJFCEJJBPE\x18\x06\x20\x01(\rR\x0b\
    IFJFCEJJBPE\x12\x20\n\x0bJDANOKNHNHL\x18\x0e\x20\x01(\rR\x0bJDANOKNHNHLb\
    \x06proto3\
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
            deps.push(super::CMBHDGKGPGP::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(QuickStartFarmElementScRsp::generated_message_descriptor_data());
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
