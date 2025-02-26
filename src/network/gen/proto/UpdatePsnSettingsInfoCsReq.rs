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

//! Generated file from `UpdatePsnSettingsInfoCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:UpdatePsnSettingsInfoCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct UpdatePsnSettingsInfoCsReq {
    // message oneof groups
    pub DFOPIIHEJFM: ::std::option::Option<update_psn_settings_info_cs_req::DFOPIIHEJFM>,
    // special fields
    // @@protoc_insertion_point(special_field:UpdatePsnSettingsInfoCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a UpdatePsnSettingsInfoCsReq {
    fn default() -> &'a UpdatePsnSettingsInfoCsReq {
        <UpdatePsnSettingsInfoCsReq as ::protobuf::Message>::default_instance()
    }
}

impl UpdatePsnSettingsInfoCsReq {
    pub fn new() -> UpdatePsnSettingsInfoCsReq {
        ::std::default::Default::default()
    }

    // .NCBIMLPODON DOOCPLCLDPD = 621;

    pub fn DOOCPLCLDPD(&self) -> &super::NCBIMLPODON::NCBIMLPODON {
        match self.DFOPIIHEJFM {
            ::std::option::Option::Some(update_psn_settings_info_cs_req::DFOPIIHEJFM::DOOCPLCLDPD(ref v)) => v,
            _ => <super::NCBIMLPODON::NCBIMLPODON as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_DOOCPLCLDPD(&mut self) {
        self.DFOPIIHEJFM = ::std::option::Option::None;
    }

    pub fn has_DOOCPLCLDPD(&self) -> bool {
        match self.DFOPIIHEJFM {
            ::std::option::Option::Some(update_psn_settings_info_cs_req::DFOPIIHEJFM::DOOCPLCLDPD(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_DOOCPLCLDPD(&mut self, v: super::NCBIMLPODON::NCBIMLPODON) {
        self.DFOPIIHEJFM = ::std::option::Option::Some(update_psn_settings_info_cs_req::DFOPIIHEJFM::DOOCPLCLDPD(v))
    }

    // Mutable pointer to the field.
    pub fn mut_DOOCPLCLDPD(&mut self) -> &mut super::NCBIMLPODON::NCBIMLPODON {
        if let ::std::option::Option::Some(update_psn_settings_info_cs_req::DFOPIIHEJFM::DOOCPLCLDPD(_)) = self.DFOPIIHEJFM {
        } else {
            self.DFOPIIHEJFM = ::std::option::Option::Some(update_psn_settings_info_cs_req::DFOPIIHEJFM::DOOCPLCLDPD(super::NCBIMLPODON::NCBIMLPODON::new()));
        }
        match self.DFOPIIHEJFM {
            ::std::option::Option::Some(update_psn_settings_info_cs_req::DFOPIIHEJFM::DOOCPLCLDPD(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_DOOCPLCLDPD(&mut self) -> super::NCBIMLPODON::NCBIMLPODON {
        if self.has_DOOCPLCLDPD() {
            match self.DFOPIIHEJFM.take() {
                ::std::option::Option::Some(update_psn_settings_info_cs_req::DFOPIIHEJFM::DOOCPLCLDPD(v)) => v,
                _ => panic!(),
            }
        } else {
            super::NCBIMLPODON::NCBIMLPODON::new()
        }
    }

    // .CJAKIBDIMMJ INHLDIDJGHA = 918;

    pub fn INHLDIDJGHA(&self) -> &super::CJAKIBDIMMJ::CJAKIBDIMMJ {
        match self.DFOPIIHEJFM {
            ::std::option::Option::Some(update_psn_settings_info_cs_req::DFOPIIHEJFM::INHLDIDJGHA(ref v)) => v,
            _ => <super::CJAKIBDIMMJ::CJAKIBDIMMJ as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_INHLDIDJGHA(&mut self) {
        self.DFOPIIHEJFM = ::std::option::Option::None;
    }

    pub fn has_INHLDIDJGHA(&self) -> bool {
        match self.DFOPIIHEJFM {
            ::std::option::Option::Some(update_psn_settings_info_cs_req::DFOPIIHEJFM::INHLDIDJGHA(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_INHLDIDJGHA(&mut self, v: super::CJAKIBDIMMJ::CJAKIBDIMMJ) {
        self.DFOPIIHEJFM = ::std::option::Option::Some(update_psn_settings_info_cs_req::DFOPIIHEJFM::INHLDIDJGHA(v))
    }

    // Mutable pointer to the field.
    pub fn mut_INHLDIDJGHA(&mut self) -> &mut super::CJAKIBDIMMJ::CJAKIBDIMMJ {
        if let ::std::option::Option::Some(update_psn_settings_info_cs_req::DFOPIIHEJFM::INHLDIDJGHA(_)) = self.DFOPIIHEJFM {
        } else {
            self.DFOPIIHEJFM = ::std::option::Option::Some(update_psn_settings_info_cs_req::DFOPIIHEJFM::INHLDIDJGHA(super::CJAKIBDIMMJ::CJAKIBDIMMJ::new()));
        }
        match self.DFOPIIHEJFM {
            ::std::option::Option::Some(update_psn_settings_info_cs_req::DFOPIIHEJFM::INHLDIDJGHA(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_INHLDIDJGHA(&mut self) -> super::CJAKIBDIMMJ::CJAKIBDIMMJ {
        if self.has_INHLDIDJGHA() {
            match self.DFOPIIHEJFM.take() {
                ::std::option::Option::Some(update_psn_settings_info_cs_req::DFOPIIHEJFM::INHLDIDJGHA(v)) => v,
                _ => panic!(),
            }
        } else {
            super::CJAKIBDIMMJ::CJAKIBDIMMJ::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::NCBIMLPODON::NCBIMLPODON>(
            "DOOCPLCLDPD",
            UpdatePsnSettingsInfoCsReq::has_DOOCPLCLDPD,
            UpdatePsnSettingsInfoCsReq::DOOCPLCLDPD,
            UpdatePsnSettingsInfoCsReq::mut_DOOCPLCLDPD,
            UpdatePsnSettingsInfoCsReq::set_DOOCPLCLDPD,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::CJAKIBDIMMJ::CJAKIBDIMMJ>(
            "INHLDIDJGHA",
            UpdatePsnSettingsInfoCsReq::has_INHLDIDJGHA,
            UpdatePsnSettingsInfoCsReq::INHLDIDJGHA,
            UpdatePsnSettingsInfoCsReq::mut_INHLDIDJGHA,
            UpdatePsnSettingsInfoCsReq::set_INHLDIDJGHA,
        ));
        oneofs.push(update_psn_settings_info_cs_req::DFOPIIHEJFM::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<UpdatePsnSettingsInfoCsReq>(
            "UpdatePsnSettingsInfoCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for UpdatePsnSettingsInfoCsReq {
    const NAME: &'static str = "UpdatePsnSettingsInfoCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                4970 => {
                    self.DFOPIIHEJFM = ::std::option::Option::Some(update_psn_settings_info_cs_req::DFOPIIHEJFM::DOOCPLCLDPD(is.read_message()?));
                },
                7346 => {
                    self.DFOPIIHEJFM = ::std::option::Option::Some(update_psn_settings_info_cs_req::DFOPIIHEJFM::INHLDIDJGHA(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.DFOPIIHEJFM {
            match v {
                &update_psn_settings_info_cs_req::DFOPIIHEJFM::DOOCPLCLDPD(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &update_psn_settings_info_cs_req::DFOPIIHEJFM::INHLDIDJGHA(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let ::std::option::Option::Some(ref v) = self.DFOPIIHEJFM {
            match v {
                &update_psn_settings_info_cs_req::DFOPIIHEJFM::DOOCPLCLDPD(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(621, v, os)?;
                },
                &update_psn_settings_info_cs_req::DFOPIIHEJFM::INHLDIDJGHA(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(918, v, os)?;
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

    fn new() -> UpdatePsnSettingsInfoCsReq {
        UpdatePsnSettingsInfoCsReq::new()
    }

    fn clear(&mut self) {
        self.DFOPIIHEJFM = ::std::option::Option::None;
        self.DFOPIIHEJFM = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static UpdatePsnSettingsInfoCsReq {
        static instance: UpdatePsnSettingsInfoCsReq = UpdatePsnSettingsInfoCsReq {
            DFOPIIHEJFM: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for UpdatePsnSettingsInfoCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("UpdatePsnSettingsInfoCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for UpdatePsnSettingsInfoCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpdatePsnSettingsInfoCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `UpdatePsnSettingsInfoCsReq`
pub mod update_psn_settings_info_cs_req {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:UpdatePsnSettingsInfoCsReq.DFOPIIHEJFM)
    pub enum DFOPIIHEJFM {
        // @@protoc_insertion_point(oneof_field:UpdatePsnSettingsInfoCsReq.DOOCPLCLDPD)
        DOOCPLCLDPD(super::super::NCBIMLPODON::NCBIMLPODON),
        // @@protoc_insertion_point(oneof_field:UpdatePsnSettingsInfoCsReq.INHLDIDJGHA)
        INHLDIDJGHA(super::super::CJAKIBDIMMJ::CJAKIBDIMMJ),
    }

    impl ::protobuf::Oneof for DFOPIIHEJFM {
    }

    impl ::protobuf::OneofFull for DFOPIIHEJFM {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::UpdatePsnSettingsInfoCsReq as ::protobuf::MessageFull>::descriptor().oneof_by_name("DFOPIIHEJFM").unwrap()).clone()
        }
    }

    impl DFOPIIHEJFM {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<DFOPIIHEJFM>("DFOPIIHEJFM")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20UpdatePsnSettingsInfoCsReq.proto\x1a\x11CJAKIBDIMMJ.proto\x1a\x11N\
    CBIMLPODON.proto\"\x91\x01\n\x1aUpdatePsnSettingsInfoCsReq\x121\n\x0bDOO\
    CPLCLDPD\x18\xed\x04\x20\x01(\x0b2\x0c.NCBIMLPODONH\0R\x0bDOOCPLCLDPD\
    \x121\n\x0bINHLDIDJGHA\x18\x96\x07\x20\x01(\x0b2\x0c.CJAKIBDIMMJH\0R\x0b\
    INHLDIDJGHAB\r\n\x0bDFOPIIHEJFMb\x06proto3\
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
            deps.push(super::CJAKIBDIMMJ::file_descriptor().clone());
            deps.push(super::NCBIMLPODON::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(UpdatePsnSettingsInfoCsReq::generated_message_descriptor_data());
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
