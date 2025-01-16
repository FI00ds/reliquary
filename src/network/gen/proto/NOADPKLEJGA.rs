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

//! Generated file from `NOADPKLEJGA.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:NOADPKLEJGA)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NOADPKLEJGA {
    // message fields
    // @@protoc_insertion_point(field:NOADPKLEJGA.IHBBDAKOGJD)
    pub IHBBDAKOGJD: bool,
    // @@protoc_insertion_point(field:NOADPKLEJGA.JNKLGAPLBCG)
    pub JNKLGAPLBCG: u32,
    // @@protoc_insertion_point(field:NOADPKLEJGA.MPFMPMLGOGE)
    pub MPFMPMLGOGE: u32,
    // @@protoc_insertion_point(field:NOADPKLEJGA.LANBEKHAGGB)
    pub LANBEKHAGGB: u32,
    // @@protoc_insertion_point(field:NOADPKLEJGA.FGAAOAELEDG)
    pub FGAAOAELEDG: u32,
    // @@protoc_insertion_point(field:NOADPKLEJGA.MGKKFINFHGJ)
    pub MGKKFINFHGJ: bool,
    // special fields
    // @@protoc_insertion_point(special_field:NOADPKLEJGA.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NOADPKLEJGA {
    fn default() -> &'a NOADPKLEJGA {
        <NOADPKLEJGA as ::protobuf::Message>::default_instance()
    }
}

impl NOADPKLEJGA {
    pub fn new() -> NOADPKLEJGA {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IHBBDAKOGJD",
            |m: &NOADPKLEJGA| { &m.IHBBDAKOGJD },
            |m: &mut NOADPKLEJGA| { &mut m.IHBBDAKOGJD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JNKLGAPLBCG",
            |m: &NOADPKLEJGA| { &m.JNKLGAPLBCG },
            |m: &mut NOADPKLEJGA| { &mut m.JNKLGAPLBCG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MPFMPMLGOGE",
            |m: &NOADPKLEJGA| { &m.MPFMPMLGOGE },
            |m: &mut NOADPKLEJGA| { &mut m.MPFMPMLGOGE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LANBEKHAGGB",
            |m: &NOADPKLEJGA| { &m.LANBEKHAGGB },
            |m: &mut NOADPKLEJGA| { &mut m.LANBEKHAGGB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FGAAOAELEDG",
            |m: &NOADPKLEJGA| { &m.FGAAOAELEDG },
            |m: &mut NOADPKLEJGA| { &mut m.FGAAOAELEDG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MGKKFINFHGJ",
            |m: &NOADPKLEJGA| { &m.MGKKFINFHGJ },
            |m: &mut NOADPKLEJGA| { &mut m.MGKKFINFHGJ },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NOADPKLEJGA>(
            "NOADPKLEJGA",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NOADPKLEJGA {
    const NAME: &'static str = "NOADPKLEJGA";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                96 => {
                    self.IHBBDAKOGJD = is.read_bool()?;
                },
                72 => {
                    self.JNKLGAPLBCG = is.read_uint32()?;
                },
                56 => {
                    self.MPFMPMLGOGE = is.read_uint32()?;
                },
                16 => {
                    self.LANBEKHAGGB = is.read_uint32()?;
                },
                8 => {
                    self.FGAAOAELEDG = is.read_uint32()?;
                },
                112 => {
                    self.MGKKFINFHGJ = is.read_bool()?;
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
        if self.IHBBDAKOGJD != false {
            my_size += 1 + 1;
        }
        if self.JNKLGAPLBCG != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.JNKLGAPLBCG);
        }
        if self.MPFMPMLGOGE != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.MPFMPMLGOGE);
        }
        if self.LANBEKHAGGB != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.LANBEKHAGGB);
        }
        if self.FGAAOAELEDG != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.FGAAOAELEDG);
        }
        if self.MGKKFINFHGJ != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IHBBDAKOGJD != false {
            os.write_bool(12, self.IHBBDAKOGJD)?;
        }
        if self.JNKLGAPLBCG != 0 {
            os.write_uint32(9, self.JNKLGAPLBCG)?;
        }
        if self.MPFMPMLGOGE != 0 {
            os.write_uint32(7, self.MPFMPMLGOGE)?;
        }
        if self.LANBEKHAGGB != 0 {
            os.write_uint32(2, self.LANBEKHAGGB)?;
        }
        if self.FGAAOAELEDG != 0 {
            os.write_uint32(1, self.FGAAOAELEDG)?;
        }
        if self.MGKKFINFHGJ != false {
            os.write_bool(14, self.MGKKFINFHGJ)?;
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

    fn new() -> NOADPKLEJGA {
        NOADPKLEJGA::new()
    }

    fn clear(&mut self) {
        self.IHBBDAKOGJD = false;
        self.JNKLGAPLBCG = 0;
        self.MPFMPMLGOGE = 0;
        self.LANBEKHAGGB = 0;
        self.FGAAOAELEDG = 0;
        self.MGKKFINFHGJ = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NOADPKLEJGA {
        static instance: NOADPKLEJGA = NOADPKLEJGA {
            IHBBDAKOGJD: false,
            JNKLGAPLBCG: 0,
            MPFMPMLGOGE: 0,
            LANBEKHAGGB: 0,
            FGAAOAELEDG: 0,
            MGKKFINFHGJ: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for NOADPKLEJGA {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NOADPKLEJGA").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NOADPKLEJGA {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NOADPKLEJGA {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11NOADPKLEJGA.proto\"\xd9\x01\n\x0bNOADPKLEJGA\x12\x20\n\x0bIHBBDAKO\
    GJD\x18\x0c\x20\x01(\x08R\x0bIHBBDAKOGJD\x12\x20\n\x0bJNKLGAPLBCG\x18\t\
    \x20\x01(\rR\x0bJNKLGAPLBCG\x12\x20\n\x0bMPFMPMLGOGE\x18\x07\x20\x01(\rR\
    \x0bMPFMPMLGOGE\x12\x20\n\x0bLANBEKHAGGB\x18\x02\x20\x01(\rR\x0bLANBEKHA\
    GGB\x12\x20\n\x0bFGAAOAELEDG\x18\x01\x20\x01(\rR\x0bFGAAOAELEDG\x12\x20\
    \n\x0bMGKKFINFHGJ\x18\x0e\x20\x01(\x08R\x0bMGKKFINFHGJb\x06proto3\
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
            messages.push(NOADPKLEJGA::generated_message_descriptor_data());
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
