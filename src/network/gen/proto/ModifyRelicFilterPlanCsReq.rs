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

//! Generated file from `ModifyRelicFilterPlanCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:ModifyRelicFilterPlanCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ModifyRelicFilterPlanCsReq {
    // message fields
    // @@protoc_insertion_point(field:ModifyRelicFilterPlanCsReq.slot_index)
    pub slot_index: u32,
    // message oneof groups
    pub info_case: ::std::option::Option<modify_relic_filter_plan_cs_req::Info_case>,
    // special fields
    // @@protoc_insertion_point(special_field:ModifyRelicFilterPlanCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ModifyRelicFilterPlanCsReq {
    fn default() -> &'a ModifyRelicFilterPlanCsReq {
        <ModifyRelicFilterPlanCsReq as ::protobuf::Message>::default_instance()
    }
}

impl ModifyRelicFilterPlanCsReq {
    pub fn new() -> ModifyRelicFilterPlanCsReq {
        ::std::default::Default::default()
    }

    // string name = 2;

    pub fn name(&self) -> &str {
        match self.info_case {
            ::std::option::Option::Some(modify_relic_filter_plan_cs_req::Info_case::Name(ref v)) => v,
            _ => "",
        }
    }

    pub fn clear_name(&mut self) {
        self.info_case = ::std::option::Option::None;
    }

    pub fn has_name(&self) -> bool {
        match self.info_case {
            ::std::option::Option::Some(modify_relic_filter_plan_cs_req::Info_case::Name(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.info_case = ::std::option::Option::Some(modify_relic_filter_plan_cs_req::Info_case::Name(v))
    }

    // Mutable pointer to the field.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(modify_relic_filter_plan_cs_req::Info_case::Name(_)) = self.info_case {
        } else {
            self.info_case = ::std::option::Option::Some(modify_relic_filter_plan_cs_req::Info_case::Name(::std::string::String::new()));
        }
        match self.info_case {
            ::std::option::Option::Some(modify_relic_filter_plan_cs_req::Info_case::Name(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        if self.has_name() {
            match self.info_case.take() {
                ::std::option::Option::Some(modify_relic_filter_plan_cs_req::Info_case::Name(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // .RelicFilterPlanIcon icon = 9;

    pub fn icon(&self) -> &super::RelicFilterPlanIcon::RelicFilterPlanIcon {
        match self.info_case {
            ::std::option::Option::Some(modify_relic_filter_plan_cs_req::Info_case::Icon(ref v)) => v,
            _ => <super::RelicFilterPlanIcon::RelicFilterPlanIcon as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_icon(&mut self) {
        self.info_case = ::std::option::Option::None;
    }

    pub fn has_icon(&self) -> bool {
        match self.info_case {
            ::std::option::Option::Some(modify_relic_filter_plan_cs_req::Info_case::Icon(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_icon(&mut self, v: super::RelicFilterPlanIcon::RelicFilterPlanIcon) {
        self.info_case = ::std::option::Option::Some(modify_relic_filter_plan_cs_req::Info_case::Icon(v))
    }

    // Mutable pointer to the field.
    pub fn mut_icon(&mut self) -> &mut super::RelicFilterPlanIcon::RelicFilterPlanIcon {
        if let ::std::option::Option::Some(modify_relic_filter_plan_cs_req::Info_case::Icon(_)) = self.info_case {
        } else {
            self.info_case = ::std::option::Option::Some(modify_relic_filter_plan_cs_req::Info_case::Icon(super::RelicFilterPlanIcon::RelicFilterPlanIcon::new()));
        }
        match self.info_case {
            ::std::option::Option::Some(modify_relic_filter_plan_cs_req::Info_case::Icon(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_icon(&mut self) -> super::RelicFilterPlanIcon::RelicFilterPlanIcon {
        if self.has_icon() {
            match self.info_case.take() {
                ::std::option::Option::Some(modify_relic_filter_plan_cs_req::Info_case::Icon(v)) => v,
                _ => panic!(),
            }
        } else {
            super::RelicFilterPlanIcon::RelicFilterPlanIcon::new()
        }
    }

    // .RelicFilterPlanSettings settings = 15;

    pub fn settings(&self) -> &super::RelicFilterPlanSettings::RelicFilterPlanSettings {
        match self.info_case {
            ::std::option::Option::Some(modify_relic_filter_plan_cs_req::Info_case::Settings(ref v)) => v,
            _ => <super::RelicFilterPlanSettings::RelicFilterPlanSettings as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_settings(&mut self) {
        self.info_case = ::std::option::Option::None;
    }

    pub fn has_settings(&self) -> bool {
        match self.info_case {
            ::std::option::Option::Some(modify_relic_filter_plan_cs_req::Info_case::Settings(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_settings(&mut self, v: super::RelicFilterPlanSettings::RelicFilterPlanSettings) {
        self.info_case = ::std::option::Option::Some(modify_relic_filter_plan_cs_req::Info_case::Settings(v))
    }

    // Mutable pointer to the field.
    pub fn mut_settings(&mut self) -> &mut super::RelicFilterPlanSettings::RelicFilterPlanSettings {
        if let ::std::option::Option::Some(modify_relic_filter_plan_cs_req::Info_case::Settings(_)) = self.info_case {
        } else {
            self.info_case = ::std::option::Option::Some(modify_relic_filter_plan_cs_req::Info_case::Settings(super::RelicFilterPlanSettings::RelicFilterPlanSettings::new()));
        }
        match self.info_case {
            ::std::option::Option::Some(modify_relic_filter_plan_cs_req::Info_case::Settings(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_settings(&mut self) -> super::RelicFilterPlanSettings::RelicFilterPlanSettings {
        if self.has_settings() {
            match self.info_case.take() {
                ::std::option::Option::Some(modify_relic_filter_plan_cs_req::Info_case::Settings(v)) => v,
                _ => panic!(),
            }
        } else {
            super::RelicFilterPlanSettings::RelicFilterPlanSettings::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "slot_index",
            |m: &ModifyRelicFilterPlanCsReq| { &m.slot_index },
            |m: &mut ModifyRelicFilterPlanCsReq| { &mut m.slot_index },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_deref_has_get_set_simpler_accessor::<_, _>(
            "name",
            ModifyRelicFilterPlanCsReq::has_name,
            ModifyRelicFilterPlanCsReq::name,
            ModifyRelicFilterPlanCsReq::set_name,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::RelicFilterPlanIcon::RelicFilterPlanIcon>(
            "icon",
            ModifyRelicFilterPlanCsReq::has_icon,
            ModifyRelicFilterPlanCsReq::icon,
            ModifyRelicFilterPlanCsReq::mut_icon,
            ModifyRelicFilterPlanCsReq::set_icon,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::RelicFilterPlanSettings::RelicFilterPlanSettings>(
            "settings",
            ModifyRelicFilterPlanCsReq::has_settings,
            ModifyRelicFilterPlanCsReq::settings,
            ModifyRelicFilterPlanCsReq::mut_settings,
            ModifyRelicFilterPlanCsReq::set_settings,
        ));
        oneofs.push(modify_relic_filter_plan_cs_req::Info_case::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ModifyRelicFilterPlanCsReq>(
            "ModifyRelicFilterPlanCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ModifyRelicFilterPlanCsReq {
    const NAME: &'static str = "ModifyRelicFilterPlanCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                80 => {
                    self.slot_index = is.read_uint32()?;
                },
                18 => {
                    self.info_case = ::std::option::Option::Some(modify_relic_filter_plan_cs_req::Info_case::Name(is.read_string()?));
                },
                74 => {
                    self.info_case = ::std::option::Option::Some(modify_relic_filter_plan_cs_req::Info_case::Icon(is.read_message()?));
                },
                122 => {
                    self.info_case = ::std::option::Option::Some(modify_relic_filter_plan_cs_req::Info_case::Settings(is.read_message()?));
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
        if self.slot_index != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.slot_index);
        }
        if let ::std::option::Option::Some(ref v) = self.info_case {
            match v {
                &modify_relic_filter_plan_cs_req::Info_case::Name(ref v) => {
                    my_size += ::protobuf::rt::string_size(2, &v);
                },
                &modify_relic_filter_plan_cs_req::Info_case::Icon(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &modify_relic_filter_plan_cs_req::Info_case::Settings(ref v) => {
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
        if self.slot_index != 0 {
            os.write_uint32(10, self.slot_index)?;
        }
        if let ::std::option::Option::Some(ref v) = self.info_case {
            match v {
                &modify_relic_filter_plan_cs_req::Info_case::Name(ref v) => {
                    os.write_string(2, v)?;
                },
                &modify_relic_filter_plan_cs_req::Info_case::Icon(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
                },
                &modify_relic_filter_plan_cs_req::Info_case::Settings(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
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

    fn new() -> ModifyRelicFilterPlanCsReq {
        ModifyRelicFilterPlanCsReq::new()
    }

    fn clear(&mut self) {
        self.slot_index = 0;
        self.info_case = ::std::option::Option::None;
        self.info_case = ::std::option::Option::None;
        self.info_case = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ModifyRelicFilterPlanCsReq {
        static instance: ModifyRelicFilterPlanCsReq = ModifyRelicFilterPlanCsReq {
            slot_index: 0,
            info_case: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ModifyRelicFilterPlanCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ModifyRelicFilterPlanCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ModifyRelicFilterPlanCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ModifyRelicFilterPlanCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `ModifyRelicFilterPlanCsReq`
pub mod modify_relic_filter_plan_cs_req {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:ModifyRelicFilterPlanCsReq.info_case)
    pub enum Info_case {
        // @@protoc_insertion_point(oneof_field:ModifyRelicFilterPlanCsReq.name)
        Name(::std::string::String),
        // @@protoc_insertion_point(oneof_field:ModifyRelicFilterPlanCsReq.icon)
        Icon(super::super::RelicFilterPlanIcon::RelicFilterPlanIcon),
        // @@protoc_insertion_point(oneof_field:ModifyRelicFilterPlanCsReq.settings)
        Settings(super::super::RelicFilterPlanSettings::RelicFilterPlanSettings),
    }

    impl ::protobuf::Oneof for Info_case {
    }

    impl ::protobuf::OneofFull for Info_case {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::ModifyRelicFilterPlanCsReq as ::protobuf::MessageFull>::descriptor().oneof_by_name("info_case").unwrap()).clone()
        }
    }

    impl Info_case {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Info_case>("info_case")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20ModifyRelicFilterPlanCsReq.proto\x1a\x19RelicFilterPlanIcon.proto\
    \x1a\x1dRelicFilterPlanSettings.proto\"\xc2\x01\n\x1aModifyRelicFilterPl\
    anCsReq\x12\x1d\n\nslot_index\x18\n\x20\x01(\rR\tslotIndex\x12\x14\n\x04\
    name\x18\x02\x20\x01(\tH\0R\x04name\x12*\n\x04icon\x18\t\x20\x01(\x0b2\
    \x14.RelicFilterPlanIconH\0R\x04icon\x126\n\x08settings\x18\x0f\x20\x01(\
    \x0b2\x18.RelicFilterPlanSettingsH\0R\x08settingsB\x0b\n\tinfo_caseb\x06\
    proto3\
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
            deps.push(super::RelicFilterPlanIcon::file_descriptor().clone());
            deps.push(super::RelicFilterPlanSettings::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ModifyRelicFilterPlanCsReq::generated_message_descriptor_data());
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
