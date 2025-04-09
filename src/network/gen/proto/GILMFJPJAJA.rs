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

//! Generated file from `GILMFJPJAJA.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:GILMFJPJAJA)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GILMFJPJAJA {
    // message fields
    // @@protoc_insertion_point(field:GILMFJPJAJA.EHJEDEONJKK)
    pub EHJEDEONJKK: ::std::collections::HashMap<u32, super::AAFKIMACHFG::AAFKIMACHFG>,
    // @@protoc_insertion_point(field:GILMFJPJAJA.IPHNKFJCKGP)
    pub IPHNKFJCKGP: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GILMFJPJAJA.JGCPAGCPHCJ)
    pub JGCPAGCPHCJ: ::protobuf::MessageField<super::CDGJKFMFKDL::CDGJKFMFKDL>,
    // @@protoc_insertion_point(field:GILMFJPJAJA.EGDMGHLDFKD)
    pub EGDMGHLDFKD: ::protobuf::MessageField<super::CDGJKFMFKDL::CDGJKFMFKDL>,
    // @@protoc_insertion_point(field:GILMFJPJAJA.NCBDNPGPEAI)
    pub NCBDNPGPEAI: bool,
    // @@protoc_insertion_point(field:GILMFJPJAJA.FOMGNPGACKC)
    pub FOMGNPGACKC: ::std::collections::HashMap<u32, super::OLDMDGLKDIM::OLDMDGLKDIM>,
    // @@protoc_insertion_point(field:GILMFJPJAJA.HEHAEIHKGNA)
    pub HEHAEIHKGNA: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:GILMFJPJAJA.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GILMFJPJAJA {
    fn default() -> &'a GILMFJPJAJA {
        <GILMFJPJAJA as ::protobuf::Message>::default_instance()
    }
}

impl GILMFJPJAJA {
    pub fn new() -> GILMFJPJAJA {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor_new::<_, _>(
            "EHJEDEONJKK",
            |m: &GILMFJPJAJA| { &m.EHJEDEONJKK },
            |m: &mut GILMFJPJAJA| { &mut m.EHJEDEONJKK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "IPHNKFJCKGP",
            |m: &GILMFJPJAJA| { &m.IPHNKFJCKGP },
            |m: &mut GILMFJPJAJA| { &mut m.IPHNKFJCKGP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::CDGJKFMFKDL::CDGJKFMFKDL>(
            "JGCPAGCPHCJ",
            |m: &GILMFJPJAJA| { &m.JGCPAGCPHCJ },
            |m: &mut GILMFJPJAJA| { &mut m.JGCPAGCPHCJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::CDGJKFMFKDL::CDGJKFMFKDL>(
            "EGDMGHLDFKD",
            |m: &GILMFJPJAJA| { &m.EGDMGHLDFKD },
            |m: &mut GILMFJPJAJA| { &mut m.EGDMGHLDFKD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NCBDNPGPEAI",
            |m: &GILMFJPJAJA| { &m.NCBDNPGPEAI },
            |m: &mut GILMFJPJAJA| { &mut m.NCBDNPGPEAI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor_new::<_, _>(
            "FOMGNPGACKC",
            |m: &GILMFJPJAJA| { &m.FOMGNPGACKC },
            |m: &mut GILMFJPJAJA| { &mut m.FOMGNPGACKC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HEHAEIHKGNA",
            |m: &GILMFJPJAJA| { &m.HEHAEIHKGNA },
            |m: &mut GILMFJPJAJA| { &mut m.HEHAEIHKGNA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GILMFJPJAJA>(
            "GILMFJPJAJA",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GILMFJPJAJA {
    const NAME: &'static str = "GILMFJPJAJA";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                26 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            18 => value = is.read_message()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.EHJEDEONJKK.insert(key, value);
                },
                34 => {
                    is.read_repeated_packed_uint32_into(&mut self.IPHNKFJCKGP)?;
                },
                32 => {
                    self.IPHNKFJCKGP.push(is.read_uint32()?);
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.JGCPAGCPHCJ)?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.EGDMGHLDFKD)?;
                },
                48 => {
                    self.NCBDNPGPEAI = is.read_bool()?;
                },
                18 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            18 => value = is.read_message()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.FOMGNPGACKC.insert(key, value);
                },
                122 => {
                    is.read_repeated_packed_uint32_into(&mut self.HEHAEIHKGNA)?;
                },
                120 => {
                    self.HEHAEIHKGNA.push(is.read_uint32()?);
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
        for (k, v) in &self.EHJEDEONJKK {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.compute_size();
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        my_size += ::protobuf::rt::vec_packed_uint32_size(4, &self.IPHNKFJCKGP);
        if let Some(v) = self.JGCPAGCPHCJ.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.EGDMGHLDFKD.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.NCBDNPGPEAI != false {
            my_size += 1 + 1;
        }
        for (k, v) in &self.FOMGNPGACKC {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.compute_size();
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        my_size += ::protobuf::rt::vec_packed_uint32_size(15, &self.HEHAEIHKGNA);
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for (k, v) in &self.EHJEDEONJKK {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.cached_size() as u64;
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            os.write_raw_varint32(26)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        os.write_repeated_packed_uint32(4, &self.IPHNKFJCKGP)?;
        if let Some(v) = self.JGCPAGCPHCJ.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if let Some(v) = self.EGDMGHLDFKD.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        if self.NCBDNPGPEAI != false {
            os.write_bool(6, self.NCBDNPGPEAI)?;
        }
        for (k, v) in &self.FOMGNPGACKC {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.cached_size() as u64;
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            os.write_raw_varint32(18)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        os.write_repeated_packed_uint32(15, &self.HEHAEIHKGNA)?;
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> GILMFJPJAJA {
        GILMFJPJAJA::new()
    }

    fn clear(&mut self) {
        self.EHJEDEONJKK.clear();
        self.IPHNKFJCKGP.clear();
        self.JGCPAGCPHCJ.clear();
        self.EGDMGHLDFKD.clear();
        self.NCBDNPGPEAI = false;
        self.FOMGNPGACKC.clear();
        self.HEHAEIHKGNA.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GILMFJPJAJA {
        static instance: ::protobuf::rt::Lazy<GILMFJPJAJA> = ::protobuf::rt::Lazy::new();
        instance.get(GILMFJPJAJA::new)
    }
}

impl ::protobuf::MessageFull for GILMFJPJAJA {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GILMFJPJAJA").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GILMFJPJAJA {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GILMFJPJAJA {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11GILMFJPJAJA.proto\x1a\x11AAFKIMACHFG.proto\x1a\x11CDGJKFMFKDL.prot\
    o\x1a\x11OLDMDGLKDIM.proto\"\xf1\x03\n\x0bGILMFJPJAJA\x12?\n\x0bEHJEDEON\
    JKK\x18\x03\x20\x03(\x0b2\x1d.GILMFJPJAJA.EHJEDEONJKKEntryR\x0bEHJEDEONJ\
    KK\x12\x20\n\x0bIPHNKFJCKGP\x18\x04\x20\x03(\rR\x0bIPHNKFJCKGP\x12.\n\
    \x0bJGCPAGCPHCJ\x18\r\x20\x01(\x0b2\x0c.CDGJKFMFKDLR\x0bJGCPAGCPHCJ\x12.\
    \n\x0bEGDMGHLDFKD\x18\x0e\x20\x01(\x0b2\x0c.CDGJKFMFKDLR\x0bEGDMGHLDFKD\
    \x12\x20\n\x0bNCBDNPGPEAI\x18\x06\x20\x01(\x08R\x0bNCBDNPGPEAI\x12?\n\
    \x0bFOMGNPGACKC\x18\x02\x20\x03(\x0b2\x1d.GILMFJPJAJA.FOMGNPGACKCEntryR\
    \x0bFOMGNPGACKC\x12\x20\n\x0bHEHAEIHKGNA\x18\x0f\x20\x03(\rR\x0bHEHAEIHK\
    GNA\x1aL\n\x10EHJEDEONJKKEntry\x12\x10\n\x03key\x18\x01\x20\x01(\rR\x03k\
    ey\x12\"\n\x05value\x18\x02\x20\x01(\x0b2\x0c.AAFKIMACHFGR\x05value:\x02\
    8\x01\x1aL\n\x10FOMGNPGACKCEntry\x12\x10\n\x03key\x18\x01\x20\x01(\rR\
    \x03key\x12\"\n\x05value\x18\x02\x20\x01(\x0b2\x0c.OLDMDGLKDIMR\x05value\
    :\x028\x01b\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::AAFKIMACHFG::file_descriptor().clone());
            deps.push(super::CDGJKFMFKDL::file_descriptor().clone());
            deps.push(super::OLDMDGLKDIM::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GILMFJPJAJA::generated_message_descriptor_data());
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
