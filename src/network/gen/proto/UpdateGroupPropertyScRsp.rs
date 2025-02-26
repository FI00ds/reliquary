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

//! Generated file from `UpdateGroupPropertyScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:UpdateGroupPropertyScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct UpdateGroupPropertyScRsp {
    // message fields
    // @@protoc_insertion_point(field:UpdateGroupPropertyScRsp.JAIBIEEKHEG)
    pub JAIBIEEKHEG: ::std::string::String,
    // @@protoc_insertion_point(field:UpdateGroupPropertyScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:UpdateGroupPropertyScRsp.FJNHDHOHBCL)
    pub FJNHDHOHBCL: u32,
    // @@protoc_insertion_point(field:UpdateGroupPropertyScRsp.GFHGLFFHFBD)
    pub GFHGLFFHFBD: u32,
    // @@protoc_insertion_point(field:UpdateGroupPropertyScRsp.AGFIJNIEBKF)
    pub AGFIJNIEBKF: i32,
    // @@protoc_insertion_point(field:UpdateGroupPropertyScRsp.LJHIJCABHEP)
    pub LJHIJCABHEP: u32,
    // @@protoc_insertion_point(field:UpdateGroupPropertyScRsp.DLMAMKINNCO)
    pub DLMAMKINNCO: i32,
    // special fields
    // @@protoc_insertion_point(special_field:UpdateGroupPropertyScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a UpdateGroupPropertyScRsp {
    fn default() -> &'a UpdateGroupPropertyScRsp {
        <UpdateGroupPropertyScRsp as ::protobuf::Message>::default_instance()
    }
}

impl UpdateGroupPropertyScRsp {
    pub fn new() -> UpdateGroupPropertyScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JAIBIEEKHEG",
            |m: &UpdateGroupPropertyScRsp| { &m.JAIBIEEKHEG },
            |m: &mut UpdateGroupPropertyScRsp| { &mut m.JAIBIEEKHEG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &UpdateGroupPropertyScRsp| { &m.retcode },
            |m: &mut UpdateGroupPropertyScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FJNHDHOHBCL",
            |m: &UpdateGroupPropertyScRsp| { &m.FJNHDHOHBCL },
            |m: &mut UpdateGroupPropertyScRsp| { &mut m.FJNHDHOHBCL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GFHGLFFHFBD",
            |m: &UpdateGroupPropertyScRsp| { &m.GFHGLFFHFBD },
            |m: &mut UpdateGroupPropertyScRsp| { &mut m.GFHGLFFHFBD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AGFIJNIEBKF",
            |m: &UpdateGroupPropertyScRsp| { &m.AGFIJNIEBKF },
            |m: &mut UpdateGroupPropertyScRsp| { &mut m.AGFIJNIEBKF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LJHIJCABHEP",
            |m: &UpdateGroupPropertyScRsp| { &m.LJHIJCABHEP },
            |m: &mut UpdateGroupPropertyScRsp| { &mut m.LJHIJCABHEP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DLMAMKINNCO",
            |m: &UpdateGroupPropertyScRsp| { &m.DLMAMKINNCO },
            |m: &mut UpdateGroupPropertyScRsp| { &mut m.DLMAMKINNCO },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<UpdateGroupPropertyScRsp>(
            "UpdateGroupPropertyScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for UpdateGroupPropertyScRsp {
    const NAME: &'static str = "UpdateGroupPropertyScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                42 => {
                    self.JAIBIEEKHEG = is.read_string()?;
                },
                120 => {
                    self.retcode = is.read_uint32()?;
                },
                56 => {
                    self.FJNHDHOHBCL = is.read_uint32()?;
                },
                96 => {
                    self.GFHGLFFHFBD = is.read_uint32()?;
                },
                48 => {
                    self.AGFIJNIEBKF = is.read_int32()?;
                },
                104 => {
                    self.LJHIJCABHEP = is.read_uint32()?;
                },
                16 => {
                    self.DLMAMKINNCO = is.read_int32()?;
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
        if !self.JAIBIEEKHEG.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.JAIBIEEKHEG);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.retcode);
        }
        if self.FJNHDHOHBCL != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.FJNHDHOHBCL);
        }
        if self.GFHGLFFHFBD != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.GFHGLFFHFBD);
        }
        if self.AGFIJNIEBKF != 0 {
            my_size += ::protobuf::rt::int32_size(6, self.AGFIJNIEBKF);
        }
        if self.LJHIJCABHEP != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.LJHIJCABHEP);
        }
        if self.DLMAMKINNCO != 0 {
            my_size += ::protobuf::rt::int32_size(2, self.DLMAMKINNCO);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.JAIBIEEKHEG.is_empty() {
            os.write_string(5, &self.JAIBIEEKHEG)?;
        }
        if self.retcode != 0 {
            os.write_uint32(15, self.retcode)?;
        }
        if self.FJNHDHOHBCL != 0 {
            os.write_uint32(7, self.FJNHDHOHBCL)?;
        }
        if self.GFHGLFFHFBD != 0 {
            os.write_uint32(12, self.GFHGLFFHFBD)?;
        }
        if self.AGFIJNIEBKF != 0 {
            os.write_int32(6, self.AGFIJNIEBKF)?;
        }
        if self.LJHIJCABHEP != 0 {
            os.write_uint32(13, self.LJHIJCABHEP)?;
        }
        if self.DLMAMKINNCO != 0 {
            os.write_int32(2, self.DLMAMKINNCO)?;
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

    fn new() -> UpdateGroupPropertyScRsp {
        UpdateGroupPropertyScRsp::new()
    }

    fn clear(&mut self) {
        self.JAIBIEEKHEG.clear();
        self.retcode = 0;
        self.FJNHDHOHBCL = 0;
        self.GFHGLFFHFBD = 0;
        self.AGFIJNIEBKF = 0;
        self.LJHIJCABHEP = 0;
        self.DLMAMKINNCO = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static UpdateGroupPropertyScRsp {
        static instance: UpdateGroupPropertyScRsp = UpdateGroupPropertyScRsp {
            JAIBIEEKHEG: ::std::string::String::new(),
            retcode: 0,
            FJNHDHOHBCL: 0,
            GFHGLFFHFBD: 0,
            AGFIJNIEBKF: 0,
            LJHIJCABHEP: 0,
            DLMAMKINNCO: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for UpdateGroupPropertyScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("UpdateGroupPropertyScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for UpdateGroupPropertyScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpdateGroupPropertyScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1eUpdateGroupPropertyScRsp.proto\"\x80\x02\n\x18UpdateGroupPropertyS\
    cRsp\x12\x20\n\x0bJAIBIEEKHEG\x18\x05\x20\x01(\tR\x0bJAIBIEEKHEG\x12\x18\
    \n\x07retcode\x18\x0f\x20\x01(\rR\x07retcode\x12\x20\n\x0bFJNHDHOHBCL\
    \x18\x07\x20\x01(\rR\x0bFJNHDHOHBCL\x12\x20\n\x0bGFHGLFFHFBD\x18\x0c\x20\
    \x01(\rR\x0bGFHGLFFHFBD\x12\x20\n\x0bAGFIJNIEBKF\x18\x06\x20\x01(\x05R\
    \x0bAGFIJNIEBKF\x12\x20\n\x0bLJHIJCABHEP\x18\r\x20\x01(\rR\x0bLJHIJCABHE\
    P\x12\x20\n\x0bDLMAMKINNCO\x18\x02\x20\x01(\x05R\x0bDLMAMKINNCOb\x06prot\
    o3\
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
            messages.push(UpdateGroupPropertyScRsp::generated_message_descriptor_data());
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
