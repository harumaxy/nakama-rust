#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Http {
    #[prost(message, repeated, tag = "1")]
    pub rules: ::prost::alloc::vec::Vec<HttpRule>,

    #[prost(bool, tag = "2")]
    pub fully_decode_reserved_expansion: bool,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpRule {
    #[prost(string, tag = "1")]
    pub selector: ::prost::alloc::string::String,

    #[prost(string, tag = "7")]
    pub body: ::prost::alloc::string::String,

    #[prost(string, tag = "12")]
    pub response_body: ::prost::alloc::string::String,

    #[prost(message, repeated, tag = "11")]
    pub additional_bindings: ::prost::alloc::vec::Vec<HttpRule>,

    #[prost(oneof = "http_rule::Pattern", tags = "2, 3, 4, 5, 6, 8")]
    pub pattern: ::core::option::Option<http_rule::Pattern>,
}

pub mod http_rule {

    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Pattern {
        #[prost(string, tag = "2")]
        Get(::prost::alloc::string::String),

        #[prost(string, tag = "3")]
        Put(::prost::alloc::string::String),

        #[prost(string, tag = "4")]
        Post(::prost::alloc::string::String),

        #[prost(string, tag = "5")]
        Delete(::prost::alloc::string::String),

        #[prost(string, tag = "6")]
        Patch(::prost::alloc::string::String),

        #[prost(message, tag = "8")]
        Custom(super::CustomHttpPattern),
    }
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomHttpPattern {
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,

    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
}
