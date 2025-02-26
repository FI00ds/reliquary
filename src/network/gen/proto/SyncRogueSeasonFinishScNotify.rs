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

//! Generated file from `SyncRogueSeasonFinishScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:SyncRogueSeasonFinishScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SyncRogueSeasonFinishScNotify {
    // message fields
    // @@protoc_insertion_point(field:SyncRogueSeasonFinishScNotify.IDOMKBKKKKL)
    pub IDOMKBKKKKL: ::protobuf::MessageField<super::PPJCDCOAFDK::PPJCDCOAFDK>,
    // @@protoc_insertion_point(field:SyncRogueSeasonFinishScNotify.NIOLDFFFELN)
    pub NIOLDFFFELN: bool,
    // @@protoc_insertion_point(field:SyncRogueSeasonFinishScNotify.PFEANIAHFPC)
    pub PFEANIAHFPC: ::protobuf::MessageField<super::HGHCOGEPIDM::HGHCOGEPIDM>,
    // @@protoc_insertion_point(field:SyncRogueSeasonFinishScNotify.CPBNMACLBEH)
    pub CPBNMACLBEH: ::protobuf::MessageField<super::EIMJEAMDFKJ::EIMJEAMDFKJ>,
    // @@protoc_insertion_point(field:SyncRogueSeasonFinishScNotify.JIPPGKHKKBG)
    pub JIPPGKHKKBG: ::protobuf::MessageField<super::IMCCCCICCKO::IMCCCCICCKO>,
    // special fields
    // @@protoc_insertion_point(special_field:SyncRogueSeasonFinishScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SyncRogueSeasonFinishScNotify {
    fn default() -> &'a SyncRogueSeasonFinishScNotify {
        <SyncRogueSeasonFinishScNotify as ::protobuf::Message>::default_instance()
    }
}

impl SyncRogueSeasonFinishScNotify {
    pub fn new() -> SyncRogueSeasonFinishScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PPJCDCOAFDK::PPJCDCOAFDK>(
            "IDOMKBKKKKL",
            |m: &SyncRogueSeasonFinishScNotify| { &m.IDOMKBKKKKL },
            |m: &mut SyncRogueSeasonFinishScNotify| { &mut m.IDOMKBKKKKL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NIOLDFFFELN",
            |m: &SyncRogueSeasonFinishScNotify| { &m.NIOLDFFFELN },
            |m: &mut SyncRogueSeasonFinishScNotify| { &mut m.NIOLDFFFELN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::HGHCOGEPIDM::HGHCOGEPIDM>(
            "PFEANIAHFPC",
            |m: &SyncRogueSeasonFinishScNotify| { &m.PFEANIAHFPC },
            |m: &mut SyncRogueSeasonFinishScNotify| { &mut m.PFEANIAHFPC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::EIMJEAMDFKJ::EIMJEAMDFKJ>(
            "CPBNMACLBEH",
            |m: &SyncRogueSeasonFinishScNotify| { &m.CPBNMACLBEH },
            |m: &mut SyncRogueSeasonFinishScNotify| { &mut m.CPBNMACLBEH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::IMCCCCICCKO::IMCCCCICCKO>(
            "JIPPGKHKKBG",
            |m: &SyncRogueSeasonFinishScNotify| { &m.JIPPGKHKKBG },
            |m: &mut SyncRogueSeasonFinishScNotify| { &mut m.JIPPGKHKKBG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SyncRogueSeasonFinishScNotify>(
            "SyncRogueSeasonFinishScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SyncRogueSeasonFinishScNotify {
    const NAME: &'static str = "SyncRogueSeasonFinishScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.IDOMKBKKKKL)?;
                },
                96 => {
                    self.NIOLDFFFELN = is.read_bool()?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.PFEANIAHFPC)?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.CPBNMACLBEH)?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.JIPPGKHKKBG)?;
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
        if let Some(v) = self.IDOMKBKKKKL.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.NIOLDFFFELN != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.PFEANIAHFPC.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.CPBNMACLBEH.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.JIPPGKHKKBG.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.IDOMKBKKKKL.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        if self.NIOLDFFFELN != false {
            os.write_bool(12, self.NIOLDFFFELN)?;
        }
        if let Some(v) = self.PFEANIAHFPC.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if let Some(v) = self.CPBNMACLBEH.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        if let Some(v) = self.JIPPGKHKKBG.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
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

    fn new() -> SyncRogueSeasonFinishScNotify {
        SyncRogueSeasonFinishScNotify::new()
    }

    fn clear(&mut self) {
        self.IDOMKBKKKKL.clear();
        self.NIOLDFFFELN = false;
        self.PFEANIAHFPC.clear();
        self.CPBNMACLBEH.clear();
        self.JIPPGKHKKBG.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SyncRogueSeasonFinishScNotify {
        static instance: SyncRogueSeasonFinishScNotify = SyncRogueSeasonFinishScNotify {
            IDOMKBKKKKL: ::protobuf::MessageField::none(),
            NIOLDFFFELN: false,
            PFEANIAHFPC: ::protobuf::MessageField::none(),
            CPBNMACLBEH: ::protobuf::MessageField::none(),
            JIPPGKHKKBG: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SyncRogueSeasonFinishScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SyncRogueSeasonFinishScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SyncRogueSeasonFinishScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SyncRogueSeasonFinishScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n#SyncRogueSeasonFinishScNotify.proto\x1a\x11EIMJEAMDFKJ.proto\x1a\x11H\
    GHCOGEPIDM.proto\x1a\x11IMCCCCICCKO.proto\x1a\x11PPJCDCOAFDK.proto\"\x81\
    \x02\n\x1dSyncRogueSeasonFinishScNotify\x12.\n\x0bIDOMKBKKKKL\x18\x06\
    \x20\x01(\x0b2\x0c.PPJCDCOAFDKR\x0bIDOMKBKKKKL\x12\x20\n\x0bNIOLDFFFELN\
    \x18\x0c\x20\x01(\x08R\x0bNIOLDFFFELN\x12.\n\x0bPFEANIAHFPC\x18\x0f\x20\
    \x01(\x0b2\x0c.HGHCOGEPIDMR\x0bPFEANIAHFPC\x12.\n\x0bCPBNMACLBEH\x18\x0e\
    \x20\x01(\x0b2\x0c.EIMJEAMDFKJR\x0bCPBNMACLBEH\x12.\n\x0bJIPPGKHKKBG\x18\
    \x0b\x20\x01(\x0b2\x0c.IMCCCCICCKOR\x0bJIPPGKHKKBGb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::EIMJEAMDFKJ::file_descriptor().clone());
            deps.push(super::HGHCOGEPIDM::file_descriptor().clone());
            deps.push(super::IMCCCCICCKO::file_descriptor().clone());
            deps.push(super::PPJCDCOAFDK::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SyncRogueSeasonFinishScNotify::generated_message_descriptor_data());
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
