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

//! Generated file from `StartAetherDivideSceneBattleCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:StartAetherDivideSceneBattleCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct StartAetherDivideSceneBattleCsReq {
    // message fields
    // @@protoc_insertion_point(field:StartAetherDivideSceneBattleCsReq.FILFACPLHBN)
    pub FILFACPLHBN: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:StartAetherDivideSceneBattleCsReq.KIAIPABLKJD)
    pub KIAIPABLKJD: ::std::vec::Vec<super::CNHNPJJDIGD::CNHNPJJDIGD>,
    // @@protoc_insertion_point(field:StartAetherDivideSceneBattleCsReq.EMNMEOOOJBH)
    pub EMNMEOOOJBH: u32,
    // @@protoc_insertion_point(field:StartAetherDivideSceneBattleCsReq.BPMDFENIDBF)
    pub BPMDFENIDBF: u32,
    // @@protoc_insertion_point(field:StartAetherDivideSceneBattleCsReq.LNJDMBPOGHH)
    pub LNJDMBPOGHH: u32,
    // special fields
    // @@protoc_insertion_point(special_field:StartAetherDivideSceneBattleCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a StartAetherDivideSceneBattleCsReq {
    fn default() -> &'a StartAetherDivideSceneBattleCsReq {
        <StartAetherDivideSceneBattleCsReq as ::protobuf::Message>::default_instance()
    }
}

impl StartAetherDivideSceneBattleCsReq {
    pub fn new() -> StartAetherDivideSceneBattleCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FILFACPLHBN",
            |m: &StartAetherDivideSceneBattleCsReq| { &m.FILFACPLHBN },
            |m: &mut StartAetherDivideSceneBattleCsReq| { &mut m.FILFACPLHBN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KIAIPABLKJD",
            |m: &StartAetherDivideSceneBattleCsReq| { &m.KIAIPABLKJD },
            |m: &mut StartAetherDivideSceneBattleCsReq| { &mut m.KIAIPABLKJD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EMNMEOOOJBH",
            |m: &StartAetherDivideSceneBattleCsReq| { &m.EMNMEOOOJBH },
            |m: &mut StartAetherDivideSceneBattleCsReq| { &mut m.EMNMEOOOJBH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BPMDFENIDBF",
            |m: &StartAetherDivideSceneBattleCsReq| { &m.BPMDFENIDBF },
            |m: &mut StartAetherDivideSceneBattleCsReq| { &mut m.BPMDFENIDBF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LNJDMBPOGHH",
            |m: &StartAetherDivideSceneBattleCsReq| { &m.LNJDMBPOGHH },
            |m: &mut StartAetherDivideSceneBattleCsReq| { &mut m.LNJDMBPOGHH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<StartAetherDivideSceneBattleCsReq>(
            "StartAetherDivideSceneBattleCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for StartAetherDivideSceneBattleCsReq {
    const NAME: &'static str = "StartAetherDivideSceneBattleCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                106 => {
                    is.read_repeated_packed_uint32_into(&mut self.FILFACPLHBN)?;
                },
                104 => {
                    self.FILFACPLHBN.push(is.read_uint32()?);
                },
                26 => {
                    self.KIAIPABLKJD.push(is.read_message()?);
                },
                96 => {
                    self.EMNMEOOOJBH = is.read_uint32()?;
                },
                48 => {
                    self.BPMDFENIDBF = is.read_uint32()?;
                },
                8 => {
                    self.LNJDMBPOGHH = is.read_uint32()?;
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
        my_size += ::protobuf::rt::vec_packed_uint32_size(13, &self.FILFACPLHBN);
        for value in &self.KIAIPABLKJD {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.EMNMEOOOJBH != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.EMNMEOOOJBH);
        }
        if self.BPMDFENIDBF != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.BPMDFENIDBF);
        }
        if self.LNJDMBPOGHH != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.LNJDMBPOGHH);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        os.write_repeated_packed_uint32(13, &self.FILFACPLHBN)?;
        for v in &self.KIAIPABLKJD {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        if self.EMNMEOOOJBH != 0 {
            os.write_uint32(12, self.EMNMEOOOJBH)?;
        }
        if self.BPMDFENIDBF != 0 {
            os.write_uint32(6, self.BPMDFENIDBF)?;
        }
        if self.LNJDMBPOGHH != 0 {
            os.write_uint32(1, self.LNJDMBPOGHH)?;
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

    fn new() -> StartAetherDivideSceneBattleCsReq {
        StartAetherDivideSceneBattleCsReq::new()
    }

    fn clear(&mut self) {
        self.FILFACPLHBN.clear();
        self.KIAIPABLKJD.clear();
        self.EMNMEOOOJBH = 0;
        self.BPMDFENIDBF = 0;
        self.LNJDMBPOGHH = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static StartAetherDivideSceneBattleCsReq {
        static instance: StartAetherDivideSceneBattleCsReq = StartAetherDivideSceneBattleCsReq {
            FILFACPLHBN: ::std::vec::Vec::new(),
            KIAIPABLKJD: ::std::vec::Vec::new(),
            EMNMEOOOJBH: 0,
            BPMDFENIDBF: 0,
            LNJDMBPOGHH: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for StartAetherDivideSceneBattleCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("StartAetherDivideSceneBattleCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for StartAetherDivideSceneBattleCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StartAetherDivideSceneBattleCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'StartAetherDivideSceneBattleCsReq.proto\x1a\x11CNHNPJJDIGD.proto\"\
    \xdb\x01\n!StartAetherDivideSceneBattleCsReq\x12\x20\n\x0bFILFACPLHBN\
    \x18\r\x20\x03(\rR\x0bFILFACPLHBN\x12.\n\x0bKIAIPABLKJD\x18\x03\x20\x03(\
    \x0b2\x0c.CNHNPJJDIGDR\x0bKIAIPABLKJD\x12\x20\n\x0bEMNMEOOOJBH\x18\x0c\
    \x20\x01(\rR\x0bEMNMEOOOJBH\x12\x20\n\x0bBPMDFENIDBF\x18\x06\x20\x01(\rR\
    \x0bBPMDFENIDBF\x12\x20\n\x0bLNJDMBPOGHH\x18\x01\x20\x01(\rR\x0bLNJDMBPO\
    GHHb\x06proto3\
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
            deps.push(super::CNHNPJJDIGD::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(StartAetherDivideSceneBattleCsReq::generated_message_descriptor_data());
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
