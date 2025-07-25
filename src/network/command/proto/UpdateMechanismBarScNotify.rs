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

//! Generated file from `UpdateMechanismBarScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:UpdateMechanismBarScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct UpdateMechanismBarScNotify {
    // message fields
    // @@protoc_insertion_point(field:UpdateMechanismBarScNotify.plane_id)
    pub plane_id: u32,
    // @@protoc_insertion_point(field:UpdateMechanismBarScNotify.GIGKDAPGNME)
    pub GIGKDAPGNME: ::protobuf::MessageField<super::MechanismBarInfo::MechanismBarInfo>,
    // @@protoc_insertion_point(field:UpdateMechanismBarScNotify.floor_id)
    pub floor_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:UpdateMechanismBarScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a UpdateMechanismBarScNotify {
    fn default() -> &'a UpdateMechanismBarScNotify {
        <UpdateMechanismBarScNotify as ::protobuf::Message>::default_instance()
    }
}

impl UpdateMechanismBarScNotify {
    pub fn new() -> UpdateMechanismBarScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "plane_id",
            |m: &UpdateMechanismBarScNotify| { &m.plane_id },
            |m: &mut UpdateMechanismBarScNotify| { &mut m.plane_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MechanismBarInfo::MechanismBarInfo>(
            "GIGKDAPGNME",
            |m: &UpdateMechanismBarScNotify| { &m.GIGKDAPGNME },
            |m: &mut UpdateMechanismBarScNotify| { &mut m.GIGKDAPGNME },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "floor_id",
            |m: &UpdateMechanismBarScNotify| { &m.floor_id },
            |m: &mut UpdateMechanismBarScNotify| { &mut m.floor_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<UpdateMechanismBarScNotify>(
            "UpdateMechanismBarScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for UpdateMechanismBarScNotify {
    const NAME: &'static str = "UpdateMechanismBarScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.plane_id = is.read_uint32()?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.GIGKDAPGNME)?;
                },
                80 => {
                    self.floor_id = is.read_uint32()?;
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
        if self.plane_id != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.plane_id);
        }
        if let Some(v) = self.GIGKDAPGNME.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.floor_id != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.floor_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.plane_id != 0 {
            os.write_uint32(3, self.plane_id)?;
        }
        if let Some(v) = self.GIGKDAPGNME.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if self.floor_id != 0 {
            os.write_uint32(10, self.floor_id)?;
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

    fn new() -> UpdateMechanismBarScNotify {
        UpdateMechanismBarScNotify::new()
    }

    fn clear(&mut self) {
        self.plane_id = 0;
        self.GIGKDAPGNME.clear();
        self.floor_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static UpdateMechanismBarScNotify {
        static instance: UpdateMechanismBarScNotify = UpdateMechanismBarScNotify {
            plane_id: 0,
            GIGKDAPGNME: ::protobuf::MessageField::none(),
            floor_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for UpdateMechanismBarScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("UpdateMechanismBarScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for UpdateMechanismBarScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpdateMechanismBarScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20UpdateMechanismBarScNotify.proto\x1a\x16MechanismBarInfo.proto\"\
    \x87\x01\n\x1aUpdateMechanismBarScNotify\x12\x19\n\x08plane_id\x18\x03\
    \x20\x01(\rR\x07planeId\x123\n\x0bGIGKDAPGNME\x18\x07\x20\x01(\x0b2\x11.\
    MechanismBarInfoR\x0bGIGKDAPGNME\x12\x19\n\x08floor_id\x18\n\x20\x01(\rR\
    \x07floorIdb\x06proto3\
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
            deps.push(super::MechanismBarInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(UpdateMechanismBarScNotify::generated_message_descriptor_data());
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
