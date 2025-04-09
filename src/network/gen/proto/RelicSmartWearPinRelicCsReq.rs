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

//! Generated file from `RelicSmartWearPinRelicCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:RelicSmartWearPinRelicCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RelicSmartWearPinRelicCsReq {
    // message fields
    // @@protoc_insertion_point(field:RelicSmartWearPinRelicCsReq.avatar_id)
    pub avatar_id: u32,
    // @@protoc_insertion_point(field:RelicSmartWearPinRelicCsReq.GEEKCAGBGMN)
    pub GEEKCAGBGMN: u32,
    // @@protoc_insertion_point(field:RelicSmartWearPinRelicCsReq.BAOONJDCFKD)
    pub BAOONJDCFKD: bool,
    // special fields
    // @@protoc_insertion_point(special_field:RelicSmartWearPinRelicCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RelicSmartWearPinRelicCsReq {
    fn default() -> &'a RelicSmartWearPinRelicCsReq {
        <RelicSmartWearPinRelicCsReq as ::protobuf::Message>::default_instance()
    }
}

impl RelicSmartWearPinRelicCsReq {
    pub fn new() -> RelicSmartWearPinRelicCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_id",
            |m: &RelicSmartWearPinRelicCsReq| { &m.avatar_id },
            |m: &mut RelicSmartWearPinRelicCsReq| { &mut m.avatar_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GEEKCAGBGMN",
            |m: &RelicSmartWearPinRelicCsReq| { &m.GEEKCAGBGMN },
            |m: &mut RelicSmartWearPinRelicCsReq| { &mut m.GEEKCAGBGMN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BAOONJDCFKD",
            |m: &RelicSmartWearPinRelicCsReq| { &m.BAOONJDCFKD },
            |m: &mut RelicSmartWearPinRelicCsReq| { &mut m.BAOONJDCFKD },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RelicSmartWearPinRelicCsReq>(
            "RelicSmartWearPinRelicCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RelicSmartWearPinRelicCsReq {
    const NAME: &'static str = "RelicSmartWearPinRelicCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.avatar_id = is.read_uint32()?;
                },
                72 => {
                    self.GEEKCAGBGMN = is.read_uint32()?;
                },
                104 => {
                    self.BAOONJDCFKD = is.read_bool()?;
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
        if self.avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.avatar_id);
        }
        if self.GEEKCAGBGMN != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.GEEKCAGBGMN);
        }
        if self.BAOONJDCFKD != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.avatar_id != 0 {
            os.write_uint32(11, self.avatar_id)?;
        }
        if self.GEEKCAGBGMN != 0 {
            os.write_uint32(9, self.GEEKCAGBGMN)?;
        }
        if self.BAOONJDCFKD != false {
            os.write_bool(13, self.BAOONJDCFKD)?;
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

    fn new() -> RelicSmartWearPinRelicCsReq {
        RelicSmartWearPinRelicCsReq::new()
    }

    fn clear(&mut self) {
        self.avatar_id = 0;
        self.GEEKCAGBGMN = 0;
        self.BAOONJDCFKD = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RelicSmartWearPinRelicCsReq {
        static instance: RelicSmartWearPinRelicCsReq = RelicSmartWearPinRelicCsReq {
            avatar_id: 0,
            GEEKCAGBGMN: 0,
            BAOONJDCFKD: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RelicSmartWearPinRelicCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RelicSmartWearPinRelicCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RelicSmartWearPinRelicCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RelicSmartWearPinRelicCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n!RelicSmartWearPinRelicCsReq.proto\"~\n\x1bRelicSmartWearPinRelicCsReq\
    \x12\x1b\n\tavatar_id\x18\x0b\x20\x01(\rR\x08avatarId\x12\x20\n\x0bGEEKC\
    AGBGMN\x18\t\x20\x01(\rR\x0bGEEKCAGBGMN\x12\x20\n\x0bBAOONJDCFKD\x18\r\
    \x20\x01(\x08R\x0bBAOONJDCFKDb\x06proto3\
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
            messages.push(RelicSmartWearPinRelicCsReq::generated_message_descriptor_data());
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
