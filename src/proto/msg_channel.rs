// This file is generated by rust-protobuf 2.25.0. Do not edit
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
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `msg_channel.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_0;

#[derive(PartialEq,Clone,Default,Debug)]
pub struct MsgChannelConnect {
    // message fields
    pub client_id: u64,
    pub client_type: ChannelClientType,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a MsgChannelConnect {
    fn default() -> &'a MsgChannelConnect {
        <MsgChannelConnect as ::protobuf::Message>::default_instance()
    }
}

impl MsgChannelConnect {
    pub fn new() -> MsgChannelConnect {
        ::std::default::Default::default()
    }

    // uint64 client_id = 1;


    pub fn get_client_id(&self) -> u64 {
        self.client_id
    }
    pub fn clear_client_id(&mut self) {
        self.client_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_client_id(&mut self, v: u64) {
        self.client_id = v;
    }

    // .ProtoMsg.ChannelClientType client_type = 2;


    pub fn get_client_type(&self) -> ChannelClientType {
        self.client_type
    }
    pub fn clear_client_type(&mut self) {
        self.client_type = ChannelClientType::LoginServer;
    }

    // Param is passed by value, moved
    pub fn set_client_type(&mut self, v: ChannelClientType) {
        self.client_type = v;
    }
}

impl ::protobuf::Message for MsgChannelConnect {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.client_id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.client_type, 2, &mut self.unknown_fields)?
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.client_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.client_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.client_type != ChannelClientType::LoginServer {
            my_size += ::protobuf::rt::enum_size(2, self.client_type);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.client_id != 0 {
            os.write_uint64(1, self.client_id)?;
        }
        if self.client_type != ChannelClientType::LoginServer {
            os.write_enum(2, ::protobuf::ProtobufEnum::value(&self.client_type))?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> MsgChannelConnect {
        MsgChannelConnect::new()
    }

    fn default_instance() -> &'static MsgChannelConnect {
        static instance: ::protobuf::rt::LazyV2<MsgChannelConnect> = ::protobuf::rt::LazyV2::INIT;
        instance.get(MsgChannelConnect::new)
    }
}

impl ::protobuf::Clear for MsgChannelConnect {
    fn clear(&mut self) {
        self.client_id = 0;
        self.client_type = ChannelClientType::LoginServer;
        self.unknown_fields.clear();
    }
}

impl ::protobuf::reflect::ProtobufValue for MsgChannelConnect {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ChannelClientType {
    LoginServer = 0,
    PlatServer = 1,
    ExploreServer = 2,
    UnDefined = 9,
}

impl ::protobuf::ProtobufEnum for ChannelClientType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ChannelClientType> {
        match value {
            0 => ::std::option::Option::Some(ChannelClientType::LoginServer),
            1 => ::std::option::Option::Some(ChannelClientType::PlatServer),
            2 => ::std::option::Option::Some(ChannelClientType::ExploreServer),
            9 => ::std::option::Option::Some(ChannelClientType::UnDefined),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ChannelClientType] = &[
            ChannelClientType::LoginServer,
            ChannelClientType::PlatServer,
            ChannelClientType::ExploreServer,
            ChannelClientType::UnDefined,
        ];
        values
    }
}

impl ::std::marker::Copy for ChannelClientType {
}

impl ::std::default::Default for ChannelClientType {
    fn default() -> Self {
        ChannelClientType::LoginServer
    }
}

impl ::protobuf::reflect::ProtobufValue for ChannelClientType {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}
