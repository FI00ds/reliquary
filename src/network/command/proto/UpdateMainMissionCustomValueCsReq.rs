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

//! Generated file from `UpdateMainMissionCustomValueCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:UpdateMainMissionCustomValueCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct UpdateMainMissionCustomValueCsReq {
    // message fields
    // @@protoc_insertion_point(field:UpdateMainMissionCustomValueCsReq.DFDEKANJBLG)
    pub DFDEKANJBLG: ::std::string::String,
    // @@protoc_insertion_point(field:UpdateMainMissionCustomValueCsReq.main_mission_id)
    pub main_mission_id: u32,
    // @@protoc_insertion_point(field:UpdateMainMissionCustomValueCsReq.value)
    pub value: u32,
    // special fields
    // @@protoc_insertion_point(special_field:UpdateMainMissionCustomValueCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a UpdateMainMissionCustomValueCsReq {
    fn default() -> &'a UpdateMainMissionCustomValueCsReq {
        <UpdateMainMissionCustomValueCsReq as ::protobuf::Message>::default_instance()
    }
}

impl UpdateMainMissionCustomValueCsReq {
    pub fn new() -> UpdateMainMissionCustomValueCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DFDEKANJBLG",
            |m: &UpdateMainMissionCustomValueCsReq| { &m.DFDEKANJBLG },
            |m: &mut UpdateMainMissionCustomValueCsReq| { &mut m.DFDEKANJBLG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "main_mission_id",
            |m: &UpdateMainMissionCustomValueCsReq| { &m.main_mission_id },
            |m: &mut UpdateMainMissionCustomValueCsReq| { &mut m.main_mission_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "value",
            |m: &UpdateMainMissionCustomValueCsReq| { &m.value },
            |m: &mut UpdateMainMissionCustomValueCsReq| { &mut m.value },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<UpdateMainMissionCustomValueCsReq>(
            "UpdateMainMissionCustomValueCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for UpdateMainMissionCustomValueCsReq {
    const NAME: &'static str = "UpdateMainMissionCustomValueCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                18 => {
                    self.DFDEKANJBLG = is.read_string()?;
                },
                120 => {
                    self.main_mission_id = is.read_uint32()?;
                },
                88 => {
                    self.value = is.read_uint32()?;
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
        if !self.DFDEKANJBLG.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.DFDEKANJBLG);
        }
        if self.main_mission_id != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.main_mission_id);
        }
        if self.value != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.DFDEKANJBLG.is_empty() {
            os.write_string(2, &self.DFDEKANJBLG)?;
        }
        if self.main_mission_id != 0 {
            os.write_uint32(15, self.main_mission_id)?;
        }
        if self.value != 0 {
            os.write_uint32(11, self.value)?;
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

    fn new() -> UpdateMainMissionCustomValueCsReq {
        UpdateMainMissionCustomValueCsReq::new()
    }

    fn clear(&mut self) {
        self.DFDEKANJBLG.clear();
        self.main_mission_id = 0;
        self.value = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static UpdateMainMissionCustomValueCsReq {
        static instance: UpdateMainMissionCustomValueCsReq = UpdateMainMissionCustomValueCsReq {
            DFDEKANJBLG: ::std::string::String::new(),
            main_mission_id: 0,
            value: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for UpdateMainMissionCustomValueCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("UpdateMainMissionCustomValueCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for UpdateMainMissionCustomValueCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpdateMainMissionCustomValueCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'UpdateMainMissionCustomValueCsReq.proto\"\x83\x01\n!UpdateMainMission\
    CustomValueCsReq\x12\x20\n\x0bDFDEKANJBLG\x18\x02\x20\x01(\tR\x0bDFDEKAN\
    JBLG\x12&\n\x0fmain_mission_id\x18\x0f\x20\x01(\rR\rmainMissionId\x12\
    \x14\n\x05value\x18\x0b\x20\x01(\rR\x05valueb\x06proto3\
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
            messages.push(UpdateMainMissionCustomValueCsReq::generated_message_descriptor_data());
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
