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

//! Generated file from `NPDIPKHDCNF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:NPDIPKHDCNF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NPDIPKHDCNF {
    // message fields
    // @@protoc_insertion_point(field:NPDIPKHDCNF.INBJPPAPCAG)
    pub INBJPPAPCAG: u32,
    // @@protoc_insertion_point(field:NPDIPKHDCNF.PIPMGACMJNN)
    pub PIPMGACMJNN: bool,
    // @@protoc_insertion_point(field:NPDIPKHDCNF.NHGOJDODGMA)
    pub NHGOJDODGMA: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:NPDIPKHDCNF.IIGOEMFHGLL)
    pub IIGOEMFHGLL: u32,
    // special fields
    // @@protoc_insertion_point(special_field:NPDIPKHDCNF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NPDIPKHDCNF {
    fn default() -> &'a NPDIPKHDCNF {
        <NPDIPKHDCNF as ::protobuf::Message>::default_instance()
    }
}

impl NPDIPKHDCNF {
    pub fn new() -> NPDIPKHDCNF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "INBJPPAPCAG",
            |m: &NPDIPKHDCNF| { &m.INBJPPAPCAG },
            |m: &mut NPDIPKHDCNF| { &mut m.INBJPPAPCAG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PIPMGACMJNN",
            |m: &NPDIPKHDCNF| { &m.PIPMGACMJNN },
            |m: &mut NPDIPKHDCNF| { &mut m.PIPMGACMJNN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NHGOJDODGMA",
            |m: &NPDIPKHDCNF| { &m.NHGOJDODGMA },
            |m: &mut NPDIPKHDCNF| { &mut m.NHGOJDODGMA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IIGOEMFHGLL",
            |m: &NPDIPKHDCNF| { &m.IIGOEMFHGLL },
            |m: &mut NPDIPKHDCNF| { &mut m.IIGOEMFHGLL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NPDIPKHDCNF>(
            "NPDIPKHDCNF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NPDIPKHDCNF {
    const NAME: &'static str = "NPDIPKHDCNF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                80 => {
                    self.INBJPPAPCAG = is.read_uint32()?;
                },
                16 => {
                    self.PIPMGACMJNN = is.read_bool()?;
                },
                122 => {
                    is.read_repeated_packed_uint32_into(&mut self.NHGOJDODGMA)?;
                },
                120 => {
                    self.NHGOJDODGMA.push(is.read_uint32()?);
                },
                64 => {
                    self.IIGOEMFHGLL = is.read_uint32()?;
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
        if self.INBJPPAPCAG != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.INBJPPAPCAG);
        }
        if self.PIPMGACMJNN != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(15, &self.NHGOJDODGMA);
        if self.IIGOEMFHGLL != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.IIGOEMFHGLL);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.INBJPPAPCAG != 0 {
            os.write_uint32(10, self.INBJPPAPCAG)?;
        }
        if self.PIPMGACMJNN != false {
            os.write_bool(2, self.PIPMGACMJNN)?;
        }
        os.write_repeated_packed_uint32(15, &self.NHGOJDODGMA)?;
        if self.IIGOEMFHGLL != 0 {
            os.write_uint32(8, self.IIGOEMFHGLL)?;
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

    fn new() -> NPDIPKHDCNF {
        NPDIPKHDCNF::new()
    }

    fn clear(&mut self) {
        self.INBJPPAPCAG = 0;
        self.PIPMGACMJNN = false;
        self.NHGOJDODGMA.clear();
        self.IIGOEMFHGLL = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NPDIPKHDCNF {
        static instance: NPDIPKHDCNF = NPDIPKHDCNF {
            INBJPPAPCAG: 0,
            PIPMGACMJNN: false,
            NHGOJDODGMA: ::std::vec::Vec::new(),
            IIGOEMFHGLL: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for NPDIPKHDCNF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NPDIPKHDCNF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NPDIPKHDCNF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NPDIPKHDCNF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11NPDIPKHDCNF.proto\"\x95\x01\n\x0bNPDIPKHDCNF\x12\x20\n\x0bINBJPPAP\
    CAG\x18\n\x20\x01(\rR\x0bINBJPPAPCAG\x12\x20\n\x0bPIPMGACMJNN\x18\x02\
    \x20\x01(\x08R\x0bPIPMGACMJNN\x12\x20\n\x0bNHGOJDODGMA\x18\x0f\x20\x03(\
    \rR\x0bNHGOJDODGMA\x12\x20\n\x0bIIGOEMFHGLL\x18\x08\x20\x01(\rR\x0bIIGOE\
    MFHGLLb\x06proto3\
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
            messages.push(NPDIPKHDCNF::generated_message_descriptor_data());
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
