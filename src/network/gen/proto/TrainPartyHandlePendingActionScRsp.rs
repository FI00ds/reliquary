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

//! Generated file from `TrainPartyHandlePendingActionScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:TrainPartyHandlePendingActionScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TrainPartyHandlePendingActionScRsp {
    // message fields
    // @@protoc_insertion_point(field:TrainPartyHandlePendingActionScRsp.HILOMEKAFBP)
    pub HILOMEKAFBP: ::protobuf::MessageField<super::FKMBFLMEGEB::FKMBFLMEGEB>,
    // @@protoc_insertion_point(field:TrainPartyHandlePendingActionScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:TrainPartyHandlePendingActionScRsp.DLNCBJFGKAA)
    pub DLNCBJFGKAA: bool,
    // @@protoc_insertion_point(field:TrainPartyHandlePendingActionScRsp.HBAPCCEGNME)
    pub HBAPCCEGNME: u32,
    // message oneof groups
    pub GNGLHJGMAOK: ::std::option::Option<train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK>,
    // special fields
    // @@protoc_insertion_point(special_field:TrainPartyHandlePendingActionScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TrainPartyHandlePendingActionScRsp {
    fn default() -> &'a TrainPartyHandlePendingActionScRsp {
        <TrainPartyHandlePendingActionScRsp as ::protobuf::Message>::default_instance()
    }
}

impl TrainPartyHandlePendingActionScRsp {
    pub fn new() -> TrainPartyHandlePendingActionScRsp {
        ::std::default::Default::default()
    }

    // .BFIFANAOCPC LAELPNHHJIK = 852;

    pub fn LAELPNHHJIK(&self) -> &super::BFIFANAOCPC::BFIFANAOCPC {
        match self.GNGLHJGMAOK {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::LAELPNHHJIK(ref v)) => v,
            _ => <super::BFIFANAOCPC::BFIFANAOCPC as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_LAELPNHHJIK(&mut self) {
        self.GNGLHJGMAOK = ::std::option::Option::None;
    }

    pub fn has_LAELPNHHJIK(&self) -> bool {
        match self.GNGLHJGMAOK {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::LAELPNHHJIK(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_LAELPNHHJIK(&mut self, v: super::BFIFANAOCPC::BFIFANAOCPC) {
        self.GNGLHJGMAOK = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::LAELPNHHJIK(v))
    }

    // Mutable pointer to the field.
    pub fn mut_LAELPNHHJIK(&mut self) -> &mut super::BFIFANAOCPC::BFIFANAOCPC {
        if let ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::LAELPNHHJIK(_)) = self.GNGLHJGMAOK {
        } else {
            self.GNGLHJGMAOK = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::LAELPNHHJIK(super::BFIFANAOCPC::BFIFANAOCPC::new()));
        }
        match self.GNGLHJGMAOK {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::LAELPNHHJIK(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_LAELPNHHJIK(&mut self) -> super::BFIFANAOCPC::BFIFANAOCPC {
        if self.has_LAELPNHHJIK() {
            match self.GNGLHJGMAOK.take() {
                ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::LAELPNHHJIK(v)) => v,
                _ => panic!(),
            }
        } else {
            super::BFIFANAOCPC::BFIFANAOCPC::new()
        }
    }

    // .MBINMAONBCD PMOJBJMKFAN = 54;

    pub fn PMOJBJMKFAN(&self) -> &super::MBINMAONBCD::MBINMAONBCD {
        match self.GNGLHJGMAOK {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::PMOJBJMKFAN(ref v)) => v,
            _ => <super::MBINMAONBCD::MBINMAONBCD as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_PMOJBJMKFAN(&mut self) {
        self.GNGLHJGMAOK = ::std::option::Option::None;
    }

    pub fn has_PMOJBJMKFAN(&self) -> bool {
        match self.GNGLHJGMAOK {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::PMOJBJMKFAN(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_PMOJBJMKFAN(&mut self, v: super::MBINMAONBCD::MBINMAONBCD) {
        self.GNGLHJGMAOK = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::PMOJBJMKFAN(v))
    }

    // Mutable pointer to the field.
    pub fn mut_PMOJBJMKFAN(&mut self) -> &mut super::MBINMAONBCD::MBINMAONBCD {
        if let ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::PMOJBJMKFAN(_)) = self.GNGLHJGMAOK {
        } else {
            self.GNGLHJGMAOK = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::PMOJBJMKFAN(super::MBINMAONBCD::MBINMAONBCD::new()));
        }
        match self.GNGLHJGMAOK {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::PMOJBJMKFAN(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_PMOJBJMKFAN(&mut self) -> super::MBINMAONBCD::MBINMAONBCD {
        if self.has_PMOJBJMKFAN() {
            match self.GNGLHJGMAOK.take() {
                ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::PMOJBJMKFAN(v)) => v,
                _ => panic!(),
            }
        } else {
            super::MBINMAONBCD::MBINMAONBCD::new()
        }
    }

    // .JBOCJHNDAMC AFDNDBADDKP = 318;

    pub fn AFDNDBADDKP(&self) -> &super::JBOCJHNDAMC::JBOCJHNDAMC {
        match self.GNGLHJGMAOK {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::AFDNDBADDKP(ref v)) => v,
            _ => <super::JBOCJHNDAMC::JBOCJHNDAMC as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_AFDNDBADDKP(&mut self) {
        self.GNGLHJGMAOK = ::std::option::Option::None;
    }

    pub fn has_AFDNDBADDKP(&self) -> bool {
        match self.GNGLHJGMAOK {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::AFDNDBADDKP(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_AFDNDBADDKP(&mut self, v: super::JBOCJHNDAMC::JBOCJHNDAMC) {
        self.GNGLHJGMAOK = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::AFDNDBADDKP(v))
    }

    // Mutable pointer to the field.
    pub fn mut_AFDNDBADDKP(&mut self) -> &mut super::JBOCJHNDAMC::JBOCJHNDAMC {
        if let ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::AFDNDBADDKP(_)) = self.GNGLHJGMAOK {
        } else {
            self.GNGLHJGMAOK = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::AFDNDBADDKP(super::JBOCJHNDAMC::JBOCJHNDAMC::new()));
        }
        match self.GNGLHJGMAOK {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::AFDNDBADDKP(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_AFDNDBADDKP(&mut self) -> super::JBOCJHNDAMC::JBOCJHNDAMC {
        if self.has_AFDNDBADDKP() {
            match self.GNGLHJGMAOK.take() {
                ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::AFDNDBADDKP(v)) => v,
                _ => panic!(),
            }
        } else {
            super::JBOCJHNDAMC::JBOCJHNDAMC::new()
        }
    }

    // .FBGLLDNLGPE LEDFNCDFAMK = 1841;

    pub fn LEDFNCDFAMK(&self) -> &super::FBGLLDNLGPE::FBGLLDNLGPE {
        match self.GNGLHJGMAOK {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::LEDFNCDFAMK(ref v)) => v,
            _ => <super::FBGLLDNLGPE::FBGLLDNLGPE as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_LEDFNCDFAMK(&mut self) {
        self.GNGLHJGMAOK = ::std::option::Option::None;
    }

    pub fn has_LEDFNCDFAMK(&self) -> bool {
        match self.GNGLHJGMAOK {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::LEDFNCDFAMK(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_LEDFNCDFAMK(&mut self, v: super::FBGLLDNLGPE::FBGLLDNLGPE) {
        self.GNGLHJGMAOK = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::LEDFNCDFAMK(v))
    }

    // Mutable pointer to the field.
    pub fn mut_LEDFNCDFAMK(&mut self) -> &mut super::FBGLLDNLGPE::FBGLLDNLGPE {
        if let ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::LEDFNCDFAMK(_)) = self.GNGLHJGMAOK {
        } else {
            self.GNGLHJGMAOK = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::LEDFNCDFAMK(super::FBGLLDNLGPE::FBGLLDNLGPE::new()));
        }
        match self.GNGLHJGMAOK {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::LEDFNCDFAMK(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_LEDFNCDFAMK(&mut self) -> super::FBGLLDNLGPE::FBGLLDNLGPE {
        if self.has_LEDFNCDFAMK() {
            match self.GNGLHJGMAOK.take() {
                ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::LEDFNCDFAMK(v)) => v,
                _ => panic!(),
            }
        } else {
            super::FBGLLDNLGPE::FBGLLDNLGPE::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::FKMBFLMEGEB::FKMBFLMEGEB>(
            "HILOMEKAFBP",
            |m: &TrainPartyHandlePendingActionScRsp| { &m.HILOMEKAFBP },
            |m: &mut TrainPartyHandlePendingActionScRsp| { &mut m.HILOMEKAFBP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &TrainPartyHandlePendingActionScRsp| { &m.retcode },
            |m: &mut TrainPartyHandlePendingActionScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DLNCBJFGKAA",
            |m: &TrainPartyHandlePendingActionScRsp| { &m.DLNCBJFGKAA },
            |m: &mut TrainPartyHandlePendingActionScRsp| { &mut m.DLNCBJFGKAA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HBAPCCEGNME",
            |m: &TrainPartyHandlePendingActionScRsp| { &m.HBAPCCEGNME },
            |m: &mut TrainPartyHandlePendingActionScRsp| { &mut m.HBAPCCEGNME },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::BFIFANAOCPC::BFIFANAOCPC>(
            "LAELPNHHJIK",
            TrainPartyHandlePendingActionScRsp::has_LAELPNHHJIK,
            TrainPartyHandlePendingActionScRsp::LAELPNHHJIK,
            TrainPartyHandlePendingActionScRsp::mut_LAELPNHHJIK,
            TrainPartyHandlePendingActionScRsp::set_LAELPNHHJIK,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::MBINMAONBCD::MBINMAONBCD>(
            "PMOJBJMKFAN",
            TrainPartyHandlePendingActionScRsp::has_PMOJBJMKFAN,
            TrainPartyHandlePendingActionScRsp::PMOJBJMKFAN,
            TrainPartyHandlePendingActionScRsp::mut_PMOJBJMKFAN,
            TrainPartyHandlePendingActionScRsp::set_PMOJBJMKFAN,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::JBOCJHNDAMC::JBOCJHNDAMC>(
            "AFDNDBADDKP",
            TrainPartyHandlePendingActionScRsp::has_AFDNDBADDKP,
            TrainPartyHandlePendingActionScRsp::AFDNDBADDKP,
            TrainPartyHandlePendingActionScRsp::mut_AFDNDBADDKP,
            TrainPartyHandlePendingActionScRsp::set_AFDNDBADDKP,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::FBGLLDNLGPE::FBGLLDNLGPE>(
            "LEDFNCDFAMK",
            TrainPartyHandlePendingActionScRsp::has_LEDFNCDFAMK,
            TrainPartyHandlePendingActionScRsp::LEDFNCDFAMK,
            TrainPartyHandlePendingActionScRsp::mut_LEDFNCDFAMK,
            TrainPartyHandlePendingActionScRsp::set_LEDFNCDFAMK,
        ));
        oneofs.push(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TrainPartyHandlePendingActionScRsp>(
            "TrainPartyHandlePendingActionScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TrainPartyHandlePendingActionScRsp {
    const NAME: &'static str = "TrainPartyHandlePendingActionScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                98 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.HILOMEKAFBP)?;
                },
                56 => {
                    self.retcode = is.read_uint32()?;
                },
                24 => {
                    self.DLNCBJFGKAA = is.read_bool()?;
                },
                88 => {
                    self.HBAPCCEGNME = is.read_uint32()?;
                },
                6818 => {
                    self.GNGLHJGMAOK = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::LAELPNHHJIK(is.read_message()?));
                },
                434 => {
                    self.GNGLHJGMAOK = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::PMOJBJMKFAN(is.read_message()?));
                },
                2546 => {
                    self.GNGLHJGMAOK = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::AFDNDBADDKP(is.read_message()?));
                },
                14730 => {
                    self.GNGLHJGMAOK = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::LEDFNCDFAMK(is.read_message()?));
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
        if let Some(v) = self.HILOMEKAFBP.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.retcode);
        }
        if self.DLNCBJFGKAA != false {
            my_size += 1 + 1;
        }
        if self.HBAPCCEGNME != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.HBAPCCEGNME);
        }
        if let ::std::option::Option::Some(ref v) = self.GNGLHJGMAOK {
            match v {
                &train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::LAELPNHHJIK(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::PMOJBJMKFAN(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::AFDNDBADDKP(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::LEDFNCDFAMK(ref v) => {
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
        if let Some(v) = self.HILOMEKAFBP.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        }
        if self.retcode != 0 {
            os.write_uint32(7, self.retcode)?;
        }
        if self.DLNCBJFGKAA != false {
            os.write_bool(3, self.DLNCBJFGKAA)?;
        }
        if self.HBAPCCEGNME != 0 {
            os.write_uint32(11, self.HBAPCCEGNME)?;
        }
        if let ::std::option::Option::Some(ref v) = self.GNGLHJGMAOK {
            match v {
                &train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::LAELPNHHJIK(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(852, v, os)?;
                },
                &train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::PMOJBJMKFAN(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(54, v, os)?;
                },
                &train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::AFDNDBADDKP(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(318, v, os)?;
                },
                &train_party_handle_pending_action_sc_rsp::GNGLHJGMAOK::LEDFNCDFAMK(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1841, v, os)?;
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

    fn new() -> TrainPartyHandlePendingActionScRsp {
        TrainPartyHandlePendingActionScRsp::new()
    }

    fn clear(&mut self) {
        self.HILOMEKAFBP.clear();
        self.retcode = 0;
        self.DLNCBJFGKAA = false;
        self.HBAPCCEGNME = 0;
        self.GNGLHJGMAOK = ::std::option::Option::None;
        self.GNGLHJGMAOK = ::std::option::Option::None;
        self.GNGLHJGMAOK = ::std::option::Option::None;
        self.GNGLHJGMAOK = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TrainPartyHandlePendingActionScRsp {
        static instance: TrainPartyHandlePendingActionScRsp = TrainPartyHandlePendingActionScRsp {
            HILOMEKAFBP: ::protobuf::MessageField::none(),
            retcode: 0,
            DLNCBJFGKAA: false,
            HBAPCCEGNME: 0,
            GNGLHJGMAOK: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TrainPartyHandlePendingActionScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TrainPartyHandlePendingActionScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TrainPartyHandlePendingActionScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TrainPartyHandlePendingActionScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `TrainPartyHandlePendingActionScRsp`
pub mod train_party_handle_pending_action_sc_rsp {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:TrainPartyHandlePendingActionScRsp.GNGLHJGMAOK)
    pub enum GNGLHJGMAOK {
        // @@protoc_insertion_point(oneof_field:TrainPartyHandlePendingActionScRsp.LAELPNHHJIK)
        LAELPNHHJIK(super::super::BFIFANAOCPC::BFIFANAOCPC),
        // @@protoc_insertion_point(oneof_field:TrainPartyHandlePendingActionScRsp.PMOJBJMKFAN)
        PMOJBJMKFAN(super::super::MBINMAONBCD::MBINMAONBCD),
        // @@protoc_insertion_point(oneof_field:TrainPartyHandlePendingActionScRsp.AFDNDBADDKP)
        AFDNDBADDKP(super::super::JBOCJHNDAMC::JBOCJHNDAMC),
        // @@protoc_insertion_point(oneof_field:TrainPartyHandlePendingActionScRsp.LEDFNCDFAMK)
        LEDFNCDFAMK(super::super::FBGLLDNLGPE::FBGLLDNLGPE),
    }

    impl ::protobuf::Oneof for GNGLHJGMAOK {
    }

    impl ::protobuf::OneofFull for GNGLHJGMAOK {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::TrainPartyHandlePendingActionScRsp as ::protobuf::MessageFull>::descriptor().oneof_by_name("GNGLHJGMAOK").unwrap()).clone()
        }
    }

    impl GNGLHJGMAOK {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<GNGLHJGMAOK>("GNGLHJGMAOK")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n(TrainPartyHandlePendingActionScRsp.proto\x1a\x11BFIFANAOCPC.proto\x1a\
    \x11FBGLLDNLGPE.proto\x1a\x11FKMBFLMEGEB.proto\x1a\x11JBOCJHNDAMC.proto\
    \x1a\x11MBINMAONBCD.proto\"\x8c\x03\n\"TrainPartyHandlePendingActionScRs\
    p\x12.\n\x0bHILOMEKAFBP\x18\x0c\x20\x01(\x0b2\x0c.FKMBFLMEGEBR\x0bHILOME\
    KAFBP\x12\x18\n\x07retcode\x18\x07\x20\x01(\rR\x07retcode\x12\x20\n\x0bD\
    LNCBJFGKAA\x18\x03\x20\x01(\x08R\x0bDLNCBJFGKAA\x12\x20\n\x0bHBAPCCEGNME\
    \x18\x0b\x20\x01(\rR\x0bHBAPCCEGNME\x121\n\x0bLAELPNHHJIK\x18\xd4\x06\
    \x20\x01(\x0b2\x0c.BFIFANAOCPCH\0R\x0bLAELPNHHJIK\x120\n\x0bPMOJBJMKFAN\
    \x186\x20\x01(\x0b2\x0c.MBINMAONBCDH\0R\x0bPMOJBJMKFAN\x121\n\x0bAFDNDBA\
    DDKP\x18\xbe\x02\x20\x01(\x0b2\x0c.JBOCJHNDAMCH\0R\x0bAFDNDBADDKP\x121\n\
    \x0bLEDFNCDFAMK\x18\xb1\x0e\x20\x01(\x0b2\x0c.FBGLLDNLGPEH\0R\x0bLEDFNCD\
    FAMKB\r\n\x0bGNGLHJGMAOKb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(5);
            deps.push(super::BFIFANAOCPC::file_descriptor().clone());
            deps.push(super::FBGLLDNLGPE::file_descriptor().clone());
            deps.push(super::FKMBFLMEGEB::file_descriptor().clone());
            deps.push(super::JBOCJHNDAMC::file_descriptor().clone());
            deps.push(super::MBINMAONBCD::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(TrainPartyHandlePendingActionScRsp::generated_message_descriptor_data());
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
