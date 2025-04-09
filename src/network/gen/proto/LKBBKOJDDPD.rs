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

//! Generated file from `LKBBKOJDDPD.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:LKBBKOJDDPD)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LKBBKOJDDPD {
    // message fields
    // @@protoc_insertion_point(field:LKBBKOJDDPD.AHIDJBJGGPP)
    pub AHIDJBJGGPP: u32,
    // message oneof groups
    pub ACNPGAJFFHI: ::std::option::Option<lkbbkojddpd::ACNPGAJFFHI>,
    // special fields
    // @@protoc_insertion_point(special_field:LKBBKOJDDPD.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LKBBKOJDDPD {
    fn default() -> &'a LKBBKOJDDPD {
        <LKBBKOJDDPD as ::protobuf::Message>::default_instance()
    }
}

impl LKBBKOJDDPD {
    pub fn new() -> LKBBKOJDDPD {
        ::std::default::Default::default()
    }

    // .LNIHJDAILDJ PNIMPJFILGF = 9;

    pub fn PNIMPJFILGF(&self) -> &super::LNIHJDAILDJ::LNIHJDAILDJ {
        match self.ACNPGAJFFHI {
            ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::PNIMPJFILGF(ref v)) => v,
            _ => <super::LNIHJDAILDJ::LNIHJDAILDJ as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_PNIMPJFILGF(&mut self) {
        self.ACNPGAJFFHI = ::std::option::Option::None;
    }

    pub fn has_PNIMPJFILGF(&self) -> bool {
        match self.ACNPGAJFFHI {
            ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::PNIMPJFILGF(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_PNIMPJFILGF(&mut self, v: super::LNIHJDAILDJ::LNIHJDAILDJ) {
        self.ACNPGAJFFHI = ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::PNIMPJFILGF(v))
    }

    // Mutable pointer to the field.
    pub fn mut_PNIMPJFILGF(&mut self) -> &mut super::LNIHJDAILDJ::LNIHJDAILDJ {
        if let ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::PNIMPJFILGF(_)) = self.ACNPGAJFFHI {
        } else {
            self.ACNPGAJFFHI = ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::PNIMPJFILGF(super::LNIHJDAILDJ::LNIHJDAILDJ::new()));
        }
        match self.ACNPGAJFFHI {
            ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::PNIMPJFILGF(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_PNIMPJFILGF(&mut self) -> super::LNIHJDAILDJ::LNIHJDAILDJ {
        if self.has_PNIMPJFILGF() {
            match self.ACNPGAJFFHI.take() {
                ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::PNIMPJFILGF(v)) => v,
                _ => panic!(),
            }
        } else {
            super::LNIHJDAILDJ::LNIHJDAILDJ::new()
        }
    }

    // .NNCCFPOOCKH DHLEEJMIIMO = 8;

    pub fn DHLEEJMIIMO(&self) -> &super::NNCCFPOOCKH::NNCCFPOOCKH {
        match self.ACNPGAJFFHI {
            ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::DHLEEJMIIMO(ref v)) => v,
            _ => <super::NNCCFPOOCKH::NNCCFPOOCKH as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_DHLEEJMIIMO(&mut self) {
        self.ACNPGAJFFHI = ::std::option::Option::None;
    }

    pub fn has_DHLEEJMIIMO(&self) -> bool {
        match self.ACNPGAJFFHI {
            ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::DHLEEJMIIMO(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_DHLEEJMIIMO(&mut self, v: super::NNCCFPOOCKH::NNCCFPOOCKH) {
        self.ACNPGAJFFHI = ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::DHLEEJMIIMO(v))
    }

    // Mutable pointer to the field.
    pub fn mut_DHLEEJMIIMO(&mut self) -> &mut super::NNCCFPOOCKH::NNCCFPOOCKH {
        if let ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::DHLEEJMIIMO(_)) = self.ACNPGAJFFHI {
        } else {
            self.ACNPGAJFFHI = ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::DHLEEJMIIMO(super::NNCCFPOOCKH::NNCCFPOOCKH::new()));
        }
        match self.ACNPGAJFFHI {
            ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::DHLEEJMIIMO(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_DHLEEJMIIMO(&mut self) -> super::NNCCFPOOCKH::NNCCFPOOCKH {
        if self.has_DHLEEJMIIMO() {
            match self.ACNPGAJFFHI.take() {
                ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::DHLEEJMIIMO(v)) => v,
                _ => panic!(),
            }
        } else {
            super::NNCCFPOOCKH::NNCCFPOOCKH::new()
        }
    }

    // .MACHNDHAMNM OEOFNNBLJIK = 2;

    pub fn OEOFNNBLJIK(&self) -> &super::MACHNDHAMNM::MACHNDHAMNM {
        match self.ACNPGAJFFHI {
            ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::OEOFNNBLJIK(ref v)) => v,
            _ => <super::MACHNDHAMNM::MACHNDHAMNM as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_OEOFNNBLJIK(&mut self) {
        self.ACNPGAJFFHI = ::std::option::Option::None;
    }

    pub fn has_OEOFNNBLJIK(&self) -> bool {
        match self.ACNPGAJFFHI {
            ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::OEOFNNBLJIK(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_OEOFNNBLJIK(&mut self, v: super::MACHNDHAMNM::MACHNDHAMNM) {
        self.ACNPGAJFFHI = ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::OEOFNNBLJIK(v))
    }

    // Mutable pointer to the field.
    pub fn mut_OEOFNNBLJIK(&mut self) -> &mut super::MACHNDHAMNM::MACHNDHAMNM {
        if let ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::OEOFNNBLJIK(_)) = self.ACNPGAJFFHI {
        } else {
            self.ACNPGAJFFHI = ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::OEOFNNBLJIK(super::MACHNDHAMNM::MACHNDHAMNM::new()));
        }
        match self.ACNPGAJFFHI {
            ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::OEOFNNBLJIK(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_OEOFNNBLJIK(&mut self) -> super::MACHNDHAMNM::MACHNDHAMNM {
        if self.has_OEOFNNBLJIK() {
            match self.ACNPGAJFFHI.take() {
                ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::OEOFNNBLJIK(v)) => v,
                _ => panic!(),
            }
        } else {
            super::MACHNDHAMNM::MACHNDHAMNM::new()
        }
    }

    // .IFBDBDCCOPO GPBGDCMJHLN = 13;

    pub fn GPBGDCMJHLN(&self) -> &super::IFBDBDCCOPO::IFBDBDCCOPO {
        match self.ACNPGAJFFHI {
            ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::GPBGDCMJHLN(ref v)) => v,
            _ => <super::IFBDBDCCOPO::IFBDBDCCOPO as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_GPBGDCMJHLN(&mut self) {
        self.ACNPGAJFFHI = ::std::option::Option::None;
    }

    pub fn has_GPBGDCMJHLN(&self) -> bool {
        match self.ACNPGAJFFHI {
            ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::GPBGDCMJHLN(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_GPBGDCMJHLN(&mut self, v: super::IFBDBDCCOPO::IFBDBDCCOPO) {
        self.ACNPGAJFFHI = ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::GPBGDCMJHLN(v))
    }

    // Mutable pointer to the field.
    pub fn mut_GPBGDCMJHLN(&mut self) -> &mut super::IFBDBDCCOPO::IFBDBDCCOPO {
        if let ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::GPBGDCMJHLN(_)) = self.ACNPGAJFFHI {
        } else {
            self.ACNPGAJFFHI = ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::GPBGDCMJHLN(super::IFBDBDCCOPO::IFBDBDCCOPO::new()));
        }
        match self.ACNPGAJFFHI {
            ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::GPBGDCMJHLN(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_GPBGDCMJHLN(&mut self) -> super::IFBDBDCCOPO::IFBDBDCCOPO {
        if self.has_GPBGDCMJHLN() {
            match self.ACNPGAJFFHI.take() {
                ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::GPBGDCMJHLN(v)) => v,
                _ => panic!(),
            }
        } else {
            super::IFBDBDCCOPO::IFBDBDCCOPO::new()
        }
    }

    // .GLIJKLOOAPA LNKPGGGKMNK = 5;

    pub fn LNKPGGGKMNK(&self) -> &super::GLIJKLOOAPA::GLIJKLOOAPA {
        match self.ACNPGAJFFHI {
            ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::LNKPGGGKMNK(ref v)) => v,
            _ => <super::GLIJKLOOAPA::GLIJKLOOAPA as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_LNKPGGGKMNK(&mut self) {
        self.ACNPGAJFFHI = ::std::option::Option::None;
    }

    pub fn has_LNKPGGGKMNK(&self) -> bool {
        match self.ACNPGAJFFHI {
            ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::LNKPGGGKMNK(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_LNKPGGGKMNK(&mut self, v: super::GLIJKLOOAPA::GLIJKLOOAPA) {
        self.ACNPGAJFFHI = ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::LNKPGGGKMNK(v))
    }

    // Mutable pointer to the field.
    pub fn mut_LNKPGGGKMNK(&mut self) -> &mut super::GLIJKLOOAPA::GLIJKLOOAPA {
        if let ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::LNKPGGGKMNK(_)) = self.ACNPGAJFFHI {
        } else {
            self.ACNPGAJFFHI = ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::LNKPGGGKMNK(super::GLIJKLOOAPA::GLIJKLOOAPA::new()));
        }
        match self.ACNPGAJFFHI {
            ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::LNKPGGGKMNK(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_LNKPGGGKMNK(&mut self) -> super::GLIJKLOOAPA::GLIJKLOOAPA {
        if self.has_LNKPGGGKMNK() {
            match self.ACNPGAJFFHI.take() {
                ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::LNKPGGGKMNK(v)) => v,
                _ => panic!(),
            }
        } else {
            super::GLIJKLOOAPA::GLIJKLOOAPA::new()
        }
    }

    // .ANBANKMLCLH AENEFMCBFGM = 4;

    pub fn AENEFMCBFGM(&self) -> &super::ANBANKMLCLH::ANBANKMLCLH {
        match self.ACNPGAJFFHI {
            ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::AENEFMCBFGM(ref v)) => v,
            _ => <super::ANBANKMLCLH::ANBANKMLCLH as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_AENEFMCBFGM(&mut self) {
        self.ACNPGAJFFHI = ::std::option::Option::None;
    }

    pub fn has_AENEFMCBFGM(&self) -> bool {
        match self.ACNPGAJFFHI {
            ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::AENEFMCBFGM(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_AENEFMCBFGM(&mut self, v: super::ANBANKMLCLH::ANBANKMLCLH) {
        self.ACNPGAJFFHI = ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::AENEFMCBFGM(v))
    }

    // Mutable pointer to the field.
    pub fn mut_AENEFMCBFGM(&mut self) -> &mut super::ANBANKMLCLH::ANBANKMLCLH {
        if let ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::AENEFMCBFGM(_)) = self.ACNPGAJFFHI {
        } else {
            self.ACNPGAJFFHI = ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::AENEFMCBFGM(super::ANBANKMLCLH::ANBANKMLCLH::new()));
        }
        match self.ACNPGAJFFHI {
            ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::AENEFMCBFGM(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_AENEFMCBFGM(&mut self) -> super::ANBANKMLCLH::ANBANKMLCLH {
        if self.has_AENEFMCBFGM() {
            match self.ACNPGAJFFHI.take() {
                ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::AENEFMCBFGM(v)) => v,
                _ => panic!(),
            }
        } else {
            super::ANBANKMLCLH::ANBANKMLCLH::new()
        }
    }

    // .EPPNKGOLAAP BCIIGHIOAPL = 7;

    pub fn BCIIGHIOAPL(&self) -> &super::EPPNKGOLAAP::EPPNKGOLAAP {
        match self.ACNPGAJFFHI {
            ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::BCIIGHIOAPL(ref v)) => v,
            _ => <super::EPPNKGOLAAP::EPPNKGOLAAP as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_BCIIGHIOAPL(&mut self) {
        self.ACNPGAJFFHI = ::std::option::Option::None;
    }

    pub fn has_BCIIGHIOAPL(&self) -> bool {
        match self.ACNPGAJFFHI {
            ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::BCIIGHIOAPL(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_BCIIGHIOAPL(&mut self, v: super::EPPNKGOLAAP::EPPNKGOLAAP) {
        self.ACNPGAJFFHI = ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::BCIIGHIOAPL(v))
    }

    // Mutable pointer to the field.
    pub fn mut_BCIIGHIOAPL(&mut self) -> &mut super::EPPNKGOLAAP::EPPNKGOLAAP {
        if let ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::BCIIGHIOAPL(_)) = self.ACNPGAJFFHI {
        } else {
            self.ACNPGAJFFHI = ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::BCIIGHIOAPL(super::EPPNKGOLAAP::EPPNKGOLAAP::new()));
        }
        match self.ACNPGAJFFHI {
            ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::BCIIGHIOAPL(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_BCIIGHIOAPL(&mut self) -> super::EPPNKGOLAAP::EPPNKGOLAAP {
        if self.has_BCIIGHIOAPL() {
            match self.ACNPGAJFFHI.take() {
                ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::BCIIGHIOAPL(v)) => v,
                _ => panic!(),
            }
        } else {
            super::EPPNKGOLAAP::EPPNKGOLAAP::new()
        }
    }

    // bool GNEOOAIFKIB = 12;

    pub fn GNEOOAIFKIB(&self) -> bool {
        match self.ACNPGAJFFHI {
            ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::GNEOOAIFKIB(v)) => v,
            _ => false,
        }
    }

    pub fn clear_GNEOOAIFKIB(&mut self) {
        self.ACNPGAJFFHI = ::std::option::Option::None;
    }

    pub fn has_GNEOOAIFKIB(&self) -> bool {
        match self.ACNPGAJFFHI {
            ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::GNEOOAIFKIB(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_GNEOOAIFKIB(&mut self, v: bool) {
        self.ACNPGAJFFHI = ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::GNEOOAIFKIB(v))
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AHIDJBJGGPP",
            |m: &LKBBKOJDDPD| { &m.AHIDJBJGGPP },
            |m: &mut LKBBKOJDDPD| { &mut m.AHIDJBJGGPP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::LNIHJDAILDJ::LNIHJDAILDJ>(
            "PNIMPJFILGF",
            LKBBKOJDDPD::has_PNIMPJFILGF,
            LKBBKOJDDPD::PNIMPJFILGF,
            LKBBKOJDDPD::mut_PNIMPJFILGF,
            LKBBKOJDDPD::set_PNIMPJFILGF,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::NNCCFPOOCKH::NNCCFPOOCKH>(
            "DHLEEJMIIMO",
            LKBBKOJDDPD::has_DHLEEJMIIMO,
            LKBBKOJDDPD::DHLEEJMIIMO,
            LKBBKOJDDPD::mut_DHLEEJMIIMO,
            LKBBKOJDDPD::set_DHLEEJMIIMO,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::MACHNDHAMNM::MACHNDHAMNM>(
            "OEOFNNBLJIK",
            LKBBKOJDDPD::has_OEOFNNBLJIK,
            LKBBKOJDDPD::OEOFNNBLJIK,
            LKBBKOJDDPD::mut_OEOFNNBLJIK,
            LKBBKOJDDPD::set_OEOFNNBLJIK,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::IFBDBDCCOPO::IFBDBDCCOPO>(
            "GPBGDCMJHLN",
            LKBBKOJDDPD::has_GPBGDCMJHLN,
            LKBBKOJDDPD::GPBGDCMJHLN,
            LKBBKOJDDPD::mut_GPBGDCMJHLN,
            LKBBKOJDDPD::set_GPBGDCMJHLN,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::GLIJKLOOAPA::GLIJKLOOAPA>(
            "LNKPGGGKMNK",
            LKBBKOJDDPD::has_LNKPGGGKMNK,
            LKBBKOJDDPD::LNKPGGGKMNK,
            LKBBKOJDDPD::mut_LNKPGGGKMNK,
            LKBBKOJDDPD::set_LNKPGGGKMNK,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::ANBANKMLCLH::ANBANKMLCLH>(
            "AENEFMCBFGM",
            LKBBKOJDDPD::has_AENEFMCBFGM,
            LKBBKOJDDPD::AENEFMCBFGM,
            LKBBKOJDDPD::mut_AENEFMCBFGM,
            LKBBKOJDDPD::set_AENEFMCBFGM,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::EPPNKGOLAAP::EPPNKGOLAAP>(
            "BCIIGHIOAPL",
            LKBBKOJDDPD::has_BCIIGHIOAPL,
            LKBBKOJDDPD::BCIIGHIOAPL,
            LKBBKOJDDPD::mut_BCIIGHIOAPL,
            LKBBKOJDDPD::set_BCIIGHIOAPL,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "GNEOOAIFKIB",
            LKBBKOJDDPD::has_GNEOOAIFKIB,
            LKBBKOJDDPD::GNEOOAIFKIB,
            LKBBKOJDDPD::set_GNEOOAIFKIB,
        ));
        oneofs.push(lkbbkojddpd::ACNPGAJFFHI::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LKBBKOJDDPD>(
            "LKBBKOJDDPD",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LKBBKOJDDPD {
    const NAME: &'static str = "LKBBKOJDDPD";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.AHIDJBJGGPP = is.read_uint32()?;
                },
                74 => {
                    self.ACNPGAJFFHI = ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::PNIMPJFILGF(is.read_message()?));
                },
                66 => {
                    self.ACNPGAJFFHI = ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::DHLEEJMIIMO(is.read_message()?));
                },
                18 => {
                    self.ACNPGAJFFHI = ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::OEOFNNBLJIK(is.read_message()?));
                },
                106 => {
                    self.ACNPGAJFFHI = ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::GPBGDCMJHLN(is.read_message()?));
                },
                42 => {
                    self.ACNPGAJFFHI = ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::LNKPGGGKMNK(is.read_message()?));
                },
                34 => {
                    self.ACNPGAJFFHI = ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::AENEFMCBFGM(is.read_message()?));
                },
                58 => {
                    self.ACNPGAJFFHI = ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::BCIIGHIOAPL(is.read_message()?));
                },
                96 => {
                    self.ACNPGAJFFHI = ::std::option::Option::Some(lkbbkojddpd::ACNPGAJFFHI::GNEOOAIFKIB(is.read_bool()?));
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
        if self.AHIDJBJGGPP != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.AHIDJBJGGPP);
        }
        if let ::std::option::Option::Some(ref v) = self.ACNPGAJFFHI {
            match v {
                &lkbbkojddpd::ACNPGAJFFHI::PNIMPJFILGF(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &lkbbkojddpd::ACNPGAJFFHI::DHLEEJMIIMO(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &lkbbkojddpd::ACNPGAJFFHI::OEOFNNBLJIK(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &lkbbkojddpd::ACNPGAJFFHI::GPBGDCMJHLN(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &lkbbkojddpd::ACNPGAJFFHI::LNKPGGGKMNK(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &lkbbkojddpd::ACNPGAJFFHI::AENEFMCBFGM(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &lkbbkojddpd::ACNPGAJFFHI::BCIIGHIOAPL(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &lkbbkojddpd::ACNPGAJFFHI::GNEOOAIFKIB(v) => {
                    my_size += 1 + 1;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.AHIDJBJGGPP != 0 {
            os.write_uint32(1, self.AHIDJBJGGPP)?;
        }
        if let ::std::option::Option::Some(ref v) = self.ACNPGAJFFHI {
            match v {
                &lkbbkojddpd::ACNPGAJFFHI::PNIMPJFILGF(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
                },
                &lkbbkojddpd::ACNPGAJFFHI::DHLEEJMIIMO(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
                },
                &lkbbkojddpd::ACNPGAJFFHI::OEOFNNBLJIK(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
                },
                &lkbbkojddpd::ACNPGAJFFHI::GPBGDCMJHLN(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
                },
                &lkbbkojddpd::ACNPGAJFFHI::LNKPGGGKMNK(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
                },
                &lkbbkojddpd::ACNPGAJFFHI::AENEFMCBFGM(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
                },
                &lkbbkojddpd::ACNPGAJFFHI::BCIIGHIOAPL(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
                },
                &lkbbkojddpd::ACNPGAJFFHI::GNEOOAIFKIB(v) => {
                    os.write_bool(12, v)?;
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

    fn new() -> LKBBKOJDDPD {
        LKBBKOJDDPD::new()
    }

    fn clear(&mut self) {
        self.AHIDJBJGGPP = 0;
        self.ACNPGAJFFHI = ::std::option::Option::None;
        self.ACNPGAJFFHI = ::std::option::Option::None;
        self.ACNPGAJFFHI = ::std::option::Option::None;
        self.ACNPGAJFFHI = ::std::option::Option::None;
        self.ACNPGAJFFHI = ::std::option::Option::None;
        self.ACNPGAJFFHI = ::std::option::Option::None;
        self.ACNPGAJFFHI = ::std::option::Option::None;
        self.ACNPGAJFFHI = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LKBBKOJDDPD {
        static instance: LKBBKOJDDPD = LKBBKOJDDPD {
            AHIDJBJGGPP: 0,
            ACNPGAJFFHI: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LKBBKOJDDPD {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LKBBKOJDDPD").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LKBBKOJDDPD {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LKBBKOJDDPD {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `LKBBKOJDDPD`
pub mod lkbbkojddpd {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:LKBBKOJDDPD.ACNPGAJFFHI)
    pub enum ACNPGAJFFHI {
        // @@protoc_insertion_point(oneof_field:LKBBKOJDDPD.PNIMPJFILGF)
        PNIMPJFILGF(super::super::LNIHJDAILDJ::LNIHJDAILDJ),
        // @@protoc_insertion_point(oneof_field:LKBBKOJDDPD.DHLEEJMIIMO)
        DHLEEJMIIMO(super::super::NNCCFPOOCKH::NNCCFPOOCKH),
        // @@protoc_insertion_point(oneof_field:LKBBKOJDDPD.OEOFNNBLJIK)
        OEOFNNBLJIK(super::super::MACHNDHAMNM::MACHNDHAMNM),
        // @@protoc_insertion_point(oneof_field:LKBBKOJDDPD.GPBGDCMJHLN)
        GPBGDCMJHLN(super::super::IFBDBDCCOPO::IFBDBDCCOPO),
        // @@protoc_insertion_point(oneof_field:LKBBKOJDDPD.LNKPGGGKMNK)
        LNKPGGGKMNK(super::super::GLIJKLOOAPA::GLIJKLOOAPA),
        // @@protoc_insertion_point(oneof_field:LKBBKOJDDPD.AENEFMCBFGM)
        AENEFMCBFGM(super::super::ANBANKMLCLH::ANBANKMLCLH),
        // @@protoc_insertion_point(oneof_field:LKBBKOJDDPD.BCIIGHIOAPL)
        BCIIGHIOAPL(super::super::EPPNKGOLAAP::EPPNKGOLAAP),
        // @@protoc_insertion_point(oneof_field:LKBBKOJDDPD.GNEOOAIFKIB)
        GNEOOAIFKIB(bool),
    }

    impl ::protobuf::Oneof for ACNPGAJFFHI {
    }

    impl ::protobuf::OneofFull for ACNPGAJFFHI {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::LKBBKOJDDPD as ::protobuf::MessageFull>::descriptor().oneof_by_name("ACNPGAJFFHI").unwrap()).clone()
        }
    }

    impl ACNPGAJFFHI {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<ACNPGAJFFHI>("ACNPGAJFFHI")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LKBBKOJDDPD.proto\x1a\x11ANBANKMLCLH.proto\x1a\x11EPPNKGOLAAP.prot\
    o\x1a\x11GLIJKLOOAPA.proto\x1a\x11IFBDBDCCOPO.proto\x1a\x11LNIHJDAILDJ.p\
    roto\x1a\x11MACHNDHAMNM.proto\x1a\x11NNCCFPOOCKH.proto\"\xc0\x03\n\x0bLK\
    BBKOJDDPD\x12\x20\n\x0bAHIDJBJGGPP\x18\x01\x20\x01(\rR\x0bAHIDJBJGGPP\
    \x120\n\x0bPNIMPJFILGF\x18\t\x20\x01(\x0b2\x0c.LNIHJDAILDJH\0R\x0bPNIMPJ\
    FILGF\x120\n\x0bDHLEEJMIIMO\x18\x08\x20\x01(\x0b2\x0c.NNCCFPOOCKHH\0R\
    \x0bDHLEEJMIIMO\x120\n\x0bOEOFNNBLJIK\x18\x02\x20\x01(\x0b2\x0c.MACHNDHA\
    MNMH\0R\x0bOEOFNNBLJIK\x120\n\x0bGPBGDCMJHLN\x18\r\x20\x01(\x0b2\x0c.IFB\
    DBDCCOPOH\0R\x0bGPBGDCMJHLN\x120\n\x0bLNKPGGGKMNK\x18\x05\x20\x01(\x0b2\
    \x0c.GLIJKLOOAPAH\0R\x0bLNKPGGGKMNK\x120\n\x0bAENEFMCBFGM\x18\x04\x20\
    \x01(\x0b2\x0c.ANBANKMLCLHH\0R\x0bAENEFMCBFGM\x120\n\x0bBCIIGHIOAPL\x18\
    \x07\x20\x01(\x0b2\x0c.EPPNKGOLAAPH\0R\x0bBCIIGHIOAPL\x12\"\n\x0bGNEOOAI\
    FKIB\x18\x0c\x20\x01(\x08H\0R\x0bGNEOOAIFKIBB\r\n\x0bACNPGAJFFHIb\x06pro\
    to3\
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
            let mut deps = ::std::vec::Vec::with_capacity(7);
            deps.push(super::ANBANKMLCLH::file_descriptor().clone());
            deps.push(super::EPPNKGOLAAP::file_descriptor().clone());
            deps.push(super::GLIJKLOOAPA::file_descriptor().clone());
            deps.push(super::IFBDBDCCOPO::file_descriptor().clone());
            deps.push(super::LNIHJDAILDJ::file_descriptor().clone());
            deps.push(super::MACHNDHAMNM::file_descriptor().clone());
            deps.push(super::NNCCFPOOCKH::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LKBBKOJDDPD::generated_message_descriptor_data());
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
