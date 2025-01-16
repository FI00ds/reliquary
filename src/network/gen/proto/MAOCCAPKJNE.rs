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

//! Generated file from `MAOCCAPKJNE.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MAOCCAPKJNE)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MAOCCAPKJNE {
    // message fields
    // @@protoc_insertion_point(field:MAOCCAPKJNE.GJIPKCCIEML)
    pub GJIPKCCIEML: u32,
    // @@protoc_insertion_point(field:MAOCCAPKJNE.IAOEOMFHLEB)
    pub IAOEOMFHLEB: u32,
    // @@protoc_insertion_point(field:MAOCCAPKJNE.DECAEMJPONM)
    pub DECAEMJPONM: u32,
    // @@protoc_insertion_point(field:MAOCCAPKJNE.NMJOFBDDLAL)
    pub NMJOFBDDLAL: u32,
    // @@protoc_insertion_point(field:MAOCCAPKJNE.LECBNGDIGBD)
    pub LECBNGDIGBD: u32,
    // @@protoc_insertion_point(field:MAOCCAPKJNE.CMNFKIDHNGE)
    pub CMNFKIDHNGE: u32,
    // @@protoc_insertion_point(field:MAOCCAPKJNE.EILNDFOLICH)
    pub EILNDFOLICH: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:MAOCCAPKJNE.GECPCOPJPNA)
    pub GECPCOPJPNA: ::std::vec::Vec<super::CLNOHAGLFKH::CLNOHAGLFKH>,
    // special fields
    // @@protoc_insertion_point(special_field:MAOCCAPKJNE.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MAOCCAPKJNE {
    fn default() -> &'a MAOCCAPKJNE {
        <MAOCCAPKJNE as ::protobuf::Message>::default_instance()
    }
}

impl MAOCCAPKJNE {
    pub fn new() -> MAOCCAPKJNE {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GJIPKCCIEML",
            |m: &MAOCCAPKJNE| { &m.GJIPKCCIEML },
            |m: &mut MAOCCAPKJNE| { &mut m.GJIPKCCIEML },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IAOEOMFHLEB",
            |m: &MAOCCAPKJNE| { &m.IAOEOMFHLEB },
            |m: &mut MAOCCAPKJNE| { &mut m.IAOEOMFHLEB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DECAEMJPONM",
            |m: &MAOCCAPKJNE| { &m.DECAEMJPONM },
            |m: &mut MAOCCAPKJNE| { &mut m.DECAEMJPONM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NMJOFBDDLAL",
            |m: &MAOCCAPKJNE| { &m.NMJOFBDDLAL },
            |m: &mut MAOCCAPKJNE| { &mut m.NMJOFBDDLAL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LECBNGDIGBD",
            |m: &MAOCCAPKJNE| { &m.LECBNGDIGBD },
            |m: &mut MAOCCAPKJNE| { &mut m.LECBNGDIGBD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CMNFKIDHNGE",
            |m: &MAOCCAPKJNE| { &m.CMNFKIDHNGE },
            |m: &mut MAOCCAPKJNE| { &mut m.CMNFKIDHNGE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EILNDFOLICH",
            |m: &MAOCCAPKJNE| { &m.EILNDFOLICH },
            |m: &mut MAOCCAPKJNE| { &mut m.EILNDFOLICH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "GECPCOPJPNA",
            |m: &MAOCCAPKJNE| { &m.GECPCOPJPNA },
            |m: &mut MAOCCAPKJNE| { &mut m.GECPCOPJPNA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MAOCCAPKJNE>(
            "MAOCCAPKJNE",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MAOCCAPKJNE {
    const NAME: &'static str = "MAOCCAPKJNE";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.GJIPKCCIEML = is.read_uint32()?;
                },
                16 => {
                    self.IAOEOMFHLEB = is.read_uint32()?;
                },
                24 => {
                    self.DECAEMJPONM = is.read_uint32()?;
                },
                32 => {
                    self.NMJOFBDDLAL = is.read_uint32()?;
                },
                40 => {
                    self.LECBNGDIGBD = is.read_uint32()?;
                },
                48 => {
                    self.CMNFKIDHNGE = is.read_uint32()?;
                },
                58 => {
                    is.read_repeated_packed_uint32_into(&mut self.EILNDFOLICH)?;
                },
                56 => {
                    self.EILNDFOLICH.push(is.read_uint32()?);
                },
                66 => {
                    self.GECPCOPJPNA.push(is.read_message()?);
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
        if self.GJIPKCCIEML != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.GJIPKCCIEML);
        }
        if self.IAOEOMFHLEB != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.IAOEOMFHLEB);
        }
        if self.DECAEMJPONM != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.DECAEMJPONM);
        }
        if self.NMJOFBDDLAL != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.NMJOFBDDLAL);
        }
        if self.LECBNGDIGBD != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.LECBNGDIGBD);
        }
        if self.CMNFKIDHNGE != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.CMNFKIDHNGE);
        }
        for value in &self.EILNDFOLICH {
            my_size += ::protobuf::rt::uint32_size(7, *value);
        };
        for value in &self.GECPCOPJPNA {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.GJIPKCCIEML != 0 {
            os.write_uint32(1, self.GJIPKCCIEML)?;
        }
        if self.IAOEOMFHLEB != 0 {
            os.write_uint32(2, self.IAOEOMFHLEB)?;
        }
        if self.DECAEMJPONM != 0 {
            os.write_uint32(3, self.DECAEMJPONM)?;
        }
        if self.NMJOFBDDLAL != 0 {
            os.write_uint32(4, self.NMJOFBDDLAL)?;
        }
        if self.LECBNGDIGBD != 0 {
            os.write_uint32(5, self.LECBNGDIGBD)?;
        }
        if self.CMNFKIDHNGE != 0 {
            os.write_uint32(6, self.CMNFKIDHNGE)?;
        }
        for v in &self.EILNDFOLICH {
            os.write_uint32(7, *v)?;
        };
        for v in &self.GECPCOPJPNA {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
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

    fn new() -> MAOCCAPKJNE {
        MAOCCAPKJNE::new()
    }

    fn clear(&mut self) {
        self.GJIPKCCIEML = 0;
        self.IAOEOMFHLEB = 0;
        self.DECAEMJPONM = 0;
        self.NMJOFBDDLAL = 0;
        self.LECBNGDIGBD = 0;
        self.CMNFKIDHNGE = 0;
        self.EILNDFOLICH.clear();
        self.GECPCOPJPNA.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MAOCCAPKJNE {
        static instance: MAOCCAPKJNE = MAOCCAPKJNE {
            GJIPKCCIEML: 0,
            IAOEOMFHLEB: 0,
            DECAEMJPONM: 0,
            NMJOFBDDLAL: 0,
            LECBNGDIGBD: 0,
            CMNFKIDHNGE: 0,
            EILNDFOLICH: ::std::vec::Vec::new(),
            GECPCOPJPNA: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MAOCCAPKJNE {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MAOCCAPKJNE").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MAOCCAPKJNE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MAOCCAPKJNE {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11MAOCCAPKJNE.proto\x1a\x11CLNOHAGLFKH.proto\"\xab\x02\n\x0bMAOCCAPK\
    JNE\x12\x20\n\x0bGJIPKCCIEML\x18\x01\x20\x01(\rR\x0bGJIPKCCIEML\x12\x20\
    \n\x0bIAOEOMFHLEB\x18\x02\x20\x01(\rR\x0bIAOEOMFHLEB\x12\x20\n\x0bDECAEM\
    JPONM\x18\x03\x20\x01(\rR\x0bDECAEMJPONM\x12\x20\n\x0bNMJOFBDDLAL\x18\
    \x04\x20\x01(\rR\x0bNMJOFBDDLAL\x12\x20\n\x0bLECBNGDIGBD\x18\x05\x20\x01\
    (\rR\x0bLECBNGDIGBD\x12\x20\n\x0bCMNFKIDHNGE\x18\x06\x20\x01(\rR\x0bCMNF\
    KIDHNGE\x12\x20\n\x0bEILNDFOLICH\x18\x07\x20\x03(\rR\x0bEILNDFOLICH\x12.\
    \n\x0bGECPCOPJPNA\x18\x08\x20\x03(\x0b2\x0c.CLNOHAGLFKHR\x0bGECPCOPJPNAb\
    \x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::CLNOHAGLFKH::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MAOCCAPKJNE::generated_message_descriptor_data());
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
