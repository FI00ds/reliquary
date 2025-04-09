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

//! Generated file from `StartCocoonStageScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:StartCocoonStageScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct StartCocoonStageScRsp {
    // message fields
    // @@protoc_insertion_point(field:StartCocoonStageScRsp.wave)
    pub wave: u32,
    // @@protoc_insertion_point(field:StartCocoonStageScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:StartCocoonStageScRsp.prop_entity_id)
    pub prop_entity_id: u32,
    // @@protoc_insertion_point(field:StartCocoonStageScRsp.battle_info)
    pub battle_info: ::protobuf::MessageField<super::CMBHDGKGPGP::CMBHDGKGPGP>,
    // @@protoc_insertion_point(field:StartCocoonStageScRsp.cocoon_id)
    pub cocoon_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:StartCocoonStageScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a StartCocoonStageScRsp {
    fn default() -> &'a StartCocoonStageScRsp {
        <StartCocoonStageScRsp as ::protobuf::Message>::default_instance()
    }
}

impl StartCocoonStageScRsp {
    pub fn new() -> StartCocoonStageScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "wave",
            |m: &StartCocoonStageScRsp| { &m.wave },
            |m: &mut StartCocoonStageScRsp| { &mut m.wave },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &StartCocoonStageScRsp| { &m.retcode },
            |m: &mut StartCocoonStageScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "prop_entity_id",
            |m: &StartCocoonStageScRsp| { &m.prop_entity_id },
            |m: &mut StartCocoonStageScRsp| { &mut m.prop_entity_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::CMBHDGKGPGP::CMBHDGKGPGP>(
            "battle_info",
            |m: &StartCocoonStageScRsp| { &m.battle_info },
            |m: &mut StartCocoonStageScRsp| { &mut m.battle_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cocoon_id",
            |m: &StartCocoonStageScRsp| { &m.cocoon_id },
            |m: &mut StartCocoonStageScRsp| { &mut m.cocoon_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<StartCocoonStageScRsp>(
            "StartCocoonStageScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for StartCocoonStageScRsp {
    const NAME: &'static str = "StartCocoonStageScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.wave = is.read_uint32()?;
                },
                96 => {
                    self.retcode = is.read_uint32()?;
                },
                40 => {
                    self.prop_entity_id = is.read_uint32()?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.battle_info)?;
                },
                64 => {
                    self.cocoon_id = is.read_uint32()?;
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
        if self.wave != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.wave);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.retcode);
        }
        if self.prop_entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.prop_entity_id);
        }
        if let Some(v) = self.battle_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.cocoon_id != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.cocoon_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.wave != 0 {
            os.write_uint32(1, self.wave)?;
        }
        if self.retcode != 0 {
            os.write_uint32(12, self.retcode)?;
        }
        if self.prop_entity_id != 0 {
            os.write_uint32(5, self.prop_entity_id)?;
        }
        if let Some(v) = self.battle_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if self.cocoon_id != 0 {
            os.write_uint32(8, self.cocoon_id)?;
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

    fn new() -> StartCocoonStageScRsp {
        StartCocoonStageScRsp::new()
    }

    fn clear(&mut self) {
        self.wave = 0;
        self.retcode = 0;
        self.prop_entity_id = 0;
        self.battle_info.clear();
        self.cocoon_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static StartCocoonStageScRsp {
        static instance: StartCocoonStageScRsp = StartCocoonStageScRsp {
            wave: 0,
            retcode: 0,
            prop_entity_id: 0,
            battle_info: ::protobuf::MessageField::none(),
            cocoon_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for StartCocoonStageScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("StartCocoonStageScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for StartCocoonStageScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StartCocoonStageScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bStartCocoonStageScRsp.proto\x1a\x11CMBHDGKGPGP.proto\"\xb7\x01\n\
    \x15StartCocoonStageScRsp\x12\x12\n\x04wave\x18\x01\x20\x01(\rR\x04wave\
    \x12\x18\n\x07retcode\x18\x0c\x20\x01(\rR\x07retcode\x12$\n\x0eprop_enti\
    ty_id\x18\x05\x20\x01(\rR\x0cpropEntityId\x12-\n\x0bbattle_info\x18\x07\
    \x20\x01(\x0b2\x0c.CMBHDGKGPGPR\nbattleInfo\x12\x1b\n\tcocoon_id\x18\x08\
    \x20\x01(\rR\x08cocoonIdb\x06proto3\
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
            deps.push(super::CMBHDGKGPGP::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(StartCocoonStageScRsp::generated_message_descriptor_data());
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
