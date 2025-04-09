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

//! Generated file from `AddMultiPathAvatarScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:AddMultiPathAvatarScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AddMultiPathAvatarScNotify {
    // message fields
    // @@protoc_insertion_point(field:AddMultiPathAvatarScNotify.BHELBOHKBBM)
    pub BHELBOHKBBM: ::protobuf::MessageField<super::ItemList::ItemList>,
    // @@protoc_insertion_point(field:AddMultiPathAvatarScNotify.AEDKJJIBAHK)
    pub AEDKJJIBAHK: bool,
    // @@protoc_insertion_point(field:AddMultiPathAvatarScNotify.avatar_id)
    pub avatar_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:AddMultiPathAvatarScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AddMultiPathAvatarScNotify {
    fn default() -> &'a AddMultiPathAvatarScNotify {
        <AddMultiPathAvatarScNotify as ::protobuf::Message>::default_instance()
    }
}

impl AddMultiPathAvatarScNotify {
    pub fn new() -> AddMultiPathAvatarScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemList::ItemList>(
            "BHELBOHKBBM",
            |m: &AddMultiPathAvatarScNotify| { &m.BHELBOHKBBM },
            |m: &mut AddMultiPathAvatarScNotify| { &mut m.BHELBOHKBBM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AEDKJJIBAHK",
            |m: &AddMultiPathAvatarScNotify| { &m.AEDKJJIBAHK },
            |m: &mut AddMultiPathAvatarScNotify| { &mut m.AEDKJJIBAHK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_id",
            |m: &AddMultiPathAvatarScNotify| { &m.avatar_id },
            |m: &mut AddMultiPathAvatarScNotify| { &mut m.avatar_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AddMultiPathAvatarScNotify>(
            "AddMultiPathAvatarScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AddMultiPathAvatarScNotify {
    const NAME: &'static str = "AddMultiPathAvatarScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.BHELBOHKBBM)?;
                },
                24 => {
                    self.AEDKJJIBAHK = is.read_bool()?;
                },
                80 => {
                    self.avatar_id = is.read_uint32()?;
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
        if let Some(v) = self.BHELBOHKBBM.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.AEDKJJIBAHK != false {
            my_size += 1 + 1;
        }
        if self.avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.avatar_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.BHELBOHKBBM.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        if self.AEDKJJIBAHK != false {
            os.write_bool(3, self.AEDKJJIBAHK)?;
        }
        if self.avatar_id != 0 {
            os.write_uint32(10, self.avatar_id)?;
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

    fn new() -> AddMultiPathAvatarScNotify {
        AddMultiPathAvatarScNotify::new()
    }

    fn clear(&mut self) {
        self.BHELBOHKBBM.clear();
        self.AEDKJJIBAHK = false;
        self.avatar_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AddMultiPathAvatarScNotify {
        static instance: AddMultiPathAvatarScNotify = AddMultiPathAvatarScNotify {
            BHELBOHKBBM: ::protobuf::MessageField::none(),
            AEDKJJIBAHK: false,
            avatar_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AddMultiPathAvatarScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AddMultiPathAvatarScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AddMultiPathAvatarScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AddMultiPathAvatarScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20AddMultiPathAvatarScNotify.proto\x1a\x0eItemList.proto\"\x88\x01\n\
    \x1aAddMultiPathAvatarScNotify\x12+\n\x0bBHELBOHKBBM\x18\x0e\x20\x01(\
    \x0b2\t.ItemListR\x0bBHELBOHKBBM\x12\x20\n\x0bAEDKJJIBAHK\x18\x03\x20\
    \x01(\x08R\x0bAEDKJJIBAHK\x12\x1b\n\tavatar_id\x18\n\x20\x01(\rR\x08avat\
    arIdb\x06proto3\
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
            deps.push(super::ItemList::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AddMultiPathAvatarScNotify::generated_message_descriptor_data());
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
