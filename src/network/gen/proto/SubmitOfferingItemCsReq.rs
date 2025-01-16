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

//! Generated file from `SubmitOfferingItemCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SubmitOfferingItemCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SubmitOfferingItemCsReq {
    // message fields
    // @@protoc_insertion_point(field:SubmitOfferingItemCsReq.CMDGNACLGID)
    pub CMDGNACLGID: u32,
    // @@protoc_insertion_point(field:SubmitOfferingItemCsReq.HNGNMJJIFJG)
    pub HNGNMJJIFJG: u32,
    // special fields
    // @@protoc_insertion_point(special_field:SubmitOfferingItemCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SubmitOfferingItemCsReq {
    fn default() -> &'a SubmitOfferingItemCsReq {
        <SubmitOfferingItemCsReq as ::protobuf::Message>::default_instance()
    }
}

impl SubmitOfferingItemCsReq {
    pub fn new() -> SubmitOfferingItemCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CMDGNACLGID",
            |m: &SubmitOfferingItemCsReq| { &m.CMDGNACLGID },
            |m: &mut SubmitOfferingItemCsReq| { &mut m.CMDGNACLGID },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HNGNMJJIFJG",
            |m: &SubmitOfferingItemCsReq| { &m.HNGNMJJIFJG },
            |m: &mut SubmitOfferingItemCsReq| { &mut m.HNGNMJJIFJG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SubmitOfferingItemCsReq>(
            "SubmitOfferingItemCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SubmitOfferingItemCsReq {
    const NAME: &'static str = "SubmitOfferingItemCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.CMDGNACLGID = is.read_uint32()?;
                },
                16 => {
                    self.HNGNMJJIFJG = is.read_uint32()?;
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
        if self.CMDGNACLGID != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.CMDGNACLGID);
        }
        if self.HNGNMJJIFJG != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.HNGNMJJIFJG);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.CMDGNACLGID != 0 {
            os.write_uint32(8, self.CMDGNACLGID)?;
        }
        if self.HNGNMJJIFJG != 0 {
            os.write_uint32(2, self.HNGNMJJIFJG)?;
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

    fn new() -> SubmitOfferingItemCsReq {
        SubmitOfferingItemCsReq::new()
    }

    fn clear(&mut self) {
        self.CMDGNACLGID = 0;
        self.HNGNMJJIFJG = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SubmitOfferingItemCsReq {
        static instance: SubmitOfferingItemCsReq = SubmitOfferingItemCsReq {
            CMDGNACLGID: 0,
            HNGNMJJIFJG: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SubmitOfferingItemCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SubmitOfferingItemCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SubmitOfferingItemCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SubmitOfferingItemCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dSubmitOfferingItemCsReq.proto\"]\n\x17SubmitOfferingItemCsReq\x12\
    \x20\n\x0bCMDGNACLGID\x18\x08\x20\x01(\rR\x0bCMDGNACLGID\x12\x20\n\x0bHN\
    GNMJJIFJG\x18\x02\x20\x01(\rR\x0bHNGNMJJIFJGb\x06proto3\
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
            messages.push(SubmitOfferingItemCsReq::generated_message_descriptor_data());
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
