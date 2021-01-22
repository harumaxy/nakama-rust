#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Swagger {
    #[prost(string, tag = "1")]
    pub swagger: ::prost::alloc::string::String,

    #[prost(message, optional, tag = "2")]
    pub info: ::core::option::Option<Info>,

    #[prost(string, tag = "3")]
    pub host: ::prost::alloc::string::String,

    #[prost(string, tag = "4")]
    pub base_path: ::prost::alloc::string::String,

    #[prost(enumeration = "Scheme", repeated, tag = "5")]
    pub schemes: ::prost::alloc::vec::Vec<i32>,

    #[prost(string, repeated, tag = "6")]
    pub consumes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,

    #[prost(string, repeated, tag = "7")]
    pub produces: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,

    #[prost(map = "string, message", tag = "10")]
    pub responses: ::std::collections::HashMap<::prost::alloc::string::String, Response>,

    #[prost(message, optional, tag = "11")]
    pub security_definitions: ::core::option::Option<SecurityDefinitions>,

    #[prost(message, repeated, tag = "12")]
    pub security: ::prost::alloc::vec::Vec<SecurityRequirement>,

    #[prost(message, optional, tag = "14")]
    pub external_docs: ::core::option::Option<ExternalDocumentation>,
    #[prost(map = "string, message", tag = "15")]
    pub extensions:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Value>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    #[prost(string, repeated, tag = "1")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,

    #[prost(string, tag = "2")]
    pub summary: ::prost::alloc::string::String,

    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,

    #[prost(message, optional, tag = "4")]
    pub external_docs: ::core::option::Option<ExternalDocumentation>,

    #[prost(string, tag = "5")]
    pub operation_id: ::prost::alloc::string::String,

    #[prost(string, repeated, tag = "6")]
    pub consumes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,

    #[prost(string, repeated, tag = "7")]
    pub produces: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,

    #[prost(map = "string, message", tag = "9")]
    pub responses: ::std::collections::HashMap<::prost::alloc::string::String, Response>,

    #[prost(enumeration = "Scheme", repeated, tag = "10")]
    pub schemes: ::prost::alloc::vec::Vec<i32>,

    #[prost(bool, tag = "11")]
    pub deprecated: bool,

    #[prost(message, repeated, tag = "12")]
    pub security: ::prost::alloc::vec::Vec<SecurityRequirement>,
    #[prost(map = "string, message", tag = "13")]
    pub extensions:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Value>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,

    #[prost(message, optional, tag = "2")]
    pub schema: ::core::option::Option<Schema>,

    #[prost(map = "string, string", tag = "4")]
    pub examples:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(map = "string, message", tag = "5")]
    pub extensions:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Value>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Info {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,

    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,

    #[prost(string, tag = "3")]
    pub terms_of_service: ::prost::alloc::string::String,

    #[prost(message, optional, tag = "4")]
    pub contact: ::core::option::Option<Contact>,

    #[prost(message, optional, tag = "5")]
    pub license: ::core::option::Option<License>,

    #[prost(string, tag = "6")]
    pub version: ::prost::alloc::string::String,
    #[prost(map = "string, message", tag = "7")]
    pub extensions:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Value>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contact {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,

    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,

    #[prost(string, tag = "3")]
    pub email: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct License {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,

    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalDocumentation {
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,

    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schema {
    #[prost(message, optional, tag = "1")]
    pub json_schema: ::core::option::Option<JsonSchema>,

    #[prost(string, tag = "2")]
    pub discriminator: ::prost::alloc::string::String,

    #[prost(bool, tag = "3")]
    pub read_only: bool,

    #[prost(message, optional, tag = "5")]
    pub external_docs: ::core::option::Option<ExternalDocumentation>,

    #[prost(string, tag = "6")]
    pub example: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JsonSchema {
    #[prost(string, tag = "3")]
    pub r#ref: ::prost::alloc::string::String,

    #[prost(string, tag = "5")]
    pub title: ::prost::alloc::string::String,

    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub default: ::prost::alloc::string::String,
    #[prost(bool, tag = "8")]
    pub read_only: bool,
    #[prost(double, tag = "10")]
    pub multiple_of: f64,

    #[prost(double, tag = "11")]
    pub maximum: f64,
    #[prost(bool, tag = "12")]
    pub exclusive_maximum: bool,

    #[prost(double, tag = "13")]
    pub minimum: f64,
    #[prost(bool, tag = "14")]
    pub exclusive_minimum: bool,
    #[prost(uint64, tag = "15")]
    pub max_length: u64,
    #[prost(uint64, tag = "16")]
    pub min_length: u64,
    #[prost(string, tag = "17")]
    pub pattern: ::prost::alloc::string::String,
    #[prost(uint64, tag = "20")]
    pub max_items: u64,
    #[prost(uint64, tag = "21")]
    pub min_items: u64,
    #[prost(bool, tag = "22")]
    pub unique_items: bool,
    #[prost(uint64, tag = "24")]
    pub max_properties: u64,
    #[prost(uint64, tag = "25")]
    pub min_properties: u64,
    #[prost(string, repeated, tag = "26")]
    pub required: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,

    #[prost(string, repeated, tag = "34")]
    pub array: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(
        enumeration = "json_schema::JsonSchemaSimpleTypes",
        repeated,
        tag = "35"
    )]
    pub r#type: ::prost::alloc::vec::Vec<i32>,
}

pub mod json_schema {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum JsonSchemaSimpleTypes {
        Unknown = 0,
        Array = 1,
        Boolean = 2,
        Integer = 3,
        Null = 4,
        Number = 5,
        Object = 6,
        String = 7,
    }
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tag {
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,

    #[prost(message, optional, tag = "3")]
    pub external_docs: ::core::option::Option<ExternalDocumentation>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityDefinitions {
    #[prost(map = "string, message", tag = "1")]
    pub security: ::std::collections::HashMap<::prost::alloc::string::String, SecurityScheme>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityScheme {
    #[prost(enumeration = "security_scheme::Type", tag = "1")]
    pub r#type: i32,

    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,

    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,

    #[prost(enumeration = "security_scheme::In", tag = "4")]
    pub r#in: i32,

    #[prost(enumeration = "security_scheme::Flow", tag = "5")]
    pub flow: i32,

    #[prost(string, tag = "6")]
    pub authorization_url: ::prost::alloc::string::String,

    #[prost(string, tag = "7")]
    pub token_url: ::prost::alloc::string::String,

    #[prost(message, optional, tag = "8")]
    pub scopes: ::core::option::Option<Scopes>,
    #[prost(map = "string, message", tag = "9")]
    pub extensions:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Value>,
}

pub mod security_scheme {

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        Invalid = 0,
        Basic = 1,
        ApiKey = 2,
        Oauth2 = 3,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum In {
        Invalid = 0,
        Query = 1,
        Header = 2,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Flow {
        Invalid = 0,
        Implicit = 1,
        Password = 2,
        Application = 3,
        AccessCode = 4,
    }
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityRequirement {
    #[prost(map = "string, message", tag = "1")]
    pub security_requirement: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        security_requirement::SecurityRequirementValue,
    >,
}

pub mod security_requirement {

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SecurityRequirementValue {
        #[prost(string, repeated, tag = "1")]
        pub scope: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Scopes {
    #[prost(map = "string, string", tag = "1")]
    pub scope:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Scheme {
    Unknown = 0,
    Http = 1,
    Https = 2,
    Ws = 3,
    Wss = 4,
}
