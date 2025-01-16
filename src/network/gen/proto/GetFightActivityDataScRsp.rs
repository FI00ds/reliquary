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

//! Generated file from `GetFightActivityDataScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GetFightActivityDataScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetFightActivityDataScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetFightActivityDataScRsp.EBKEMIEJMKM)
    pub EBKEMIEJMKM: ::std::vec::Vec<super::FightActivityGroup::FightActivityGroup>,
    // @@protoc_insertion_point(field:GetFightActivityDataScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:GetFightActivityDataScRsp.CPLOLHNJAHO)
    pub CPLOLHNJAHO: bool,
    // @@protoc_insertion_point(field:GetFightActivityDataScRsp.KHGBGLLGBFE)
    pub KHGBGLLGBFE: ::std::collections::HashMap<u32, u32>,
    // @@protoc_insertion_point(field:GetFightActivityDataScRsp.CFNJJEJIGOK)
    pub CFNJJEJIGOK: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GetFightActivityDataScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetFightActivityDataScRsp {
    fn default() -> &'a GetFightActivityDataScRsp {
        <GetFightActivityDataScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetFightActivityDataScRsp {
    pub fn new() -> GetFightActivityDataScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EBKEMIEJMKM",
            |m: &GetFightActivityDataScRsp| { &m.EBKEMIEJMKM },
            |m: &mut GetFightActivityDataScRsp| { &mut m.EBKEMIEJMKM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetFightActivityDataScRsp| { &m.retcode },
            |m: &mut GetFightActivityDataScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CPLOLHNJAHO",
            |m: &GetFightActivityDataScRsp| { &m.CPLOLHNJAHO },
            |m: &mut GetFightActivityDataScRsp| { &mut m.CPLOLHNJAHO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "KHGBGLLGBFE",
            |m: &GetFightActivityDataScRsp| { &m.KHGBGLLGBFE },
            |m: &mut GetFightActivityDataScRsp| { &mut m.KHGBGLLGBFE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CFNJJEJIGOK",
            |m: &GetFightActivityDataScRsp| { &m.CFNJJEJIGOK },
            |m: &mut GetFightActivityDataScRsp| { &mut m.CFNJJEJIGOK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetFightActivityDataScRsp>(
            "GetFightActivityDataScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetFightActivityDataScRsp {
    const NAME: &'static str = "GetFightActivityDataScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                42 => {
                    self.EBKEMIEJMKM.push(is.read_message()?);
                },
                8 => {
                    self.retcode = is.read_uint32()?;
                },
                88 => {
                    self.CPLOLHNJAHO = is.read_bool()?;
                },
                26 => {
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
                    self.KHGBGLLGBFE.insert(key, value);
                },
                16 => {
                    self.CFNJJEJIGOK = is.read_uint32()?;
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
        for value in &self.EBKEMIEJMKM {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.retcode);
        }
        if self.CPLOLHNJAHO != false {
            my_size += 1 + 1;
        }
        for (k, v) in &self.KHGBGLLGBFE {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if self.CFNJJEJIGOK != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.CFNJJEJIGOK);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.EBKEMIEJMKM {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        if self.retcode != 0 {
            os.write_uint32(1, self.retcode)?;
        }
        if self.CPLOLHNJAHO != false {
            os.write_bool(11, self.CPLOLHNJAHO)?;
        }
        for (k, v) in &self.KHGBGLLGBFE {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(26)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        if self.CFNJJEJIGOK != 0 {
            os.write_uint32(2, self.CFNJJEJIGOK)?;
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

    fn new() -> GetFightActivityDataScRsp {
        GetFightActivityDataScRsp::new()
    }

    fn clear(&mut self) {
        self.EBKEMIEJMKM.clear();
        self.retcode = 0;
        self.CPLOLHNJAHO = false;
        self.KHGBGLLGBFE.clear();
        self.CFNJJEJIGOK = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetFightActivityDataScRsp {
        static instance: ::protobuf::rt::Lazy<GetFightActivityDataScRsp> = ::protobuf::rt::Lazy::new();
        instance.get(GetFightActivityDataScRsp::new)
    }
}

impl ::protobuf::MessageFull for GetFightActivityDataScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetFightActivityDataScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetFightActivityDataScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetFightActivityDataScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1fGetFightActivityDataScRsp.proto\x1a\x18FightActivityGroup.proto\"\
    \xbf\x02\n\x19GetFightActivityDataScRsp\x125\n\x0bEBKEMIEJMKM\x18\x05\
    \x20\x03(\x0b2\x13.FightActivityGroupR\x0bEBKEMIEJMKM\x12\x18\n\x07retco\
    de\x18\x01\x20\x01(\rR\x07retcode\x12\x20\n\x0bCPLOLHNJAHO\x18\x0b\x20\
    \x01(\x08R\x0bCPLOLHNJAHO\x12M\n\x0bKHGBGLLGBFE\x18\x03\x20\x03(\x0b2+.G\
    etFightActivityDataScRsp.KHGBGLLGBFEEntryR\x0bKHGBGLLGBFE\x12\x20\n\x0bC\
    FNJJEJIGOK\x18\x02\x20\x01(\rR\x0bCFNJJEJIGOK\x1a>\n\x10KHGBGLLGBFEEntry\
    \x12\x10\n\x03key\x18\x01\x20\x01(\rR\x03key\x12\x14\n\x05value\x18\x02\
    \x20\x01(\rR\x05value:\x028\x01b\x06proto3\
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
            deps.push(super::FightActivityGroup::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetFightActivityDataScRsp::generated_message_descriptor_data());
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
