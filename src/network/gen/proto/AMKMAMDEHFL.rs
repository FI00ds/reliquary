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

//! Generated file from `AMKMAMDEHFL.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:AMKMAMDEHFL)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AMKMAMDEHFL {
    // message fields
    // @@protoc_insertion_point(field:AMKMAMDEHFL.PLPNPNDGHCP)
    pub PLPNPNDGHCP: bool,
    // @@protoc_insertion_point(field:AMKMAMDEHFL.FGGCOBNAFMF)
    pub FGGCOBNAFMF: u32,
    // @@protoc_insertion_point(field:AMKMAMDEHFL.OLEADGFBBMJ)
    pub OLEADGFBBMJ: u32,
    // special fields
    // @@protoc_insertion_point(special_field:AMKMAMDEHFL.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AMKMAMDEHFL {
    fn default() -> &'a AMKMAMDEHFL {
        <AMKMAMDEHFL as ::protobuf::Message>::default_instance()
    }
}

impl AMKMAMDEHFL {
    pub fn new() -> AMKMAMDEHFL {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PLPNPNDGHCP",
            |m: &AMKMAMDEHFL| { &m.PLPNPNDGHCP },
            |m: &mut AMKMAMDEHFL| { &mut m.PLPNPNDGHCP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FGGCOBNAFMF",
            |m: &AMKMAMDEHFL| { &m.FGGCOBNAFMF },
            |m: &mut AMKMAMDEHFL| { &mut m.FGGCOBNAFMF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OLEADGFBBMJ",
            |m: &AMKMAMDEHFL| { &m.OLEADGFBBMJ },
            |m: &mut AMKMAMDEHFL| { &mut m.OLEADGFBBMJ },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AMKMAMDEHFL>(
            "AMKMAMDEHFL",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AMKMAMDEHFL {
    const NAME: &'static str = "AMKMAMDEHFL";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.PLPNPNDGHCP = is.read_bool()?;
                },
                16 => {
                    self.FGGCOBNAFMF = is.read_uint32()?;
                },
                32 => {
                    self.OLEADGFBBMJ = is.read_uint32()?;
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
        if self.PLPNPNDGHCP != false {
            my_size += 1 + 1;
        }
        if self.FGGCOBNAFMF != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.FGGCOBNAFMF);
        }
        if self.OLEADGFBBMJ != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.OLEADGFBBMJ);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.PLPNPNDGHCP != false {
            os.write_bool(13, self.PLPNPNDGHCP)?;
        }
        if self.FGGCOBNAFMF != 0 {
            os.write_uint32(2, self.FGGCOBNAFMF)?;
        }
        if self.OLEADGFBBMJ != 0 {
            os.write_uint32(4, self.OLEADGFBBMJ)?;
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

    fn new() -> AMKMAMDEHFL {
        AMKMAMDEHFL::new()
    }

    fn clear(&mut self) {
        self.PLPNPNDGHCP = false;
        self.FGGCOBNAFMF = 0;
        self.OLEADGFBBMJ = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AMKMAMDEHFL {
        static instance: AMKMAMDEHFL = AMKMAMDEHFL {
            PLPNPNDGHCP: false,
            FGGCOBNAFMF: 0,
            OLEADGFBBMJ: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AMKMAMDEHFL {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AMKMAMDEHFL").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AMKMAMDEHFL {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AMKMAMDEHFL {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11AMKMAMDEHFL.proto\"s\n\x0bAMKMAMDEHFL\x12\x20\n\x0bPLPNPNDGHCP\x18\
    \r\x20\x01(\x08R\x0bPLPNPNDGHCP\x12\x20\n\x0bFGGCOBNAFMF\x18\x02\x20\x01\
    (\rR\x0bFGGCOBNAFMF\x12\x20\n\x0bOLEADGFBBMJ\x18\x04\x20\x01(\rR\x0bOLEA\
    DGFBBMJb\x06proto3\
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
            messages.push(AMKMAMDEHFL::generated_message_descriptor_data());
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
