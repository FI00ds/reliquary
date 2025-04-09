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

//! Generated file from `GetFightActivityDataScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:GetFightActivityDataScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetFightActivityDataScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetFightActivityDataScRsp.IFJFCEJJBPE)
    pub IFJFCEJJBPE: u32,
    // @@protoc_insertion_point(field:GetFightActivityDataScRsp.JKHIFDGHJDO)
    pub JKHIFDGHJDO: ::std::vec::Vec<super::FightActivityGroup::FightActivityGroup>,
    // @@protoc_insertion_point(field:GetFightActivityDataScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:GetFightActivityDataScRsp.KAIOMPFBGKL)
    pub KAIOMPFBGKL: bool,
    // @@protoc_insertion_point(field:GetFightActivityDataScRsp.DGNFCMDJOPA)
    pub DGNFCMDJOPA: ::std::collections::HashMap<u32, u32>,
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
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IFJFCEJJBPE",
            |m: &GetFightActivityDataScRsp| { &m.IFJFCEJJBPE },
            |m: &mut GetFightActivityDataScRsp| { &mut m.IFJFCEJJBPE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JKHIFDGHJDO",
            |m: &GetFightActivityDataScRsp| { &m.JKHIFDGHJDO },
            |m: &mut GetFightActivityDataScRsp| { &mut m.JKHIFDGHJDO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetFightActivityDataScRsp| { &m.retcode },
            |m: &mut GetFightActivityDataScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KAIOMPFBGKL",
            |m: &GetFightActivityDataScRsp| { &m.KAIOMPFBGKL },
            |m: &mut GetFightActivityDataScRsp| { &mut m.KAIOMPFBGKL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor_new::<_, _>(
            "DGNFCMDJOPA",
            |m: &GetFightActivityDataScRsp| { &m.DGNFCMDJOPA },
            |m: &mut GetFightActivityDataScRsp| { &mut m.DGNFCMDJOPA },
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
                40 => {
                    self.IFJFCEJJBPE = is.read_uint32()?;
                },
                10 => {
                    self.JKHIFDGHJDO.push(is.read_message()?);
                },
                32 => {
                    self.retcode = is.read_uint32()?;
                },
                24 => {
                    self.KAIOMPFBGKL = is.read_bool()?;
                },
                90 => {
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
                    self.DGNFCMDJOPA.insert(key, value);
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
        if self.IFJFCEJJBPE != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.IFJFCEJJBPE);
        }
        for value in &self.JKHIFDGHJDO {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.retcode);
        }
        if self.KAIOMPFBGKL != false {
            my_size += 1 + 1;
        }
        for (k, v) in &self.DGNFCMDJOPA {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IFJFCEJJBPE != 0 {
            os.write_uint32(5, self.IFJFCEJJBPE)?;
        }
        for v in &self.JKHIFDGHJDO {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        if self.retcode != 0 {
            os.write_uint32(4, self.retcode)?;
        }
        if self.KAIOMPFBGKL != false {
            os.write_bool(3, self.KAIOMPFBGKL)?;
        }
        for (k, v) in &self.DGNFCMDJOPA {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(90)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
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

    fn new() -> GetFightActivityDataScRsp {
        GetFightActivityDataScRsp::new()
    }

    fn clear(&mut self) {
        self.IFJFCEJJBPE = 0;
        self.JKHIFDGHJDO.clear();
        self.retcode = 0;
        self.KAIOMPFBGKL = false;
        self.DGNFCMDJOPA.clear();
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
    \xbf\x02\n\x19GetFightActivityDataScRsp\x12\x20\n\x0bIFJFCEJJBPE\x18\x05\
    \x20\x01(\rR\x0bIFJFCEJJBPE\x125\n\x0bJKHIFDGHJDO\x18\x01\x20\x03(\x0b2\
    \x13.FightActivityGroupR\x0bJKHIFDGHJDO\x12\x18\n\x07retcode\x18\x04\x20\
    \x01(\rR\x07retcode\x12\x20\n\x0bKAIOMPFBGKL\x18\x03\x20\x01(\x08R\x0bKA\
    IOMPFBGKL\x12M\n\x0bDGNFCMDJOPA\x18\x0b\x20\x03(\x0b2+.GetFightActivityD\
    ataScRsp.DGNFCMDJOPAEntryR\x0bDGNFCMDJOPA\x1a>\n\x10DGNFCMDJOPAEntry\x12\
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
