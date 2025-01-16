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

//! Generated file from `IFGNFCLAIGM.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:IFGNFCLAIGM)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct IFGNFCLAIGM {
    // message fields
    // @@protoc_insertion_point(field:IFGNFCLAIGM.ACKBCMNNFPC)
    pub ACKBCMNNFPC: u32,
    // @@protoc_insertion_point(field:IFGNFCLAIGM.JNLOABDHEIH)
    pub JNLOABDHEIH: i32,
    // @@protoc_insertion_point(field:IFGNFCLAIGM.KHMIHMPCLJA)
    pub KHMIHMPCLJA: u64,
    // @@protoc_insertion_point(field:IFGNFCLAIGM.KKLJDACENJP)
    pub KKLJDACENJP: u32,
    // @@protoc_insertion_point(field:IFGNFCLAIGM.PPAMLEBAFPI)
    pub PPAMLEBAFPI: ::std::vec::Vec<::std::string::String>,
    // @@protoc_insertion_point(field:IFGNFCLAIGM.BJAACANIEDL)
    pub BJAACANIEDL: u32,
    // special fields
    // @@protoc_insertion_point(special_field:IFGNFCLAIGM.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a IFGNFCLAIGM {
    fn default() -> &'a IFGNFCLAIGM {
        <IFGNFCLAIGM as ::protobuf::Message>::default_instance()
    }
}

impl IFGNFCLAIGM {
    pub fn new() -> IFGNFCLAIGM {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ACKBCMNNFPC",
            |m: &IFGNFCLAIGM| { &m.ACKBCMNNFPC },
            |m: &mut IFGNFCLAIGM| { &mut m.ACKBCMNNFPC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JNLOABDHEIH",
            |m: &IFGNFCLAIGM| { &m.JNLOABDHEIH },
            |m: &mut IFGNFCLAIGM| { &mut m.JNLOABDHEIH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KHMIHMPCLJA",
            |m: &IFGNFCLAIGM| { &m.KHMIHMPCLJA },
            |m: &mut IFGNFCLAIGM| { &mut m.KHMIHMPCLJA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KKLJDACENJP",
            |m: &IFGNFCLAIGM| { &m.KKLJDACENJP },
            |m: &mut IFGNFCLAIGM| { &mut m.KKLJDACENJP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PPAMLEBAFPI",
            |m: &IFGNFCLAIGM| { &m.PPAMLEBAFPI },
            |m: &mut IFGNFCLAIGM| { &mut m.PPAMLEBAFPI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BJAACANIEDL",
            |m: &IFGNFCLAIGM| { &m.BJAACANIEDL },
            |m: &mut IFGNFCLAIGM| { &mut m.BJAACANIEDL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<IFGNFCLAIGM>(
            "IFGNFCLAIGM",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for IFGNFCLAIGM {
    const NAME: &'static str = "IFGNFCLAIGM";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                32 => {
                    self.ACKBCMNNFPC = is.read_uint32()?;
                },
                48 => {
                    self.JNLOABDHEIH = is.read_int32()?;
                },
                80 => {
                    self.KHMIHMPCLJA = is.read_uint64()?;
                },
                104 => {
                    self.KKLJDACENJP = is.read_uint32()?;
                },
                114 => {
                    self.PPAMLEBAFPI.push(is.read_string()?);
                },
                40 => {
                    self.BJAACANIEDL = is.read_uint32()?;
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
        if self.ACKBCMNNFPC != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.ACKBCMNNFPC);
        }
        if self.JNLOABDHEIH != 0 {
            my_size += ::protobuf::rt::int32_size(6, self.JNLOABDHEIH);
        }
        if self.KHMIHMPCLJA != 0 {
            my_size += ::protobuf::rt::uint64_size(10, self.KHMIHMPCLJA);
        }
        if self.KKLJDACENJP != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.KKLJDACENJP);
        }
        for value in &self.PPAMLEBAFPI {
            my_size += ::protobuf::rt::string_size(14, &value);
        };
        if self.BJAACANIEDL != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.BJAACANIEDL);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.ACKBCMNNFPC != 0 {
            os.write_uint32(4, self.ACKBCMNNFPC)?;
        }
        if self.JNLOABDHEIH != 0 {
            os.write_int32(6, self.JNLOABDHEIH)?;
        }
        if self.KHMIHMPCLJA != 0 {
            os.write_uint64(10, self.KHMIHMPCLJA)?;
        }
        if self.KKLJDACENJP != 0 {
            os.write_uint32(13, self.KKLJDACENJP)?;
        }
        for v in &self.PPAMLEBAFPI {
            os.write_string(14, &v)?;
        };
        if self.BJAACANIEDL != 0 {
            os.write_uint32(5, self.BJAACANIEDL)?;
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

    fn new() -> IFGNFCLAIGM {
        IFGNFCLAIGM::new()
    }

    fn clear(&mut self) {
        self.ACKBCMNNFPC = 0;
        self.JNLOABDHEIH = 0;
        self.KHMIHMPCLJA = 0;
        self.KKLJDACENJP = 0;
        self.PPAMLEBAFPI.clear();
        self.BJAACANIEDL = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static IFGNFCLAIGM {
        static instance: IFGNFCLAIGM = IFGNFCLAIGM {
            ACKBCMNNFPC: 0,
            JNLOABDHEIH: 0,
            KHMIHMPCLJA: 0,
            KKLJDACENJP: 0,
            PPAMLEBAFPI: ::std::vec::Vec::new(),
            BJAACANIEDL: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for IFGNFCLAIGM {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("IFGNFCLAIGM").unwrap()).clone()
    }
}

impl ::std::fmt::Display for IFGNFCLAIGM {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IFGNFCLAIGM {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11IFGNFCLAIGM.proto\"\xd9\x01\n\x0bIFGNFCLAIGM\x12\x20\n\x0bACKBCMNN\
    FPC\x18\x04\x20\x01(\rR\x0bACKBCMNNFPC\x12\x20\n\x0bJNLOABDHEIH\x18\x06\
    \x20\x01(\x05R\x0bJNLOABDHEIH\x12\x20\n\x0bKHMIHMPCLJA\x18\n\x20\x01(\
    \x04R\x0bKHMIHMPCLJA\x12\x20\n\x0bKKLJDACENJP\x18\r\x20\x01(\rR\x0bKKLJD\
    ACENJP\x12\x20\n\x0bPPAMLEBAFPI\x18\x0e\x20\x03(\tR\x0bPPAMLEBAFPI\x12\
    \x20\n\x0bBJAACANIEDL\x18\x05\x20\x01(\rR\x0bBJAACANIEDLb\x06proto3\
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
            messages.push(IFGNFCLAIGM::generated_message_descriptor_data());
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
