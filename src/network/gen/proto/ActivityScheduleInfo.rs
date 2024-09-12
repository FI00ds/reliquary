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

//! Generated file from `ActivityScheduleInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ActivityScheduleInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ActivityScheduleInfo {
    // message fields
    // @@protoc_insertion_point(field:ActivityScheduleInfo.end_time)
    pub end_time: i64,
    // @@protoc_insertion_point(field:ActivityScheduleInfo.activity_id)
    pub activity_id: u32,
    // @@protoc_insertion_point(field:ActivityScheduleInfo.module_id)
    pub module_id: u32,
    // @@protoc_insertion_point(field:ActivityScheduleInfo.begin_time)
    pub begin_time: i64,
    // special fields
    // @@protoc_insertion_point(special_field:ActivityScheduleInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ActivityScheduleInfo {
    fn default() -> &'a ActivityScheduleInfo {
        <ActivityScheduleInfo as ::protobuf::Message>::default_instance()
    }
}

impl ActivityScheduleInfo {
    pub fn new() -> ActivityScheduleInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "end_time",
            |m: &ActivityScheduleInfo| { &m.end_time },
            |m: &mut ActivityScheduleInfo| { &mut m.end_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "activity_id",
            |m: &ActivityScheduleInfo| { &m.activity_id },
            |m: &mut ActivityScheduleInfo| { &mut m.activity_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "module_id",
            |m: &ActivityScheduleInfo| { &m.module_id },
            |m: &mut ActivityScheduleInfo| { &mut m.module_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "begin_time",
            |m: &ActivityScheduleInfo| { &m.begin_time },
            |m: &mut ActivityScheduleInfo| { &mut m.begin_time },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ActivityScheduleInfo>(
            "ActivityScheduleInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ActivityScheduleInfo {
    const NAME: &'static str = "ActivityScheduleInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.end_time = is.read_int64()?;
                },
                88 => {
                    self.activity_id = is.read_uint32()?;
                },
                96 => {
                    self.module_id = is.read_uint32()?;
                },
                40 => {
                    self.begin_time = is.read_int64()?;
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
        if self.end_time != 0 {
            my_size += ::protobuf::rt::int64_size(9, self.end_time);
        }
        if self.activity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.activity_id);
        }
        if self.module_id != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.module_id);
        }
        if self.begin_time != 0 {
            my_size += ::protobuf::rt::int64_size(5, self.begin_time);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.end_time != 0 {
            os.write_int64(9, self.end_time)?;
        }
        if self.activity_id != 0 {
            os.write_uint32(11, self.activity_id)?;
        }
        if self.module_id != 0 {
            os.write_uint32(12, self.module_id)?;
        }
        if self.begin_time != 0 {
            os.write_int64(5, self.begin_time)?;
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

    fn new() -> ActivityScheduleInfo {
        ActivityScheduleInfo::new()
    }

    fn clear(&mut self) {
        self.end_time = 0;
        self.activity_id = 0;
        self.module_id = 0;
        self.begin_time = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ActivityScheduleInfo {
        static instance: ActivityScheduleInfo = ActivityScheduleInfo {
            end_time: 0,
            activity_id: 0,
            module_id: 0,
            begin_time: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ActivityScheduleInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ActivityScheduleInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ActivityScheduleInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ActivityScheduleInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aActivityScheduleInfo.proto\"\x8e\x01\n\x14ActivityScheduleInfo\x12\
    \x19\n\x08end_time\x18\t\x20\x01(\x03R\x07endTime\x12\x1f\n\x0bactivity_\
    id\x18\x0b\x20\x01(\rR\nactivityId\x12\x1b\n\tmodule_id\x18\x0c\x20\x01(\
    \rR\x08moduleId\x12\x1d\n\nbegin_time\x18\x05\x20\x01(\x03R\tbeginTimeB\
    \x15\n\x13emu.lunarcore.protob\x06proto3\
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
            messages.push(ActivityScheduleInfo::generated_message_descriptor_data());
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
