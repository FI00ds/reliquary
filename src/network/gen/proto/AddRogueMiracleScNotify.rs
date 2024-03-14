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

//! Generated file from `AddRogueMiracleScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:AddRogueMiracleScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AddRogueMiracleScNotify {
    // message fields
    // @@protoc_insertion_point(field:AddRogueMiracleScNotify.source)
    pub source: ::protobuf::EnumOrUnknown<super::RogueMiracleSource::RogueMiracleSource>,
    // @@protoc_insertion_point(field:AddRogueMiracleScNotify.rogue_miracle)
    pub rogue_miracle: ::protobuf::MessageField<super::RogueMiracle::RogueMiracle>,
    // special fields
    // @@protoc_insertion_point(special_field:AddRogueMiracleScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AddRogueMiracleScNotify {
    fn default() -> &'a AddRogueMiracleScNotify {
        <AddRogueMiracleScNotify as ::protobuf::Message>::default_instance()
    }
}

impl AddRogueMiracleScNotify {
    pub fn new() -> AddRogueMiracleScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "source",
            |m: &AddRogueMiracleScNotify| { &m.source },
            |m: &mut AddRogueMiracleScNotify| { &mut m.source },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::RogueMiracle::RogueMiracle>(
            "rogue_miracle",
            |m: &AddRogueMiracleScNotify| { &m.rogue_miracle },
            |m: &mut AddRogueMiracleScNotify| { &mut m.rogue_miracle },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AddRogueMiracleScNotify>(
            "AddRogueMiracleScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AddRogueMiracleScNotify {
    const NAME: &'static str = "AddRogueMiracleScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.source = is.read_enum_or_unknown()?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.rogue_miracle)?;
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
        if self.source != ::protobuf::EnumOrUnknown::new(super::RogueMiracleSource::RogueMiracleSource::ROGUE_MIRACLE_SOURCE_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(15, self.source.value());
        }
        if let Some(v) = self.rogue_miracle.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.source != ::protobuf::EnumOrUnknown::new(super::RogueMiracleSource::RogueMiracleSource::ROGUE_MIRACLE_SOURCE_TYPE_NONE) {
            os.write_enum(15, ::protobuf::EnumOrUnknown::value(&self.source))?;
        }
        if let Some(v) = self.rogue_miracle.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
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

    fn new() -> AddRogueMiracleScNotify {
        AddRogueMiracleScNotify::new()
    }

    fn clear(&mut self) {
        self.source = ::protobuf::EnumOrUnknown::new(super::RogueMiracleSource::RogueMiracleSource::ROGUE_MIRACLE_SOURCE_TYPE_NONE);
        self.rogue_miracle.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AddRogueMiracleScNotify {
        static instance: AddRogueMiracleScNotify = AddRogueMiracleScNotify {
            source: ::protobuf::EnumOrUnknown::from_i32(0),
            rogue_miracle: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AddRogueMiracleScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AddRogueMiracleScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AddRogueMiracleScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AddRogueMiracleScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dAddRogueMiracleScNotify.proto\x1a\x18RogueMiracleSource.proto\x1a\
    \x12RogueMiracle.proto\"z\n\x17AddRogueMiracleScNotify\x12+\n\x06source\
    \x18\x0f\x20\x01(\x0e2\x13.RogueMiracleSourceR\x06source\x122\n\rrogue_m\
    iracle\x18\x04\x20\x01(\x0b2\r.RogueMiracleR\x0crogueMiracleB\x15\n\x13e\
    mu.lunarcore.protob\x06proto3\
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
            deps.push(super::RogueMiracleSource::file_descriptor().clone());
            deps.push(super::RogueMiracle::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AddRogueMiracleScNotify::generated_message_descriptor_data());
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
