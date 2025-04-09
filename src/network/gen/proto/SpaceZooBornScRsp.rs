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

//! Generated file from `SpaceZooBornScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:SpaceZooBornScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SpaceZooBornScRsp {
    // message fields
    // @@protoc_insertion_point(field:SpaceZooBornScRsp.KPKDHGHDGNB)
    pub KPKDHGHDGNB: ::protobuf::MessageField<super::FAFGMLPADMI::FAFGMLPADMI>,
    // @@protoc_insertion_point(field:SpaceZooBornScRsp.GOEAOFNFJOD)
    pub GOEAOFNFJOD: ::std::vec::Vec<super::IPJAIINEGEL::IPJAIINEGEL>,
    // @@protoc_insertion_point(field:SpaceZooBornScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:SpaceZooBornScRsp.IKLPNCGBPPC)
    pub IKLPNCGBPPC: bool,
    // special fields
    // @@protoc_insertion_point(special_field:SpaceZooBornScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SpaceZooBornScRsp {
    fn default() -> &'a SpaceZooBornScRsp {
        <SpaceZooBornScRsp as ::protobuf::Message>::default_instance()
    }
}

impl SpaceZooBornScRsp {
    pub fn new() -> SpaceZooBornScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::FAFGMLPADMI::FAFGMLPADMI>(
            "KPKDHGHDGNB",
            |m: &SpaceZooBornScRsp| { &m.KPKDHGHDGNB },
            |m: &mut SpaceZooBornScRsp| { &mut m.KPKDHGHDGNB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "GOEAOFNFJOD",
            |m: &SpaceZooBornScRsp| { &m.GOEAOFNFJOD },
            |m: &mut SpaceZooBornScRsp| { &mut m.GOEAOFNFJOD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &SpaceZooBornScRsp| { &m.retcode },
            |m: &mut SpaceZooBornScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IKLPNCGBPPC",
            |m: &SpaceZooBornScRsp| { &m.IKLPNCGBPPC },
            |m: &mut SpaceZooBornScRsp| { &mut m.IKLPNCGBPPC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SpaceZooBornScRsp>(
            "SpaceZooBornScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SpaceZooBornScRsp {
    const NAME: &'static str = "SpaceZooBornScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KPKDHGHDGNB)?;
                },
                90 => {
                    self.GOEAOFNFJOD.push(is.read_message()?);
                },
                40 => {
                    self.retcode = is.read_uint32()?;
                },
                64 => {
                    self.IKLPNCGBPPC = is.read_bool()?;
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
        if let Some(v) = self.KPKDHGHDGNB.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.GOEAOFNFJOD {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.retcode);
        }
        if self.IKLPNCGBPPC != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.KPKDHGHDGNB.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        for v in &self.GOEAOFNFJOD {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        if self.retcode != 0 {
            os.write_uint32(5, self.retcode)?;
        }
        if self.IKLPNCGBPPC != false {
            os.write_bool(8, self.IKLPNCGBPPC)?;
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

    fn new() -> SpaceZooBornScRsp {
        SpaceZooBornScRsp::new()
    }

    fn clear(&mut self) {
        self.KPKDHGHDGNB.clear();
        self.GOEAOFNFJOD.clear();
        self.retcode = 0;
        self.IKLPNCGBPPC = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SpaceZooBornScRsp {
        static instance: SpaceZooBornScRsp = SpaceZooBornScRsp {
            KPKDHGHDGNB: ::protobuf::MessageField::none(),
            GOEAOFNFJOD: ::std::vec::Vec::new(),
            retcode: 0,
            IKLPNCGBPPC: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SpaceZooBornScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SpaceZooBornScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SpaceZooBornScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SpaceZooBornScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17SpaceZooBornScRsp.proto\x1a\x11FAFGMLPADMI.proto\x1a\x11IPJAIINEGE\
    L.proto\"\xaf\x01\n\x11SpaceZooBornScRsp\x12.\n\x0bKPKDHGHDGNB\x18\x02\
    \x20\x01(\x0b2\x0c.FAFGMLPADMIR\x0bKPKDHGHDGNB\x12.\n\x0bGOEAOFNFJOD\x18\
    \x0b\x20\x03(\x0b2\x0c.IPJAIINEGELR\x0bGOEAOFNFJOD\x12\x18\n\x07retcode\
    \x18\x05\x20\x01(\rR\x07retcode\x12\x20\n\x0bIKLPNCGBPPC\x18\x08\x20\x01\
    (\x08R\x0bIKLPNCGBPPCb\x06proto3\
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
            deps.push(super::FAFGMLPADMI::file_descriptor().clone());
            deps.push(super::IPJAIINEGEL::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SpaceZooBornScRsp::generated_message_descriptor_data());
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
