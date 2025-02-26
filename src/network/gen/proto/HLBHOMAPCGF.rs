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

//! Generated file from `HLBHOMAPCGF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:HLBHOMAPCGF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HLBHOMAPCGF {
    // message fields
    // @@protoc_insertion_point(field:HLBHOMAPCGF.ELGANMDPMID)
    pub ELGANMDPMID: u32,
    // @@protoc_insertion_point(field:HLBHOMAPCGF.level)
    pub level: u32,
    // @@protoc_insertion_point(field:HLBHOMAPCGF.promotion)
    pub promotion: u32,
    // @@protoc_insertion_point(field:HLBHOMAPCGF.rank)
    pub rank: u32,
    // special fields
    // @@protoc_insertion_point(special_field:HLBHOMAPCGF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HLBHOMAPCGF {
    fn default() -> &'a HLBHOMAPCGF {
        <HLBHOMAPCGF as ::protobuf::Message>::default_instance()
    }
}

impl HLBHOMAPCGF {
    pub fn new() -> HLBHOMAPCGF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ELGANMDPMID",
            |m: &HLBHOMAPCGF| { &m.ELGANMDPMID },
            |m: &mut HLBHOMAPCGF| { &mut m.ELGANMDPMID },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &HLBHOMAPCGF| { &m.level },
            |m: &mut HLBHOMAPCGF| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "promotion",
            |m: &HLBHOMAPCGF| { &m.promotion },
            |m: &mut HLBHOMAPCGF| { &mut m.promotion },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "rank",
            |m: &HLBHOMAPCGF| { &m.rank },
            |m: &mut HLBHOMAPCGF| { &mut m.rank },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HLBHOMAPCGF>(
            "HLBHOMAPCGF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HLBHOMAPCGF {
    const NAME: &'static str = "HLBHOMAPCGF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.ELGANMDPMID = is.read_uint32()?;
                },
                16 => {
                    self.level = is.read_uint32()?;
                },
                24 => {
                    self.promotion = is.read_uint32()?;
                },
                32 => {
                    self.rank = is.read_uint32()?;
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
        if self.ELGANMDPMID != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.ELGANMDPMID);
        }
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.level);
        }
        if self.promotion != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.promotion);
        }
        if self.rank != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.rank);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.ELGANMDPMID != 0 {
            os.write_uint32(1, self.ELGANMDPMID)?;
        }
        if self.level != 0 {
            os.write_uint32(2, self.level)?;
        }
        if self.promotion != 0 {
            os.write_uint32(3, self.promotion)?;
        }
        if self.rank != 0 {
            os.write_uint32(4, self.rank)?;
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

    fn new() -> HLBHOMAPCGF {
        HLBHOMAPCGF::new()
    }

    fn clear(&mut self) {
        self.ELGANMDPMID = 0;
        self.level = 0;
        self.promotion = 0;
        self.rank = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HLBHOMAPCGF {
        static instance: HLBHOMAPCGF = HLBHOMAPCGF {
            ELGANMDPMID: 0,
            level: 0,
            promotion: 0,
            rank: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HLBHOMAPCGF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HLBHOMAPCGF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HLBHOMAPCGF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HLBHOMAPCGF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11HLBHOMAPCGF.proto\"w\n\x0bHLBHOMAPCGF\x12\x20\n\x0bELGANMDPMID\x18\
    \x01\x20\x01(\rR\x0bELGANMDPMID\x12\x14\n\x05level\x18\x02\x20\x01(\rR\
    \x05level\x12\x1c\n\tpromotion\x18\x03\x20\x01(\rR\tpromotion\x12\x12\n\
    \x04rank\x18\x04\x20\x01(\rR\x04rankb\x06proto3\
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
            messages.push(HLBHOMAPCGF::generated_message_descriptor_data());
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
