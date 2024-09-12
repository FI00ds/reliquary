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

//! Generated file from `RogueCommonDialogueOptionResultInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RogueCommonDialogueOptionResultInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueCommonDialogueOptionResultInfo {
    // message oneof groups
    pub option_result: ::std::option::Option<rogue_common_dialogue_option_result_info::Option_result>,
    // special fields
    // @@protoc_insertion_point(special_field:RogueCommonDialogueOptionResultInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueCommonDialogueOptionResultInfo {
    fn default() -> &'a RogueCommonDialogueOptionResultInfo {
        <RogueCommonDialogueOptionResultInfo as ::protobuf::Message>::default_instance()
    }
}

impl RogueCommonDialogueOptionResultInfo {
    pub fn new() -> RogueCommonDialogueOptionResultInfo {
        ::std::default::Default::default()
    }

    // .RogueCommonDialogueOptionBattleResultInfo battle_result_info = 14;

    pub fn battle_result_info(&self) -> &super::RogueCommonDialogueOptionBattleResultInfo::RogueCommonDialogueOptionBattleResultInfo {
        match self.option_result {
            ::std::option::Option::Some(rogue_common_dialogue_option_result_info::Option_result::BattleResultInfo(ref v)) => v,
            _ => <super::RogueCommonDialogueOptionBattleResultInfo::RogueCommonDialogueOptionBattleResultInfo as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_battle_result_info(&mut self) {
        self.option_result = ::std::option::Option::None;
    }

    pub fn has_battle_result_info(&self) -> bool {
        match self.option_result {
            ::std::option::Option::Some(rogue_common_dialogue_option_result_info::Option_result::BattleResultInfo(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_battle_result_info(&mut self, v: super::RogueCommonDialogueOptionBattleResultInfo::RogueCommonDialogueOptionBattleResultInfo) {
        self.option_result = ::std::option::Option::Some(rogue_common_dialogue_option_result_info::Option_result::BattleResultInfo(v))
    }

    // Mutable pointer to the field.
    pub fn mut_battle_result_info(&mut self) -> &mut super::RogueCommonDialogueOptionBattleResultInfo::RogueCommonDialogueOptionBattleResultInfo {
        if let ::std::option::Option::Some(rogue_common_dialogue_option_result_info::Option_result::BattleResultInfo(_)) = self.option_result {
        } else {
            self.option_result = ::std::option::Option::Some(rogue_common_dialogue_option_result_info::Option_result::BattleResultInfo(super::RogueCommonDialogueOptionBattleResultInfo::RogueCommonDialogueOptionBattleResultInfo::new()));
        }
        match self.option_result {
            ::std::option::Option::Some(rogue_common_dialogue_option_result_info::Option_result::BattleResultInfo(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_battle_result_info(&mut self) -> super::RogueCommonDialogueOptionBattleResultInfo::RogueCommonDialogueOptionBattleResultInfo {
        if self.has_battle_result_info() {
            match self.option_result.take() {
                ::std::option::Option::Some(rogue_common_dialogue_option_result_info::Option_result::BattleResultInfo(v)) => v,
                _ => panic!(),
            }
        } else {
            super::RogueCommonDialogueOptionBattleResultInfo::RogueCommonDialogueOptionBattleResultInfo::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::RogueCommonDialogueOptionBattleResultInfo::RogueCommonDialogueOptionBattleResultInfo>(
            "battle_result_info",
            RogueCommonDialogueOptionResultInfo::has_battle_result_info,
            RogueCommonDialogueOptionResultInfo::battle_result_info,
            RogueCommonDialogueOptionResultInfo::mut_battle_result_info,
            RogueCommonDialogueOptionResultInfo::set_battle_result_info,
        ));
        oneofs.push(rogue_common_dialogue_option_result_info::Option_result::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueCommonDialogueOptionResultInfo>(
            "RogueCommonDialogueOptionResultInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueCommonDialogueOptionResultInfo {
    const NAME: &'static str = "RogueCommonDialogueOptionResultInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                114 => {
                    self.option_result = ::std::option::Option::Some(rogue_common_dialogue_option_result_info::Option_result::BattleResultInfo(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.option_result {
            match v {
                &rogue_common_dialogue_option_result_info::Option_result::BattleResultInfo(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let ::std::option::Option::Some(ref v) = self.option_result {
            match v {
                &rogue_common_dialogue_option_result_info::Option_result::BattleResultInfo(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
                },
            };
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

    fn new() -> RogueCommonDialogueOptionResultInfo {
        RogueCommonDialogueOptionResultInfo::new()
    }

    fn clear(&mut self) {
        self.option_result = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueCommonDialogueOptionResultInfo {
        static instance: RogueCommonDialogueOptionResultInfo = RogueCommonDialogueOptionResultInfo {
            option_result: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RogueCommonDialogueOptionResultInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueCommonDialogueOptionResultInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueCommonDialogueOptionResultInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueCommonDialogueOptionResultInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `RogueCommonDialogueOptionResultInfo`
pub mod rogue_common_dialogue_option_result_info {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:RogueCommonDialogueOptionResultInfo.option_result)
    pub enum Option_result {
        // @@protoc_insertion_point(oneof_field:RogueCommonDialogueOptionResultInfo.battle_result_info)
        BattleResultInfo(super::super::RogueCommonDialogueOptionBattleResultInfo::RogueCommonDialogueOptionBattleResultInfo),
    }

    impl ::protobuf::Oneof for Option_result {
    }

    impl ::protobuf::OneofFull for Option_result {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::RogueCommonDialogueOptionResultInfo as ::protobuf::MessageFull>::descriptor().oneof_by_name("option_result").unwrap()).clone()
        }
    }

    impl Option_result {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Option_result>("option_result")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n)RogueCommonDialogueOptionResultInfo.proto\x1a/RogueCommonDialogueOpti\
    onBattleResultInfo.proto\"\x92\x01\n#RogueCommonDialogueOptionResultInfo\
    \x12Z\n\x12battle_result_info\x18\x0e\x20\x01(\x0b2*.RogueCommonDialogue\
    OptionBattleResultInfoH\0R\x10battleResultInfoB\x0f\n\roption_resultB\
    \x15\n\x13emu.lunarcore.protob\x06proto3\
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
            deps.push(super::RogueCommonDialogueOptionBattleResultInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RogueCommonDialogueOptionResultInfo::generated_message_descriptor_data());
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
