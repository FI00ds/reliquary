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

//! Generated file from `PMFIGAGMLOJ.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:PMFIGAGMLOJ)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PMFIGAGMLOJ {
    // message fields
    // @@protoc_insertion_point(field:PMFIGAGMLOJ.EDCNKKMGPCO)
    pub EDCNKKMGPCO: u32,
    // @@protoc_insertion_point(field:PMFIGAGMLOJ.DNEAMPLLFME)
    pub DNEAMPLLFME: u32,
    // @@protoc_insertion_point(field:PMFIGAGMLOJ.KPIEAIKHCHD)
    pub KPIEAIKHCHD: u32,
    // @@protoc_insertion_point(field:PMFIGAGMLOJ.level)
    pub level: u32,
    // special fields
    // @@protoc_insertion_point(special_field:PMFIGAGMLOJ.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PMFIGAGMLOJ {
    fn default() -> &'a PMFIGAGMLOJ {
        <PMFIGAGMLOJ as ::protobuf::Message>::default_instance()
    }
}

impl PMFIGAGMLOJ {
    pub fn new() -> PMFIGAGMLOJ {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EDCNKKMGPCO",
            |m: &PMFIGAGMLOJ| { &m.EDCNKKMGPCO },
            |m: &mut PMFIGAGMLOJ| { &mut m.EDCNKKMGPCO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DNEAMPLLFME",
            |m: &PMFIGAGMLOJ| { &m.DNEAMPLLFME },
            |m: &mut PMFIGAGMLOJ| { &mut m.DNEAMPLLFME },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KPIEAIKHCHD",
            |m: &PMFIGAGMLOJ| { &m.KPIEAIKHCHD },
            |m: &mut PMFIGAGMLOJ| { &mut m.KPIEAIKHCHD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &PMFIGAGMLOJ| { &m.level },
            |m: &mut PMFIGAGMLOJ| { &mut m.level },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PMFIGAGMLOJ>(
            "PMFIGAGMLOJ",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PMFIGAGMLOJ {
    const NAME: &'static str = "PMFIGAGMLOJ";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.EDCNKKMGPCO = is.read_uint32()?;
                },
                96 => {
                    self.DNEAMPLLFME = is.read_uint32()?;
                },
                56 => {
                    self.KPIEAIKHCHD = is.read_uint32()?;
                },
                48 => {
                    self.level = is.read_uint32()?;
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
        if self.EDCNKKMGPCO != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.EDCNKKMGPCO);
        }
        if self.DNEAMPLLFME != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.DNEAMPLLFME);
        }
        if self.KPIEAIKHCHD != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.KPIEAIKHCHD);
        }
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.level);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.EDCNKKMGPCO != 0 {
            os.write_uint32(1, self.EDCNKKMGPCO)?;
        }
        if self.DNEAMPLLFME != 0 {
            os.write_uint32(12, self.DNEAMPLLFME)?;
        }
        if self.KPIEAIKHCHD != 0 {
            os.write_uint32(7, self.KPIEAIKHCHD)?;
        }
        if self.level != 0 {
            os.write_uint32(6, self.level)?;
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

    fn new() -> PMFIGAGMLOJ {
        PMFIGAGMLOJ::new()
    }

    fn clear(&mut self) {
        self.EDCNKKMGPCO = 0;
        self.DNEAMPLLFME = 0;
        self.KPIEAIKHCHD = 0;
        self.level = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PMFIGAGMLOJ {
        static instance: PMFIGAGMLOJ = PMFIGAGMLOJ {
            EDCNKKMGPCO: 0,
            DNEAMPLLFME: 0,
            KPIEAIKHCHD: 0,
            level: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PMFIGAGMLOJ {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PMFIGAGMLOJ").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PMFIGAGMLOJ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PMFIGAGMLOJ {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PMFIGAGMLOJ.proto\"\x89\x01\n\x0bPMFIGAGMLOJ\x12\x20\n\x0bEDCNKKMG\
    PCO\x18\x01\x20\x01(\rR\x0bEDCNKKMGPCO\x12\x20\n\x0bDNEAMPLLFME\x18\x0c\
    \x20\x01(\rR\x0bDNEAMPLLFME\x12\x20\n\x0bKPIEAIKHCHD\x18\x07\x20\x01(\rR\
    \x0bKPIEAIKHCHD\x12\x14\n\x05level\x18\x06\x20\x01(\rR\x05levelb\x06prot\
    o3\
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
            messages.push(PMFIGAGMLOJ::generated_message_descriptor_data());
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
