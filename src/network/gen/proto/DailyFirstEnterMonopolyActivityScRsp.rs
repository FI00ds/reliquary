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

//! Generated file from `DailyFirstEnterMonopolyActivityScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:DailyFirstEnterMonopolyActivityScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DailyFirstEnterMonopolyActivityScRsp {
    // message fields
    // @@protoc_insertion_point(field:DailyFirstEnterMonopolyActivityScRsp.OICAGHGMMFP)
    pub OICAGHGMMFP: bool,
    // @@protoc_insertion_point(field:DailyFirstEnterMonopolyActivityScRsp.KEKJCDMIDDL)
    pub KEKJCDMIDDL: u32,
    // @@protoc_insertion_point(field:DailyFirstEnterMonopolyActivityScRsp.LJAOGAPDFHA)
    pub LJAOGAPDFHA: ::protobuf::MessageField<super::AEDKPBFCKGO::AEDKPBFCKGO>,
    // @@protoc_insertion_point(field:DailyFirstEnterMonopolyActivityScRsp.IIHKIKLIOJI)
    pub IIHKIKLIOJI: i64,
    // @@protoc_insertion_point(field:DailyFirstEnterMonopolyActivityScRsp.retcode)
    pub retcode: u32,
    // special fields
    // @@protoc_insertion_point(special_field:DailyFirstEnterMonopolyActivityScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DailyFirstEnterMonopolyActivityScRsp {
    fn default() -> &'a DailyFirstEnterMonopolyActivityScRsp {
        <DailyFirstEnterMonopolyActivityScRsp as ::protobuf::Message>::default_instance()
    }
}

impl DailyFirstEnterMonopolyActivityScRsp {
    pub fn new() -> DailyFirstEnterMonopolyActivityScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OICAGHGMMFP",
            |m: &DailyFirstEnterMonopolyActivityScRsp| { &m.OICAGHGMMFP },
            |m: &mut DailyFirstEnterMonopolyActivityScRsp| { &mut m.OICAGHGMMFP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KEKJCDMIDDL",
            |m: &DailyFirstEnterMonopolyActivityScRsp| { &m.KEKJCDMIDDL },
            |m: &mut DailyFirstEnterMonopolyActivityScRsp| { &mut m.KEKJCDMIDDL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::AEDKPBFCKGO::AEDKPBFCKGO>(
            "LJAOGAPDFHA",
            |m: &DailyFirstEnterMonopolyActivityScRsp| { &m.LJAOGAPDFHA },
            |m: &mut DailyFirstEnterMonopolyActivityScRsp| { &mut m.LJAOGAPDFHA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IIHKIKLIOJI",
            |m: &DailyFirstEnterMonopolyActivityScRsp| { &m.IIHKIKLIOJI },
            |m: &mut DailyFirstEnterMonopolyActivityScRsp| { &mut m.IIHKIKLIOJI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &DailyFirstEnterMonopolyActivityScRsp| { &m.retcode },
            |m: &mut DailyFirstEnterMonopolyActivityScRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DailyFirstEnterMonopolyActivityScRsp>(
            "DailyFirstEnterMonopolyActivityScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DailyFirstEnterMonopolyActivityScRsp {
    const NAME: &'static str = "DailyFirstEnterMonopolyActivityScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.OICAGHGMMFP = is.read_bool()?;
                },
                24 => {
                    self.KEKJCDMIDDL = is.read_uint32()?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LJAOGAPDFHA)?;
                },
                96 => {
                    self.IIHKIKLIOJI = is.read_int64()?;
                },
                16 => {
                    self.retcode = is.read_uint32()?;
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
        if self.OICAGHGMMFP != false {
            my_size += 1 + 1;
        }
        if self.KEKJCDMIDDL != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.KEKJCDMIDDL);
        }
        if let Some(v) = self.LJAOGAPDFHA.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.IIHKIKLIOJI != 0 {
            my_size += ::protobuf::rt::int64_size(12, self.IIHKIKLIOJI);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.OICAGHGMMFP != false {
            os.write_bool(11, self.OICAGHGMMFP)?;
        }
        if self.KEKJCDMIDDL != 0 {
            os.write_uint32(3, self.KEKJCDMIDDL)?;
        }
        if let Some(v) = self.LJAOGAPDFHA.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if self.IIHKIKLIOJI != 0 {
            os.write_int64(12, self.IIHKIKLIOJI)?;
        }
        if self.retcode != 0 {
            os.write_uint32(2, self.retcode)?;
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

    fn new() -> DailyFirstEnterMonopolyActivityScRsp {
        DailyFirstEnterMonopolyActivityScRsp::new()
    }

    fn clear(&mut self) {
        self.OICAGHGMMFP = false;
        self.KEKJCDMIDDL = 0;
        self.LJAOGAPDFHA.clear();
        self.IIHKIKLIOJI = 0;
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DailyFirstEnterMonopolyActivityScRsp {
        static instance: DailyFirstEnterMonopolyActivityScRsp = DailyFirstEnterMonopolyActivityScRsp {
            OICAGHGMMFP: false,
            KEKJCDMIDDL: 0,
            LJAOGAPDFHA: ::protobuf::MessageField::none(),
            IIHKIKLIOJI: 0,
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DailyFirstEnterMonopolyActivityScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DailyFirstEnterMonopolyActivityScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DailyFirstEnterMonopolyActivityScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DailyFirstEnterMonopolyActivityScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n*DailyFirstEnterMonopolyActivityScRsp.proto\x1a\x11AEDKPBFCKGO.proto\"\
    \xd6\x01\n$DailyFirstEnterMonopolyActivityScRsp\x12\x20\n\x0bOICAGHGMMFP\
    \x18\x0b\x20\x01(\x08R\x0bOICAGHGMMFP\x12\x20\n\x0bKEKJCDMIDDL\x18\x03\
    \x20\x01(\rR\x0bKEKJCDMIDDL\x12.\n\x0bLJAOGAPDFHA\x18\x07\x20\x01(\x0b2\
    \x0c.AEDKPBFCKGOR\x0bLJAOGAPDFHA\x12\x20\n\x0bIIHKIKLIOJI\x18\x0c\x20\
    \x01(\x03R\x0bIIHKIKLIOJI\x12\x18\n\x07retcode\x18\x02\x20\x01(\rR\x07re\
    tcodeb\x06proto3\
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
            deps.push(super::AEDKPBFCKGO::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(DailyFirstEnterMonopolyActivityScRsp::generated_message_descriptor_data());
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
