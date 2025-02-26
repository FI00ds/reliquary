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

//! Generated file from `HOCHOIHKKDG.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:HOCHOIHKKDG)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HOCHOIHKKDG {
    // message fields
    // @@protoc_insertion_point(field:HOCHOIHKKDG.DLPPDPBJIIM)
    pub DLPPDPBJIIM: u32,
    // @@protoc_insertion_point(field:HOCHOIHKKDG.MDDOFMCJJHH)
    pub MDDOFMCJJHH: ::protobuf::EnumOrUnknown<super::FOCHDFJANPC::FOCHDFJANPC>,
    // @@protoc_insertion_point(field:HOCHOIHKKDG.HFALJIHKECN)
    pub HFALJIHKECN: u32,
    // special fields
    // @@protoc_insertion_point(special_field:HOCHOIHKKDG.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HOCHOIHKKDG {
    fn default() -> &'a HOCHOIHKKDG {
        <HOCHOIHKKDG as ::protobuf::Message>::default_instance()
    }
}

impl HOCHOIHKKDG {
    pub fn new() -> HOCHOIHKKDG {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DLPPDPBJIIM",
            |m: &HOCHOIHKKDG| { &m.DLPPDPBJIIM },
            |m: &mut HOCHOIHKKDG| { &mut m.DLPPDPBJIIM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MDDOFMCJJHH",
            |m: &HOCHOIHKKDG| { &m.MDDOFMCJJHH },
            |m: &mut HOCHOIHKKDG| { &mut m.MDDOFMCJJHH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HFALJIHKECN",
            |m: &HOCHOIHKKDG| { &m.HFALJIHKECN },
            |m: &mut HOCHOIHKKDG| { &mut m.HFALJIHKECN },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HOCHOIHKKDG>(
            "HOCHOIHKKDG",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HOCHOIHKKDG {
    const NAME: &'static str = "HOCHOIHKKDG";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.DLPPDPBJIIM = is.read_uint32()?;
                },
                32 => {
                    self.MDDOFMCJJHH = is.read_enum_or_unknown()?;
                },
                24 => {
                    self.HFALJIHKECN = is.read_uint32()?;
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
        if self.DLPPDPBJIIM != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.DLPPDPBJIIM);
        }
        if self.MDDOFMCJJHH != ::protobuf::EnumOrUnknown::new(super::FOCHDFJANPC::FOCHDFJANPC::RAID_TARGET_STATUS_NONE) {
            my_size += ::protobuf::rt::int32_size(4, self.MDDOFMCJJHH.value());
        }
        if self.HFALJIHKECN != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.HFALJIHKECN);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.DLPPDPBJIIM != 0 {
            os.write_uint32(11, self.DLPPDPBJIIM)?;
        }
        if self.MDDOFMCJJHH != ::protobuf::EnumOrUnknown::new(super::FOCHDFJANPC::FOCHDFJANPC::RAID_TARGET_STATUS_NONE) {
            os.write_enum(4, ::protobuf::EnumOrUnknown::value(&self.MDDOFMCJJHH))?;
        }
        if self.HFALJIHKECN != 0 {
            os.write_uint32(3, self.HFALJIHKECN)?;
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

    fn new() -> HOCHOIHKKDG {
        HOCHOIHKKDG::new()
    }

    fn clear(&mut self) {
        self.DLPPDPBJIIM = 0;
        self.MDDOFMCJJHH = ::protobuf::EnumOrUnknown::new(super::FOCHDFJANPC::FOCHDFJANPC::RAID_TARGET_STATUS_NONE);
        self.HFALJIHKECN = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HOCHOIHKKDG {
        static instance: HOCHOIHKKDG = HOCHOIHKKDG {
            DLPPDPBJIIM: 0,
            MDDOFMCJJHH: ::protobuf::EnumOrUnknown::from_i32(0),
            HFALJIHKECN: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HOCHOIHKKDG {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HOCHOIHKKDG").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HOCHOIHKKDG {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HOCHOIHKKDG {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11HOCHOIHKKDG.proto\x1a\x11FOCHDFJANPC.proto\"\x81\x01\n\x0bHOCHOIHK\
    KDG\x12\x20\n\x0bDLPPDPBJIIM\x18\x0b\x20\x01(\rR\x0bDLPPDPBJIIM\x12.\n\
    \x0bMDDOFMCJJHH\x18\x04\x20\x01(\x0e2\x0c.FOCHDFJANPCR\x0bMDDOFMCJJHH\
    \x12\x20\n\x0bHFALJIHKECN\x18\x03\x20\x01(\rR\x0bHFALJIHKECNb\x06proto3\
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
            deps.push(super::FOCHDFJANPC::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(HOCHOIHKKDG::generated_message_descriptor_data());
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
