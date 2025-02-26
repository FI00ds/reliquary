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

//! Generated file from `LPFMHAJHDMM.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:LPFMHAJHDMM)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LPFMHAJHDMM {
    // message fields
    // @@protoc_insertion_point(field:LPFMHAJHDMM.OMAALPKJIIH)
    pub OMAALPKJIIH: ::std::vec::Vec<super::LOPCJEOJHCB::LOPCJEOJHCB>,
    // @@protoc_insertion_point(field:LPFMHAJHDMM.CNGLDJNPOPI)
    pub CNGLDJNPOPI: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:LPFMHAJHDMM.LNEJLGEFPLE)
    pub LNEJLGEFPLE: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:LPFMHAJHDMM.KMMBEANDIJH)
    pub KMMBEANDIJH: ::std::vec::Vec<super::GEMJDHNLKLC::GEMJDHNLKLC>,
    // special fields
    // @@protoc_insertion_point(special_field:LPFMHAJHDMM.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LPFMHAJHDMM {
    fn default() -> &'a LPFMHAJHDMM {
        <LPFMHAJHDMM as ::protobuf::Message>::default_instance()
    }
}

impl LPFMHAJHDMM {
    pub fn new() -> LPFMHAJHDMM {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "OMAALPKJIIH",
            |m: &LPFMHAJHDMM| { &m.OMAALPKJIIH },
            |m: &mut LPFMHAJHDMM| { &mut m.OMAALPKJIIH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CNGLDJNPOPI",
            |m: &LPFMHAJHDMM| { &m.CNGLDJNPOPI },
            |m: &mut LPFMHAJHDMM| { &mut m.CNGLDJNPOPI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LNEJLGEFPLE",
            |m: &LPFMHAJHDMM| { &m.LNEJLGEFPLE },
            |m: &mut LPFMHAJHDMM| { &mut m.LNEJLGEFPLE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KMMBEANDIJH",
            |m: &LPFMHAJHDMM| { &m.KMMBEANDIJH },
            |m: &mut LPFMHAJHDMM| { &mut m.KMMBEANDIJH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LPFMHAJHDMM>(
            "LPFMHAJHDMM",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LPFMHAJHDMM {
    const NAME: &'static str = "LPFMHAJHDMM";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                34 => {
                    self.OMAALPKJIIH.push(is.read_message()?);
                },
                66 => {
                    is.read_repeated_packed_uint32_into(&mut self.CNGLDJNPOPI)?;
                },
                64 => {
                    self.CNGLDJNPOPI.push(is.read_uint32()?);
                },
                114 => {
                    is.read_repeated_packed_uint32_into(&mut self.LNEJLGEFPLE)?;
                },
                112 => {
                    self.LNEJLGEFPLE.push(is.read_uint32()?);
                },
                90 => {
                    self.KMMBEANDIJH.push(is.read_message()?);
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
        for value in &self.OMAALPKJIIH {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::vec_packed_uint32_size(8, &self.CNGLDJNPOPI);
        my_size += ::protobuf::rt::vec_packed_uint32_size(14, &self.LNEJLGEFPLE);
        for value in &self.KMMBEANDIJH {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.OMAALPKJIIH {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        os.write_repeated_packed_uint32(8, &self.CNGLDJNPOPI)?;
        os.write_repeated_packed_uint32(14, &self.LNEJLGEFPLE)?;
        for v in &self.KMMBEANDIJH {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
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

    fn new() -> LPFMHAJHDMM {
        LPFMHAJHDMM::new()
    }

    fn clear(&mut self) {
        self.OMAALPKJIIH.clear();
        self.CNGLDJNPOPI.clear();
        self.LNEJLGEFPLE.clear();
        self.KMMBEANDIJH.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LPFMHAJHDMM {
        static instance: LPFMHAJHDMM = LPFMHAJHDMM {
            OMAALPKJIIH: ::std::vec::Vec::new(),
            CNGLDJNPOPI: ::std::vec::Vec::new(),
            LNEJLGEFPLE: ::std::vec::Vec::new(),
            KMMBEANDIJH: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LPFMHAJHDMM {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LPFMHAJHDMM").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LPFMHAJHDMM {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LPFMHAJHDMM {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LPFMHAJHDMM.proto\x1a\x11GEMJDHNLKLC.proto\x1a\x11LOPCJEOJHCB.prot\
    o\"\xb1\x01\n\x0bLPFMHAJHDMM\x12.\n\x0bOMAALPKJIIH\x18\x04\x20\x03(\x0b2\
    \x0c.LOPCJEOJHCBR\x0bOMAALPKJIIH\x12\x20\n\x0bCNGLDJNPOPI\x18\x08\x20\
    \x03(\rR\x0bCNGLDJNPOPI\x12\x20\n\x0bLNEJLGEFPLE\x18\x0e\x20\x03(\rR\x0b\
    LNEJLGEFPLE\x12.\n\x0bKMMBEANDIJH\x18\x0b\x20\x03(\x0b2\x0c.GEMJDHNLKLCR\
    \x0bKMMBEANDIJHb\x06proto3\
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
            deps.push(super::GEMJDHNLKLC::file_descriptor().clone());
            deps.push(super::LOPCJEOJHCB::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LPFMHAJHDMM::generated_message_descriptor_data());
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
