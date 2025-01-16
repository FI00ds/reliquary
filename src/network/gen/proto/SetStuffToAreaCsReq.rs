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

//! Generated file from `SetStuffToAreaCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SetStuffToAreaCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SetStuffToAreaCsReq {
    // message fields
    // @@protoc_insertion_point(field:SetStuffToAreaCsReq.POMEOFHEMGH)
    pub POMEOFHEMGH: u32,
    // @@protoc_insertion_point(field:SetStuffToAreaCsReq.KFAHMGFLAAA)
    pub KFAHMGFLAAA: ::protobuf::EnumOrUnknown<super::MEDPDEALGMJ::MEDPDEALGMJ>,
    // @@protoc_insertion_point(field:SetStuffToAreaCsReq.APBJMEOCBFA)
    pub APBJMEOCBFA: u32,
    // special fields
    // @@protoc_insertion_point(special_field:SetStuffToAreaCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SetStuffToAreaCsReq {
    fn default() -> &'a SetStuffToAreaCsReq {
        <SetStuffToAreaCsReq as ::protobuf::Message>::default_instance()
    }
}

impl SetStuffToAreaCsReq {
    pub fn new() -> SetStuffToAreaCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "POMEOFHEMGH",
            |m: &SetStuffToAreaCsReq| { &m.POMEOFHEMGH },
            |m: &mut SetStuffToAreaCsReq| { &mut m.POMEOFHEMGH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KFAHMGFLAAA",
            |m: &SetStuffToAreaCsReq| { &m.KFAHMGFLAAA },
            |m: &mut SetStuffToAreaCsReq| { &mut m.KFAHMGFLAAA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "APBJMEOCBFA",
            |m: &SetStuffToAreaCsReq| { &m.APBJMEOCBFA },
            |m: &mut SetStuffToAreaCsReq| { &mut m.APBJMEOCBFA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SetStuffToAreaCsReq>(
            "SetStuffToAreaCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SetStuffToAreaCsReq {
    const NAME: &'static str = "SetStuffToAreaCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                96 => {
                    self.POMEOFHEMGH = is.read_uint32()?;
                },
                64 => {
                    self.KFAHMGFLAAA = is.read_enum_or_unknown()?;
                },
                104 => {
                    self.APBJMEOCBFA = is.read_uint32()?;
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
        if self.POMEOFHEMGH != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.POMEOFHEMGH);
        }
        if self.KFAHMGFLAAA != ::protobuf::EnumOrUnknown::new(super::MEDPDEALGMJ::MEDPDEALGMJ::WORK_POS_NONE) {
            my_size += ::protobuf::rt::int32_size(8, self.KFAHMGFLAAA.value());
        }
        if self.APBJMEOCBFA != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.APBJMEOCBFA);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.POMEOFHEMGH != 0 {
            os.write_uint32(12, self.POMEOFHEMGH)?;
        }
        if self.KFAHMGFLAAA != ::protobuf::EnumOrUnknown::new(super::MEDPDEALGMJ::MEDPDEALGMJ::WORK_POS_NONE) {
            os.write_enum(8, ::protobuf::EnumOrUnknown::value(&self.KFAHMGFLAAA))?;
        }
        if self.APBJMEOCBFA != 0 {
            os.write_uint32(13, self.APBJMEOCBFA)?;
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

    fn new() -> SetStuffToAreaCsReq {
        SetStuffToAreaCsReq::new()
    }

    fn clear(&mut self) {
        self.POMEOFHEMGH = 0;
        self.KFAHMGFLAAA = ::protobuf::EnumOrUnknown::new(super::MEDPDEALGMJ::MEDPDEALGMJ::WORK_POS_NONE);
        self.APBJMEOCBFA = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SetStuffToAreaCsReq {
        static instance: SetStuffToAreaCsReq = SetStuffToAreaCsReq {
            POMEOFHEMGH: 0,
            KFAHMGFLAAA: ::protobuf::EnumOrUnknown::from_i32(0),
            APBJMEOCBFA: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SetStuffToAreaCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SetStuffToAreaCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SetStuffToAreaCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetStuffToAreaCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19SetStuffToAreaCsReq.proto\x1a\x11MEDPDEALGMJ.proto\"\x89\x01\n\x13\
    SetStuffToAreaCsReq\x12\x20\n\x0bPOMEOFHEMGH\x18\x0c\x20\x01(\rR\x0bPOME\
    OFHEMGH\x12.\n\x0bKFAHMGFLAAA\x18\x08\x20\x01(\x0e2\x0c.MEDPDEALGMJR\x0b\
    KFAHMGFLAAA\x12\x20\n\x0bAPBJMEOCBFA\x18\r\x20\x01(\rR\x0bAPBJMEOCBFAb\
    \x06proto3\
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
            deps.push(super::MEDPDEALGMJ::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SetStuffToAreaCsReq::generated_message_descriptor_data());
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
