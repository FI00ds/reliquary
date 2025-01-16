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

//! Generated file from `FCELGNCENDN.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:FCELGNCENDN)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FCELGNCENDN {
    // message fields
    // @@protoc_insertion_point(field:FCELGNCENDN.DJEFIPIFDLE)
    pub DJEFIPIFDLE: ::protobuf::MessageField<super::KIMJKGFGBDB::KIMJKGFGBDB>,
    // @@protoc_insertion_point(field:FCELGNCENDN.GPKAJOPANAG)
    pub GPKAJOPANAG: ::protobuf::MessageField<super::IKFLJJMDIFE::IKFLJJMDIFE>,
    // special fields
    // @@protoc_insertion_point(special_field:FCELGNCENDN.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FCELGNCENDN {
    fn default() -> &'a FCELGNCENDN {
        <FCELGNCENDN as ::protobuf::Message>::default_instance()
    }
}

impl FCELGNCENDN {
    pub fn new() -> FCELGNCENDN {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::KIMJKGFGBDB::KIMJKGFGBDB>(
            "DJEFIPIFDLE",
            |m: &FCELGNCENDN| { &m.DJEFIPIFDLE },
            |m: &mut FCELGNCENDN| { &mut m.DJEFIPIFDLE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::IKFLJJMDIFE::IKFLJJMDIFE>(
            "GPKAJOPANAG",
            |m: &FCELGNCENDN| { &m.GPKAJOPANAG },
            |m: &mut FCELGNCENDN| { &mut m.GPKAJOPANAG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FCELGNCENDN>(
            "FCELGNCENDN",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FCELGNCENDN {
    const NAME: &'static str = "FCELGNCENDN";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.DJEFIPIFDLE)?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.GPKAJOPANAG)?;
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
        if let Some(v) = self.DJEFIPIFDLE.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.GPKAJOPANAG.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.DJEFIPIFDLE.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        if let Some(v) = self.GPKAJOPANAG.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
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

    fn new() -> FCELGNCENDN {
        FCELGNCENDN::new()
    }

    fn clear(&mut self) {
        self.DJEFIPIFDLE.clear();
        self.GPKAJOPANAG.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FCELGNCENDN {
        static instance: FCELGNCENDN = FCELGNCENDN {
            DJEFIPIFDLE: ::protobuf::MessageField::none(),
            GPKAJOPANAG: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FCELGNCENDN {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FCELGNCENDN").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FCELGNCENDN {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FCELGNCENDN {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11FCELGNCENDN.proto\x1a\x11IKFLJJMDIFE.proto\x1a\x11KIMJKGFGBDB.prot\
    o\"m\n\x0bFCELGNCENDN\x12.\n\x0bDJEFIPIFDLE\x18\x0e\x20\x01(\x0b2\x0c.KI\
    MJKGFGBDBR\x0bDJEFIPIFDLE\x12.\n\x0bGPKAJOPANAG\x18\x03\x20\x01(\x0b2\
    \x0c.IKFLJJMDIFER\x0bGPKAJOPANAGb\x06proto3\
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
            deps.push(super::IKFLJJMDIFE::file_descriptor().clone());
            deps.push(super::KIMJKGFGBDB::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(FCELGNCENDN::generated_message_descriptor_data());
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
