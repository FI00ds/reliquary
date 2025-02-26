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

//! Generated file from `ChallengeBossPhaseSettleNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:ChallengeBossPhaseSettleNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChallengeBossPhaseSettleNotify {
    // message fields
    // @@protoc_insertion_point(field:ChallengeBossPhaseSettleNotify.NJBFDIAKFHN)
    pub NJBFDIAKFHN: bool,
    // @@protoc_insertion_point(field:ChallengeBossPhaseSettleNotify.FEPKAMAILMK)
    pub FEPKAMAILMK: u32,
    // @@protoc_insertion_point(field:ChallengeBossPhaseSettleNotify.FIBIBDEABFN)
    pub FIBIBDEABFN: ::std::vec::Vec<super::BattleTarget::BattleTarget>,
    // @@protoc_insertion_point(field:ChallengeBossPhaseSettleNotify.BPPLLCHMIDD)
    pub BPPLLCHMIDD: bool,
    // @@protoc_insertion_point(field:ChallengeBossPhaseSettleNotify.CFAAFJJAADP)
    pub CFAAFJJAADP: u32,
    // @@protoc_insertion_point(field:ChallengeBossPhaseSettleNotify.AFCHLCMCMHG)
    pub AFCHLCMCMHG: u32,
    // @@protoc_insertion_point(field:ChallengeBossPhaseSettleNotify.FCMAGJBLGOJ)
    pub FCMAGJBLGOJ: u32,
    // @@protoc_insertion_point(field:ChallengeBossPhaseSettleNotify.HALGPJMCMFP)
    pub HALGPJMCMFP: u32,
    // @@protoc_insertion_point(field:ChallengeBossPhaseSettleNotify.ILHAFBKCKOG)
    pub ILHAFBKCKOG: bool,
    // @@protoc_insertion_point(field:ChallengeBossPhaseSettleNotify.PIMBLBKEECJ)
    pub PIMBLBKEECJ: u32,
    // special fields
    // @@protoc_insertion_point(special_field:ChallengeBossPhaseSettleNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChallengeBossPhaseSettleNotify {
    fn default() -> &'a ChallengeBossPhaseSettleNotify {
        <ChallengeBossPhaseSettleNotify as ::protobuf::Message>::default_instance()
    }
}

impl ChallengeBossPhaseSettleNotify {
    pub fn new() -> ChallengeBossPhaseSettleNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(10);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NJBFDIAKFHN",
            |m: &ChallengeBossPhaseSettleNotify| { &m.NJBFDIAKFHN },
            |m: &mut ChallengeBossPhaseSettleNotify| { &mut m.NJBFDIAKFHN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FEPKAMAILMK",
            |m: &ChallengeBossPhaseSettleNotify| { &m.FEPKAMAILMK },
            |m: &mut ChallengeBossPhaseSettleNotify| { &mut m.FEPKAMAILMK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FIBIBDEABFN",
            |m: &ChallengeBossPhaseSettleNotify| { &m.FIBIBDEABFN },
            |m: &mut ChallengeBossPhaseSettleNotify| { &mut m.FIBIBDEABFN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BPPLLCHMIDD",
            |m: &ChallengeBossPhaseSettleNotify| { &m.BPPLLCHMIDD },
            |m: &mut ChallengeBossPhaseSettleNotify| { &mut m.BPPLLCHMIDD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CFAAFJJAADP",
            |m: &ChallengeBossPhaseSettleNotify| { &m.CFAAFJJAADP },
            |m: &mut ChallengeBossPhaseSettleNotify| { &mut m.CFAAFJJAADP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AFCHLCMCMHG",
            |m: &ChallengeBossPhaseSettleNotify| { &m.AFCHLCMCMHG },
            |m: &mut ChallengeBossPhaseSettleNotify| { &mut m.AFCHLCMCMHG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FCMAGJBLGOJ",
            |m: &ChallengeBossPhaseSettleNotify| { &m.FCMAGJBLGOJ },
            |m: &mut ChallengeBossPhaseSettleNotify| { &mut m.FCMAGJBLGOJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HALGPJMCMFP",
            |m: &ChallengeBossPhaseSettleNotify| { &m.HALGPJMCMFP },
            |m: &mut ChallengeBossPhaseSettleNotify| { &mut m.HALGPJMCMFP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ILHAFBKCKOG",
            |m: &ChallengeBossPhaseSettleNotify| { &m.ILHAFBKCKOG },
            |m: &mut ChallengeBossPhaseSettleNotify| { &mut m.ILHAFBKCKOG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PIMBLBKEECJ",
            |m: &ChallengeBossPhaseSettleNotify| { &m.PIMBLBKEECJ },
            |m: &mut ChallengeBossPhaseSettleNotify| { &mut m.PIMBLBKEECJ },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChallengeBossPhaseSettleNotify>(
            "ChallengeBossPhaseSettleNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChallengeBossPhaseSettleNotify {
    const NAME: &'static str = "ChallengeBossPhaseSettleNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.NJBFDIAKFHN = is.read_bool()?;
                },
                72 => {
                    self.FEPKAMAILMK = is.read_uint32()?;
                },
                66 => {
                    self.FIBIBDEABFN.push(is.read_message()?);
                },
                80 => {
                    self.BPPLLCHMIDD = is.read_bool()?;
                },
                48 => {
                    self.CFAAFJJAADP = is.read_uint32()?;
                },
                24 => {
                    self.AFCHLCMCMHG = is.read_uint32()?;
                },
                32 => {
                    self.FCMAGJBLGOJ = is.read_uint32()?;
                },
                96 => {
                    self.HALGPJMCMFP = is.read_uint32()?;
                },
                56 => {
                    self.ILHAFBKCKOG = is.read_bool()?;
                },
                112 => {
                    self.PIMBLBKEECJ = is.read_uint32()?;
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
        if self.NJBFDIAKFHN != false {
            my_size += 1 + 1;
        }
        if self.FEPKAMAILMK != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.FEPKAMAILMK);
        }
        for value in &self.FIBIBDEABFN {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.BPPLLCHMIDD != false {
            my_size += 1 + 1;
        }
        if self.CFAAFJJAADP != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.CFAAFJJAADP);
        }
        if self.AFCHLCMCMHG != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.AFCHLCMCMHG);
        }
        if self.FCMAGJBLGOJ != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.FCMAGJBLGOJ);
        }
        if self.HALGPJMCMFP != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.HALGPJMCMFP);
        }
        if self.ILHAFBKCKOG != false {
            my_size += 1 + 1;
        }
        if self.PIMBLBKEECJ != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.PIMBLBKEECJ);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.NJBFDIAKFHN != false {
            os.write_bool(13, self.NJBFDIAKFHN)?;
        }
        if self.FEPKAMAILMK != 0 {
            os.write_uint32(9, self.FEPKAMAILMK)?;
        }
        for v in &self.FIBIBDEABFN {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        if self.BPPLLCHMIDD != false {
            os.write_bool(10, self.BPPLLCHMIDD)?;
        }
        if self.CFAAFJJAADP != 0 {
            os.write_uint32(6, self.CFAAFJJAADP)?;
        }
        if self.AFCHLCMCMHG != 0 {
            os.write_uint32(3, self.AFCHLCMCMHG)?;
        }
        if self.FCMAGJBLGOJ != 0 {
            os.write_uint32(4, self.FCMAGJBLGOJ)?;
        }
        if self.HALGPJMCMFP != 0 {
            os.write_uint32(12, self.HALGPJMCMFP)?;
        }
        if self.ILHAFBKCKOG != false {
            os.write_bool(7, self.ILHAFBKCKOG)?;
        }
        if self.PIMBLBKEECJ != 0 {
            os.write_uint32(14, self.PIMBLBKEECJ)?;
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

    fn new() -> ChallengeBossPhaseSettleNotify {
        ChallengeBossPhaseSettleNotify::new()
    }

    fn clear(&mut self) {
        self.NJBFDIAKFHN = false;
        self.FEPKAMAILMK = 0;
        self.FIBIBDEABFN.clear();
        self.BPPLLCHMIDD = false;
        self.CFAAFJJAADP = 0;
        self.AFCHLCMCMHG = 0;
        self.FCMAGJBLGOJ = 0;
        self.HALGPJMCMFP = 0;
        self.ILHAFBKCKOG = false;
        self.PIMBLBKEECJ = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChallengeBossPhaseSettleNotify {
        static instance: ChallengeBossPhaseSettleNotify = ChallengeBossPhaseSettleNotify {
            NJBFDIAKFHN: false,
            FEPKAMAILMK: 0,
            FIBIBDEABFN: ::std::vec::Vec::new(),
            BPPLLCHMIDD: false,
            CFAAFJJAADP: 0,
            AFCHLCMCMHG: 0,
            FCMAGJBLGOJ: 0,
            HALGPJMCMFP: 0,
            ILHAFBKCKOG: false,
            PIMBLBKEECJ: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ChallengeBossPhaseSettleNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChallengeBossPhaseSettleNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChallengeBossPhaseSettleNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChallengeBossPhaseSettleNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n$ChallengeBossPhaseSettleNotify.proto\x1a\x12BattleTarget.proto\"\x83\
    \x03\n\x1eChallengeBossPhaseSettleNotify\x12\x20\n\x0bNJBFDIAKFHN\x18\r\
    \x20\x01(\x08R\x0bNJBFDIAKFHN\x12\x20\n\x0bFEPKAMAILMK\x18\t\x20\x01(\rR\
    \x0bFEPKAMAILMK\x12/\n\x0bFIBIBDEABFN\x18\x08\x20\x03(\x0b2\r.BattleTarg\
    etR\x0bFIBIBDEABFN\x12\x20\n\x0bBPPLLCHMIDD\x18\n\x20\x01(\x08R\x0bBPPLL\
    CHMIDD\x12\x20\n\x0bCFAAFJJAADP\x18\x06\x20\x01(\rR\x0bCFAAFJJAADP\x12\
    \x20\n\x0bAFCHLCMCMHG\x18\x03\x20\x01(\rR\x0bAFCHLCMCMHG\x12\x20\n\x0bFC\
    MAGJBLGOJ\x18\x04\x20\x01(\rR\x0bFCMAGJBLGOJ\x12\x20\n\x0bHALGPJMCMFP\
    \x18\x0c\x20\x01(\rR\x0bHALGPJMCMFP\x12\x20\n\x0bILHAFBKCKOG\x18\x07\x20\
    \x01(\x08R\x0bILHAFBKCKOG\x12\x20\n\x0bPIMBLBKEECJ\x18\x0e\x20\x01(\rR\
    \x0bPIMBLBKEECJb\x06proto3\
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
            deps.push(super::BattleTarget::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ChallengeBossPhaseSettleNotify::generated_message_descriptor_data());
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
