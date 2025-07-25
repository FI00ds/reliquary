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

//! Generated file from `HMOPIBLFCLN.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:HMOPIBLFCLN)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HMOPIBLFCLN {
    // message fields
    // @@protoc_insertion_point(field:HMOPIBLFCLN.extra_ratio)
    pub extra_ratio: u32,
    // @@protoc_insertion_point(field:HMOPIBLFCLN.has_modify_all_passenger_stat_effect)
    pub has_modify_all_passenger_stat_effect: bool,
    // @@protoc_insertion_point(field:HMOPIBLFCLN.LNMMKFMEAJM)
    pub LNMMKFMEAJM: ::protobuf::MessageField<super::TrainPartyCards::TrainPartyCards>,
    // @@protoc_insertion_point(field:HMOPIBLFCLN.meeting_count_info)
    pub meeting_count_info: ::protobuf::MessageField<super::TrainPartyMeetingCountInfo::TrainPartyMeetingCountInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:HMOPIBLFCLN.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HMOPIBLFCLN {
    fn default() -> &'a HMOPIBLFCLN {
        <HMOPIBLFCLN as ::protobuf::Message>::default_instance()
    }
}

impl HMOPIBLFCLN {
    pub fn new() -> HMOPIBLFCLN {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "extra_ratio",
            |m: &HMOPIBLFCLN| { &m.extra_ratio },
            |m: &mut HMOPIBLFCLN| { &mut m.extra_ratio },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "has_modify_all_passenger_stat_effect",
            |m: &HMOPIBLFCLN| { &m.has_modify_all_passenger_stat_effect },
            |m: &mut HMOPIBLFCLN| { &mut m.has_modify_all_passenger_stat_effect },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::TrainPartyCards::TrainPartyCards>(
            "LNMMKFMEAJM",
            |m: &HMOPIBLFCLN| { &m.LNMMKFMEAJM },
            |m: &mut HMOPIBLFCLN| { &mut m.LNMMKFMEAJM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::TrainPartyMeetingCountInfo::TrainPartyMeetingCountInfo>(
            "meeting_count_info",
            |m: &HMOPIBLFCLN| { &m.meeting_count_info },
            |m: &mut HMOPIBLFCLN| { &mut m.meeting_count_info },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HMOPIBLFCLN>(
            "HMOPIBLFCLN",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HMOPIBLFCLN {
    const NAME: &'static str = "HMOPIBLFCLN";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.extra_ratio = is.read_uint32()?;
                },
                32 => {
                    self.has_modify_all_passenger_stat_effect = is.read_bool()?;
                },
                98 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LNMMKFMEAJM)?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.meeting_count_info)?;
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
        if self.extra_ratio != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.extra_ratio);
        }
        if self.has_modify_all_passenger_stat_effect != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.LNMMKFMEAJM.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.meeting_count_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.extra_ratio != 0 {
            os.write_uint32(13, self.extra_ratio)?;
        }
        if self.has_modify_all_passenger_stat_effect != false {
            os.write_bool(4, self.has_modify_all_passenger_stat_effect)?;
        }
        if let Some(v) = self.LNMMKFMEAJM.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        }
        if let Some(v) = self.meeting_count_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
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

    fn new() -> HMOPIBLFCLN {
        HMOPIBLFCLN::new()
    }

    fn clear(&mut self) {
        self.extra_ratio = 0;
        self.has_modify_all_passenger_stat_effect = false;
        self.LNMMKFMEAJM.clear();
        self.meeting_count_info.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HMOPIBLFCLN {
        static instance: HMOPIBLFCLN = HMOPIBLFCLN {
            extra_ratio: 0,
            has_modify_all_passenger_stat_effect: false,
            LNMMKFMEAJM: ::protobuf::MessageField::none(),
            meeting_count_info: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HMOPIBLFCLN {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HMOPIBLFCLN").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HMOPIBLFCLN {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HMOPIBLFCLN {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11HMOPIBLFCLN.proto\x1a\x15TrainPartyCards.proto\x1a\x20TrainPartyMe\
    etingCountInfo.proto\"\xfc\x01\n\x0bHMOPIBLFCLN\x12\x1f\n\x0bextra_ratio\
    \x18\r\x20\x01(\rR\nextraRatio\x12M\n$has_modify_all_passenger_stat_effe\
    ct\x18\x04\x20\x01(\x08R\x1fhasModifyAllPassengerStatEffect\x122\n\x0bLN\
    MMKFMEAJM\x18\x0c\x20\x01(\x0b2\x10.TrainPartyCardsR\x0bLNMMKFMEAJM\x12I\
    \n\x12meeting_count_info\x18\x03\x20\x01(\x0b2\x1b.TrainPartyMeetingCoun\
    tInfoR\x10meetingCountInfob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::TrainPartyCards::file_descriptor().clone());
            deps.push(super::TrainPartyMeetingCountInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(HMOPIBLFCLN::generated_message_descriptor_data());
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
