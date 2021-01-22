#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Envelope {
    #[prost(string, tag = "1")]
    pub cid: ::prost::alloc::string::String,
    #[prost(
        oneof = "envelope::Message",
        tags = "2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50"
    )]
    pub message: ::core::option::Option<envelope::Message>,
}

pub mod envelope {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag = "2")]
        Channel(super::Channel),

        #[prost(message, tag = "3")]
        ChannelJoin(super::ChannelJoin),

        #[prost(message, tag = "4")]
        ChannelLeave(super::ChannelLeave),

        #[prost(message, tag = "5")]
        ChannelMessage(super::super::api::ChannelMessage),

        #[prost(message, tag = "6")]
        ChannelMessageAck(super::ChannelMessageAck),

        #[prost(message, tag = "7")]
        ChannelMessageSend(super::ChannelMessageSend),

        #[prost(message, tag = "8")]
        ChannelMessageUpdate(super::ChannelMessageUpdate),

        #[prost(message, tag = "9")]
        ChannelMessageRemove(super::ChannelMessageRemove),

        #[prost(message, tag = "10")]
        ChannelPresenceEvent(super::ChannelPresenceEvent),

        #[prost(message, tag = "11")]
        Error(super::Error),

        #[prost(message, tag = "12")]
        Match(super::Match),

        #[prost(message, tag = "13")]
        MatchCreate(super::MatchCreate),

        #[prost(message, tag = "14")]
        MatchData(super::MatchData),

        #[prost(message, tag = "15")]
        MatchDataSend(super::MatchDataSend),

        #[prost(message, tag = "16")]
        MatchJoin(super::MatchJoin),

        #[prost(message, tag = "17")]
        MatchLeave(super::MatchLeave),

        #[prost(message, tag = "18")]
        MatchPresenceEvent(super::MatchPresenceEvent),

        #[prost(message, tag = "19")]
        MatchmakerAdd(super::MatchmakerAdd),

        #[prost(message, tag = "20")]
        MatchmakerMatched(super::MatchmakerMatched),

        #[prost(message, tag = "21")]
        MatchmakerRemove(super::MatchmakerRemove),

        #[prost(message, tag = "22")]
        MatchmakerTicket(super::MatchmakerTicket),

        #[prost(message, tag = "23")]
        Notifications(super::Notifications),

        #[prost(message, tag = "24")]
        Rpc(super::super::api::Rpc),

        #[prost(message, tag = "25")]
        Status(super::Status),

        #[prost(message, tag = "26")]
        StatusFollow(super::StatusFollow),

        #[prost(message, tag = "27")]
        StatusPresenceEvent(super::StatusPresenceEvent),

        #[prost(message, tag = "28")]
        StatusUnfollow(super::StatusUnfollow),

        #[prost(message, tag = "29")]
        StatusUpdate(super::StatusUpdate),

        #[prost(message, tag = "30")]
        StreamData(super::StreamData),

        #[prost(message, tag = "31")]
        StreamPresenceEvent(super::StreamPresenceEvent),

        #[prost(message, tag = "32")]
        Ping(super::Ping),

        #[prost(message, tag = "33")]
        Pong(super::Pong),

        #[prost(message, tag = "34")]
        Party(super::Party),

        #[prost(message, tag = "35")]
        PartyCreate(super::PartyCreate),

        #[prost(message, tag = "36")]
        PartyJoin(super::PartyJoin),

        #[prost(message, tag = "37")]
        PartyLeave(super::PartyLeave),

        #[prost(message, tag = "38")]
        PartyPromote(super::PartyPromote),

        #[prost(message, tag = "39")]
        PartyLeader(super::PartyLeader),

        #[prost(message, tag = "40")]
        PartyAccept(super::PartyAccept),

        #[prost(message, tag = "41")]
        PartyRemove(super::PartyRemove),

        #[prost(message, tag = "42")]
        PartyClose(super::PartyClose),

        #[prost(message, tag = "43")]
        PartyJoinRequestList(super::PartyJoinRequestList),

        #[prost(message, tag = "44")]
        PartyJoinRequest(super::PartyJoinRequest),

        #[prost(message, tag = "45")]
        PartyMatchmakerAdd(super::PartyMatchmakerAdd),

        #[prost(message, tag = "46")]
        PartyMatchmakerRemove(super::PartyMatchmakerRemove),

        #[prost(message, tag = "47")]
        PartyMatchmakerTicket(super::PartyMatchmakerTicket),

        #[prost(message, tag = "48")]
        PartyData(super::PartyData),

        #[prost(message, tag = "49")]
        PartyDataSend(super::PartyDataSend),

        #[prost(message, tag = "50")]
        PartyPresenceEvent(super::PartyPresenceEvent),
    }
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Channel {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,

    #[prost(message, repeated, tag = "2")]
    pub presences: ::prost::alloc::vec::Vec<UserPresence>,

    #[prost(message, optional, tag = "3")]
    pub self_: ::core::option::Option<UserPresence>,

    #[prost(string, tag = "4")]
    pub room_name: ::prost::alloc::string::String,

    #[prost(string, tag = "5")]
    pub group_id: ::prost::alloc::string::String,

    #[prost(string, tag = "6")]
    pub user_id_one: ::prost::alloc::string::String,

    #[prost(string, tag = "7")]
    pub user_id_two: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelJoin {
    #[prost(string, tag = "1")]
    pub target: ::prost::alloc::string::String,

    #[prost(int32, tag = "2")]
    pub r#type: i32,

    #[prost(message, optional, tag = "3")]
    pub persistence: ::core::option::Option<bool>,

    #[prost(message, optional, tag = "4")]
    pub hidden: ::core::option::Option<bool>,
}

pub mod channel_join {

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        Unspecified = 0,

        Room = 1,

        DirectMessage = 2,

        Group = 3,
    }
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelLeave {
    #[prost(string, tag = "1")]
    pub channel_id: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelMessageAck {
    #[prost(string, tag = "1")]
    pub channel_id: ::prost::alloc::string::String,

    #[prost(string, tag = "2")]
    pub message_id: ::prost::alloc::string::String,

    #[prost(message, optional, tag = "3")]
    pub code: ::core::option::Option<i32>,

    #[prost(string, tag = "4")]
    pub username: ::prost::alloc::string::String,

    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,

    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,

    #[prost(message, optional, tag = "7")]
    pub persistent: ::core::option::Option<bool>,

    #[prost(string, tag = "8")]
    pub room_name: ::prost::alloc::string::String,

    #[prost(string, tag = "9")]
    pub group_id: ::prost::alloc::string::String,

    #[prost(string, tag = "10")]
    pub user_id_one: ::prost::alloc::string::String,

    #[prost(string, tag = "11")]
    pub user_id_two: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelMessageSend {
    #[prost(string, tag = "1")]
    pub channel_id: ::prost::alloc::string::String,

    #[prost(string, tag = "2")]
    pub content: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelMessageUpdate {
    #[prost(string, tag = "1")]
    pub channel_id: ::prost::alloc::string::String,

    #[prost(string, tag = "2")]
    pub message_id: ::prost::alloc::string::String,

    #[prost(string, tag = "3")]
    pub content: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelMessageRemove {
    #[prost(string, tag = "1")]
    pub channel_id: ::prost::alloc::string::String,

    #[prost(string, tag = "2")]
    pub message_id: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelPresenceEvent {
    #[prost(string, tag = "1")]
    pub channel_id: ::prost::alloc::string::String,

    #[prost(message, repeated, tag = "2")]
    pub joins: ::prost::alloc::vec::Vec<UserPresence>,

    #[prost(message, repeated, tag = "3")]
    pub leaves: ::prost::alloc::vec::Vec<UserPresence>,

    #[prost(string, tag = "4")]
    pub room_name: ::prost::alloc::string::String,

    #[prost(string, tag = "5")]
    pub group_id: ::prost::alloc::string::String,

    #[prost(string, tag = "6")]
    pub user_id_one: ::prost::alloc::string::String,

    #[prost(string, tag = "7")]
    pub user_id_two: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    #[prost(int32, tag = "1")]
    pub code: i32,

    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,

    #[prost(map = "string, string", tag = "3")]
    pub context:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}

pub mod error {

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Code {
        RuntimeException = 0,

        UnrecognizedPayload = 1,

        MissingPayload = 2,

        BadInput = 3,

        MatchNotFound = 4,

        MatchJoinRejected = 5,

        RuntimeFunctionNotFound = 6,

        RuntimeFunctionException = 7,
    }
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Match {
    #[prost(string, tag = "1")]
    pub match_id: ::prost::alloc::string::String,

    #[prost(bool, tag = "2")]
    pub authoritative: bool,

    #[prost(message, optional, tag = "3")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,

    #[prost(int32, tag = "4")]
    pub size: i32,

    #[prost(message, repeated, tag = "5")]
    pub presences: ::prost::alloc::vec::Vec<UserPresence>,

    #[prost(message, optional, tag = "6")]
    pub self_: ::core::option::Option<UserPresence>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchCreate {}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchData {
    #[prost(string, tag = "1")]
    pub match_id: ::prost::alloc::string::String,

    #[prost(message, optional, tag = "2")]
    pub presence: ::core::option::Option<UserPresence>,

    #[prost(int64, tag = "3")]
    pub op_code: i64,

    #[prost(bytes = "vec", tag = "4")]
    pub data: ::prost::alloc::vec::Vec<u8>,

    #[prost(bool, tag = "5")]
    pub reliable: bool,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchDataSend {
    #[prost(string, tag = "1")]
    pub match_id: ::prost::alloc::string::String,

    #[prost(int64, tag = "2")]
    pub op_code: i64,

    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,

    #[prost(message, repeated, tag = "4")]
    pub presences: ::prost::alloc::vec::Vec<UserPresence>,

    #[prost(bool, tag = "5")]
    pub reliable: bool,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchJoin {
    #[prost(map = "string, string", tag = "3")]
    pub metadata:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(oneof = "match_join::Id", tags = "1, 2")]
    pub id: ::core::option::Option<match_join::Id>,
}

pub mod match_join {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Id {
        #[prost(string, tag = "1")]
        MatchId(::prost::alloc::string::String),

        #[prost(string, tag = "2")]
        Token(::prost::alloc::string::String),
    }
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchLeave {
    #[prost(string, tag = "1")]
    pub match_id: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchPresenceEvent {
    #[prost(string, tag = "1")]
    pub match_id: ::prost::alloc::string::String,

    #[prost(message, repeated, tag = "2")]
    pub joins: ::prost::alloc::vec::Vec<UserPresence>,

    #[prost(message, repeated, tag = "3")]
    pub leaves: ::prost::alloc::vec::Vec<UserPresence>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchmakerAdd {
    #[prost(int32, tag = "1")]
    pub min_count: i32,

    #[prost(int32, tag = "2")]
    pub max_count: i32,

    #[prost(string, tag = "3")]
    pub query: ::prost::alloc::string::String,

    #[prost(map = "string, string", tag = "4")]
    pub string_properties:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,

    #[prost(map = "string, double", tag = "5")]
    pub numeric_properties: ::std::collections::HashMap<::prost::alloc::string::String, f64>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchmakerMatched {
    #[prost(string, tag = "1")]
    pub ticket: ::prost::alloc::string::String,

    #[prost(message, repeated, tag = "4")]
    pub users: ::prost::alloc::vec::Vec<matchmaker_matched::MatchmakerUser>,

    #[prost(message, optional, tag = "5")]
    pub self_: ::core::option::Option<matchmaker_matched::MatchmakerUser>,

    #[prost(oneof = "matchmaker_matched::Id", tags = "2, 3")]
    pub id: ::core::option::Option<matchmaker_matched::Id>,
}

pub mod matchmaker_matched {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MatchmakerUser {
        #[prost(message, optional, tag = "1")]
        pub presence: ::core::option::Option<super::UserPresence>,

        #[prost(string, tag = "2")]
        pub party_id: ::prost::alloc::string::String,

        #[prost(map = "string, string", tag = "5")]
        pub string_properties: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,

        #[prost(map = "string, double", tag = "6")]
        pub numeric_properties: ::std::collections::HashMap<::prost::alloc::string::String, f64>,
    }

    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Id {
        #[prost(string, tag = "2")]
        MatchId(::prost::alloc::string::String),

        #[prost(string, tag = "3")]
        Token(::prost::alloc::string::String),
    }
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchmakerRemove {
    #[prost(string, tag = "1")]
    pub ticket: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchmakerTicket {
    #[prost(string, tag = "1")]
    pub ticket: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Notifications {
    #[prost(message, repeated, tag = "1")]
    pub notifications: ::prost::alloc::vec::Vec<super::api::Notification>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Party {
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,

    #[prost(bool, tag = "2")]
    pub open: bool,

    #[prost(int32, tag = "3")]
    pub max_size: i32,

    #[prost(message, optional, tag = "4")]
    pub self_: ::core::option::Option<UserPresence>,

    #[prost(message, optional, tag = "5")]
    pub leader: ::core::option::Option<UserPresence>,

    #[prost(message, repeated, tag = "6")]
    pub presences: ::prost::alloc::vec::Vec<UserPresence>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyCreate {
    #[prost(bool, tag = "1")]
    pub open: bool,

    #[prost(int32, tag = "2")]
    pub max_size: i32,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyJoin {
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyLeave {
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyPromote {
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,

    #[prost(message, optional, tag = "2")]
    pub presence: ::core::option::Option<UserPresence>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyLeader {
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,

    #[prost(message, optional, tag = "2")]
    pub presence: ::core::option::Option<UserPresence>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyAccept {
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,

    #[prost(message, optional, tag = "2")]
    pub presence: ::core::option::Option<UserPresence>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyRemove {
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,

    #[prost(message, optional, tag = "2")]
    pub presence: ::core::option::Option<UserPresence>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyClose {
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyJoinRequestList {
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyJoinRequest {
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,

    #[prost(message, repeated, tag = "2")]
    pub presences: ::prost::alloc::vec::Vec<UserPresence>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyMatchmakerAdd {
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,

    #[prost(int32, tag = "2")]
    pub min_count: i32,

    #[prost(int32, tag = "3")]
    pub max_count: i32,

    #[prost(string, tag = "4")]
    pub query: ::prost::alloc::string::String,

    #[prost(map = "string, string", tag = "5")]
    pub string_properties:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,

    #[prost(map = "string, double", tag = "6")]
    pub numeric_properties: ::std::collections::HashMap<::prost::alloc::string::String, f64>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyMatchmakerRemove {
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,

    #[prost(string, tag = "2")]
    pub ticket: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyMatchmakerTicket {
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,

    #[prost(string, tag = "2")]
    pub ticket: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyData {
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,

    #[prost(message, optional, tag = "2")]
    pub presence: ::core::option::Option<UserPresence>,

    #[prost(int64, tag = "3")]
    pub op_code: i64,

    #[prost(bytes = "vec", tag = "4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyDataSend {
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,

    #[prost(int64, tag = "2")]
    pub op_code: i64,

    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyPresenceEvent {
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,

    #[prost(message, repeated, tag = "2")]
    pub joins: ::prost::alloc::vec::Vec<UserPresence>,

    #[prost(message, repeated, tag = "3")]
    pub leaves: ::prost::alloc::vec::Vec<UserPresence>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ping {}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pong {}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    #[prost(message, repeated, tag = "1")]
    pub presences: ::prost::alloc::vec::Vec<UserPresence>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusFollow {
    #[prost(string, repeated, tag = "1")]
    pub user_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,

    #[prost(string, repeated, tag = "2")]
    pub usernames: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusPresenceEvent {
    #[prost(message, repeated, tag = "2")]
    pub joins: ::prost::alloc::vec::Vec<UserPresence>,

    #[prost(message, repeated, tag = "3")]
    pub leaves: ::prost::alloc::vec::Vec<UserPresence>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusUnfollow {
    #[prost(string, repeated, tag = "1")]
    pub user_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusUpdate {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stream {
    #[prost(int32, tag = "1")]
    pub mode: i32,

    #[prost(string, tag = "2")]
    pub subject: ::prost::alloc::string::String,

    #[prost(string, tag = "3")]
    pub subcontext: ::prost::alloc::string::String,

    #[prost(string, tag = "4")]
    pub label: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamData {
    #[prost(message, optional, tag = "1")]
    pub stream: ::core::option::Option<Stream>,

    #[prost(message, optional, tag = "2")]
    pub sender: ::core::option::Option<UserPresence>,

    #[prost(string, tag = "3")]
    pub data: ::prost::alloc::string::String,

    #[prost(bool, tag = "4")]
    pub reliable: bool,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamPresenceEvent {
    #[prost(message, optional, tag = "1")]
    pub stream: ::core::option::Option<Stream>,

    #[prost(message, repeated, tag = "2")]
    pub joins: ::prost::alloc::vec::Vec<UserPresence>,

    #[prost(message, repeated, tag = "3")]
    pub leaves: ::prost::alloc::vec::Vec<UserPresence>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserPresence {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,

    #[prost(string, tag = "2")]
    pub session_id: ::prost::alloc::string::String,

    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,

    #[prost(bool, tag = "4")]
    pub persistence: bool,

    #[prost(message, optional, tag = "5")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
}
