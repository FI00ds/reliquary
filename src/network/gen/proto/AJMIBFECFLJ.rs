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

//! Generated file from `AJMIBFECFLJ.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:AJMIBFECFLJ)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AJMIBFECFLJ {
    // message fields
    // @@protoc_insertion_point(field:AJMIBFECFLJ.CBOGMIHHMOP)
    pub CBOGMIHHMOP: u32,
    // @@protoc_insertion_point(field:AJMIBFECFLJ.HCPACBHCMHN)
    pub HCPACBHCMHN: ::std::collections::HashMap<u32, u32>,
    // @@protoc_insertion_point(field:AJMIBFECFLJ.HAKLFFPFOKE)
    pub HAKLFFPFOKE: u32,
    // special fields
    // @@protoc_insertion_point(special_field:AJMIBFECFLJ.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AJMIBFECFLJ {
    fn default() -> &'a AJMIBFECFLJ {
        <AJMIBFECFLJ as ::protobuf::Message>::default_instance()
    }
}

impl AJMIBFECFLJ {
    pub fn new() -> AJMIBFECFLJ {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CBOGMIHHMOP",
            |m: &AJMIBFECFLJ| { &m.CBOGMIHHMOP },
            |m: &mut AJMIBFECFLJ| { &mut m.CBOGMIHHMOP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor_new::<_, _>(
            "HCPACBHCMHN",
            |m: &AJMIBFECFLJ| { &m.HCPACBHCMHN },
            |m: &mut AJMIBFECFLJ| { &mut m.HCPACBHCMHN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HAKLFFPFOKE",
            |m: &AJMIBFECFLJ| { &m.HAKLFFPFOKE },
            |m: &mut AJMIBFECFLJ| { &mut m.HAKLFFPFOKE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AJMIBFECFLJ>(
            "AJMIBFECFLJ",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AJMIBFECFLJ {
    const NAME: &'static str = "AJMIBFECFLJ";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.CBOGMIHHMOP = is.read_uint32()?;
                },
                122 => {
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
                    self.HCPACBHCMHN.insert(key, value);
                },
                48 => {
                    self.HAKLFFPFOKE = is.read_uint32()?;
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
        if self.CBOGMIHHMOP != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.CBOGMIHHMOP);
        }
        for (k, v) in &self.HCPACBHCMHN {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if self.HAKLFFPFOKE != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.HAKLFFPFOKE);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.CBOGMIHHMOP != 0 {
            os.write_uint32(1, self.CBOGMIHHMOP)?;
        }
        for (k, v) in &self.HCPACBHCMHN {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(122)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        if self.HAKLFFPFOKE != 0 {
            os.write_uint32(6, self.HAKLFFPFOKE)?;
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

    fn new() -> AJMIBFECFLJ {
        AJMIBFECFLJ::new()
    }

    fn clear(&mut self) {
        self.CBOGMIHHMOP = 0;
        self.HCPACBHCMHN.clear();
        self.HAKLFFPFOKE = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AJMIBFECFLJ {
        static instance: ::protobuf::rt::Lazy<AJMIBFECFLJ> = ::protobuf::rt::Lazy::new();
        instance.get(AJMIBFECFLJ::new)
    }
}

impl ::protobuf::MessageFull for AJMIBFECFLJ {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AJMIBFECFLJ").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AJMIBFECFLJ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AJMIBFECFLJ {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11AJMIBFECFLJ.proto\"\xd2\x01\n\x0bAJMIBFECFLJ\x12\x20\n\x0bCBOGMIHH\
    MOP\x18\x01\x20\x01(\rR\x0bCBOGMIHHMOP\x12?\n\x0bHCPACBHCMHN\x18\x0f\x20\
    \x03(\x0b2\x1d.AJMIBFECFLJ.HCPACBHCMHNEntryR\x0bHCPACBHCMHN\x12\x20\n\
    \x0bHAKLFFPFOKE\x18\x06\x20\x01(\rR\x0bHAKLFFPFOKE\x1a>\n\x10HCPACBHCMHN\
    Entry\x12\x10\n\x03key\x18\x01\x20\x01(\rR\x03key\x12\x14\n\x05value\x18\
    \x02\x20\x01(\rR\x05value:\x028\x01b\x06proto3\
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
            messages.push(AJMIBFECFLJ::generated_message_descriptor_data());
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
