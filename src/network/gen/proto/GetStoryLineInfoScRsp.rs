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

//! Generated file from `GetStoryLineInfoScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GetStoryLineInfoScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetStoryLineInfoScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetStoryLineInfoScRsp.GIIIGPLNIBN)
    pub GIIIGPLNIBN: u32,
    // @@protoc_insertion_point(field:GetStoryLineInfoScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:GetStoryLineInfoScRsp.ELJINBJEAEL)
    pub ELJINBJEAEL: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetStoryLineInfoScRsp.GKMBNPAFGGB)
    pub GKMBNPAFGGB: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:GetStoryLineInfoScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetStoryLineInfoScRsp {
    fn default() -> &'a GetStoryLineInfoScRsp {
        <GetStoryLineInfoScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetStoryLineInfoScRsp {
    pub fn new() -> GetStoryLineInfoScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GIIIGPLNIBN",
            |m: &GetStoryLineInfoScRsp| { &m.GIIIGPLNIBN },
            |m: &mut GetStoryLineInfoScRsp| { &mut m.GIIIGPLNIBN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetStoryLineInfoScRsp| { &m.retcode },
            |m: &mut GetStoryLineInfoScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "ELJINBJEAEL",
            |m: &GetStoryLineInfoScRsp| { &m.ELJINBJEAEL },
            |m: &mut GetStoryLineInfoScRsp| { &mut m.ELJINBJEAEL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "GKMBNPAFGGB",
            |m: &GetStoryLineInfoScRsp| { &m.GKMBNPAFGGB },
            |m: &mut GetStoryLineInfoScRsp| { &mut m.GKMBNPAFGGB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetStoryLineInfoScRsp>(
            "GetStoryLineInfoScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetStoryLineInfoScRsp {
    const NAME: &'static str = "GetStoryLineInfoScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.GIIIGPLNIBN = is.read_uint32()?;
                },
                16 => {
                    self.retcode = is.read_uint32()?;
                },
                90 => {
                    is.read_repeated_packed_uint32_into(&mut self.ELJINBJEAEL)?;
                },
                88 => {
                    self.ELJINBJEAEL.push(is.read_uint32()?);
                },
                74 => {
                    is.read_repeated_packed_uint32_into(&mut self.GKMBNPAFGGB)?;
                },
                72 => {
                    self.GKMBNPAFGGB.push(is.read_uint32()?);
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
        if self.GIIIGPLNIBN != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.GIIIGPLNIBN);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.retcode);
        }
        for value in &self.ELJINBJEAEL {
            my_size += ::protobuf::rt::uint32_size(11, *value);
        };
        for value in &self.GKMBNPAFGGB {
            my_size += ::protobuf::rt::uint32_size(9, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.GIIIGPLNIBN != 0 {
            os.write_uint32(1, self.GIIIGPLNIBN)?;
        }
        if self.retcode != 0 {
            os.write_uint32(2, self.retcode)?;
        }
        for v in &self.ELJINBJEAEL {
            os.write_uint32(11, *v)?;
        };
        for v in &self.GKMBNPAFGGB {
            os.write_uint32(9, *v)?;
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

    fn new() -> GetStoryLineInfoScRsp {
        GetStoryLineInfoScRsp::new()
    }

    fn clear(&mut self) {
        self.GIIIGPLNIBN = 0;
        self.retcode = 0;
        self.ELJINBJEAEL.clear();
        self.GKMBNPAFGGB.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetStoryLineInfoScRsp {
        static instance: GetStoryLineInfoScRsp = GetStoryLineInfoScRsp {
            GIIIGPLNIBN: 0,
            retcode: 0,
            ELJINBJEAEL: ::std::vec::Vec::new(),
            GKMBNPAFGGB: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetStoryLineInfoScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetStoryLineInfoScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetStoryLineInfoScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetStoryLineInfoScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bGetStoryLineInfoScRsp.proto\"\x97\x01\n\x15GetStoryLineInfoScRsp\
    \x12\x20\n\x0bGIIIGPLNIBN\x18\x01\x20\x01(\rR\x0bGIIIGPLNIBN\x12\x18\n\
    \x07retcode\x18\x02\x20\x01(\rR\x07retcode\x12\x20\n\x0bELJINBJEAEL\x18\
    \x0b\x20\x03(\rR\x0bELJINBJEAEL\x12\x20\n\x0bGKMBNPAFGGB\x18\t\x20\x03(\
    \rR\x0bGKMBNPAFGGBb\x06proto3\
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
            messages.push(GetStoryLineInfoScRsp::generated_message_descriptor_data());
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
