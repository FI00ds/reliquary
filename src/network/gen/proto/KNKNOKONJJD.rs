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

//! Generated file from `KNKNOKONJJD.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:KNKNOKONJJD)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct KNKNOKONJJD {
    // message fields
    // @@protoc_insertion_point(field:KNKNOKONJJD.KCOHMMBLCGH)
    pub KCOHMMBLCGH: ::std::vec::Vec<super::GFDCMEBGCHI::GFDCMEBGCHI>,
    // @@protoc_insertion_point(field:KNKNOKONJJD.CIIHFIGJLBJ)
    pub CIIHFIGJLBJ: ::std::vec::Vec<super::PGELHHCIIDP::PGELHHCIIDP>,
    // @@protoc_insertion_point(field:KNKNOKONJJD.DEIFIEMCKLD)
    pub DEIFIEMCKLD: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:KNKNOKONJJD.HHDOOMJLGGC)
    pub HHDOOMJLGGC: ::std::vec::Vec<super::LogisticsScore::LogisticsScore>,
    // special fields
    // @@protoc_insertion_point(special_field:KNKNOKONJJD.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a KNKNOKONJJD {
    fn default() -> &'a KNKNOKONJJD {
        <KNKNOKONJJD as ::protobuf::Message>::default_instance()
    }
}

impl KNKNOKONJJD {
    pub fn new() -> KNKNOKONJJD {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KCOHMMBLCGH",
            |m: &KNKNOKONJJD| { &m.KCOHMMBLCGH },
            |m: &mut KNKNOKONJJD| { &mut m.KCOHMMBLCGH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CIIHFIGJLBJ",
            |m: &KNKNOKONJJD| { &m.CIIHFIGJLBJ },
            |m: &mut KNKNOKONJJD| { &mut m.CIIHFIGJLBJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "DEIFIEMCKLD",
            |m: &KNKNOKONJJD| { &m.DEIFIEMCKLD },
            |m: &mut KNKNOKONJJD| { &mut m.DEIFIEMCKLD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HHDOOMJLGGC",
            |m: &KNKNOKONJJD| { &m.HHDOOMJLGGC },
            |m: &mut KNKNOKONJJD| { &mut m.HHDOOMJLGGC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<KNKNOKONJJD>(
            "KNKNOKONJJD",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for KNKNOKONJJD {
    const NAME: &'static str = "KNKNOKONJJD";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                90 => {
                    self.KCOHMMBLCGH.push(is.read_message()?);
                },
                26 => {
                    self.CIIHFIGJLBJ.push(is.read_message()?);
                },
                114 => {
                    is.read_repeated_packed_uint32_into(&mut self.DEIFIEMCKLD)?;
                },
                112 => {
                    self.DEIFIEMCKLD.push(is.read_uint32()?);
                },
                42 => {
                    self.HHDOOMJLGGC.push(is.read_message()?);
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
        for value in &self.KCOHMMBLCGH {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.CIIHFIGJLBJ {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.DEIFIEMCKLD {
            my_size += ::protobuf::rt::uint32_size(14, *value);
        };
        for value in &self.HHDOOMJLGGC {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.KCOHMMBLCGH {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        for v in &self.CIIHFIGJLBJ {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        for v in &self.DEIFIEMCKLD {
            os.write_uint32(14, *v)?;
        };
        for v in &self.HHDOOMJLGGC {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> KNKNOKONJJD {
        KNKNOKONJJD::new()
    }

    fn clear(&mut self) {
        self.KCOHMMBLCGH.clear();
        self.CIIHFIGJLBJ.clear();
        self.DEIFIEMCKLD.clear();
        self.HHDOOMJLGGC.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static KNKNOKONJJD {
        static instance: KNKNOKONJJD = KNKNOKONJJD {
            KCOHMMBLCGH: ::std::vec::Vec::new(),
            CIIHFIGJLBJ: ::std::vec::Vec::new(),
            DEIFIEMCKLD: ::std::vec::Vec::new(),
            HHDOOMJLGGC: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for KNKNOKONJJD {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("KNKNOKONJJD").unwrap()).clone()
    }
}

impl ::std::fmt::Display for KNKNOKONJJD {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KNKNOKONJJD {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11KNKNOKONJJD.proto\x1a\x11GFDCMEBGCHI.proto\x1a\x14LogisticsScore.p\
    roto\x1a\x11PGELHHCIIDP.proto\"\xc2\x01\n\x0bKNKNOKONJJD\x12.\n\x0bKCOHM\
    MBLCGH\x18\x0b\x20\x03(\x0b2\x0c.GFDCMEBGCHIR\x0bKCOHMMBLCGH\x12.\n\x0bC\
    IIHFIGJLBJ\x18\x03\x20\x03(\x0b2\x0c.PGELHHCIIDPR\x0bCIIHFIGJLBJ\x12\x20\
    \n\x0bDEIFIEMCKLD\x18\x0e\x20\x03(\rR\x0bDEIFIEMCKLD\x121\n\x0bHHDOOMJLG\
    GC\x18\x05\x20\x03(\x0b2\x0f.LogisticsScoreR\x0bHHDOOMJLGGCb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::GFDCMEBGCHI::file_descriptor().clone());
            deps.push(super::LogisticsScore::file_descriptor().clone());
            deps.push(super::PGELHHCIIDP::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(KNKNOKONJJD::generated_message_descriptor_data());
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
