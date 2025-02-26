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

//! Generated file from `JGFKICDCFLJ.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:JGFKICDCFLJ)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct JGFKICDCFLJ {
    // message fields
    // @@protoc_insertion_point(field:JGFKICDCFLJ.FDKGFDICMFD)
    pub FDKGFDICMFD: ::std::string::String,
    // @@protoc_insertion_point(field:JGFKICDCFLJ.MDJCAOAGCKO)
    pub MDJCAOAGCKO: ::std::string::String,
    // @@protoc_insertion_point(field:JGFKICDCFLJ.GIOOHOOMJHO)
    pub GIOOHOOMJHO: ::std::string::String,
    // @@protoc_insertion_point(field:JGFKICDCFLJ.FBMLLNKCFEN)
    pub FBMLLNKCFEN: ::std::string::String,
    // @@protoc_insertion_point(field:JGFKICDCFLJ.DGOOHIBAOEE)
    pub DGOOHIBAOEE: ::std::string::String,
    // @@protoc_insertion_point(field:JGFKICDCFLJ.FKBAMBOODKJ)
    pub FKBAMBOODKJ: ::std::string::String,
    // @@protoc_insertion_point(field:JGFKICDCFLJ.MAC)
    pub MAC: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:JGFKICDCFLJ.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a JGFKICDCFLJ {
    fn default() -> &'a JGFKICDCFLJ {
        <JGFKICDCFLJ as ::protobuf::Message>::default_instance()
    }
}

impl JGFKICDCFLJ {
    pub fn new() -> JGFKICDCFLJ {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FDKGFDICMFD",
            |m: &JGFKICDCFLJ| { &m.FDKGFDICMFD },
            |m: &mut JGFKICDCFLJ| { &mut m.FDKGFDICMFD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MDJCAOAGCKO",
            |m: &JGFKICDCFLJ| { &m.MDJCAOAGCKO },
            |m: &mut JGFKICDCFLJ| { &mut m.MDJCAOAGCKO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GIOOHOOMJHO",
            |m: &JGFKICDCFLJ| { &m.GIOOHOOMJHO },
            |m: &mut JGFKICDCFLJ| { &mut m.GIOOHOOMJHO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FBMLLNKCFEN",
            |m: &JGFKICDCFLJ| { &m.FBMLLNKCFEN },
            |m: &mut JGFKICDCFLJ| { &mut m.FBMLLNKCFEN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DGOOHIBAOEE",
            |m: &JGFKICDCFLJ| { &m.DGOOHIBAOEE },
            |m: &mut JGFKICDCFLJ| { &mut m.DGOOHIBAOEE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FKBAMBOODKJ",
            |m: &JGFKICDCFLJ| { &m.FKBAMBOODKJ },
            |m: &mut JGFKICDCFLJ| { &mut m.FKBAMBOODKJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MAC",
            |m: &JGFKICDCFLJ| { &m.MAC },
            |m: &mut JGFKICDCFLJ| { &mut m.MAC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<JGFKICDCFLJ>(
            "JGFKICDCFLJ",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for JGFKICDCFLJ {
    const NAME: &'static str = "JGFKICDCFLJ";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.FDKGFDICMFD = is.read_string()?;
                },
                18 => {
                    self.MDJCAOAGCKO = is.read_string()?;
                },
                26 => {
                    self.GIOOHOOMJHO = is.read_string()?;
                },
                34 => {
                    self.FBMLLNKCFEN = is.read_string()?;
                },
                42 => {
                    self.DGOOHIBAOEE = is.read_string()?;
                },
                50 => {
                    self.FKBAMBOODKJ = is.read_string()?;
                },
                58 => {
                    self.MAC = is.read_string()?;
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
        if !self.FDKGFDICMFD.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.FDKGFDICMFD);
        }
        if !self.MDJCAOAGCKO.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.MDJCAOAGCKO);
        }
        if !self.GIOOHOOMJHO.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.GIOOHOOMJHO);
        }
        if !self.FBMLLNKCFEN.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.FBMLLNKCFEN);
        }
        if !self.DGOOHIBAOEE.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.DGOOHIBAOEE);
        }
        if !self.FKBAMBOODKJ.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.FKBAMBOODKJ);
        }
        if !self.MAC.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.MAC);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.FDKGFDICMFD.is_empty() {
            os.write_string(1, &self.FDKGFDICMFD)?;
        }
        if !self.MDJCAOAGCKO.is_empty() {
            os.write_string(2, &self.MDJCAOAGCKO)?;
        }
        if !self.GIOOHOOMJHO.is_empty() {
            os.write_string(3, &self.GIOOHOOMJHO)?;
        }
        if !self.FBMLLNKCFEN.is_empty() {
            os.write_string(4, &self.FBMLLNKCFEN)?;
        }
        if !self.DGOOHIBAOEE.is_empty() {
            os.write_string(5, &self.DGOOHIBAOEE)?;
        }
        if !self.FKBAMBOODKJ.is_empty() {
            os.write_string(6, &self.FKBAMBOODKJ)?;
        }
        if !self.MAC.is_empty() {
            os.write_string(7, &self.MAC)?;
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

    fn new() -> JGFKICDCFLJ {
        JGFKICDCFLJ::new()
    }

    fn clear(&mut self) {
        self.FDKGFDICMFD.clear();
        self.MDJCAOAGCKO.clear();
        self.GIOOHOOMJHO.clear();
        self.FBMLLNKCFEN.clear();
        self.DGOOHIBAOEE.clear();
        self.FKBAMBOODKJ.clear();
        self.MAC.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static JGFKICDCFLJ {
        static instance: JGFKICDCFLJ = JGFKICDCFLJ {
            FDKGFDICMFD: ::std::string::String::new(),
            MDJCAOAGCKO: ::std::string::String::new(),
            GIOOHOOMJHO: ::std::string::String::new(),
            FBMLLNKCFEN: ::std::string::String::new(),
            DGOOHIBAOEE: ::std::string::String::new(),
            FKBAMBOODKJ: ::std::string::String::new(),
            MAC: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for JGFKICDCFLJ {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("JGFKICDCFLJ").unwrap()).clone()
    }
}

impl ::std::fmt::Display for JGFKICDCFLJ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JGFKICDCFLJ {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11JGFKICDCFLJ.proto\"\xeb\x01\n\x0bJGFKICDCFLJ\x12\x20\n\x0bFDKGFDIC\
    MFD\x18\x01\x20\x01(\tR\x0bFDKGFDICMFD\x12\x20\n\x0bMDJCAOAGCKO\x18\x02\
    \x20\x01(\tR\x0bMDJCAOAGCKO\x12\x20\n\x0bGIOOHOOMJHO\x18\x03\x20\x01(\tR\
    \x0bGIOOHOOMJHO\x12\x20\n\x0bFBMLLNKCFEN\x18\x04\x20\x01(\tR\x0bFBMLLNKC\
    FEN\x12\x20\n\x0bDGOOHIBAOEE\x18\x05\x20\x01(\tR\x0bDGOOHIBAOEE\x12\x20\
    \n\x0bFKBAMBOODKJ\x18\x06\x20\x01(\tR\x0bFKBAMBOODKJ\x12\x10\n\x03MAC\
    \x18\x07\x20\x01(\tR\x03MACb\x06proto3\
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
            messages.push(JGFKICDCFLJ::generated_message_descriptor_data());
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
