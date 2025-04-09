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

//! Generated file from `GBABEKPBLHN.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:GBABEKPBLHN)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GBABEKPBLHN {
    // message fields
    // @@protoc_insertion_point(field:GBABEKPBLHN.GDDCGEKOJOI)
    pub GDDCGEKOJOI: u32,
    // @@protoc_insertion_point(field:GBABEKPBLHN.KDMLLLGHJON)
    pub KDMLLLGHJON: u32,
    // @@protoc_insertion_point(field:GBABEKPBLHN.PIPMGACMJNN)
    pub PIPMGACMJNN: bool,
    // @@protoc_insertion_point(field:GBABEKPBLHN.POCJCMKKJIK)
    pub POCJCMKKJIK: ::std::vec::Vec<super::HJPGNCNGILG::HJPGNCNGILG>,
    // @@protoc_insertion_point(field:GBABEKPBLHN.CAOAPDCCPCA)
    pub CAOAPDCCPCA: ::protobuf::MessageField<super::ABENFANELFL::ABENFANELFL>,
    // @@protoc_insertion_point(field:GBABEKPBLHN.GLDJNHIGGJE)
    pub GLDJNHIGGJE: bool,
    // special fields
    // @@protoc_insertion_point(special_field:GBABEKPBLHN.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GBABEKPBLHN {
    fn default() -> &'a GBABEKPBLHN {
        <GBABEKPBLHN as ::protobuf::Message>::default_instance()
    }
}

impl GBABEKPBLHN {
    pub fn new() -> GBABEKPBLHN {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GDDCGEKOJOI",
            |m: &GBABEKPBLHN| { &m.GDDCGEKOJOI },
            |m: &mut GBABEKPBLHN| { &mut m.GDDCGEKOJOI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KDMLLLGHJON",
            |m: &GBABEKPBLHN| { &m.KDMLLLGHJON },
            |m: &mut GBABEKPBLHN| { &mut m.KDMLLLGHJON },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PIPMGACMJNN",
            |m: &GBABEKPBLHN| { &m.PIPMGACMJNN },
            |m: &mut GBABEKPBLHN| { &mut m.PIPMGACMJNN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "POCJCMKKJIK",
            |m: &GBABEKPBLHN| { &m.POCJCMKKJIK },
            |m: &mut GBABEKPBLHN| { &mut m.POCJCMKKJIK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ABENFANELFL::ABENFANELFL>(
            "CAOAPDCCPCA",
            |m: &GBABEKPBLHN| { &m.CAOAPDCCPCA },
            |m: &mut GBABEKPBLHN| { &mut m.CAOAPDCCPCA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GLDJNHIGGJE",
            |m: &GBABEKPBLHN| { &m.GLDJNHIGGJE },
            |m: &mut GBABEKPBLHN| { &mut m.GLDJNHIGGJE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GBABEKPBLHN>(
            "GBABEKPBLHN",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GBABEKPBLHN {
    const NAME: &'static str = "GBABEKPBLHN";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                40 => {
                    self.GDDCGEKOJOI = is.read_uint32()?;
                },
                16 => {
                    self.KDMLLLGHJON = is.read_uint32()?;
                },
                96 => {
                    self.PIPMGACMJNN = is.read_bool()?;
                },
                66 => {
                    self.POCJCMKKJIK.push(is.read_message()?);
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.CAOAPDCCPCA)?;
                },
                48 => {
                    self.GLDJNHIGGJE = is.read_bool()?;
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
        if self.GDDCGEKOJOI != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.GDDCGEKOJOI);
        }
        if self.KDMLLLGHJON != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.KDMLLLGHJON);
        }
        if self.PIPMGACMJNN != false {
            my_size += 1 + 1;
        }
        for value in &self.POCJCMKKJIK {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.CAOAPDCCPCA.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.GLDJNHIGGJE != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.GDDCGEKOJOI != 0 {
            os.write_uint32(5, self.GDDCGEKOJOI)?;
        }
        if self.KDMLLLGHJON != 0 {
            os.write_uint32(2, self.KDMLLLGHJON)?;
        }
        if self.PIPMGACMJNN != false {
            os.write_bool(12, self.PIPMGACMJNN)?;
        }
        for v in &self.POCJCMKKJIK {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        if let Some(v) = self.CAOAPDCCPCA.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        if self.GLDJNHIGGJE != false {
            os.write_bool(6, self.GLDJNHIGGJE)?;
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

    fn new() -> GBABEKPBLHN {
        GBABEKPBLHN::new()
    }

    fn clear(&mut self) {
        self.GDDCGEKOJOI = 0;
        self.KDMLLLGHJON = 0;
        self.PIPMGACMJNN = false;
        self.POCJCMKKJIK.clear();
        self.CAOAPDCCPCA.clear();
        self.GLDJNHIGGJE = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GBABEKPBLHN {
        static instance: GBABEKPBLHN = GBABEKPBLHN {
            GDDCGEKOJOI: 0,
            KDMLLLGHJON: 0,
            PIPMGACMJNN: false,
            POCJCMKKJIK: ::std::vec::Vec::new(),
            CAOAPDCCPCA: ::protobuf::MessageField::none(),
            GLDJNHIGGJE: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GBABEKPBLHN {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GBABEKPBLHN").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GBABEKPBLHN {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GBABEKPBLHN {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11GBABEKPBLHN.proto\x1a\x11ABENFANELFL.proto\x1a\x11HJPGNCNGILG.prot\
    o\"\xf5\x01\n\x0bGBABEKPBLHN\x12\x20\n\x0bGDDCGEKOJOI\x18\x05\x20\x01(\r\
    R\x0bGDDCGEKOJOI\x12\x20\n\x0bKDMLLLGHJON\x18\x02\x20\x01(\rR\x0bKDMLLLG\
    HJON\x12\x20\n\x0bPIPMGACMJNN\x18\x0c\x20\x01(\x08R\x0bPIPMGACMJNN\x12.\
    \n\x0bPOCJCMKKJIK\x18\x08\x20\x03(\x0b2\x0c.HJPGNCNGILGR\x0bPOCJCMKKJIK\
    \x12.\n\x0bCAOAPDCCPCA\x18\x0e\x20\x01(\x0b2\x0c.ABENFANELFLR\x0bCAOAPDC\
    CPCA\x12\x20\n\x0bGLDJNHIGGJE\x18\x06\x20\x01(\x08R\x0bGLDJNHIGGJEb\x06p\
    roto3\
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
            deps.push(super::ABENFANELFL::file_descriptor().clone());
            deps.push(super::HJPGNCNGILG::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GBABEKPBLHN::generated_message_descriptor_data());
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
