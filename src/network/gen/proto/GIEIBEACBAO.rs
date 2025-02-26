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

//! Generated file from `GIEIBEACBAO.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:GIEIBEACBAO)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GIEIBEACBAO {
    // message fields
    // @@protoc_insertion_point(field:GIEIBEACBAO.GCCJDHKHMNK)
    pub GCCJDHKHMNK: ::std::collections::HashMap<u32, u32>,
    // @@protoc_insertion_point(field:GIEIBEACBAO.FPBNIPMHANH)
    pub FPBNIPMHANH: u32,
    // @@protoc_insertion_point(field:GIEIBEACBAO.AAGIANCIEEG)
    pub AAGIANCIEEG: u32,
    // @@protoc_insertion_point(field:GIEIBEACBAO.ELPFOMLCOBM)
    pub ELPFOMLCOBM: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GIEIBEACBAO.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GIEIBEACBAO {
    fn default() -> &'a GIEIBEACBAO {
        <GIEIBEACBAO as ::protobuf::Message>::default_instance()
    }
}

impl GIEIBEACBAO {
    pub fn new() -> GIEIBEACBAO {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor_new::<_, _>(
            "GCCJDHKHMNK",
            |m: &GIEIBEACBAO| { &m.GCCJDHKHMNK },
            |m: &mut GIEIBEACBAO| { &mut m.GCCJDHKHMNK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FPBNIPMHANH",
            |m: &GIEIBEACBAO| { &m.FPBNIPMHANH },
            |m: &mut GIEIBEACBAO| { &mut m.FPBNIPMHANH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AAGIANCIEEG",
            |m: &GIEIBEACBAO| { &m.AAGIANCIEEG },
            |m: &mut GIEIBEACBAO| { &mut m.AAGIANCIEEG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ELPFOMLCOBM",
            |m: &GIEIBEACBAO| { &m.ELPFOMLCOBM },
            |m: &mut GIEIBEACBAO| { &mut m.ELPFOMLCOBM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GIEIBEACBAO>(
            "GIEIBEACBAO",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GIEIBEACBAO {
    const NAME: &'static str = "GIEIBEACBAO";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            16 => value = is.read_uint32()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.GCCJDHKHMNK.insert(key, value);
                },
                16 => {
                    self.FPBNIPMHANH = is.read_uint32()?;
                },
                24 => {
                    self.AAGIANCIEEG = is.read_uint32()?;
                },
                32 => {
                    self.ELPFOMLCOBM = is.read_uint32()?;
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
        for (k, v) in &self.GCCJDHKHMNK {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if self.FPBNIPMHANH != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.FPBNIPMHANH);
        }
        if self.AAGIANCIEEG != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.AAGIANCIEEG);
        }
        if self.ELPFOMLCOBM != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.ELPFOMLCOBM);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for (k, v) in &self.GCCJDHKHMNK {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(10)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        if self.FPBNIPMHANH != 0 {
            os.write_uint32(2, self.FPBNIPMHANH)?;
        }
        if self.AAGIANCIEEG != 0 {
            os.write_uint32(3, self.AAGIANCIEEG)?;
        }
        if self.ELPFOMLCOBM != 0 {
            os.write_uint32(4, self.ELPFOMLCOBM)?;
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

    fn new() -> GIEIBEACBAO {
        GIEIBEACBAO::new()
    }

    fn clear(&mut self) {
        self.GCCJDHKHMNK.clear();
        self.FPBNIPMHANH = 0;
        self.AAGIANCIEEG = 0;
        self.ELPFOMLCOBM = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GIEIBEACBAO {
        static instance: ::protobuf::rt::Lazy<GIEIBEACBAO> = ::protobuf::rt::Lazy::new();
        instance.get(GIEIBEACBAO::new)
    }
}

impl ::protobuf::MessageFull for GIEIBEACBAO {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GIEIBEACBAO").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GIEIBEACBAO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GIEIBEACBAO {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11GIEIBEACBAO.proto\"\xf4\x01\n\x0bGIEIBEACBAO\x12?\n\x0bGCCJDHKHMNK\
    \x18\x01\x20\x03(\x0b2\x1d.GIEIBEACBAO.GCCJDHKHMNKEntryR\x0bGCCJDHKHMNK\
    \x12\x20\n\x0bFPBNIPMHANH\x18\x02\x20\x01(\rR\x0bFPBNIPMHANH\x12\x20\n\
    \x0bAAGIANCIEEG\x18\x03\x20\x01(\rR\x0bAAGIANCIEEG\x12\x20\n\x0bELPFOMLC\
    OBM\x18\x04\x20\x01(\rR\x0bELPFOMLCOBM\x1a>\n\x10GCCJDHKHMNKEntry\x12\
    \x10\n\x03key\x18\x01\x20\x01(\rR\x03key\x12\x14\n\x05value\x18\x02\x20\
    \x01(\rR\x05value:\x028\x01b\x06proto3\
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
            messages.push(GIEIBEACBAO::generated_message_descriptor_data());
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
