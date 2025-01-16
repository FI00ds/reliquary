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

//! Generated file from `IONNOLIHAAG.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:IONNOLIHAAG)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct IONNOLIHAAG {
    // message fields
    // @@protoc_insertion_point(field:IONNOLIHAAG.ENFBJBHPGGN)
    pub ENFBJBHPGGN: ::protobuf::MessageField<super::FLPFHGIMEND::FLPFHGIMEND>,
    // @@protoc_insertion_point(field:IONNOLIHAAG.EELBEPMNPGI)
    pub EELBEPMNPGI: ::protobuf::MessageField<super::PMBHFPCDGBM::PMBHFPCDGBM>,
    // @@protoc_insertion_point(field:IONNOLIHAAG.IOHMENLMDDA)
    pub IOHMENLMDDA: ::protobuf::MessageField<super::ANGBDPLNIDK::ANGBDPLNIDK>,
    // @@protoc_insertion_point(field:IONNOLIHAAG.BGLBBOGHHIM)
    pub BGLBBOGHHIM: ::protobuf::MessageField<super::EAGFKGAICCA::EAGFKGAICCA>,
    // @@protoc_insertion_point(field:IONNOLIHAAG.KCFMIIDPPBH)
    pub KCFMIIDPPBH: ::protobuf::MessageField<super::NFAEIMEGIHG::NFAEIMEGIHG>,
    // special fields
    // @@protoc_insertion_point(special_field:IONNOLIHAAG.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a IONNOLIHAAG {
    fn default() -> &'a IONNOLIHAAG {
        <IONNOLIHAAG as ::protobuf::Message>::default_instance()
    }
}

impl IONNOLIHAAG {
    pub fn new() -> IONNOLIHAAG {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::FLPFHGIMEND::FLPFHGIMEND>(
            "ENFBJBHPGGN",
            |m: &IONNOLIHAAG| { &m.ENFBJBHPGGN },
            |m: &mut IONNOLIHAAG| { &mut m.ENFBJBHPGGN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PMBHFPCDGBM::PMBHFPCDGBM>(
            "EELBEPMNPGI",
            |m: &IONNOLIHAAG| { &m.EELBEPMNPGI },
            |m: &mut IONNOLIHAAG| { &mut m.EELBEPMNPGI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ANGBDPLNIDK::ANGBDPLNIDK>(
            "IOHMENLMDDA",
            |m: &IONNOLIHAAG| { &m.IOHMENLMDDA },
            |m: &mut IONNOLIHAAG| { &mut m.IOHMENLMDDA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::EAGFKGAICCA::EAGFKGAICCA>(
            "BGLBBOGHHIM",
            |m: &IONNOLIHAAG| { &m.BGLBBOGHHIM },
            |m: &mut IONNOLIHAAG| { &mut m.BGLBBOGHHIM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::NFAEIMEGIHG::NFAEIMEGIHG>(
            "KCFMIIDPPBH",
            |m: &IONNOLIHAAG| { &m.KCFMIIDPPBH },
            |m: &mut IONNOLIHAAG| { &mut m.KCFMIIDPPBH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<IONNOLIHAAG>(
            "IONNOLIHAAG",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for IONNOLIHAAG {
    const NAME: &'static str = "IONNOLIHAAG";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ENFBJBHPGGN)?;
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.EELBEPMNPGI)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.IOHMENLMDDA)?;
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.BGLBBOGHHIM)?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KCFMIIDPPBH)?;
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
        if let Some(v) = self.ENFBJBHPGGN.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.EELBEPMNPGI.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.IOHMENLMDDA.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.BGLBBOGHHIM.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.KCFMIIDPPBH.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.ENFBJBHPGGN.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if let Some(v) = self.EELBEPMNPGI.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        if let Some(v) = self.IOHMENLMDDA.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if let Some(v) = self.BGLBBOGHHIM.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        if let Some(v) = self.KCFMIIDPPBH.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
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

    fn new() -> IONNOLIHAAG {
        IONNOLIHAAG::new()
    }

    fn clear(&mut self) {
        self.ENFBJBHPGGN.clear();
        self.EELBEPMNPGI.clear();
        self.IOHMENLMDDA.clear();
        self.BGLBBOGHHIM.clear();
        self.KCFMIIDPPBH.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static IONNOLIHAAG {
        static instance: IONNOLIHAAG = IONNOLIHAAG {
            ENFBJBHPGGN: ::protobuf::MessageField::none(),
            EELBEPMNPGI: ::protobuf::MessageField::none(),
            IOHMENLMDDA: ::protobuf::MessageField::none(),
            BGLBBOGHHIM: ::protobuf::MessageField::none(),
            KCFMIIDPPBH: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for IONNOLIHAAG {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("IONNOLIHAAG").unwrap()).clone()
    }
}

impl ::std::fmt::Display for IONNOLIHAAG {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IONNOLIHAAG {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11IONNOLIHAAG.proto\x1a\x11ANGBDPLNIDK.proto\x1a\x11EAGFKGAICCA.prot\
    o\x1a\x11FLPFHGIMEND.proto\x1a\x11NFAEIMEGIHG.proto\x1a\x11PMBHFPCDGBM.p\
    roto\"\xfd\x01\n\x0bIONNOLIHAAG\x12.\n\x0bENFBJBHPGGN\x18\r\x20\x01(\x0b\
    2\x0c.FLPFHGIMENDR\x0bENFBJBHPGGN\x12.\n\x0bEELBEPMNPGI\x18\x06\x20\x01(\
    \x0b2\x0c.PMBHFPCDGBMR\x0bEELBEPMNPGI\x12.\n\x0bIOHMENLMDDA\x18\x02\x20\
    \x01(\x0b2\x0c.ANGBDPLNIDKR\x0bIOHMENLMDDA\x12.\n\x0bBGLBBOGHHIM\x18\n\
    \x20\x01(\x0b2\x0c.EAGFKGAICCAR\x0bBGLBBOGHHIM\x12.\n\x0bKCFMIIDPPBH\x18\
    \t\x20\x01(\x0b2\x0c.NFAEIMEGIHGR\x0bKCFMIIDPPBHb\x06proto3\
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
            deps.push(super::ANGBDPLNIDK::file_descriptor().clone());
            deps.push(super::EAGFKGAICCA::file_descriptor().clone());
            deps.push(super::FLPFHGIMEND::file_descriptor().clone());
            deps.push(super::NFAEIMEGIHG::file_descriptor().clone());
            deps.push(super::PMBHFPCDGBM::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(IONNOLIHAAG::generated_message_descriptor_data());
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
