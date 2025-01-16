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

//! Generated file from `GIBGFEMMKAA.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GIBGFEMMKAA)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GIBGFEMMKAA {
    // message fields
    // @@protoc_insertion_point(field:GIBGFEMMKAA.INNJEIHEIOP)
    pub INNJEIHEIOP: ::protobuf::MessageField<super::EBJNAEAMCMN::EBJNAEAMCMN>,
    // @@protoc_insertion_point(field:GIBGFEMMKAA.AFLBINKAJNE)
    pub AFLBINKAJNE: ::protobuf::MessageField<super::ADKIDBBBHNC::ADKIDBBBHNC>,
    // @@protoc_insertion_point(field:GIBGFEMMKAA.ELALCFPLBNC)
    pub ELALCFPLBNC: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GIBGFEMMKAA.ODDEPLIMDDD)
    pub ODDEPLIMDDD: ::protobuf::MessageField<super::IJONOLHFNEI::IJONOLHFNEI>,
    // @@protoc_insertion_point(field:GIBGFEMMKAA.EBAMONBJKBN)
    pub EBAMONBJKBN: ::protobuf::MessageField<super::OGBEGNBIALI::OGBEGNBIALI>,
    // @@protoc_insertion_point(field:GIBGFEMMKAA.PPLCEGEKMKC)
    pub PPLCEGEKMKC: ::protobuf::MessageField<super::JOHIKGBGJBD::JOHIKGBGJBD>,
    // @@protoc_insertion_point(field:GIBGFEMMKAA.LCFANOLIPIJ)
    pub LCFANOLIPIJ: ::protobuf::MessageField<super::IJKIMBLIPID::IJKIMBLIPID>,
    // @@protoc_insertion_point(field:GIBGFEMMKAA.LMBPKEOBMHK)
    pub LMBPKEOBMHK: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GIBGFEMMKAA.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GIBGFEMMKAA {
    fn default() -> &'a GIBGFEMMKAA {
        <GIBGFEMMKAA as ::protobuf::Message>::default_instance()
    }
}

impl GIBGFEMMKAA {
    pub fn new() -> GIBGFEMMKAA {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::EBJNAEAMCMN::EBJNAEAMCMN>(
            "INNJEIHEIOP",
            |m: &GIBGFEMMKAA| { &m.INNJEIHEIOP },
            |m: &mut GIBGFEMMKAA| { &mut m.INNJEIHEIOP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ADKIDBBBHNC::ADKIDBBBHNC>(
            "AFLBINKAJNE",
            |m: &GIBGFEMMKAA| { &m.AFLBINKAJNE },
            |m: &mut GIBGFEMMKAA| { &mut m.AFLBINKAJNE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "ELALCFPLBNC",
            |m: &GIBGFEMMKAA| { &m.ELALCFPLBNC },
            |m: &mut GIBGFEMMKAA| { &mut m.ELALCFPLBNC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::IJONOLHFNEI::IJONOLHFNEI>(
            "ODDEPLIMDDD",
            |m: &GIBGFEMMKAA| { &m.ODDEPLIMDDD },
            |m: &mut GIBGFEMMKAA| { &mut m.ODDEPLIMDDD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::OGBEGNBIALI::OGBEGNBIALI>(
            "EBAMONBJKBN",
            |m: &GIBGFEMMKAA| { &m.EBAMONBJKBN },
            |m: &mut GIBGFEMMKAA| { &mut m.EBAMONBJKBN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::JOHIKGBGJBD::JOHIKGBGJBD>(
            "PPLCEGEKMKC",
            |m: &GIBGFEMMKAA| { &m.PPLCEGEKMKC },
            |m: &mut GIBGFEMMKAA| { &mut m.PPLCEGEKMKC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::IJKIMBLIPID::IJKIMBLIPID>(
            "LCFANOLIPIJ",
            |m: &GIBGFEMMKAA| { &m.LCFANOLIPIJ },
            |m: &mut GIBGFEMMKAA| { &mut m.LCFANOLIPIJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LMBPKEOBMHK",
            |m: &GIBGFEMMKAA| { &m.LMBPKEOBMHK },
            |m: &mut GIBGFEMMKAA| { &mut m.LMBPKEOBMHK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GIBGFEMMKAA>(
            "GIBGFEMMKAA",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GIBGFEMMKAA {
    const NAME: &'static str = "GIBGFEMMKAA";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.INNJEIHEIOP)?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.AFLBINKAJNE)?;
                },
                58 => {
                    is.read_repeated_packed_uint32_into(&mut self.ELALCFPLBNC)?;
                },
                56 => {
                    self.ELALCFPLBNC.push(is.read_uint32()?);
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ODDEPLIMDDD)?;
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.EBAMONBJKBN)?;
                },
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.PPLCEGEKMKC)?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LCFANOLIPIJ)?;
                },
                120 => {
                    self.LMBPKEOBMHK = is.read_uint32()?;
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
        if let Some(v) = self.INNJEIHEIOP.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.AFLBINKAJNE.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.ELALCFPLBNC {
            my_size += ::protobuf::rt::uint32_size(7, *value);
        };
        if let Some(v) = self.ODDEPLIMDDD.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.EBAMONBJKBN.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.PPLCEGEKMKC.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.LCFANOLIPIJ.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.LMBPKEOBMHK != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.LMBPKEOBMHK);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.INNJEIHEIOP.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if let Some(v) = self.AFLBINKAJNE.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        for v in &self.ELALCFPLBNC {
            os.write_uint32(7, *v)?;
        };
        if let Some(v) = self.ODDEPLIMDDD.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if let Some(v) = self.EBAMONBJKBN.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        if let Some(v) = self.PPLCEGEKMKC.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        if let Some(v) = self.LCFANOLIPIJ.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if self.LMBPKEOBMHK != 0 {
            os.write_uint32(15, self.LMBPKEOBMHK)?;
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

    fn new() -> GIBGFEMMKAA {
        GIBGFEMMKAA::new()
    }

    fn clear(&mut self) {
        self.INNJEIHEIOP.clear();
        self.AFLBINKAJNE.clear();
        self.ELALCFPLBNC.clear();
        self.ODDEPLIMDDD.clear();
        self.EBAMONBJKBN.clear();
        self.PPLCEGEKMKC.clear();
        self.LCFANOLIPIJ.clear();
        self.LMBPKEOBMHK = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GIBGFEMMKAA {
        static instance: GIBGFEMMKAA = GIBGFEMMKAA {
            INNJEIHEIOP: ::protobuf::MessageField::none(),
            AFLBINKAJNE: ::protobuf::MessageField::none(),
            ELALCFPLBNC: ::std::vec::Vec::new(),
            ODDEPLIMDDD: ::protobuf::MessageField::none(),
            EBAMONBJKBN: ::protobuf::MessageField::none(),
            PPLCEGEKMKC: ::protobuf::MessageField::none(),
            LCFANOLIPIJ: ::protobuf::MessageField::none(),
            LMBPKEOBMHK: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GIBGFEMMKAA {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GIBGFEMMKAA").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GIBGFEMMKAA {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GIBGFEMMKAA {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11GIBGFEMMKAA.proto\x1a\x11ADKIDBBBHNC.proto\x1a\x11EBJNAEAMCMN.prot\
    o\x1a\x11IJKIMBLIPID.proto\x1a\x11IJONOLHFNEI.proto\x1a\x11JOHIKGBGJBD.p\
    roto\x1a\x11OGBEGNBIALI.proto\"\xf1\x02\n\x0bGIBGFEMMKAA\x12.\n\x0bINNJE\
    IHEIOP\x18\x0b\x20\x01(\x0b2\x0c.EBJNAEAMCMNR\x0bINNJEIHEIOP\x12.\n\x0bA\
    FLBINKAJNE\x18\t\x20\x01(\x0b2\x0c.ADKIDBBBHNCR\x0bAFLBINKAJNE\x12\x20\n\
    \x0bELALCFPLBNC\x18\x07\x20\x03(\rR\x0bELALCFPLBNC\x12.\n\x0bODDEPLIMDDD\
    \x18\r\x20\x01(\x0b2\x0c.IJONOLHFNEIR\x0bODDEPLIMDDD\x12.\n\x0bEBAMONBJK\
    BN\x18\x06\x20\x01(\x0b2\x0c.OGBEGNBIALIR\x0bEBAMONBJKBN\x12.\n\x0bPPLCE\
    GEKMKC\x18\x08\x20\x01(\x0b2\x0c.JOHIKGBGJBDR\x0bPPLCEGEKMKC\x12.\n\x0bL\
    CFANOLIPIJ\x18\x04\x20\x01(\x0b2\x0c.IJKIMBLIPIDR\x0bLCFANOLIPIJ\x12\x20\
    \n\x0bLMBPKEOBMHK\x18\x0f\x20\x01(\rR\x0bLMBPKEOBMHKb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(6);
            deps.push(super::ADKIDBBBHNC::file_descriptor().clone());
            deps.push(super::EBJNAEAMCMN::file_descriptor().clone());
            deps.push(super::IJKIMBLIPID::file_descriptor().clone());
            deps.push(super::IJONOLHFNEI::file_descriptor().clone());
            deps.push(super::JOHIKGBGJBD::file_descriptor().clone());
            deps.push(super::OGBEGNBIALI::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GIBGFEMMKAA::generated_message_descriptor_data());
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
