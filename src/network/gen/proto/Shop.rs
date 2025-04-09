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

//! Generated file from `Shop.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:Shop)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Shop {
    // message fields
    // @@protoc_insertion_point(field:Shop.goods_list)
    pub goods_list: ::std::vec::Vec<super::Goods::Goods>,
    // @@protoc_insertion_point(field:Shop.begin_time)
    pub begin_time: i64,
    // @@protoc_insertion_point(field:Shop.city_taken_level_reward)
    pub city_taken_level_reward: u64,
    // @@protoc_insertion_point(field:Shop.end_time)
    pub end_time: i64,
    // @@protoc_insertion_point(field:Shop.city_exp)
    pub city_exp: u32,
    // @@protoc_insertion_point(field:Shop.city_level)
    pub city_level: u32,
    // @@protoc_insertion_point(field:Shop.shop_id)
    pub shop_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:Shop.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Shop {
    fn default() -> &'a Shop {
        <Shop as ::protobuf::Message>::default_instance()
    }
}

impl Shop {
    pub fn new() -> Shop {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "goods_list",
            |m: &Shop| { &m.goods_list },
            |m: &mut Shop| { &mut m.goods_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "begin_time",
            |m: &Shop| { &m.begin_time },
            |m: &mut Shop| { &mut m.begin_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "city_taken_level_reward",
            |m: &Shop| { &m.city_taken_level_reward },
            |m: &mut Shop| { &mut m.city_taken_level_reward },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "end_time",
            |m: &Shop| { &m.end_time },
            |m: &mut Shop| { &mut m.end_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "city_exp",
            |m: &Shop| { &m.city_exp },
            |m: &mut Shop| { &mut m.city_exp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "city_level",
            |m: &Shop| { &m.city_level },
            |m: &mut Shop| { &mut m.city_level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "shop_id",
            |m: &Shop| { &m.shop_id },
            |m: &mut Shop| { &mut m.shop_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Shop>(
            "Shop",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Shop {
    const NAME: &'static str = "Shop";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                18 => {
                    self.goods_list.push(is.read_message()?);
                },
                96 => {
                    self.begin_time = is.read_int64()?;
                },
                112 => {
                    self.city_taken_level_reward = is.read_uint64()?;
                },
                72 => {
                    self.end_time = is.read_int64()?;
                },
                24 => {
                    self.city_exp = is.read_uint32()?;
                },
                32 => {
                    self.city_level = is.read_uint32()?;
                },
                80 => {
                    self.shop_id = is.read_uint32()?;
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
        for value in &self.goods_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.begin_time != 0 {
            my_size += ::protobuf::rt::int64_size(12, self.begin_time);
        }
        if self.city_taken_level_reward != 0 {
            my_size += ::protobuf::rt::uint64_size(14, self.city_taken_level_reward);
        }
        if self.end_time != 0 {
            my_size += ::protobuf::rt::int64_size(9, self.end_time);
        }
        if self.city_exp != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.city_exp);
        }
        if self.city_level != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.city_level);
        }
        if self.shop_id != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.shop_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.goods_list {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        if self.begin_time != 0 {
            os.write_int64(12, self.begin_time)?;
        }
        if self.city_taken_level_reward != 0 {
            os.write_uint64(14, self.city_taken_level_reward)?;
        }
        if self.end_time != 0 {
            os.write_int64(9, self.end_time)?;
        }
        if self.city_exp != 0 {
            os.write_uint32(3, self.city_exp)?;
        }
        if self.city_level != 0 {
            os.write_uint32(4, self.city_level)?;
        }
        if self.shop_id != 0 {
            os.write_uint32(10, self.shop_id)?;
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

    fn new() -> Shop {
        Shop::new()
    }

    fn clear(&mut self) {
        self.goods_list.clear();
        self.begin_time = 0;
        self.city_taken_level_reward = 0;
        self.end_time = 0;
        self.city_exp = 0;
        self.city_level = 0;
        self.shop_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Shop {
        static instance: Shop = Shop {
            goods_list: ::std::vec::Vec::new(),
            begin_time: 0,
            city_taken_level_reward: 0,
            end_time: 0,
            city_exp: 0,
            city_level: 0,
            shop_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Shop {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Shop").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Shop {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Shop {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\nShop.proto\x1a\x0bGoods.proto\"\xf1\x01\n\x04Shop\x12%\n\ngoods_list\
    \x18\x02\x20\x03(\x0b2\x06.GoodsR\tgoodsList\x12\x1d\n\nbegin_time\x18\
    \x0c\x20\x01(\x03R\tbeginTime\x125\n\x17city_taken_level_reward\x18\x0e\
    \x20\x01(\x04R\x14cityTakenLevelReward\x12\x19\n\x08end_time\x18\t\x20\
    \x01(\x03R\x07endTime\x12\x19\n\x08city_exp\x18\x03\x20\x01(\rR\x07cityE\
    xp\x12\x1d\n\ncity_level\x18\x04\x20\x01(\rR\tcityLevel\x12\x17\n\x07sho\
    p_id\x18\n\x20\x01(\rR\x06shopIdb\x06proto3\
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
            deps.push(super::Goods::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Shop::generated_message_descriptor_data());
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
