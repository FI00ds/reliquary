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

//! Generated file from `ChimeraFinishEndlessRoundScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:ChimeraFinishEndlessRoundScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChimeraFinishEndlessRoundScRsp {
    // message fields
    // @@protoc_insertion_point(field:ChimeraFinishEndlessRoundScRsp.PAHMAGPFDDJ)
    pub PAHMAGPFDDJ: bool,
    // @@protoc_insertion_point(field:ChimeraFinishEndlessRoundScRsp.BBBGKCHNOCK)
    pub BBBGKCHNOCK: u32,
    // @@protoc_insertion_point(field:ChimeraFinishEndlessRoundScRsp.GBEMDNCKKBA)
    pub GBEMDNCKKBA: u32,
    // @@protoc_insertion_point(field:ChimeraFinishEndlessRoundScRsp.CPBDBAIDAEH)
    pub CPBDBAIDAEH: u32,
    // @@protoc_insertion_point(field:ChimeraFinishEndlessRoundScRsp.LFKFOCJFNCJ)
    pub LFKFOCJFNCJ: u32,
    // @@protoc_insertion_point(field:ChimeraFinishEndlessRoundScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:ChimeraFinishEndlessRoundScRsp.IGAGIBNELCK)
    pub IGAGIBNELCK: ::std::vec::Vec<super::JCNJDFFCLDG::JCNJDFFCLDG>,
    // @@protoc_insertion_point(field:ChimeraFinishEndlessRoundScRsp.AEBJNGIMHCJ)
    pub AEBJNGIMHCJ: u32,
    // special fields
    // @@protoc_insertion_point(special_field:ChimeraFinishEndlessRoundScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChimeraFinishEndlessRoundScRsp {
    fn default() -> &'a ChimeraFinishEndlessRoundScRsp {
        <ChimeraFinishEndlessRoundScRsp as ::protobuf::Message>::default_instance()
    }
}

impl ChimeraFinishEndlessRoundScRsp {
    pub fn new() -> ChimeraFinishEndlessRoundScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PAHMAGPFDDJ",
            |m: &ChimeraFinishEndlessRoundScRsp| { &m.PAHMAGPFDDJ },
            |m: &mut ChimeraFinishEndlessRoundScRsp| { &mut m.PAHMAGPFDDJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BBBGKCHNOCK",
            |m: &ChimeraFinishEndlessRoundScRsp| { &m.BBBGKCHNOCK },
            |m: &mut ChimeraFinishEndlessRoundScRsp| { &mut m.BBBGKCHNOCK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GBEMDNCKKBA",
            |m: &ChimeraFinishEndlessRoundScRsp| { &m.GBEMDNCKKBA },
            |m: &mut ChimeraFinishEndlessRoundScRsp| { &mut m.GBEMDNCKKBA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CPBDBAIDAEH",
            |m: &ChimeraFinishEndlessRoundScRsp| { &m.CPBDBAIDAEH },
            |m: &mut ChimeraFinishEndlessRoundScRsp| { &mut m.CPBDBAIDAEH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LFKFOCJFNCJ",
            |m: &ChimeraFinishEndlessRoundScRsp| { &m.LFKFOCJFNCJ },
            |m: &mut ChimeraFinishEndlessRoundScRsp| { &mut m.LFKFOCJFNCJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &ChimeraFinishEndlessRoundScRsp| { &m.retcode },
            |m: &mut ChimeraFinishEndlessRoundScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "IGAGIBNELCK",
            |m: &ChimeraFinishEndlessRoundScRsp| { &m.IGAGIBNELCK },
            |m: &mut ChimeraFinishEndlessRoundScRsp| { &mut m.IGAGIBNELCK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AEBJNGIMHCJ",
            |m: &ChimeraFinishEndlessRoundScRsp| { &m.AEBJNGIMHCJ },
            |m: &mut ChimeraFinishEndlessRoundScRsp| { &mut m.AEBJNGIMHCJ },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChimeraFinishEndlessRoundScRsp>(
            "ChimeraFinishEndlessRoundScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChimeraFinishEndlessRoundScRsp {
    const NAME: &'static str = "ChimeraFinishEndlessRoundScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.PAHMAGPFDDJ = is.read_bool()?;
                },
                24 => {
                    self.BBBGKCHNOCK = is.read_uint32()?;
                },
                64 => {
                    self.GBEMDNCKKBA = is.read_uint32()?;
                },
                120 => {
                    self.CPBDBAIDAEH = is.read_uint32()?;
                },
                40 => {
                    self.LFKFOCJFNCJ = is.read_uint32()?;
                },
                56 => {
                    self.retcode = is.read_uint32()?;
                },
                114 => {
                    self.IGAGIBNELCK.push(is.read_message()?);
                },
                88 => {
                    self.AEBJNGIMHCJ = is.read_uint32()?;
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
        if self.PAHMAGPFDDJ != false {
            my_size += 1 + 1;
        }
        if self.BBBGKCHNOCK != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.BBBGKCHNOCK);
        }
        if self.GBEMDNCKKBA != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.GBEMDNCKKBA);
        }
        if self.CPBDBAIDAEH != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.CPBDBAIDAEH);
        }
        if self.LFKFOCJFNCJ != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.LFKFOCJFNCJ);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.retcode);
        }
        for value in &self.IGAGIBNELCK {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.AEBJNGIMHCJ != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.AEBJNGIMHCJ);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.PAHMAGPFDDJ != false {
            os.write_bool(2, self.PAHMAGPFDDJ)?;
        }
        if self.BBBGKCHNOCK != 0 {
            os.write_uint32(3, self.BBBGKCHNOCK)?;
        }
        if self.GBEMDNCKKBA != 0 {
            os.write_uint32(8, self.GBEMDNCKKBA)?;
        }
        if self.CPBDBAIDAEH != 0 {
            os.write_uint32(15, self.CPBDBAIDAEH)?;
        }
        if self.LFKFOCJFNCJ != 0 {
            os.write_uint32(5, self.LFKFOCJFNCJ)?;
        }
        if self.retcode != 0 {
            os.write_uint32(7, self.retcode)?;
        }
        for v in &self.IGAGIBNELCK {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        };
        if self.AEBJNGIMHCJ != 0 {
            os.write_uint32(11, self.AEBJNGIMHCJ)?;
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

    fn new() -> ChimeraFinishEndlessRoundScRsp {
        ChimeraFinishEndlessRoundScRsp::new()
    }

    fn clear(&mut self) {
        self.PAHMAGPFDDJ = false;
        self.BBBGKCHNOCK = 0;
        self.GBEMDNCKKBA = 0;
        self.CPBDBAIDAEH = 0;
        self.LFKFOCJFNCJ = 0;
        self.retcode = 0;
        self.IGAGIBNELCK.clear();
        self.AEBJNGIMHCJ = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChimeraFinishEndlessRoundScRsp {
        static instance: ChimeraFinishEndlessRoundScRsp = ChimeraFinishEndlessRoundScRsp {
            PAHMAGPFDDJ: false,
            BBBGKCHNOCK: 0,
            GBEMDNCKKBA: 0,
            CPBDBAIDAEH: 0,
            LFKFOCJFNCJ: 0,
            retcode: 0,
            IGAGIBNELCK: ::std::vec::Vec::new(),
            AEBJNGIMHCJ: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ChimeraFinishEndlessRoundScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChimeraFinishEndlessRoundScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChimeraFinishEndlessRoundScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChimeraFinishEndlessRoundScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n$ChimeraFinishEndlessRoundScRsp.proto\x1a\x11JCNJDFFCLDG.proto\"\xb6\
    \x02\n\x1eChimeraFinishEndlessRoundScRsp\x12\x20\n\x0bPAHMAGPFDDJ\x18\
    \x02\x20\x01(\x08R\x0bPAHMAGPFDDJ\x12\x20\n\x0bBBBGKCHNOCK\x18\x03\x20\
    \x01(\rR\x0bBBBGKCHNOCK\x12\x20\n\x0bGBEMDNCKKBA\x18\x08\x20\x01(\rR\x0b\
    GBEMDNCKKBA\x12\x20\n\x0bCPBDBAIDAEH\x18\x0f\x20\x01(\rR\x0bCPBDBAIDAEH\
    \x12\x20\n\x0bLFKFOCJFNCJ\x18\x05\x20\x01(\rR\x0bLFKFOCJFNCJ\x12\x18\n\
    \x07retcode\x18\x07\x20\x01(\rR\x07retcode\x12.\n\x0bIGAGIBNELCK\x18\x0e\
    \x20\x03(\x0b2\x0c.JCNJDFFCLDGR\x0bIGAGIBNELCK\x12\x20\n\x0bAEBJNGIMHCJ\
    \x18\x0b\x20\x01(\rR\x0bAEBJNGIMHCJb\x06proto3\
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
            deps.push(super::JCNJDFFCLDG::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ChimeraFinishEndlessRoundScRsp::generated_message_descriptor_data());
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
