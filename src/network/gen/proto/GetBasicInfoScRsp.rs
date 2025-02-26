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

//! Generated file from `GetBasicInfoScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:GetBasicInfoScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetBasicInfoScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetBasicInfoScRsp.week_cocoon_finished_count)
    pub week_cocoon_finished_count: u32,
    // @@protoc_insertion_point(field:GetBasicInfoScRsp.gameplay_birthday)
    pub gameplay_birthday: u32,
    // @@protoc_insertion_point(field:GetBasicInfoScRsp.is_gender_set)
    pub is_gender_set: bool,
    // @@protoc_insertion_point(field:GetBasicInfoScRsp.cur_day)
    pub cur_day: u32,
    // @@protoc_insertion_point(field:GetBasicInfoScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:GetBasicInfoScRsp.last_set_nickname_time)
    pub last_set_nickname_time: i64,
    // @@protoc_insertion_point(field:GetBasicInfoScRsp.player_setting_info)
    pub player_setting_info: ::protobuf::MessageField<super::DJBLLOKKAND::DJBLLOKKAND>,
    // @@protoc_insertion_point(field:GetBasicInfoScRsp.next_recover_time)
    pub next_recover_time: i64,
    // @@protoc_insertion_point(field:GetBasicInfoScRsp.exchange_times)
    pub exchange_times: u32,
    // @@protoc_insertion_point(field:GetBasicInfoScRsp.gender)
    pub gender: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GetBasicInfoScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetBasicInfoScRsp {
    fn default() -> &'a GetBasicInfoScRsp {
        <GetBasicInfoScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetBasicInfoScRsp {
    pub fn new() -> GetBasicInfoScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(10);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "week_cocoon_finished_count",
            |m: &GetBasicInfoScRsp| { &m.week_cocoon_finished_count },
            |m: &mut GetBasicInfoScRsp| { &mut m.week_cocoon_finished_count },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "gameplay_birthday",
            |m: &GetBasicInfoScRsp| { &m.gameplay_birthday },
            |m: &mut GetBasicInfoScRsp| { &mut m.gameplay_birthday },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_gender_set",
            |m: &GetBasicInfoScRsp| { &m.is_gender_set },
            |m: &mut GetBasicInfoScRsp| { &mut m.is_gender_set },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cur_day",
            |m: &GetBasicInfoScRsp| { &m.cur_day },
            |m: &mut GetBasicInfoScRsp| { &mut m.cur_day },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetBasicInfoScRsp| { &m.retcode },
            |m: &mut GetBasicInfoScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "last_set_nickname_time",
            |m: &GetBasicInfoScRsp| { &m.last_set_nickname_time },
            |m: &mut GetBasicInfoScRsp| { &mut m.last_set_nickname_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::DJBLLOKKAND::DJBLLOKKAND>(
            "player_setting_info",
            |m: &GetBasicInfoScRsp| { &m.player_setting_info },
            |m: &mut GetBasicInfoScRsp| { &mut m.player_setting_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "next_recover_time",
            |m: &GetBasicInfoScRsp| { &m.next_recover_time },
            |m: &mut GetBasicInfoScRsp| { &mut m.next_recover_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "exchange_times",
            |m: &GetBasicInfoScRsp| { &m.exchange_times },
            |m: &mut GetBasicInfoScRsp| { &mut m.exchange_times },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "gender",
            |m: &GetBasicInfoScRsp| { &m.gender },
            |m: &mut GetBasicInfoScRsp| { &mut m.gender },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetBasicInfoScRsp>(
            "GetBasicInfoScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetBasicInfoScRsp {
    const NAME: &'static str = "GetBasicInfoScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.week_cocoon_finished_count = is.read_uint32()?;
                },
                8 => {
                    self.gameplay_birthday = is.read_uint32()?;
                },
                24 => {
                    self.is_gender_set = is.read_bool()?;
                },
                96 => {
                    self.cur_day = is.read_uint32()?;
                },
                48 => {
                    self.retcode = is.read_uint32()?;
                },
                112 => {
                    self.last_set_nickname_time = is.read_int64()?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.player_setting_info)?;
                },
                32 => {
                    self.next_recover_time = is.read_int64()?;
                },
                64 => {
                    self.exchange_times = is.read_uint32()?;
                },
                16 => {
                    self.gender = is.read_uint32()?;
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
        if self.week_cocoon_finished_count != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.week_cocoon_finished_count);
        }
        if self.gameplay_birthday != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.gameplay_birthday);
        }
        if self.is_gender_set != false {
            my_size += 1 + 1;
        }
        if self.cur_day != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.cur_day);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.retcode);
        }
        if self.last_set_nickname_time != 0 {
            my_size += ::protobuf::rt::int64_size(14, self.last_set_nickname_time);
        }
        if let Some(v) = self.player_setting_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.next_recover_time != 0 {
            my_size += ::protobuf::rt::int64_size(4, self.next_recover_time);
        }
        if self.exchange_times != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.exchange_times);
        }
        if self.gender != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.gender);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.week_cocoon_finished_count != 0 {
            os.write_uint32(11, self.week_cocoon_finished_count)?;
        }
        if self.gameplay_birthday != 0 {
            os.write_uint32(1, self.gameplay_birthday)?;
        }
        if self.is_gender_set != false {
            os.write_bool(3, self.is_gender_set)?;
        }
        if self.cur_day != 0 {
            os.write_uint32(12, self.cur_day)?;
        }
        if self.retcode != 0 {
            os.write_uint32(6, self.retcode)?;
        }
        if self.last_set_nickname_time != 0 {
            os.write_int64(14, self.last_set_nickname_time)?;
        }
        if let Some(v) = self.player_setting_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if self.next_recover_time != 0 {
            os.write_int64(4, self.next_recover_time)?;
        }
        if self.exchange_times != 0 {
            os.write_uint32(8, self.exchange_times)?;
        }
        if self.gender != 0 {
            os.write_uint32(2, self.gender)?;
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

    fn new() -> GetBasicInfoScRsp {
        GetBasicInfoScRsp::new()
    }

    fn clear(&mut self) {
        self.week_cocoon_finished_count = 0;
        self.gameplay_birthday = 0;
        self.is_gender_set = false;
        self.cur_day = 0;
        self.retcode = 0;
        self.last_set_nickname_time = 0;
        self.player_setting_info.clear();
        self.next_recover_time = 0;
        self.exchange_times = 0;
        self.gender = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetBasicInfoScRsp {
        static instance: GetBasicInfoScRsp = GetBasicInfoScRsp {
            week_cocoon_finished_count: 0,
            gameplay_birthday: 0,
            is_gender_set: false,
            cur_day: 0,
            retcode: 0,
            last_set_nickname_time: 0,
            player_setting_info: ::protobuf::MessageField::none(),
            next_recover_time: 0,
            exchange_times: 0,
            gender: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetBasicInfoScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetBasicInfoScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetBasicInfoScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetBasicInfoScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17GetBasicInfoScRsp.proto\x1a\x11DJBLLOKKAND.proto\"\xb2\x03\n\x11Ge\
    tBasicInfoScRsp\x12;\n\x1aweek_cocoon_finished_count\x18\x0b\x20\x01(\rR\
    \x17weekCocoonFinishedCount\x12+\n\x11gameplay_birthday\x18\x01\x20\x01(\
    \rR\x10gameplayBirthday\x12\"\n\ris_gender_set\x18\x03\x20\x01(\x08R\x0b\
    isGenderSet\x12\x17\n\x07cur_day\x18\x0c\x20\x01(\rR\x06curDay\x12\x18\n\
    \x07retcode\x18\x06\x20\x01(\rR\x07retcode\x123\n\x16last_set_nickname_t\
    ime\x18\x0e\x20\x01(\x03R\x13lastSetNicknameTime\x12<\n\x13player_settin\
    g_info\x18\x0f\x20\x01(\x0b2\x0c.DJBLLOKKANDR\x11playerSettingInfo\x12*\
    \n\x11next_recover_time\x18\x04\x20\x01(\x03R\x0fnextRecoverTime\x12%\n\
    \x0eexchange_times\x18\x08\x20\x01(\rR\rexchangeTimes\x12\x16\n\x06gende\
    r\x18\x02\x20\x01(\rR\x06genderb\x06proto3\
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
            deps.push(super::DJBLLOKKAND::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetBasicInfoScRsp::generated_message_descriptor_data());
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
