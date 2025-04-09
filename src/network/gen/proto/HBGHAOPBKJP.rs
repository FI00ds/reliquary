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

//! Generated file from `HBGHAOPBKJP.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:HBGHAOPBKJP)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HBGHAOPBKJP {
    // message fields
    // @@protoc_insertion_point(field:HBGHAOPBKJP.PLFKOCCDBAG)
    pub PLFKOCCDBAG: bool,
    // @@protoc_insertion_point(field:HBGHAOPBKJP.ELGANMDPMID)
    pub ELGANMDPMID: u32,
    // @@protoc_insertion_point(field:HBGHAOPBKJP.level)
    pub level: u32,
    // @@protoc_insertion_point(field:HBGHAOPBKJP.LNBPGPGGIMN)
    pub LNBPGPGGIMN: u32,
    // @@protoc_insertion_point(field:HBGHAOPBKJP.PMANBPLFLKL)
    pub PMANBPLFLKL: bool,
    // @@protoc_insertion_point(field:HBGHAOPBKJP.FBNHDEFNECI)
    pub FBNHDEFNECI: i32,
    // @@protoc_insertion_point(field:HBGHAOPBKJP.JLCIKBLNENH)
    pub JLCIKBLNENH: u32,
    // @@protoc_insertion_point(field:HBGHAOPBKJP.MNBEMGNNFOD)
    pub MNBEMGNNFOD: f32,
    // @@protoc_insertion_point(field:HBGHAOPBKJP.DMBBMFFEJGI)
    pub DMBBMFFEJGI: bool,
    // @@protoc_insertion_point(field:HBGHAOPBKJP.BMNECPIOPDN)
    pub BMNECPIOPDN: bool,
    // @@protoc_insertion_point(field:HBGHAOPBKJP.LKEFOLCGFGD)
    pub LKEFOLCGFGD: ::protobuf::MessageField<super::PFGAIEBGHCP::PFGAIEBGHCP>,
    // @@protoc_insertion_point(field:HBGHAOPBKJP.TURN_FOOD_SWITCH_ATTACK)
    pub TURN_FOOD_SWITCH_ATTACK: i32,
    // @@protoc_insertion_point(field:HBGHAOPBKJP.FNIHJJJGOEE)
    pub FNIHJJJGOEE: ::protobuf::MessageField<super::PFGAIEBGHCP::PFGAIEBGHCP>,
    // @@protoc_insertion_point(field:HBGHAOPBKJP.CLFGPLOMPKG)
    pub CLFGPLOMPKG: i32,
    // special fields
    // @@protoc_insertion_point(special_field:HBGHAOPBKJP.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HBGHAOPBKJP {
    fn default() -> &'a HBGHAOPBKJP {
        <HBGHAOPBKJP as ::protobuf::Message>::default_instance()
    }
}

impl HBGHAOPBKJP {
    pub fn new() -> HBGHAOPBKJP {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(14);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PLFKOCCDBAG",
            |m: &HBGHAOPBKJP| { &m.PLFKOCCDBAG },
            |m: &mut HBGHAOPBKJP| { &mut m.PLFKOCCDBAG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ELGANMDPMID",
            |m: &HBGHAOPBKJP| { &m.ELGANMDPMID },
            |m: &mut HBGHAOPBKJP| { &mut m.ELGANMDPMID },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &HBGHAOPBKJP| { &m.level },
            |m: &mut HBGHAOPBKJP| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LNBPGPGGIMN",
            |m: &HBGHAOPBKJP| { &m.LNBPGPGGIMN },
            |m: &mut HBGHAOPBKJP| { &mut m.LNBPGPGGIMN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PMANBPLFLKL",
            |m: &HBGHAOPBKJP| { &m.PMANBPLFLKL },
            |m: &mut HBGHAOPBKJP| { &mut m.PMANBPLFLKL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FBNHDEFNECI",
            |m: &HBGHAOPBKJP| { &m.FBNHDEFNECI },
            |m: &mut HBGHAOPBKJP| { &mut m.FBNHDEFNECI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JLCIKBLNENH",
            |m: &HBGHAOPBKJP| { &m.JLCIKBLNENH },
            |m: &mut HBGHAOPBKJP| { &mut m.JLCIKBLNENH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MNBEMGNNFOD",
            |m: &HBGHAOPBKJP| { &m.MNBEMGNNFOD },
            |m: &mut HBGHAOPBKJP| { &mut m.MNBEMGNNFOD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DMBBMFFEJGI",
            |m: &HBGHAOPBKJP| { &m.DMBBMFFEJGI },
            |m: &mut HBGHAOPBKJP| { &mut m.DMBBMFFEJGI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BMNECPIOPDN",
            |m: &HBGHAOPBKJP| { &m.BMNECPIOPDN },
            |m: &mut HBGHAOPBKJP| { &mut m.BMNECPIOPDN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PFGAIEBGHCP::PFGAIEBGHCP>(
            "LKEFOLCGFGD",
            |m: &HBGHAOPBKJP| { &m.LKEFOLCGFGD },
            |m: &mut HBGHAOPBKJP| { &mut m.LKEFOLCGFGD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "TURN_FOOD_SWITCH_ATTACK",
            |m: &HBGHAOPBKJP| { &m.TURN_FOOD_SWITCH_ATTACK },
            |m: &mut HBGHAOPBKJP| { &mut m.TURN_FOOD_SWITCH_ATTACK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PFGAIEBGHCP::PFGAIEBGHCP>(
            "FNIHJJJGOEE",
            |m: &HBGHAOPBKJP| { &m.FNIHJJJGOEE },
            |m: &mut HBGHAOPBKJP| { &mut m.FNIHJJJGOEE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CLFGPLOMPKG",
            |m: &HBGHAOPBKJP| { &m.CLFGPLOMPKG },
            |m: &mut HBGHAOPBKJP| { &mut m.CLFGPLOMPKG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HBGHAOPBKJP>(
            "HBGHAOPBKJP",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HBGHAOPBKJP {
    const NAME: &'static str = "HBGHAOPBKJP";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.PLFKOCCDBAG = is.read_bool()?;
                },
                72 => {
                    self.ELGANMDPMID = is.read_uint32()?;
                },
                32 => {
                    self.level = is.read_uint32()?;
                },
                120 => {
                    self.LNBPGPGGIMN = is.read_uint32()?;
                },
                16 => {
                    self.PMANBPLFLKL = is.read_bool()?;
                },
                80 => {
                    self.FBNHDEFNECI = is.read_int32()?;
                },
                24 => {
                    self.JLCIKBLNENH = is.read_uint32()?;
                },
                69 => {
                    self.MNBEMGNNFOD = is.read_float()?;
                },
                40 => {
                    self.DMBBMFFEJGI = is.read_bool()?;
                },
                112 => {
                    self.BMNECPIOPDN = is.read_bool()?;
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LKEFOLCGFGD)?;
                },
                56 => {
                    self.TURN_FOOD_SWITCH_ATTACK = is.read_int32()?;
                },
                98 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.FNIHJJJGOEE)?;
                },
                48 => {
                    self.CLFGPLOMPKG = is.read_int32()?;
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
        if self.PLFKOCCDBAG != false {
            my_size += 1 + 1;
        }
        if self.ELGANMDPMID != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.ELGANMDPMID);
        }
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.level);
        }
        if self.LNBPGPGGIMN != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.LNBPGPGGIMN);
        }
        if self.PMANBPLFLKL != false {
            my_size += 1 + 1;
        }
        if self.FBNHDEFNECI != 0 {
            my_size += ::protobuf::rt::int32_size(10, self.FBNHDEFNECI);
        }
        if self.JLCIKBLNENH != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.JLCIKBLNENH);
        }
        if self.MNBEMGNNFOD != 0. {
            my_size += 1 + 4;
        }
        if self.DMBBMFFEJGI != false {
            my_size += 1 + 1;
        }
        if self.BMNECPIOPDN != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.LKEFOLCGFGD.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.TURN_FOOD_SWITCH_ATTACK != 0 {
            my_size += ::protobuf::rt::int32_size(7, self.TURN_FOOD_SWITCH_ATTACK);
        }
        if let Some(v) = self.FNIHJJJGOEE.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.CLFGPLOMPKG != 0 {
            my_size += ::protobuf::rt::int32_size(6, self.CLFGPLOMPKG);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.PLFKOCCDBAG != false {
            os.write_bool(1, self.PLFKOCCDBAG)?;
        }
        if self.ELGANMDPMID != 0 {
            os.write_uint32(9, self.ELGANMDPMID)?;
        }
        if self.level != 0 {
            os.write_uint32(4, self.level)?;
        }
        if self.LNBPGPGGIMN != 0 {
            os.write_uint32(15, self.LNBPGPGGIMN)?;
        }
        if self.PMANBPLFLKL != false {
            os.write_bool(2, self.PMANBPLFLKL)?;
        }
        if self.FBNHDEFNECI != 0 {
            os.write_int32(10, self.FBNHDEFNECI)?;
        }
        if self.JLCIKBLNENH != 0 {
            os.write_uint32(3, self.JLCIKBLNENH)?;
        }
        if self.MNBEMGNNFOD != 0. {
            os.write_float(8, self.MNBEMGNNFOD)?;
        }
        if self.DMBBMFFEJGI != false {
            os.write_bool(5, self.DMBBMFFEJGI)?;
        }
        if self.BMNECPIOPDN != false {
            os.write_bool(14, self.BMNECPIOPDN)?;
        }
        if let Some(v) = self.LKEFOLCGFGD.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if self.TURN_FOOD_SWITCH_ATTACK != 0 {
            os.write_int32(7, self.TURN_FOOD_SWITCH_ATTACK)?;
        }
        if let Some(v) = self.FNIHJJJGOEE.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        }
        if self.CLFGPLOMPKG != 0 {
            os.write_int32(6, self.CLFGPLOMPKG)?;
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

    fn new() -> HBGHAOPBKJP {
        HBGHAOPBKJP::new()
    }

    fn clear(&mut self) {
        self.PLFKOCCDBAG = false;
        self.ELGANMDPMID = 0;
        self.level = 0;
        self.LNBPGPGGIMN = 0;
        self.PMANBPLFLKL = false;
        self.FBNHDEFNECI = 0;
        self.JLCIKBLNENH = 0;
        self.MNBEMGNNFOD = 0.;
        self.DMBBMFFEJGI = false;
        self.BMNECPIOPDN = false;
        self.LKEFOLCGFGD.clear();
        self.TURN_FOOD_SWITCH_ATTACK = 0;
        self.FNIHJJJGOEE.clear();
        self.CLFGPLOMPKG = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HBGHAOPBKJP {
        static instance: HBGHAOPBKJP = HBGHAOPBKJP {
            PLFKOCCDBAG: false,
            ELGANMDPMID: 0,
            level: 0,
            LNBPGPGGIMN: 0,
            PMANBPLFLKL: false,
            FBNHDEFNECI: 0,
            JLCIKBLNENH: 0,
            MNBEMGNNFOD: 0.,
            DMBBMFFEJGI: false,
            BMNECPIOPDN: false,
            LKEFOLCGFGD: ::protobuf::MessageField::none(),
            TURN_FOOD_SWITCH_ATTACK: 0,
            FNIHJJJGOEE: ::protobuf::MessageField::none(),
            CLFGPLOMPKG: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HBGHAOPBKJP {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HBGHAOPBKJP").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HBGHAOPBKJP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HBGHAOPBKJP {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11HBGHAOPBKJP.proto\x1a\x11PFGAIEBGHCP.proto\"\x8e\x04\n\x0bHBGHAOPB\
    KJP\x12\x20\n\x0bPLFKOCCDBAG\x18\x01\x20\x01(\x08R\x0bPLFKOCCDBAG\x12\
    \x20\n\x0bELGANMDPMID\x18\t\x20\x01(\rR\x0bELGANMDPMID\x12\x14\n\x05leve\
    l\x18\x04\x20\x01(\rR\x05level\x12\x20\n\x0bLNBPGPGGIMN\x18\x0f\x20\x01(\
    \rR\x0bLNBPGPGGIMN\x12\x20\n\x0bPMANBPLFLKL\x18\x02\x20\x01(\x08R\x0bPMA\
    NBPLFLKL\x12\x20\n\x0bFBNHDEFNECI\x18\n\x20\x01(\x05R\x0bFBNHDEFNECI\x12\
    \x20\n\x0bJLCIKBLNENH\x18\x03\x20\x01(\rR\x0bJLCIKBLNENH\x12\x20\n\x0bMN\
    BEMGNNFOD\x18\x08\x20\x01(\x02R\x0bMNBEMGNNFOD\x12\x20\n\x0bDMBBMFFEJGI\
    \x18\x05\x20\x01(\x08R\x0bDMBBMFFEJGI\x12\x20\n\x0bBMNECPIOPDN\x18\x0e\
    \x20\x01(\x08R\x0bBMNECPIOPDN\x12.\n\x0bLKEFOLCGFGD\x18\r\x20\x01(\x0b2\
    \x0c.PFGAIEBGHCPR\x0bLKEFOLCGFGD\x125\n\x17TURN_FOOD_SWITCH_ATTACK\x18\
    \x07\x20\x01(\x05R\x14TURNFOODSWITCHATTACK\x12.\n\x0bFNIHJJJGOEE\x18\x0c\
    \x20\x01(\x0b2\x0c.PFGAIEBGHCPR\x0bFNIHJJJGOEE\x12\x20\n\x0bCLFGPLOMPKG\
    \x18\x06\x20\x01(\x05R\x0bCLFGPLOMPKGb\x06proto3\
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
            deps.push(super::PFGAIEBGHCP::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(HBGHAOPBKJP::generated_message_descriptor_data());
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
