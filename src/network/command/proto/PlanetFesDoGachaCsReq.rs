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

//! Generated file from `PlanetFesDoGachaCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:PlanetFesDoGachaCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PlanetFesDoGachaCsReq {
    // message fields
    // @@protoc_insertion_point(field:PlanetFesDoGachaCsReq.gacha_id)
    pub gacha_id: u32,
    // @@protoc_insertion_point(field:PlanetFesDoGachaCsReq.gacha_count)
    pub gacha_count: u32,
    // @@protoc_insertion_point(field:PlanetFesDoGachaCsReq.simulate_magic)
    pub simulate_magic: u32,
    // special fields
    // @@protoc_insertion_point(special_field:PlanetFesDoGachaCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PlanetFesDoGachaCsReq {
    fn default() -> &'a PlanetFesDoGachaCsReq {
        <PlanetFesDoGachaCsReq as ::protobuf::Message>::default_instance()
    }
}

impl PlanetFesDoGachaCsReq {
    pub fn new() -> PlanetFesDoGachaCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "gacha_id",
            |m: &PlanetFesDoGachaCsReq| { &m.gacha_id },
            |m: &mut PlanetFesDoGachaCsReq| { &mut m.gacha_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "gacha_count",
            |m: &PlanetFesDoGachaCsReq| { &m.gacha_count },
            |m: &mut PlanetFesDoGachaCsReq| { &mut m.gacha_count },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "simulate_magic",
            |m: &PlanetFesDoGachaCsReq| { &m.simulate_magic },
            |m: &mut PlanetFesDoGachaCsReq| { &mut m.simulate_magic },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PlanetFesDoGachaCsReq>(
            "PlanetFesDoGachaCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PlanetFesDoGachaCsReq {
    const NAME: &'static str = "PlanetFesDoGachaCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                112 => {
                    self.gacha_id = is.read_uint32()?;
                },
                48 => {
                    self.gacha_count = is.read_uint32()?;
                },
                120 => {
                    self.simulate_magic = is.read_uint32()?;
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
        if self.gacha_id != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.gacha_id);
        }
        if self.gacha_count != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.gacha_count);
        }
        if self.simulate_magic != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.simulate_magic);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.gacha_id != 0 {
            os.write_uint32(14, self.gacha_id)?;
        }
        if self.gacha_count != 0 {
            os.write_uint32(6, self.gacha_count)?;
        }
        if self.simulate_magic != 0 {
            os.write_uint32(15, self.simulate_magic)?;
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

    fn new() -> PlanetFesDoGachaCsReq {
        PlanetFesDoGachaCsReq::new()
    }

    fn clear(&mut self) {
        self.gacha_id = 0;
        self.gacha_count = 0;
        self.simulate_magic = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PlanetFesDoGachaCsReq {
        static instance: PlanetFesDoGachaCsReq = PlanetFesDoGachaCsReq {
            gacha_id: 0,
            gacha_count: 0,
            simulate_magic: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PlanetFesDoGachaCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PlanetFesDoGachaCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PlanetFesDoGachaCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlanetFesDoGachaCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bPlanetFesDoGachaCsReq.proto\"z\n\x15PlanetFesDoGachaCsReq\x12\x19\
    \n\x08gacha_id\x18\x0e\x20\x01(\rR\x07gachaId\x12\x1f\n\x0bgacha_count\
    \x18\x06\x20\x01(\rR\ngachaCount\x12%\n\x0esimulate_magic\x18\x0f\x20\
    \x01(\rR\rsimulateMagicb\x06proto3\
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
            messages.push(PlanetFesDoGachaCsReq::generated_message_descriptor_data());
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
