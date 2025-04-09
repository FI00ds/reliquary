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

//! Generated file from `KIPHKHHMFAC.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:KIPHKHHMFAC)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct KIPHKHHMFAC {
    // message fields
    // @@protoc_insertion_point(field:KIPHKHHMFAC.end_time)
    pub end_time: i64,
    // @@protoc_insertion_point(field:KIPHKHHMFAC.BCAFJHGDODI)
    pub BCAFJHGDODI: ::std::string::String,
    // @@protoc_insertion_point(field:KIPHKHHMFAC.DBCKEIOJLDI)
    pub DBCKEIOJLDI: ::std::string::String,
    // @@protoc_insertion_point(field:KIPHKHHMFAC.MNCIHJHGNMJ)
    pub MNCIHJHGNMJ: u32,
    // @@protoc_insertion_point(field:KIPHKHHMFAC.POKIDOENKEK)
    pub POKIDOENKEK: u32,
    // @@protoc_insertion_point(field:KIPHKHHMFAC.KDCIGPAMGHO)
    pub KDCIGPAMGHO: u32,
    // @@protoc_insertion_point(field:KIPHKHHMFAC.begin_time)
    pub begin_time: i64,
    // @@protoc_insertion_point(field:KIPHKHHMFAC.GHCAAONIGCA)
    pub GHCAAONIGCA: bool,
    // @@protoc_insertion_point(field:KIPHKHHMFAC.ADJCAKNJCGO)
    pub ADJCAKNJCGO: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:KIPHKHHMFAC.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a KIPHKHHMFAC {
    fn default() -> &'a KIPHKHHMFAC {
        <KIPHKHHMFAC as ::protobuf::Message>::default_instance()
    }
}

impl KIPHKHHMFAC {
    pub fn new() -> KIPHKHHMFAC {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "end_time",
            |m: &KIPHKHHMFAC| { &m.end_time },
            |m: &mut KIPHKHHMFAC| { &mut m.end_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BCAFJHGDODI",
            |m: &KIPHKHHMFAC| { &m.BCAFJHGDODI },
            |m: &mut KIPHKHHMFAC| { &mut m.BCAFJHGDODI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DBCKEIOJLDI",
            |m: &KIPHKHHMFAC| { &m.DBCKEIOJLDI },
            |m: &mut KIPHKHHMFAC| { &mut m.DBCKEIOJLDI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MNCIHJHGNMJ",
            |m: &KIPHKHHMFAC| { &m.MNCIHJHGNMJ },
            |m: &mut KIPHKHHMFAC| { &mut m.MNCIHJHGNMJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "POKIDOENKEK",
            |m: &KIPHKHHMFAC| { &m.POKIDOENKEK },
            |m: &mut KIPHKHHMFAC| { &mut m.POKIDOENKEK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KDCIGPAMGHO",
            |m: &KIPHKHHMFAC| { &m.KDCIGPAMGHO },
            |m: &mut KIPHKHHMFAC| { &mut m.KDCIGPAMGHO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "begin_time",
            |m: &KIPHKHHMFAC| { &m.begin_time },
            |m: &mut KIPHKHHMFAC| { &mut m.begin_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GHCAAONIGCA",
            |m: &KIPHKHHMFAC| { &m.GHCAAONIGCA },
            |m: &mut KIPHKHHMFAC| { &mut m.GHCAAONIGCA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADJCAKNJCGO",
            |m: &KIPHKHHMFAC| { &m.ADJCAKNJCGO },
            |m: &mut KIPHKHHMFAC| { &mut m.ADJCAKNJCGO },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<KIPHKHHMFAC>(
            "KIPHKHHMFAC",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for KIPHKHHMFAC {
    const NAME: &'static str = "KIPHKHHMFAC";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.end_time = is.read_int64()?;
                },
                74 => {
                    self.BCAFJHGDODI = is.read_string()?;
                },
                18 => {
                    self.DBCKEIOJLDI = is.read_string()?;
                },
                112 => {
                    self.MNCIHJHGNMJ = is.read_uint32()?;
                },
                120 => {
                    self.POKIDOENKEK = is.read_uint32()?;
                },
                104 => {
                    self.KDCIGPAMGHO = is.read_uint32()?;
                },
                48 => {
                    self.begin_time = is.read_int64()?;
                },
                80 => {
                    self.GHCAAONIGCA = is.read_bool()?;
                },
                10 => {
                    self.ADJCAKNJCGO = is.read_string()?;
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
        if self.end_time != 0 {
            my_size += ::protobuf::rt::int64_size(3, self.end_time);
        }
        if !self.BCAFJHGDODI.is_empty() {
            my_size += ::protobuf::rt::string_size(9, &self.BCAFJHGDODI);
        }
        if !self.DBCKEIOJLDI.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.DBCKEIOJLDI);
        }
        if self.MNCIHJHGNMJ != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.MNCIHJHGNMJ);
        }
        if self.POKIDOENKEK != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.POKIDOENKEK);
        }
        if self.KDCIGPAMGHO != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.KDCIGPAMGHO);
        }
        if self.begin_time != 0 {
            my_size += ::protobuf::rt::int64_size(6, self.begin_time);
        }
        if self.GHCAAONIGCA != false {
            my_size += 1 + 1;
        }
        if !self.ADJCAKNJCGO.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.ADJCAKNJCGO);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.end_time != 0 {
            os.write_int64(3, self.end_time)?;
        }
        if !self.BCAFJHGDODI.is_empty() {
            os.write_string(9, &self.BCAFJHGDODI)?;
        }
        if !self.DBCKEIOJLDI.is_empty() {
            os.write_string(2, &self.DBCKEIOJLDI)?;
        }
        if self.MNCIHJHGNMJ != 0 {
            os.write_uint32(14, self.MNCIHJHGNMJ)?;
        }
        if self.POKIDOENKEK != 0 {
            os.write_uint32(15, self.POKIDOENKEK)?;
        }
        if self.KDCIGPAMGHO != 0 {
            os.write_uint32(13, self.KDCIGPAMGHO)?;
        }
        if self.begin_time != 0 {
            os.write_int64(6, self.begin_time)?;
        }
        if self.GHCAAONIGCA != false {
            os.write_bool(10, self.GHCAAONIGCA)?;
        }
        if !self.ADJCAKNJCGO.is_empty() {
            os.write_string(1, &self.ADJCAKNJCGO)?;
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

    fn new() -> KIPHKHHMFAC {
        KIPHKHHMFAC::new()
    }

    fn clear(&mut self) {
        self.end_time = 0;
        self.BCAFJHGDODI.clear();
        self.DBCKEIOJLDI.clear();
        self.MNCIHJHGNMJ = 0;
        self.POKIDOENKEK = 0;
        self.KDCIGPAMGHO = 0;
        self.begin_time = 0;
        self.GHCAAONIGCA = false;
        self.ADJCAKNJCGO.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static KIPHKHHMFAC {
        static instance: KIPHKHHMFAC = KIPHKHHMFAC {
            end_time: 0,
            BCAFJHGDODI: ::std::string::String::new(),
            DBCKEIOJLDI: ::std::string::String::new(),
            MNCIHJHGNMJ: 0,
            POKIDOENKEK: 0,
            KDCIGPAMGHO: 0,
            begin_time: 0,
            GHCAAONIGCA: false,
            ADJCAKNJCGO: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for KIPHKHHMFAC {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("KIPHKHHMFAC").unwrap()).clone()
    }
}

impl ::std::fmt::Display for KIPHKHHMFAC {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KIPHKHHMFAC {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11KIPHKHHMFAC.proto\"\xb5\x02\n\x0bKIPHKHHMFAC\x12\x19\n\x08end_time\
    \x18\x03\x20\x01(\x03R\x07endTime\x12\x20\n\x0bBCAFJHGDODI\x18\t\x20\x01\
    (\tR\x0bBCAFJHGDODI\x12\x20\n\x0bDBCKEIOJLDI\x18\x02\x20\x01(\tR\x0bDBCK\
    EIOJLDI\x12\x20\n\x0bMNCIHJHGNMJ\x18\x0e\x20\x01(\rR\x0bMNCIHJHGNMJ\x12\
    \x20\n\x0bPOKIDOENKEK\x18\x0f\x20\x01(\rR\x0bPOKIDOENKEK\x12\x20\n\x0bKD\
    CIGPAMGHO\x18\r\x20\x01(\rR\x0bKDCIGPAMGHO\x12\x1d\n\nbegin_time\x18\x06\
    \x20\x01(\x03R\tbeginTime\x12\x20\n\x0bGHCAAONIGCA\x18\n\x20\x01(\x08R\
    \x0bGHCAAONIGCA\x12\x20\n\x0bADJCAKNJCGO\x18\x01\x20\x01(\tR\x0bADJCAKNJ\
    CGOb\x06proto3\
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
            messages.push(KIPHKHHMFAC::generated_message_descriptor_data());
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
