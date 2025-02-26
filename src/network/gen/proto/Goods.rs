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

//! Generated file from `Goods.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:Goods)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Goods {
    // message fields
    // @@protoc_insertion_point(field:Goods.item_id)
    pub item_id: u32,
    // @@protoc_insertion_point(field:Goods.buy_times)
    pub buy_times: u32,
    // @@protoc_insertion_point(field:Goods.goods_id)
    pub goods_id: u32,
    // @@protoc_insertion_point(field:Goods.begin_time)
    pub begin_time: i64,
    // @@protoc_insertion_point(field:Goods.end_time)
    pub end_time: i64,
    // special fields
    // @@protoc_insertion_point(special_field:Goods.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Goods {
    fn default() -> &'a Goods {
        <Goods as ::protobuf::Message>::default_instance()
    }
}

impl Goods {
    pub fn new() -> Goods {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "item_id",
            |m: &Goods| { &m.item_id },
            |m: &mut Goods| { &mut m.item_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "buy_times",
            |m: &Goods| { &m.buy_times },
            |m: &mut Goods| { &mut m.buy_times },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "goods_id",
            |m: &Goods| { &m.goods_id },
            |m: &mut Goods| { &mut m.goods_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "begin_time",
            |m: &Goods| { &m.begin_time },
            |m: &mut Goods| { &mut m.begin_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "end_time",
            |m: &Goods| { &m.end_time },
            |m: &mut Goods| { &mut m.end_time },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Goods>(
            "Goods",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Goods {
    const NAME: &'static str = "Goods";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.item_id = is.read_uint32()?;
                },
                104 => {
                    self.buy_times = is.read_uint32()?;
                },
                88 => {
                    self.goods_id = is.read_uint32()?;
                },
                72 => {
                    self.begin_time = is.read_int64()?;
                },
                16 => {
                    self.end_time = is.read_int64()?;
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
        if self.item_id != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.item_id);
        }
        if self.buy_times != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.buy_times);
        }
        if self.goods_id != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.goods_id);
        }
        if self.begin_time != 0 {
            my_size += ::protobuf::rt::int64_size(9, self.begin_time);
        }
        if self.end_time != 0 {
            my_size += ::protobuf::rt::int64_size(2, self.end_time);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.item_id != 0 {
            os.write_uint32(8, self.item_id)?;
        }
        if self.buy_times != 0 {
            os.write_uint32(13, self.buy_times)?;
        }
        if self.goods_id != 0 {
            os.write_uint32(11, self.goods_id)?;
        }
        if self.begin_time != 0 {
            os.write_int64(9, self.begin_time)?;
        }
        if self.end_time != 0 {
            os.write_int64(2, self.end_time)?;
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

    fn new() -> Goods {
        Goods::new()
    }

    fn clear(&mut self) {
        self.item_id = 0;
        self.buy_times = 0;
        self.goods_id = 0;
        self.begin_time = 0;
        self.end_time = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Goods {
        static instance: Goods = Goods {
            item_id: 0,
            buy_times: 0,
            goods_id: 0,
            begin_time: 0,
            end_time: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Goods {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Goods").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Goods {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Goods {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0bGoods.proto\"\x92\x01\n\x05Goods\x12\x17\n\x07item_id\x18\x08\x20\
    \x01(\rR\x06itemId\x12\x1b\n\tbuy_times\x18\r\x20\x01(\rR\x08buyTimes\
    \x12\x19\n\x08goods_id\x18\x0b\x20\x01(\rR\x07goodsId\x12\x1d\n\nbegin_t\
    ime\x18\t\x20\x01(\x03R\tbeginTime\x12\x19\n\x08end_time\x18\x02\x20\x01\
    (\x03R\x07endTimeb\x06proto3\
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
            messages.push(Goods::generated_message_descriptor_data());
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
