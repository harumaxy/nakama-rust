/// An envelope for a realtime message.
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
/// Nested message and enum types in `Envelope`.
pub mod envelope {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// A response from a channel join operation.
        #[prost(message, tag = "2")]
        Channel(super::Channel),
        /// Join a realtime chat channel.
        #[prost(message, tag = "3")]
        ChannelJoin(super::ChannelJoin),
        /// Leave a realtime chat channel.
        #[prost(message, tag = "4")]
        ChannelLeave(super::ChannelLeave),
        /// An incoming message on a realtime chat channel.
        #[prost(message, tag = "5")]
        ChannelMessage(super::super::api::ChannelMessage),
        /// An acknowledgement received in response to sending a message on a chat channel.
        #[prost(message, tag = "6")]
        ChannelMessageAck(super::ChannelMessageAck),
        /// Send a message to a realtime chat channel.
        #[prost(message, tag = "7")]
        ChannelMessageSend(super::ChannelMessageSend),
        /// Update a message previously sent to a realtime chat channel.
        #[prost(message, tag = "8")]
        ChannelMessageUpdate(super::ChannelMessageUpdate),
        /// Remove a message previously sent to a realtime chat channel.
        #[prost(message, tag = "9")]
        ChannelMessageRemove(super::ChannelMessageRemove),
        /// Presence update for a particular realtime chat channel.
        #[prost(message, tag = "10")]
        ChannelPresenceEvent(super::ChannelPresenceEvent),
        /// Describes an error which occurred on the server.
        #[prost(message, tag = "11")]
        Error(super::Error),
        /// Incoming information about a realtime match.
        #[prost(message, tag = "12")]
        Match(super::Match),
        /// A client to server request to create a realtime match.
        #[prost(message, tag = "13")]
        MatchCreate(super::MatchCreate),
        /// Incoming realtime match data delivered from the server.
        #[prost(message, tag = "14")]
        MatchData(super::MatchData),
        /// A client to server request to send data to a realtime match.
        #[prost(message, tag = "15")]
        MatchDataSend(super::MatchDataSend),
        /// A client to server request to join a realtime match.
        #[prost(message, tag = "16")]
        MatchJoin(super::MatchJoin),
        /// A client to server request to leave a realtime match.
        #[prost(message, tag = "17")]
        MatchLeave(super::MatchLeave),
        /// Presence update for a particular realtime match.
        #[prost(message, tag = "18")]
        MatchPresenceEvent(super::MatchPresenceEvent),
        /// Submit a new matchmaking process request.
        #[prost(message, tag = "19")]
        MatchmakerAdd(super::MatchmakerAdd),
        /// A successful matchmaking result.
        #[prost(message, tag = "20")]
        MatchmakerMatched(super::MatchmakerMatched),
        /// Cancel a matchmaking process using a ticket.
        #[prost(message, tag = "21")]
        MatchmakerRemove(super::MatchmakerRemove),
        /// A response from starting a new matchmaking process.
        #[prost(message, tag = "22")]
        MatchmakerTicket(super::MatchmakerTicket),
        /// Notifications send by the server.
        #[prost(message, tag = "23")]
        Notifications(super::Notifications),
        /// RPC call or response.
        #[prost(message, tag = "24")]
        Rpc(super::super::api::Rpc),
        /// An incoming status snapshot for some set of users.
        #[prost(message, tag = "25")]
        Status(super::Status),
        /// Start following some set of users to receive their status updates.
        #[prost(message, tag = "26")]
        StatusFollow(super::StatusFollow),
        /// An incoming status update.
        #[prost(message, tag = "27")]
        StatusPresenceEvent(super::StatusPresenceEvent),
        /// Stop following some set of users to no longer receive their status updates.
        #[prost(message, tag = "28")]
        StatusUnfollow(super::StatusUnfollow),
        /// Set the user's own status.
        #[prost(message, tag = "29")]
        StatusUpdate(super::StatusUpdate),
        /// A data message delivered over a stream.
        #[prost(message, tag = "30")]
        StreamData(super::StreamData),
        /// Presence update for a particular stream.
        #[prost(message, tag = "31")]
        StreamPresenceEvent(super::StreamPresenceEvent),
        /// Application-level heartbeat and connection check.
        #[prost(message, tag = "32")]
        Ping(super::Ping),
        /// Application-level heartbeat and connection check response.
        #[prost(message, tag = "33")]
        Pong(super::Pong),
        /// Incoming information about a party.
        #[prost(message, tag = "34")]
        Party(super::Party),
        /// Create a party.
        #[prost(message, tag = "35")]
        PartyCreate(super::PartyCreate),
        /// Join a party, or request to join if the party is not open.
        #[prost(message, tag = "36")]
        PartyJoin(super::PartyJoin),
        /// Leave a party.
        #[prost(message, tag = "37")]
        PartyLeave(super::PartyLeave),
        /// Promote a new party leader.
        #[prost(message, tag = "38")]
        PartyPromote(super::PartyPromote),
        /// Announcement of a new party leader.
        #[prost(message, tag = "39")]
        PartyLeader(super::PartyLeader),
        /// Accept a request to join.
        #[prost(message, tag = "40")]
        PartyAccept(super::PartyAccept),
        /// Kick a party member, or decline a request to join.
        #[prost(message, tag = "41")]
        PartyRemove(super::PartyRemove),
        /// End a party, kicking all party members and closing it.
        #[prost(message, tag = "42")]
        PartyClose(super::PartyClose),
        /// Request a list of pending join requests for a party.
        #[prost(message, tag = "43")]
        PartyJoinRequestList(super::PartyJoinRequestList),
        /// Incoming notification for one or more new presences attempting to join the party.
        #[prost(message, tag = "44")]
        PartyJoinRequest(super::PartyJoinRequest),
        /// Begin matchmaking as a party.
        #[prost(message, tag = "45")]
        PartyMatchmakerAdd(super::PartyMatchmakerAdd),
        /// Cancel a party matchmaking process using a ticket.
        #[prost(message, tag = "46")]
        PartyMatchmakerRemove(super::PartyMatchmakerRemove),
        /// A response from starting a new party matchmaking process.
        #[prost(message, tag = "47")]
        PartyMatchmakerTicket(super::PartyMatchmakerTicket),
        /// Incoming party data delivered from the server.
        #[prost(message, tag = "48")]
        PartyData(super::PartyData),
        /// A client to server request to send data to a party.
        #[prost(message, tag = "49")]
        PartyDataSend(super::PartyDataSend),
        /// Presence update for a particular party.
        #[prost(message, tag = "50")]
        PartyPresenceEvent(super::PartyPresenceEvent),
    }
}
/// A realtime chat channel.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Channel {
    /// The ID of the channel.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The users currently in the channel.
    #[prost(message, repeated, tag = "2")]
    pub presences: ::prost::alloc::vec::Vec<UserPresence>,
    /// A reference to the current user's presence in the channel.
    #[prost(message, optional, tag = "3")]
    pub self_: ::core::option::Option<UserPresence>,
    /// The name of the chat room, or an empty string if this message was not sent through a chat room.
    #[prost(string, tag = "4")]
    pub room_name: ::prost::alloc::string::String,
    /// The ID of the group, or an empty string if this message was not sent through a group channel.
    #[prost(string, tag = "5")]
    pub group_id: ::prost::alloc::string::String,
    /// The ID of the first DM user, or an empty string if this message was not sent through a DM chat.
    #[prost(string, tag = "6")]
    pub user_id_one: ::prost::alloc::string::String,
    /// The ID of the second DM user, or an empty string if this message was not sent through a DM chat.
    #[prost(string, tag = "7")]
    pub user_id_two: ::prost::alloc::string::String,
}
/// Join operation for a realtime chat channel.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelJoin {
    /// The user ID to DM with, group ID to chat with, or room channel name to join.
    #[prost(string, tag = "1")]
    pub target: ::prost::alloc::string::String,
    /// The type of the chat channel.
    ///
    /// one of "ChannelId.Type".
    #[prost(int32, tag = "2")]
    pub r#type: i32,
    /// Whether messages sent on this channel should be persistent.
    #[prost(message, optional, tag = "3")]
    pub persistence: ::core::option::Option<bool>,
    /// Whether the user should appear in the channel's presence list and events.
    #[prost(message, optional, tag = "4")]
    pub hidden: ::core::option::Option<bool>,
}
/// Nested message and enum types in `ChannelJoin`.
pub mod channel_join {
    /// The type of chat channel.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Default case. Assumed as ROOM type.
        Unspecified = 0,
        /// A room which anyone can join to chat.
        Room = 1,
        /// A private channel for 1-on-1 chat.
        DirectMessage = 2,
        /// A channel for group chat.
        Group = 3,
    }
}
/// Leave a realtime channel.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelLeave {
    /// The ID of the channel to leave.
    #[prost(string, tag = "1")]
    pub channel_id: ::prost::alloc::string::String,
}
/// A receipt reply from a channel message send operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelMessageAck {
    /// The channel the message was sent to.
    #[prost(string, tag = "1")]
    pub channel_id: ::prost::alloc::string::String,
    /// The unique ID assigned to the message.
    #[prost(string, tag = "2")]
    pub message_id: ::prost::alloc::string::String,
    /// The code representing a message type or category.
    #[prost(message, optional, tag = "3")]
    pub code: ::core::option::Option<i32>,
    /// Username of the message sender.
    #[prost(string, tag = "4")]
    pub username: ::prost::alloc::string::String,
    /// The UNIX time when the message was created.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The UNIX time when the message was last updated.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// True if the message was persisted to the channel's history, false otherwise.
    #[prost(message, optional, tag = "7")]
    pub persistent: ::core::option::Option<bool>,
    /// The name of the chat room, or an empty string if this message was not sent through a chat room.
    #[prost(string, tag = "8")]
    pub room_name: ::prost::alloc::string::String,
    /// The ID of the group, or an empty string if this message was not sent through a group channel.
    #[prost(string, tag = "9")]
    pub group_id: ::prost::alloc::string::String,
    /// The ID of the first DM user, or an empty string if this message was not sent through a DM chat.
    #[prost(string, tag = "10")]
    pub user_id_one: ::prost::alloc::string::String,
    /// The ID of the second DM user, or an empty string if this message was not sent through a DM chat.
    #[prost(string, tag = "11")]
    pub user_id_two: ::prost::alloc::string::String,
}
/// Send a message to a realtime channel.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelMessageSend {
    /// The channel to sent to.
    #[prost(string, tag = "1")]
    pub channel_id: ::prost::alloc::string::String,
    /// Message content.
    #[prost(string, tag = "2")]
    pub content: ::prost::alloc::string::String,
}
/// Update a message previously sent to a realtime channel.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelMessageUpdate {
    /// The channel the message was sent to.
    #[prost(string, tag = "1")]
    pub channel_id: ::prost::alloc::string::String,
    /// The ID assigned to the message to update.
    #[prost(string, tag = "2")]
    pub message_id: ::prost::alloc::string::String,
    /// New message content.
    #[prost(string, tag = "3")]
    pub content: ::prost::alloc::string::String,
}
/// Remove a message previously sent to a realtime channel.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelMessageRemove {
    /// The channel the message was sent to.
    #[prost(string, tag = "1")]
    pub channel_id: ::prost::alloc::string::String,
    /// The ID assigned to the message to update.
    #[prost(string, tag = "2")]
    pub message_id: ::prost::alloc::string::String,
}
/// A set of joins and leaves on a particular channel.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelPresenceEvent {
    /// The channel identifier this event is for.
    #[prost(string, tag = "1")]
    pub channel_id: ::prost::alloc::string::String,
    /// Presences joining the channel as part of this event, if any.
    #[prost(message, repeated, tag = "2")]
    pub joins: ::prost::alloc::vec::Vec<UserPresence>,
    /// Presences leaving the channel as part of this event, if any.
    #[prost(message, repeated, tag = "3")]
    pub leaves: ::prost::alloc::vec::Vec<UserPresence>,
    /// The name of the chat room, or an empty string if this message was not sent through a chat room.
    #[prost(string, tag = "4")]
    pub room_name: ::prost::alloc::string::String,
    /// The ID of the group, or an empty string if this message was not sent through a group channel.
    #[prost(string, tag = "5")]
    pub group_id: ::prost::alloc::string::String,
    /// The ID of the first DM user, or an empty string if this message was not sent through a DM chat.
    #[prost(string, tag = "6")]
    pub user_id_one: ::prost::alloc::string::String,
    /// The ID of the second DM user, or an empty string if this message was not sent through a DM chat.
    #[prost(string, tag = "7")]
    pub user_id_two: ::prost::alloc::string::String,
}
/// A logical error which may occur on the server.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    /// The error code which should be one of "Error.Code" enums.
    #[prost(int32, tag = "1")]
    pub code: i32,
    /// A message in English to help developers debug the response.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// Additional error details which may be different for each response.
    #[prost(map = "string, string", tag = "3")]
    pub context:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Nested message and enum types in `Error`.
pub mod error {
    /// The selection of possible error codes.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Code {
        /// An unexpected result from the server.
        RuntimeException = 0,
        /// The server received a message which is not recognised.
        UnrecognizedPayload = 1,
        /// A message was expected but contains no content.
        MissingPayload = 2,
        /// Fields in the message have an invalid format.
        BadInput = 3,
        /// The match id was not found.
        MatchNotFound = 4,
        /// The match join was rejected.
        MatchJoinRejected = 5,
        /// The runtime function does not exist on the server.
        RuntimeFunctionNotFound = 6,
        /// The runtime function executed with an error.
        RuntimeFunctionException = 7,
    }
}
/// A realtime match.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Match {
    /// The match unique ID.
    #[prost(string, tag = "1")]
    pub match_id: ::prost::alloc::string::String,
    /// True if it's an server-managed authoritative match, false otherwise.
    #[prost(bool, tag = "2")]
    pub authoritative: bool,
    /// Match label, if any.
    #[prost(message, optional, tag = "3")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
    /// The number of users currently in the match.
    #[prost(int32, tag = "4")]
    pub size: i32,
    /// The users currently in the match.
    #[prost(message, repeated, tag = "5")]
    pub presences: ::prost::alloc::vec::Vec<UserPresence>,
    /// A reference to the current user's presence in the match.
    #[prost(message, optional, tag = "6")]
    pub self_: ::core::option::Option<UserPresence>,
}
/// Create a new realtime match.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchCreate {}
/// Realtime match data received from the server.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchData {
    /// The match unique ID.
    #[prost(string, tag = "1")]
    pub match_id: ::prost::alloc::string::String,
    /// A reference to the user presence that sent this data, if any.
    #[prost(message, optional, tag = "2")]
    pub presence: ::core::option::Option<UserPresence>,
    /// Op code value.
    #[prost(int64, tag = "3")]
    pub op_code: i64,
    /// Data payload, if any.
    #[prost(bytes = "vec", tag = "4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// True if this data was delivered reliably, false otherwise.
    #[prost(bool, tag = "5")]
    pub reliable: bool,
}
/// Send realtime match data to the server.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchDataSend {
    /// The match unique ID.
    #[prost(string, tag = "1")]
    pub match_id: ::prost::alloc::string::String,
    /// Op code value.
    #[prost(int64, tag = "2")]
    pub op_code: i64,
    /// Data payload, if any.
    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// List of presences in the match to deliver to, if filtering is required. Otherwise deliver to everyone in the match.
    #[prost(message, repeated, tag = "4")]
    pub presences: ::prost::alloc::vec::Vec<UserPresence>,
    /// True if the data should be sent reliably, false otherwise.
    #[prost(bool, tag = "5")]
    pub reliable: bool,
}
/// Join an existing realtime match.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchJoin {
    /// An optional set of key-value metadata pairs to be passed to the match handler, if any.
    #[prost(map = "string, string", tag = "3")]
    pub metadata:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(oneof = "match_join::Id", tags = "1, 2")]
    pub id: ::core::option::Option<match_join::Id>,
}
/// Nested message and enum types in `MatchJoin`.
pub mod match_join {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Id {
        /// The match unique ID.
        #[prost(string, tag = "1")]
        MatchId(::prost::alloc::string::String),
        /// A matchmaking result token.
        #[prost(string, tag = "2")]
        Token(::prost::alloc::string::String),
    }
}
/// Leave a realtime match.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchLeave {
    /// The match unique ID.
    #[prost(string, tag = "1")]
    pub match_id: ::prost::alloc::string::String,
}
/// A set of joins and leaves on a particular realtime match.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchPresenceEvent {
    /// The match unique ID.
    #[prost(string, tag = "1")]
    pub match_id: ::prost::alloc::string::String,
    /// User presences that have just joined the match.
    #[prost(message, repeated, tag = "2")]
    pub joins: ::prost::alloc::vec::Vec<UserPresence>,
    /// User presences that have just left the match.
    #[prost(message, repeated, tag = "3")]
    pub leaves: ::prost::alloc::vec::Vec<UserPresence>,
}
/// Start a new matchmaking process.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchmakerAdd {
    /// Minimum total user count to match together.
    #[prost(int32, tag = "1")]
    pub min_count: i32,
    /// Maximum total user count to match together.
    #[prost(int32, tag = "2")]
    pub max_count: i32,
    /// Filter query used to identify suitable users.
    #[prost(string, tag = "3")]
    pub query: ::prost::alloc::string::String,
    /// String properties.
    #[prost(map = "string, string", tag = "4")]
    pub string_properties:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Numeric properties.
    #[prost(map = "string, double", tag = "5")]
    pub numeric_properties: ::std::collections::HashMap<::prost::alloc::string::String, f64>,
}
/// A successful matchmaking result.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchmakerMatched {
    /// The matchmaking ticket that has completed.
    #[prost(string, tag = "1")]
    pub ticket: ::prost::alloc::string::String,
    /// The users that have been matched together, and information about their matchmaking data.
    #[prost(message, repeated, tag = "4")]
    pub users: ::prost::alloc::vec::Vec<matchmaker_matched::MatchmakerUser>,
    /// A reference to the current user and their properties.
    #[prost(message, optional, tag = "5")]
    pub self_: ::core::option::Option<matchmaker_matched::MatchmakerUser>,
    /// The match token or match ID to join.
    #[prost(oneof = "matchmaker_matched::Id", tags = "2, 3")]
    pub id: ::core::option::Option<matchmaker_matched::Id>,
}
/// Nested message and enum types in `MatchmakerMatched`.
pub mod matchmaker_matched {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MatchmakerUser {
        /// User info.
        #[prost(message, optional, tag = "1")]
        pub presence: ::core::option::Option<super::UserPresence>,
        /// Party identifier, if this user was matched as a party member.
        #[prost(string, tag = "2")]
        pub party_id: ::prost::alloc::string::String,
        /// String properties.
        #[prost(map = "string, string", tag = "5")]
        pub string_properties: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
        /// Numeric properties.
        #[prost(map = "string, double", tag = "6")]
        pub numeric_properties: ::std::collections::HashMap<::prost::alloc::string::String, f64>,
    }
    /// The match token or match ID to join.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Id {
        /// Match ID.
        #[prost(string, tag = "2")]
        MatchId(::prost::alloc::string::String),
        /// Match join token.
        #[prost(string, tag = "3")]
        Token(::prost::alloc::string::String),
    }
}
/// Cancel an existing ongoing matchmaking process.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchmakerRemove {
    /// The ticket to cancel.
    #[prost(string, tag = "1")]
    pub ticket: ::prost::alloc::string::String,
}
/// A ticket representing a new matchmaking process.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchmakerTicket {
    /// The ticket that can be used to cancel matchmaking.
    #[prost(string, tag = "1")]
    pub ticket: ::prost::alloc::string::String,
}
/// A collection of zero or more notifications.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Notifications {
    /// Collection of notifications.
    #[prost(message, repeated, tag = "1")]
    pub notifications: ::prost::alloc::vec::Vec<super::api::Notification>,
}
/// Incoming information about a party.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Party {
    /// Unique party identifier.
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,
    /// Open flag.
    #[prost(bool, tag = "2")]
    pub open: bool,
    /// Maximum number of party members.
    #[prost(int32, tag = "3")]
    pub max_size: i32,
    /// Self.
    #[prost(message, optional, tag = "4")]
    pub self_: ::core::option::Option<UserPresence>,
    /// Leader.
    #[prost(message, optional, tag = "5")]
    pub leader: ::core::option::Option<UserPresence>,
    /// All current party members.
    #[prost(message, repeated, tag = "6")]
    pub presences: ::prost::alloc::vec::Vec<UserPresence>,
}
/// Create a party.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyCreate {
    /// Whether or not the party will require join requests to be approved by the party leader.
    #[prost(bool, tag = "1")]
    pub open: bool,
    /// Maximum number of party members.
    #[prost(int32, tag = "2")]
    pub max_size: i32,
}
/// Join a party, or request to join if the party is not open.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyJoin {
    /// Party ID to join.
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,
}
/// Leave a party.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyLeave {
    /// Party ID to leave.
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,
}
/// Promote a new party leader.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyPromote {
    /// Party ID to promote a new leader for.
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,
    /// The presence of an existing party member to promote as the new leader.
    #[prost(message, optional, tag = "2")]
    pub presence: ::core::option::Option<UserPresence>,
}
/// Announcement of a new party leader.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyLeader {
    /// Party ID to announce the new leader for.
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,
    /// The presence of the new party leader.
    #[prost(message, optional, tag = "2")]
    pub presence: ::core::option::Option<UserPresence>,
}
/// Accept a request to join.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyAccept {
    /// Party ID to accept a join request for.
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,
    /// The presence to accept as a party member.
    #[prost(message, optional, tag = "2")]
    pub presence: ::core::option::Option<UserPresence>,
}
/// Kick a party member, or decline a request to join.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyRemove {
    /// Party ID to remove/reject from.
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,
    /// The presence to remove or reject.
    #[prost(message, optional, tag = "2")]
    pub presence: ::core::option::Option<UserPresence>,
}
/// End a party, kicking all party members and closing it.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyClose {
    /// Party ID to close.
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,
}
/// Request a list of pending join requests for a party.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyJoinRequestList {
    /// Party ID to get a list of join requests for.
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,
}
/// Incoming notification for one or more new presences attempting to join the party.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyJoinRequest {
    /// Party ID these presences are attempting to join.
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,
    /// Presences attempting to join.
    #[prost(message, repeated, tag = "2")]
    pub presences: ::prost::alloc::vec::Vec<UserPresence>,
}
/// Begin matchmaking as a party.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyMatchmakerAdd {
    /// Party ID.
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,
    /// Minimum total user count to match together.
    #[prost(int32, tag = "2")]
    pub min_count: i32,
    /// Maximum total user count to match together.
    #[prost(int32, tag = "3")]
    pub max_count: i32,
    /// Filter query used to identify suitable users.
    #[prost(string, tag = "4")]
    pub query: ::prost::alloc::string::String,
    /// String properties.
    #[prost(map = "string, string", tag = "5")]
    pub string_properties:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Numeric properties.
    #[prost(map = "string, double", tag = "6")]
    pub numeric_properties: ::std::collections::HashMap<::prost::alloc::string::String, f64>,
}
/// Cancel a party matchmaking process using a ticket.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyMatchmakerRemove {
    /// Party ID.
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,
    /// The ticket to cancel.
    #[prost(string, tag = "2")]
    pub ticket: ::prost::alloc::string::String,
}
/// A response from starting a new party matchmaking process.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyMatchmakerTicket {
    /// Party ID.
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,
    /// The ticket that can be used to cancel matchmaking.
    #[prost(string, tag = "2")]
    pub ticket: ::prost::alloc::string::String,
}
/// Incoming party data delivered from the server.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyData {
    /// The party ID.
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,
    /// A reference to the user presence that sent this data, if any.
    #[prost(message, optional, tag = "2")]
    pub presence: ::core::option::Option<UserPresence>,
    /// Op code value.
    #[prost(int64, tag = "3")]
    pub op_code: i64,
    /// Data payload, if any.
    #[prost(bytes = "vec", tag = "4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Send data to a party.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyDataSend {
    /// Party ID to send to.
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,
    /// Op code value.
    #[prost(int64, tag = "2")]
    pub op_code: i64,
    /// Data payload, if any.
    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Presence update for a particular party.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyPresenceEvent {
    /// The party ID.
    #[prost(string, tag = "1")]
    pub party_id: ::prost::alloc::string::String,
    /// User presences that have just joined the party.
    #[prost(message, repeated, tag = "2")]
    pub joins: ::prost::alloc::vec::Vec<UserPresence>,
    /// User presences that have just left the party.
    #[prost(message, repeated, tag = "3")]
    pub leaves: ::prost::alloc::vec::Vec<UserPresence>,
}
/// Application-level heartbeat and connection check.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ping {}
/// Application-level heartbeat and connection check response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pong {}
/// A snapshot of statuses for some set of users.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    /// User statuses.
    #[prost(message, repeated, tag = "1")]
    pub presences: ::prost::alloc::vec::Vec<UserPresence>,
}
/// Start receiving status updates for some set of users.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusFollow {
    /// User IDs to follow.
    #[prost(string, repeated, tag = "1")]
    pub user_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Usernames to follow.
    #[prost(string, repeated, tag = "2")]
    pub usernames: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A batch of status updates for a given user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusPresenceEvent {
    /// New statuses for the user.
    #[prost(message, repeated, tag = "2")]
    pub joins: ::prost::alloc::vec::Vec<UserPresence>,
    /// Previous statuses for the user.
    #[prost(message, repeated, tag = "3")]
    pub leaves: ::prost::alloc::vec::Vec<UserPresence>,
}
/// Stop receiving status updates for some set of users.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusUnfollow {
    /// Users to unfollow.
    #[prost(string, repeated, tag = "1")]
    pub user_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Set the user's own status.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusUpdate {
    /// Status string to set, if not present the user will appear offline.
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
}
/// Represents identifying information for a stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stream {
    /// Mode identifies the type of stream.
    #[prost(int32, tag = "1")]
    pub mode: i32,
    /// Subject is the primary identifier, if any.
    #[prost(string, tag = "2")]
    pub subject: ::prost::alloc::string::String,
    /// Subcontext is a secondary identifier, if any.
    #[prost(string, tag = "3")]
    pub subcontext: ::prost::alloc::string::String,
    /// The label is an arbitrary identifying string, if the stream has one.
    #[prost(string, tag = "4")]
    pub label: ::prost::alloc::string::String,
}
/// A data message delivered over a stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamData {
    /// The stream this data message relates to.
    #[prost(message, optional, tag = "1")]
    pub stream: ::core::option::Option<Stream>,
    /// The sender, if any.
    #[prost(message, optional, tag = "2")]
    pub sender: ::core::option::Option<UserPresence>,
    /// Arbitrary contents of the data message.
    #[prost(string, tag = "3")]
    pub data: ::prost::alloc::string::String,
    /// True if this data was delivered reliably, false otherwise.
    #[prost(bool, tag = "4")]
    pub reliable: bool,
}
/// A set of joins and leaves on a particular stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamPresenceEvent {
    /// The stream this event relates to.
    #[prost(message, optional, tag = "1")]
    pub stream: ::core::option::Option<Stream>,
    /// Presences joining the stream as part of this event, if any.
    #[prost(message, repeated, tag = "2")]
    pub joins: ::prost::alloc::vec::Vec<UserPresence>,
    /// Presences leaving the stream as part of this event, if any.
    #[prost(message, repeated, tag = "3")]
    pub leaves: ::prost::alloc::vec::Vec<UserPresence>,
}
/// A user session associated to a stream, usually through a list operation or a join/leave event.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserPresence {
    /// The user this presence belongs to.
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    /// A unique session ID identifying the particular connection, because the user may have many.
    #[prost(string, tag = "2")]
    pub session_id: ::prost::alloc::string::String,
    /// The username for display purposes.
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
    /// Whether this presence generates persistent data/messages, if applicable for the stream type.
    #[prost(bool, tag = "4")]
    pub persistence: bool,
    /// A user-set status message for this stream, if applicable.
    #[prost(message, optional, tag = "5")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
}
