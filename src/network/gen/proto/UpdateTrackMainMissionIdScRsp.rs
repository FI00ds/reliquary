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

//! Generated file from `UpdateTrackMainMissionIdScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:UpdateTrackMainMissionIdScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct UpdateTrackMainMissionIdScRsp {
    // message fields
    // @@protoc_insertion_point(field:UpdateTrackMainMissionIdScRsp.LFLBIOPJFGE)
    pub LFLBIOPJFGE: u32,
    // @@protoc_insertion_point(field:UpdateTrackMainMissionIdScRsp.DKMBBNLMNNE)
    pub DKMBBNLMNNE: u32,
    // @@protoc_insertion_point(field:UpdateTrackMainMissionIdScRsp.retcode)
    pub retcode: u32,
    // special fields
    // @@protoc_insertion_point(special_field:UpdateTrackMainMissionIdScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a UpdateTrackMainMissionIdScRsp {
    fn default() -> &'a UpdateTrackMainMissionIdScRsp {
        <UpdateTrackMainMissionIdScRsp as ::protobuf::Message>::default_instance()
    }
}

impl UpdateTrackMainMissionIdScRsp {
    pub fn new() -> UpdateTrackMainMissionIdScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LFLBIOPJFGE",
            |m: &UpdateTrackMainMissionIdScRsp| { &m.LFLBIOPJFGE },
            |m: &mut UpdateTrackMainMissionIdScRsp| { &mut m.LFLBIOPJFGE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DKMBBNLMNNE",
            |m: &UpdateTrackMainMissionIdScRsp| { &m.DKMBBNLMNNE },
            |m: &mut UpdateTrackMainMissionIdScRsp| { &mut m.DKMBBNLMNNE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &UpdateTrackMainMissionIdScRsp| { &m.retcode },
            |m: &mut UpdateTrackMainMissionIdScRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<UpdateTrackMainMissionIdScRsp>(
            "UpdateTrackMainMissionIdScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for UpdateTrackMainMissionIdScRsp {
    const NAME: &'static str = "UpdateTrackMainMissionIdScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                40 => {
                    self.LFLBIOPJFGE = is.read_uint32()?;
                },
                16 => {
                    self.DKMBBNLMNNE = is.read_uint32()?;
                },
                64 => {
                    self.retcode = is.read_uint32()?;
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
        if self.LFLBIOPJFGE != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.LFLBIOPJFGE);
        }
        if self.DKMBBNLMNNE != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.DKMBBNLMNNE);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.LFLBIOPJFGE != 0 {
            os.write_uint32(5, self.LFLBIOPJFGE)?;
        }
        if self.DKMBBNLMNNE != 0 {
            os.write_uint32(2, self.DKMBBNLMNNE)?;
        }
        if self.retcode != 0 {
            os.write_uint32(8, self.retcode)?;
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

    fn new() -> UpdateTrackMainMissionIdScRsp {
        UpdateTrackMainMissionIdScRsp::new()
    }

    fn clear(&mut self) {
        self.LFLBIOPJFGE = 0;
        self.DKMBBNLMNNE = 0;
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static UpdateTrackMainMissionIdScRsp {
        static instance: UpdateTrackMainMissionIdScRsp = UpdateTrackMainMissionIdScRsp {
            LFLBIOPJFGE: 0,
            DKMBBNLMNNE: 0,
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for UpdateTrackMainMissionIdScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("UpdateTrackMainMissionIdScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for UpdateTrackMainMissionIdScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpdateTrackMainMissionIdScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n#UpdateTrackMainMissionIdScRsp.proto\"}\n\x1dUpdateTrackMainMissionIdS\
    cRsp\x12\x20\n\x0bLFLBIOPJFGE\x18\x05\x20\x01(\rR\x0bLFLBIOPJFGE\x12\x20\
    \n\x0bDKMBBNLMNNE\x18\x02\x20\x01(\rR\x0bDKMBBNLMNNE\x12\x18\n\x07retcod\
    e\x18\x08\x20\x01(\rR\x07retcodeb\x06proto3\
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
            messages.push(UpdateTrackMainMissionIdScRsp::generated_message_descriptor_data());
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
