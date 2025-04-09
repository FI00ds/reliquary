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

//! Generated file from `DiscardRelicScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:DiscardRelicScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DiscardRelicScRsp {
    // message fields
    // @@protoc_insertion_point(field:DiscardRelicScRsp.KGEFHOECMMN)
    pub KGEFHOECMMN: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:DiscardRelicScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:DiscardRelicScRsp.NLPCONNJONF)
    pub NLPCONNJONF: ::protobuf::EnumOrUnknown<super::RelicDiscardType::RelicDiscardType>,
    // @@protoc_insertion_point(field:DiscardRelicScRsp.JNKHGFILJPB)
    pub JNKHGFILJPB: bool,
    // special fields
    // @@protoc_insertion_point(special_field:DiscardRelicScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DiscardRelicScRsp {
    fn default() -> &'a DiscardRelicScRsp {
        <DiscardRelicScRsp as ::protobuf::Message>::default_instance()
    }
}

impl DiscardRelicScRsp {
    pub fn new() -> DiscardRelicScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KGEFHOECMMN",
            |m: &DiscardRelicScRsp| { &m.KGEFHOECMMN },
            |m: &mut DiscardRelicScRsp| { &mut m.KGEFHOECMMN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &DiscardRelicScRsp| { &m.retcode },
            |m: &mut DiscardRelicScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NLPCONNJONF",
            |m: &DiscardRelicScRsp| { &m.NLPCONNJONF },
            |m: &mut DiscardRelicScRsp| { &mut m.NLPCONNJONF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JNKHGFILJPB",
            |m: &DiscardRelicScRsp| { &m.JNKHGFILJPB },
            |m: &mut DiscardRelicScRsp| { &mut m.JNKHGFILJPB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DiscardRelicScRsp>(
            "DiscardRelicScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DiscardRelicScRsp {
    const NAME: &'static str = "DiscardRelicScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                98 => {
                    is.read_repeated_packed_uint32_into(&mut self.KGEFHOECMMN)?;
                },
                96 => {
                    self.KGEFHOECMMN.push(is.read_uint32()?);
                },
                8 => {
                    self.retcode = is.read_uint32()?;
                },
                88 => {
                    self.NLPCONNJONF = is.read_enum_or_unknown()?;
                },
                16 => {
                    self.JNKHGFILJPB = is.read_bool()?;
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
        my_size += ::protobuf::rt::vec_packed_uint32_size(12, &self.KGEFHOECMMN);
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.retcode);
        }
        if self.NLPCONNJONF != ::protobuf::EnumOrUnknown::new(super::RelicDiscardType::RelicDiscardType::RELIC_DISCARD_TYPE_SINGLE) {
            my_size += ::protobuf::rt::int32_size(11, self.NLPCONNJONF.value());
        }
        if self.JNKHGFILJPB != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        os.write_repeated_packed_uint32(12, &self.KGEFHOECMMN)?;
        if self.retcode != 0 {
            os.write_uint32(1, self.retcode)?;
        }
        if self.NLPCONNJONF != ::protobuf::EnumOrUnknown::new(super::RelicDiscardType::RelicDiscardType::RELIC_DISCARD_TYPE_SINGLE) {
            os.write_enum(11, ::protobuf::EnumOrUnknown::value(&self.NLPCONNJONF))?;
        }
        if self.JNKHGFILJPB != false {
            os.write_bool(2, self.JNKHGFILJPB)?;
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

    fn new() -> DiscardRelicScRsp {
        DiscardRelicScRsp::new()
    }

    fn clear(&mut self) {
        self.KGEFHOECMMN.clear();
        self.retcode = 0;
        self.NLPCONNJONF = ::protobuf::EnumOrUnknown::new(super::RelicDiscardType::RelicDiscardType::RELIC_DISCARD_TYPE_SINGLE);
        self.JNKHGFILJPB = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DiscardRelicScRsp {
        static instance: DiscardRelicScRsp = DiscardRelicScRsp {
            KGEFHOECMMN: ::std::vec::Vec::new(),
            retcode: 0,
            NLPCONNJONF: ::protobuf::EnumOrUnknown::from_i32(0),
            JNKHGFILJPB: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DiscardRelicScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DiscardRelicScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DiscardRelicScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DiscardRelicScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17DiscardRelicScRsp.proto\x1a\x16RelicDiscardType.proto\"\xa6\x01\n\
    \x11DiscardRelicScRsp\x12\x20\n\x0bKGEFHOECMMN\x18\x0c\x20\x03(\rR\x0bKG\
    EFHOECMMN\x12\x18\n\x07retcode\x18\x01\x20\x01(\rR\x07retcode\x123\n\x0b\
    NLPCONNJONF\x18\x0b\x20\x01(\x0e2\x11.RelicDiscardTypeR\x0bNLPCONNJONF\
    \x12\x20\n\x0bJNKHGFILJPB\x18\x02\x20\x01(\x08R\x0bJNKHGFILJPBb\x06proto\
    3\
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
            deps.push(super::RelicDiscardType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(DiscardRelicScRsp::generated_message_descriptor_data());
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
