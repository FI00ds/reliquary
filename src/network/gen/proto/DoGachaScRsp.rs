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

//! Generated file from `DoGachaScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:DoGachaScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DoGachaScRsp {
    // message fields
    // @@protoc_insertion_point(field:DoGachaScRsp.gacha_id)
    pub gacha_id: u32,
    // @@protoc_insertion_point(field:DoGachaScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:DoGachaScRsp.ceiling_num)
    pub ceiling_num: u32,
    // @@protoc_insertion_point(field:DoGachaScRsp.gacha_item_list)
    pub gacha_item_list: ::std::vec::Vec<super::GachaItem::GachaItem>,
    // @@protoc_insertion_point(field:DoGachaScRsp.gacha_num)
    pub gacha_num: u32,
    // special fields
    // @@protoc_insertion_point(special_field:DoGachaScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DoGachaScRsp {
    fn default() -> &'a DoGachaScRsp {
        <DoGachaScRsp as ::protobuf::Message>::default_instance()
    }
}

impl DoGachaScRsp {
    pub fn new() -> DoGachaScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "gacha_id",
            |m: &DoGachaScRsp| { &m.gacha_id },
            |m: &mut DoGachaScRsp| { &mut m.gacha_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &DoGachaScRsp| { &m.retcode },
            |m: &mut DoGachaScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ceiling_num",
            |m: &DoGachaScRsp| { &m.ceiling_num },
            |m: &mut DoGachaScRsp| { &mut m.ceiling_num },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "gacha_item_list",
            |m: &DoGachaScRsp| { &m.gacha_item_list },
            |m: &mut DoGachaScRsp| { &mut m.gacha_item_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "gacha_num",
            |m: &DoGachaScRsp| { &m.gacha_num },
            |m: &mut DoGachaScRsp| { &mut m.gacha_num },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DoGachaScRsp>(
            "DoGachaScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DoGachaScRsp {
    const NAME: &'static str = "DoGachaScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.gacha_id = is.read_uint32()?;
                },
                96 => {
                    self.retcode = is.read_uint32()?;
                },
                72 => {
                    self.ceiling_num = is.read_uint32()?;
                },
                26 => {
                    self.gacha_item_list.push(is.read_message()?);
                },
                16 => {
                    self.gacha_num = is.read_uint32()?;
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
            my_size += ::protobuf::rt::uint32_size(13, self.gacha_id);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.retcode);
        }
        if self.ceiling_num != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.ceiling_num);
        }
        for value in &self.gacha_item_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.gacha_num != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.gacha_num);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.gacha_id != 0 {
            os.write_uint32(13, self.gacha_id)?;
        }
        if self.retcode != 0 {
            os.write_uint32(12, self.retcode)?;
        }
        if self.ceiling_num != 0 {
            os.write_uint32(9, self.ceiling_num)?;
        }
        for v in &self.gacha_item_list {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        if self.gacha_num != 0 {
            os.write_uint32(2, self.gacha_num)?;
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

    fn new() -> DoGachaScRsp {
        DoGachaScRsp::new()
    }

    fn clear(&mut self) {
        self.gacha_id = 0;
        self.retcode = 0;
        self.ceiling_num = 0;
        self.gacha_item_list.clear();
        self.gacha_num = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DoGachaScRsp {
        static instance: DoGachaScRsp = DoGachaScRsp {
            gacha_id: 0,
            retcode: 0,
            ceiling_num: 0,
            gacha_item_list: ::std::vec::Vec::new(),
            gacha_num: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DoGachaScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DoGachaScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DoGachaScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DoGachaScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12DoGachaScRsp.proto\x1a\x0fGachaItem.proto\"\xb5\x01\n\x0cDoGachaSc\
    Rsp\x12\x19\n\x08gacha_id\x18\r\x20\x01(\rR\x07gachaId\x12\x18\n\x07retc\
    ode\x18\x0c\x20\x01(\rR\x07retcode\x12\x1f\n\x0bceiling_num\x18\t\x20\
    \x01(\rR\nceilingNum\x122\n\x0fgacha_item_list\x18\x03\x20\x03(\x0b2\n.G\
    achaItemR\rgachaItemList\x12\x1b\n\tgacha_num\x18\x02\x20\x01(\rR\x08gac\
    haNumB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            deps.push(super::GachaItem::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(DoGachaScRsp::generated_message_descriptor_data());
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
