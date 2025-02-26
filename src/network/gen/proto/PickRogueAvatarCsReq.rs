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

//! Generated file from `PickRogueAvatarCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:PickRogueAvatarCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PickRogueAvatarCsReq {
    // message fields
    // @@protoc_insertion_point(field:PickRogueAvatarCsReq.LMPHJMDONMH)
    pub LMPHJMDONMH: u32,
    // @@protoc_insertion_point(field:PickRogueAvatarCsReq.NBCGLEFOKDM)
    pub NBCGLEFOKDM: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:PickRogueAvatarCsReq.LGHCAHBBCAM)
    pub LGHCAHBBCAM: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:PickRogueAvatarCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PickRogueAvatarCsReq {
    fn default() -> &'a PickRogueAvatarCsReq {
        <PickRogueAvatarCsReq as ::protobuf::Message>::default_instance()
    }
}

impl PickRogueAvatarCsReq {
    pub fn new() -> PickRogueAvatarCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LMPHJMDONMH",
            |m: &PickRogueAvatarCsReq| { &m.LMPHJMDONMH },
            |m: &mut PickRogueAvatarCsReq| { &mut m.LMPHJMDONMH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NBCGLEFOKDM",
            |m: &PickRogueAvatarCsReq| { &m.NBCGLEFOKDM },
            |m: &mut PickRogueAvatarCsReq| { &mut m.NBCGLEFOKDM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LGHCAHBBCAM",
            |m: &PickRogueAvatarCsReq| { &m.LGHCAHBBCAM },
            |m: &mut PickRogueAvatarCsReq| { &mut m.LGHCAHBBCAM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PickRogueAvatarCsReq>(
            "PickRogueAvatarCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PickRogueAvatarCsReq {
    const NAME: &'static str = "PickRogueAvatarCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.LMPHJMDONMH = is.read_uint32()?;
                },
                74 => {
                    is.read_repeated_packed_uint32_into(&mut self.NBCGLEFOKDM)?;
                },
                72 => {
                    self.NBCGLEFOKDM.push(is.read_uint32()?);
                },
                106 => {
                    is.read_repeated_packed_uint32_into(&mut self.LGHCAHBBCAM)?;
                },
                104 => {
                    self.LGHCAHBBCAM.push(is.read_uint32()?);
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
        if self.LMPHJMDONMH != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.LMPHJMDONMH);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(9, &self.NBCGLEFOKDM);
        my_size += ::protobuf::rt::vec_packed_uint32_size(13, &self.LGHCAHBBCAM);
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.LMPHJMDONMH != 0 {
            os.write_uint32(3, self.LMPHJMDONMH)?;
        }
        os.write_repeated_packed_uint32(9, &self.NBCGLEFOKDM)?;
        os.write_repeated_packed_uint32(13, &self.LGHCAHBBCAM)?;
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> PickRogueAvatarCsReq {
        PickRogueAvatarCsReq::new()
    }

    fn clear(&mut self) {
        self.LMPHJMDONMH = 0;
        self.NBCGLEFOKDM.clear();
        self.LGHCAHBBCAM.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PickRogueAvatarCsReq {
        static instance: PickRogueAvatarCsReq = PickRogueAvatarCsReq {
            LMPHJMDONMH: 0,
            NBCGLEFOKDM: ::std::vec::Vec::new(),
            LGHCAHBBCAM: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PickRogueAvatarCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PickRogueAvatarCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PickRogueAvatarCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PickRogueAvatarCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aPickRogueAvatarCsReq.proto\"|\n\x14PickRogueAvatarCsReq\x12\x20\n\
    \x0bLMPHJMDONMH\x18\x03\x20\x01(\rR\x0bLMPHJMDONMH\x12\x20\n\x0bNBCGLEFO\
    KDM\x18\t\x20\x03(\rR\x0bNBCGLEFOKDM\x12\x20\n\x0bLGHCAHBBCAM\x18\r\x20\
    \x03(\rR\x0bLGHCAHBBCAMb\x06proto3\
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
            messages.push(PickRogueAvatarCsReq::generated_message_descriptor_data());
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
