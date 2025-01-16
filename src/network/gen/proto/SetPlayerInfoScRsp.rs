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

//! Generated file from `SetPlayerInfoScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SetPlayerInfoScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SetPlayerInfoScRsp {
    // message fields
    // @@protoc_insertion_point(field:SetPlayerInfoScRsp.OADEDDNNDDJ)
    pub OADEDDNNDDJ: ::std::vec::Vec<super::MultiPathAvatarTypeInfo::MultiPathAvatarTypeInfo>,
    // @@protoc_insertion_point(field:SetPlayerInfoScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:SetPlayerInfoScRsp.MPOHAEFEHOE)
    pub MPOHAEFEHOE: bool,
    // @@protoc_insertion_point(field:SetPlayerInfoScRsp.EFADNKKLHKM)
    pub EFADNKKLHKM: ::protobuf::EnumOrUnknown<super::MultiPathAvatarType::MultiPathAvatarType>,
    // @@protoc_insertion_point(field:SetPlayerInfoScRsp.GJKEMKAPJAL)
    pub GJKEMKAPJAL: i64,
    // special fields
    // @@protoc_insertion_point(special_field:SetPlayerInfoScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SetPlayerInfoScRsp {
    fn default() -> &'a SetPlayerInfoScRsp {
        <SetPlayerInfoScRsp as ::protobuf::Message>::default_instance()
    }
}

impl SetPlayerInfoScRsp {
    pub fn new() -> SetPlayerInfoScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "OADEDDNNDDJ",
            |m: &SetPlayerInfoScRsp| { &m.OADEDDNNDDJ },
            |m: &mut SetPlayerInfoScRsp| { &mut m.OADEDDNNDDJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &SetPlayerInfoScRsp| { &m.retcode },
            |m: &mut SetPlayerInfoScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MPOHAEFEHOE",
            |m: &SetPlayerInfoScRsp| { &m.MPOHAEFEHOE },
            |m: &mut SetPlayerInfoScRsp| { &mut m.MPOHAEFEHOE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EFADNKKLHKM",
            |m: &SetPlayerInfoScRsp| { &m.EFADNKKLHKM },
            |m: &mut SetPlayerInfoScRsp| { &mut m.EFADNKKLHKM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GJKEMKAPJAL",
            |m: &SetPlayerInfoScRsp| { &m.GJKEMKAPJAL },
            |m: &mut SetPlayerInfoScRsp| { &mut m.GJKEMKAPJAL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SetPlayerInfoScRsp>(
            "SetPlayerInfoScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SetPlayerInfoScRsp {
    const NAME: &'static str = "SetPlayerInfoScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                66 => {
                    self.OADEDDNNDDJ.push(is.read_message()?);
                },
                32 => {
                    self.retcode = is.read_uint32()?;
                },
                56 => {
                    self.MPOHAEFEHOE = is.read_bool()?;
                },
                112 => {
                    self.EFADNKKLHKM = is.read_enum_or_unknown()?;
                },
                8 => {
                    self.GJKEMKAPJAL = is.read_int64()?;
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
        for value in &self.OADEDDNNDDJ {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.retcode);
        }
        if self.MPOHAEFEHOE != false {
            my_size += 1 + 1;
        }
        if self.EFADNKKLHKM != ::protobuf::EnumOrUnknown::new(super::MultiPathAvatarType::MultiPathAvatarType::MultiPathAvatarTypeNone) {
            my_size += ::protobuf::rt::int32_size(14, self.EFADNKKLHKM.value());
        }
        if self.GJKEMKAPJAL != 0 {
            my_size += ::protobuf::rt::int64_size(1, self.GJKEMKAPJAL);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.OADEDDNNDDJ {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        if self.retcode != 0 {
            os.write_uint32(4, self.retcode)?;
        }
        if self.MPOHAEFEHOE != false {
            os.write_bool(7, self.MPOHAEFEHOE)?;
        }
        if self.EFADNKKLHKM != ::protobuf::EnumOrUnknown::new(super::MultiPathAvatarType::MultiPathAvatarType::MultiPathAvatarTypeNone) {
            os.write_enum(14, ::protobuf::EnumOrUnknown::value(&self.EFADNKKLHKM))?;
        }
        if self.GJKEMKAPJAL != 0 {
            os.write_int64(1, self.GJKEMKAPJAL)?;
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

    fn new() -> SetPlayerInfoScRsp {
        SetPlayerInfoScRsp::new()
    }

    fn clear(&mut self) {
        self.OADEDDNNDDJ.clear();
        self.retcode = 0;
        self.MPOHAEFEHOE = false;
        self.EFADNKKLHKM = ::protobuf::EnumOrUnknown::new(super::MultiPathAvatarType::MultiPathAvatarType::MultiPathAvatarTypeNone);
        self.GJKEMKAPJAL = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SetPlayerInfoScRsp {
        static instance: SetPlayerInfoScRsp = SetPlayerInfoScRsp {
            OADEDDNNDDJ: ::std::vec::Vec::new(),
            retcode: 0,
            MPOHAEFEHOE: false,
            EFADNKKLHKM: ::protobuf::EnumOrUnknown::from_i32(0),
            GJKEMKAPJAL: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SetPlayerInfoScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SetPlayerInfoScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SetPlayerInfoScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetPlayerInfoScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18SetPlayerInfoScRsp.proto\x1a\x19MultiPathAvatarType.proto\x1a\x1dM\
    ultiPathAvatarTypeInfo.proto\"\xe6\x01\n\x12SetPlayerInfoScRsp\x12:\n\
    \x0bOADEDDNNDDJ\x18\x08\x20\x03(\x0b2\x18.MultiPathAvatarTypeInfoR\x0bOA\
    DEDDNNDDJ\x12\x18\n\x07retcode\x18\x04\x20\x01(\rR\x07retcode\x12\x20\n\
    \x0bMPOHAEFEHOE\x18\x07\x20\x01(\x08R\x0bMPOHAEFEHOE\x126\n\x0bEFADNKKLH\
    KM\x18\x0e\x20\x01(\x0e2\x14.MultiPathAvatarTypeR\x0bEFADNKKLHKM\x12\x20\
    \n\x0bGJKEMKAPJAL\x18\x01\x20\x01(\x03R\x0bGJKEMKAPJALb\x06proto3\
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
            deps.push(super::MultiPathAvatarType::file_descriptor().clone());
            deps.push(super::MultiPathAvatarTypeInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SetPlayerInfoScRsp::generated_message_descriptor_data());
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
