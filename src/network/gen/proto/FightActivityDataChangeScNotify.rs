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

//! Generated file from `FightActivityDataChangeScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:FightActivityDataChangeScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FightActivityDataChangeScNotify {
    // message fields
    // @@protoc_insertion_point(field:FightActivityDataChangeScNotify.JKHIFDGHJDO)
    pub JKHIFDGHJDO: ::std::vec::Vec<super::FightActivityGroup::FightActivityGroup>,
    // @@protoc_insertion_point(field:FightActivityDataChangeScNotify.DGNFCMDJOPA)
    pub DGNFCMDJOPA: ::std::collections::HashMap<u32, u32>,
    // special fields
    // @@protoc_insertion_point(special_field:FightActivityDataChangeScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FightActivityDataChangeScNotify {
    fn default() -> &'a FightActivityDataChangeScNotify {
        <FightActivityDataChangeScNotify as ::protobuf::Message>::default_instance()
    }
}

impl FightActivityDataChangeScNotify {
    pub fn new() -> FightActivityDataChangeScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JKHIFDGHJDO",
            |m: &FightActivityDataChangeScNotify| { &m.JKHIFDGHJDO },
            |m: &mut FightActivityDataChangeScNotify| { &mut m.JKHIFDGHJDO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor_new::<_, _>(
            "DGNFCMDJOPA",
            |m: &FightActivityDataChangeScNotify| { &m.DGNFCMDJOPA },
            |m: &mut FightActivityDataChangeScNotify| { &mut m.DGNFCMDJOPA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FightActivityDataChangeScNotify>(
            "FightActivityDataChangeScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FightActivityDataChangeScNotify {
    const NAME: &'static str = "FightActivityDataChangeScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                50 => {
                    self.JKHIFDGHJDO.push(is.read_message()?);
                },
                66 => {
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
        for value in &self.JKHIFDGHJDO {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
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
        for v in &self.JKHIFDGHJDO {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        };
        for (k, v) in &self.DGNFCMDJOPA {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(66)?; // Tag.
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

    fn new() -> FightActivityDataChangeScNotify {
        FightActivityDataChangeScNotify::new()
    }

    fn clear(&mut self) {
        self.JKHIFDGHJDO.clear();
        self.DGNFCMDJOPA.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FightActivityDataChangeScNotify {
        static instance: ::protobuf::rt::Lazy<FightActivityDataChangeScNotify> = ::protobuf::rt::Lazy::new();
        instance.get(FightActivityDataChangeScNotify::new)
    }
}

impl ::protobuf::MessageFull for FightActivityDataChangeScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FightActivityDataChangeScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FightActivityDataChangeScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FightActivityDataChangeScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n%FightActivityDataChangeScNotify.proto\x1a\x18FightActivityGroup.proto\
    \"\xed\x01\n\x1fFightActivityDataChangeScNotify\x125\n\x0bJKHIFDGHJDO\
    \x18\x06\x20\x03(\x0b2\x13.FightActivityGroupR\x0bJKHIFDGHJDO\x12S\n\x0b\
    DGNFCMDJOPA\x18\x08\x20\x03(\x0b21.FightActivityDataChangeScNotify.DGNFC\
    MDJOPAEntryR\x0bDGNFCMDJOPA\x1a>\n\x10DGNFCMDJOPAEntry\x12\x10\n\x03key\
    \x18\x01\x20\x01(\rR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\rR\x05va\
    lue:\x028\x01b\x06proto3\
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
            messages.push(FightActivityDataChangeScNotify::generated_message_descriptor_data());
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
