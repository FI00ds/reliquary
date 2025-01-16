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

//! Generated file from `TravelBrochureGetDataScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:TravelBrochureGetDataScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TravelBrochureGetDataScRsp {
    // message fields
    // @@protoc_insertion_point(field:TravelBrochureGetDataScRsp.PODBFAMOHGC)
    pub PODBFAMOHGC: ::std::collections::HashMap<u32, u32>,
    // @@protoc_insertion_point(field:TravelBrochureGetDataScRsp.MPBALPGEIHI)
    pub MPBALPGEIHI: u32,
    // @@protoc_insertion_point(field:TravelBrochureGetDataScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:TravelBrochureGetDataScRsp.FJLNJJGDHLK)
    pub FJLNJJGDHLK: ::std::collections::HashMap<u32, super::BCENNALFCCO::BCENNALFCCO>,
    // special fields
    // @@protoc_insertion_point(special_field:TravelBrochureGetDataScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TravelBrochureGetDataScRsp {
    fn default() -> &'a TravelBrochureGetDataScRsp {
        <TravelBrochureGetDataScRsp as ::protobuf::Message>::default_instance()
    }
}

impl TravelBrochureGetDataScRsp {
    pub fn new() -> TravelBrochureGetDataScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "PODBFAMOHGC",
            |m: &TravelBrochureGetDataScRsp| { &m.PODBFAMOHGC },
            |m: &mut TravelBrochureGetDataScRsp| { &mut m.PODBFAMOHGC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MPBALPGEIHI",
            |m: &TravelBrochureGetDataScRsp| { &m.MPBALPGEIHI },
            |m: &mut TravelBrochureGetDataScRsp| { &mut m.MPBALPGEIHI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &TravelBrochureGetDataScRsp| { &m.retcode },
            |m: &mut TravelBrochureGetDataScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "FJLNJJGDHLK",
            |m: &TravelBrochureGetDataScRsp| { &m.FJLNJJGDHLK },
            |m: &mut TravelBrochureGetDataScRsp| { &mut m.FJLNJJGDHLK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TravelBrochureGetDataScRsp>(
            "TravelBrochureGetDataScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TravelBrochureGetDataScRsp {
    const NAME: &'static str = "TravelBrochureGetDataScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                58 => {
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
                    self.PODBFAMOHGC.insert(key, value);
                },
                8 => {
                    self.MPBALPGEIHI = is.read_uint32()?;
                },
                32 => {
                    self.retcode = is.read_uint32()?;
                },
                122 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            18 => value = is.read_message()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.FJLNJJGDHLK.insert(key, value);
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
        for (k, v) in &self.PODBFAMOHGC {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if self.MPBALPGEIHI != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.MPBALPGEIHI);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.retcode);
        }
        for (k, v) in &self.FJLNJJGDHLK {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.compute_size();
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for (k, v) in &self.PODBFAMOHGC {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(58)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        if self.MPBALPGEIHI != 0 {
            os.write_uint32(1, self.MPBALPGEIHI)?;
        }
        if self.retcode != 0 {
            os.write_uint32(4, self.retcode)?;
        }
        for (k, v) in &self.FJLNJJGDHLK {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.cached_size() as u64;
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            os.write_raw_varint32(122)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
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

    fn new() -> TravelBrochureGetDataScRsp {
        TravelBrochureGetDataScRsp::new()
    }

    fn clear(&mut self) {
        self.PODBFAMOHGC.clear();
        self.MPBALPGEIHI = 0;
        self.retcode = 0;
        self.FJLNJJGDHLK.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TravelBrochureGetDataScRsp {
        static instance: ::protobuf::rt::Lazy<TravelBrochureGetDataScRsp> = ::protobuf::rt::Lazy::new();
        instance.get(TravelBrochureGetDataScRsp::new)
    }
}

impl ::protobuf::MessageFull for TravelBrochureGetDataScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TravelBrochureGetDataScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TravelBrochureGetDataScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TravelBrochureGetDataScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20TravelBrochureGetDataScRsp.proto\x1a\x11BCENNALFCCO.proto\"\x86\
    \x03\n\x1aTravelBrochureGetDataScRsp\x12N\n\x0bPODBFAMOHGC\x18\x07\x20\
    \x03(\x0b2,.TravelBrochureGetDataScRsp.PODBFAMOHGCEntryR\x0bPODBFAMOHGC\
    \x12\x20\n\x0bMPBALPGEIHI\x18\x01\x20\x01(\rR\x0bMPBALPGEIHI\x12\x18\n\
    \x07retcode\x18\x04\x20\x01(\rR\x07retcode\x12N\n\x0bFJLNJJGDHLK\x18\x0f\
    \x20\x03(\x0b2,.TravelBrochureGetDataScRsp.FJLNJJGDHLKEntryR\x0bFJLNJJGD\
    HLK\x1a>\n\x10PODBFAMOHGCEntry\x12\x10\n\x03key\x18\x01\x20\x01(\rR\x03k\
    ey\x12\x14\n\x05value\x18\x02\x20\x01(\rR\x05value:\x028\x01\x1aL\n\x10F\
    JLNJJGDHLKEntry\x12\x10\n\x03key\x18\x01\x20\x01(\rR\x03key\x12\"\n\x05v\
    alue\x18\x02\x20\x01(\x0b2\x0c.BCENNALFCCOR\x05value:\x028\x01b\x06proto\
    3\
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
            deps.push(super::BCENNALFCCO::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(TravelBrochureGetDataScRsp::generated_message_descriptor_data());
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
