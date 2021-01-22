/// A user with additional account details. Always the current user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    /// The user object.
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<User>,
    /// The user's wallet data.
    #[prost(string, tag = "2")]
    pub wallet: ::prost::alloc::string::String,
    /// The email address of the user.
    #[prost(string, tag = "3")]
    pub email: ::prost::alloc::string::String,
    /// The devices which belong to the user's account.
    #[prost(message, repeated, tag = "4")]
    pub devices: ::prost::alloc::vec::Vec<AccountDevice>,
    /// The custom id in the user's account.
    #[prost(string, tag = "5")]
    pub custom_id: ::prost::alloc::string::String,
    /// The UNIX time when the user's email was verified.
    #[prost(message, optional, tag = "6")]
    pub verify_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The UNIX time when the user's account was disabled/banned.
    #[prost(message, optional, tag = "7")]
    pub disable_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Obtain a new authentication token using a refresh token.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountRefresh {
    /// Refresh token.
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    /// Extra information that will be bundled in the session token.
    #[prost(map = "string, string", tag = "2")]
    pub vars:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Send a Apple Sign In token to the server. Used with authenticate/link/unlink.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountApple {
    /// The ID token received from Apple to validate.
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    /// Extra information that will be bundled in the session token.
    #[prost(map = "string, string", tag = "2")]
    pub vars:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Send a custom ID to the server. Used with authenticate/link/unlink.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountCustom {
    /// A custom identifier.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Extra information that will be bundled in the session token.
    #[prost(map = "string, string", tag = "2")]
    pub vars:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Send a device to the server. Used with authenticate/link/unlink and user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountDevice {
    /// A device identifier. Should be obtained by a platform-specific device API.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Extra information that will be bundled in the session token.
    #[prost(map = "string, string", tag = "2")]
    pub vars:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Send an email with password to the server. Used with authenticate/link/unlink.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountEmail {
    /// A valid RFC-5322 email address.
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
    /// A password for the user account.
    ///
    /// Ignored with unlink operations.
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
    /// Extra information that will be bundled in the session token.
    #[prost(map = "string, string", tag = "3")]
    pub vars:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Send a Facebook token to the server. Used with authenticate/link/unlink.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountFacebook {
    /// The OAuth token received from Facebook to access their profile API.
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    /// Extra information that will be bundled in the session token.
    #[prost(map = "string, string", tag = "2")]
    pub vars:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Send a Facebook Instant Game token to the server. Used with authenticate/link/unlink.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountFacebookInstantGame {
    /// The OAuth token received from a Facebook Instant Game that may be decoded with the Application Secret (must be available with the nakama configuration)
    #[prost(string, tag = "1")]
    pub signed_player_info: ::prost::alloc::string::String,
    /// Extra information that will be bundled in the session token.
    #[prost(map = "string, string", tag = "2")]
    pub vars:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Send Apple's Game Center account credentials to the server. Used with authenticate/link/unlink.
///
/// https://developer.apple.com/documentation/gamekit/gklocalplayer/1515407-generateidentityverificationsign
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountGameCenter {
    /// Player ID (generated by GameCenter).
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    /// Bundle ID (generated by GameCenter).
    #[prost(string, tag = "2")]
    pub bundle_id: ::prost::alloc::string::String,
    /// Time since UNIX epoch when the signature was created.
    #[prost(int64, tag = "3")]
    pub timestamp_seconds: i64,
    /// A random "NSString" used to compute the hash and keep it randomized.
    #[prost(string, tag = "4")]
    pub salt: ::prost::alloc::string::String,
    /// The verification signature data generated.
    #[prost(string, tag = "5")]
    pub signature: ::prost::alloc::string::String,
    /// The URL for the public encryption key.
    #[prost(string, tag = "6")]
    pub public_key_url: ::prost::alloc::string::String,
    /// Extra information that will be bundled in the session token.
    #[prost(map = "string, string", tag = "7")]
    pub vars:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Send a Google token to the server. Used with authenticate/link/unlink.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountGoogle {
    /// The OAuth token received from Google to access their profile API.
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    /// Extra information that will be bundled in the session token.
    #[prost(map = "string, string", tag = "2")]
    pub vars:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Send a Steam token to the server. Used with authenticate/link/unlink.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountSteam {
    /// The account token received from Steam to access their profile API.
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    /// Extra information that will be bundled in the session token.
    #[prost(map = "string, string", tag = "2")]
    pub vars:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Add one or more friends to the current user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddFriendsRequest {
    /// The account id of a user.
    #[prost(string, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The account username of a user.
    #[prost(string, repeated, tag = "2")]
    pub usernames: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Add users to a group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddGroupUsersRequest {
    /// The group to add users to.
    #[prost(string, tag = "1")]
    pub group_id: ::prost::alloc::string::String,
    /// The users to add.
    #[prost(string, repeated, tag = "2")]
    pub user_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Authenticate against the server with a refresh token.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionRefreshRequest {
    /// Refresh token.
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    /// Extra information that will be bundled in the session token.
    #[prost(map = "string, string", tag = "2")]
    pub vars:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Authenticate against the server with Apple Sign In.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateAppleRequest {
    /// The Apple account details.
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<AccountApple>,
    /// Register the account if the user does not already exist.
    #[prost(message, optional, tag = "2")]
    pub create: ::core::option::Option<bool>,
    /// Set the username on the account at register. Must be unique.
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
}
/// Authenticate against the server with a custom ID.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateCustomRequest {
    /// The custom account details.
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<AccountCustom>,
    /// Register the account if the user does not already exist.
    #[prost(message, optional, tag = "2")]
    pub create: ::core::option::Option<bool>,
    /// Set the username on the account at register. Must be unique.
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
}
/// Authenticate against the server with a device ID.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateDeviceRequest {
    /// The device account details.
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<AccountDevice>,
    /// Register the account if the user does not already exist.
    #[prost(message, optional, tag = "2")]
    pub create: ::core::option::Option<bool>,
    /// Set the username on the account at register. Must be unique.
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
}
/// Authenticate against the server with email+password.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateEmailRequest {
    /// The email account details.
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<AccountEmail>,
    /// Register the account if the user does not already exist.
    #[prost(message, optional, tag = "2")]
    pub create: ::core::option::Option<bool>,
    /// Set the username on the account at register. Must be unique.
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
}
/// Authenticate against the server with Facebook.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateFacebookRequest {
    /// The Facebook account details.
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<AccountFacebook>,
    /// Register the account if the user does not already exist.
    #[prost(message, optional, tag = "2")]
    pub create: ::core::option::Option<bool>,
    /// Set the username on the account at register. Must be unique.
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
    /// Import Facebook friends for the user.
    #[prost(message, optional, tag = "4")]
    pub sync: ::core::option::Option<bool>,
}
/// Authenticate against the server with Facebook Instant Game token.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateFacebookInstantGameRequest {
    /// The Facebook Instant Game account details.
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<AccountFacebookInstantGame>,
    /// Register the account if the user does not already exist.
    #[prost(message, optional, tag = "2")]
    pub create: ::core::option::Option<bool>,
    /// Set the username on the account at register. Must be unique.
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
}
/// Authenticate against the server with Apple's Game Center.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateGameCenterRequest {
    /// The Game Center account details.
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<AccountGameCenter>,
    /// Register the account if the user does not already exist.
    #[prost(message, optional, tag = "2")]
    pub create: ::core::option::Option<bool>,
    /// Set the username on the account at register. Must be unique.
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
}
/// Authenticate against the server with Google.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateGoogleRequest {
    /// The Google account details.
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<AccountGoogle>,
    /// Register the account if the user does not already exist.
    #[prost(message, optional, tag = "2")]
    pub create: ::core::option::Option<bool>,
    /// Set the username on the account at register. Must be unique.
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
}
/// Authenticate against the server with Steam.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateSteamRequest {
    /// The Steam account details.
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<AccountSteam>,
    /// Register the account if the user does not already exist.
    #[prost(message, optional, tag = "2")]
    pub create: ::core::option::Option<bool>,
    /// Set the username on the account at register. Must be unique.
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
}
/// Ban users from a group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BanGroupUsersRequest {
    /// The group to ban users from.
    #[prost(string, tag = "1")]
    pub group_id: ::prost::alloc::string::String,
    /// The users to ban.
    #[prost(string, repeated, tag = "2")]
    pub user_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Block one or more friends for the current user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockFriendsRequest {
    /// The account id of a user.
    #[prost(string, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The account username of a user.
    #[prost(string, repeated, tag = "2")]
    pub usernames: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A message sent on a channel.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelMessage {
    /// The channel this message belongs to.
    #[prost(string, tag = "1")]
    pub channel_id: ::prost::alloc::string::String,
    /// The unique ID of this message.
    #[prost(string, tag = "2")]
    pub message_id: ::prost::alloc::string::String,
    /// The code representing a message type or category.
    #[prost(message, optional, tag = "3")]
    pub code: ::core::option::Option<i32>,
    /// Message sender, usually a user ID.
    #[prost(string, tag = "4")]
    pub sender_id: ::prost::alloc::string::String,
    /// The username of the message sender, if any.
    #[prost(string, tag = "5")]
    pub username: ::prost::alloc::string::String,
    /// The content payload.
    #[prost(string, tag = "6")]
    pub content: ::prost::alloc::string::String,
    /// The UNIX time when the message was created.
    #[prost(message, optional, tag = "7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The UNIX time when the message was last updated.
    #[prost(message, optional, tag = "8")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// True if the message was persisted to the channel's history, false otherwise.
    #[prost(message, optional, tag = "9")]
    pub persistent: ::core::option::Option<bool>,
    /// The name of the chat room, or an empty string if this message was not sent through a chat room.
    #[prost(string, tag = "10")]
    pub room_name: ::prost::alloc::string::String,
    /// The ID of the group, or an empty string if this message was not sent through a group channel.
    #[prost(string, tag = "11")]
    pub group_id: ::prost::alloc::string::String,
    /// The ID of the first DM user, or an empty string if this message was not sent through a DM chat.
    #[prost(string, tag = "12")]
    pub user_id_one: ::prost::alloc::string::String,
    /// The ID of the second DM user, or an empty string if this message was not sent through a DM chat.
    #[prost(string, tag = "13")]
    pub user_id_two: ::prost::alloc::string::String,
}
/// A list of channel messages, usually a result of a list operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelMessageList {
    /// A list of messages.
    #[prost(message, repeated, tag = "1")]
    pub messages: ::prost::alloc::vec::Vec<ChannelMessage>,
    /// The cursor to send when retrieving the next page, if any.
    #[prost(string, tag = "2")]
    pub next_cursor: ::prost::alloc::string::String,
    /// The cursor to send when retrieving the previous page, if any.
    #[prost(string, tag = "3")]
    pub prev_cursor: ::prost::alloc::string::String,
    /// Cacheable cursor to list newer messages. Durable and designed to be stored, unlike next/prev cursors.
    #[prost(string, tag = "4")]
    pub cacheable_cursor: ::prost::alloc::string::String,
}
/// Create a group with the current user as owner.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGroupRequest {
    /// A unique name for the group.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A description for the group.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// The language expected to be a tag which follows the BCP-47 spec.
    #[prost(string, tag = "3")]
    pub lang_tag: ::prost::alloc::string::String,
    /// A URL for an avatar image.
    #[prost(string, tag = "4")]
    pub avatar_url: ::prost::alloc::string::String,
    /// Mark a group as open or not where only admins can accept members.
    #[prost(bool, tag = "5")]
    pub open: bool,
    /// Maximum number of group members.
    #[prost(int32, tag = "6")]
    pub max_count: i32,
}
/// Delete one or more friends for the current user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFriendsRequest {
    /// The account id of a user.
    #[prost(string, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The account username of a user.
    #[prost(string, repeated, tag = "2")]
    pub usernames: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Delete a group the user has access to.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteGroupRequest {
    /// The id of a group.
    #[prost(string, tag = "1")]
    pub group_id: ::prost::alloc::string::String,
}
/// Delete a leaderboard record.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteLeaderboardRecordRequest {
    /// The leaderboard ID to delete from.
    #[prost(string, tag = "1")]
    pub leaderboard_id: ::prost::alloc::string::String,
}
/// Delete one or more notifications for the current user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteNotificationsRequest {
    /// The id of notifications.
    #[prost(string, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Storage objects to delete.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteStorageObjectId {
    /// The collection which stores the object.
    #[prost(string, tag = "1")]
    pub collection: ::prost::alloc::string::String,
    /// The key of the object within the collection.
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    /// The version hash of the object.
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
}
/// Batch delete storage objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteStorageObjectsRequest {
    /// Batch of storage objects.
    #[prost(message, repeated, tag = "1")]
    pub object_ids: ::prost::alloc::vec::Vec<DeleteStorageObjectId>,
}
/// Represents an event to be passed through the server to registered event handlers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// An event name, type, category, or identifier.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Arbitrary event property values.
    #[prost(map = "string, string", tag = "2")]
    pub properties:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// The time when the event was triggered.
    #[prost(message, optional, tag = "3")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    /// True if the event came directly from a client call, false otherwise.
    #[prost(bool, tag = "4")]
    pub external: bool,
}
/// A friend of a user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Friend {
    /// The user object.
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<User>,
    /// The friend status.
    ///
    /// one of "Friend.State".
    #[prost(message, optional, tag = "2")]
    pub state: ::core::option::Option<i32>,
    /// Time of the latest relationship update.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `Friend`.
pub mod friend {
    /// The friendship status.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The user is a friend of the current user.
        Friend = 0,
        /// The current user has sent an invite to the user.
        InviteSent = 1,
        /// The current user has received an invite from this user.
        InviteReceived = 2,
        /// The current user has blocked this user.
        Blocked = 3,
    }
}
/// A collection of zero or more friends of the user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendList {
    /// The Friend objects.
    #[prost(message, repeated, tag = "1")]
    pub friends: ::prost::alloc::vec::Vec<Friend>,
    /// Cursor for the next page of results, if any.
    #[prost(string, tag = "2")]
    pub cursor: ::prost::alloc::string::String,
}
/// Fetch a batch of zero or more users from the server.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUsersRequest {
    /// The account id of a user.
    #[prost(string, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The account username of a user.
    #[prost(string, repeated, tag = "2")]
    pub usernames: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The Facebook ID of a user.
    #[prost(string, repeated, tag = "3")]
    pub facebook_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A group in the server.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Group {
    /// The id of a group.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The id of the user who created the group.
    #[prost(string, tag = "2")]
    pub creator_id: ::prost::alloc::string::String,
    /// The unique name of the group.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// A description for the group.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// The language expected to be a tag which follows the BCP-47 spec.
    #[prost(string, tag = "5")]
    pub lang_tag: ::prost::alloc::string::String,
    /// Additional information stored as a JSON object.
    #[prost(string, tag = "6")]
    pub metadata: ::prost::alloc::string::String,
    /// A URL for an avatar image.
    #[prost(string, tag = "7")]
    pub avatar_url: ::prost::alloc::string::String,
    /// Anyone can join open groups, otherwise only admins can accept members.
    #[prost(message, optional, tag = "8")]
    pub open: ::core::option::Option<bool>,
    /// The current count of all members in the group.
    #[prost(int32, tag = "9")]
    pub edge_count: i32,
    /// The maximum number of members allowed.
    #[prost(int32, tag = "10")]
    pub max_count: i32,
    /// The UNIX time when the group was created.
    #[prost(message, optional, tag = "11")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The UNIX time when the group was last updated.
    #[prost(message, optional, tag = "12")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// One or more groups returned from a listing operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupList {
    /// One or more groups.
    #[prost(message, repeated, tag = "1")]
    pub groups: ::prost::alloc::vec::Vec<Group>,
    /// A cursor used to get the next page.
    #[prost(string, tag = "2")]
    pub cursor: ::prost::alloc::string::String,
}
/// A list of users belonging to a group, along with their role.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupUserList {
    /// User-role pairs for a group.
    #[prost(message, repeated, tag = "1")]
    pub group_users: ::prost::alloc::vec::Vec<group_user_list::GroupUser>,
    /// Cursor for the next page of results, if any.
    #[prost(string, tag = "2")]
    pub cursor: ::prost::alloc::string::String,
}
/// Nested message and enum types in `GroupUserList`.
pub mod group_user_list {
    /// A single user-role pair.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GroupUser {
        /// User.
        #[prost(message, optional, tag = "1")]
        pub user: ::core::option::Option<super::User>,
        /// Their relationship to the group.
        #[prost(message, optional, tag = "2")]
        pub state: ::core::option::Option<i32>,
    }
    /// Nested message and enum types in `GroupUser`.
    pub mod group_user {
        /// The group role status.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum State {
            /// The user is a superadmin with full control of the group.
            Superadmin = 0,
            /// The user is an admin with additional privileges.
            Admin = 1,
            /// The user is a regular member.
            Member = 2,
            /// The user has requested to join the group
            JoinRequest = 3,
        }
    }
}
/// Import Facebook friends into the current user's account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportFacebookFriendsRequest {
    /// The Facebook account details.
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<AccountFacebook>,
    /// Reset the current user's friends list.
    #[prost(message, optional, tag = "2")]
    pub reset: ::core::option::Option<bool>,
}
/// Immediately join an open group, or request to join a closed one.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinGroupRequest {
    /// The group ID to join. The group must already exist.
    #[prost(string, tag = "1")]
    pub group_id: ::prost::alloc::string::String,
}
/// The request to join a tournament.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinTournamentRequest {
    /// The ID of the tournament to join. The tournament must already exist.
    #[prost(string, tag = "1")]
    pub tournament_id: ::prost::alloc::string::String,
}
/// Kick a set of users from a group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KickGroupUsersRequest {
    /// The group ID to kick from.
    #[prost(string, tag = "1")]
    pub group_id: ::prost::alloc::string::String,
    /// The users to kick.
    #[prost(string, repeated, tag = "2")]
    pub user_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Represents a complete leaderboard record with all scores and associated metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaderboardRecord {
    /// The ID of the leaderboard this score belongs to.
    #[prost(string, tag = "1")]
    pub leaderboard_id: ::prost::alloc::string::String,
    /// The ID of the score owner, usually a user or group.
    #[prost(string, tag = "2")]
    pub owner_id: ::prost::alloc::string::String,
    /// The username of the score owner, if the owner is a user.
    #[prost(message, optional, tag = "3")]
    pub username: ::core::option::Option<::prost::alloc::string::String>,
    /// The score value.
    #[prost(int64, tag = "4")]
    pub score: i64,
    /// An optional subscore value.
    #[prost(int64, tag = "5")]
    pub subscore: i64,
    /// The number of submissions to this score record.
    #[prost(int32, tag = "6")]
    pub num_score: i32,
    /// Metadata.
    #[prost(string, tag = "7")]
    pub metadata: ::prost::alloc::string::String,
    /// The UNIX time when the leaderboard record was created.
    #[prost(message, optional, tag = "8")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The UNIX time when the leaderboard record was updated.
    #[prost(message, optional, tag = "9")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The UNIX time when the leaderboard record expires.
    #[prost(message, optional, tag = "10")]
    pub expiry_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The rank of this record.
    #[prost(int64, tag = "11")]
    pub rank: i64,
    /// The maximum number of score updates allowed by the owner.
    #[prost(uint32, tag = "12")]
    pub max_num_score: u32,
}
/// A set of leaderboard records, may be part of a leaderboard records page or a batch of individual records.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaderboardRecordList {
    /// A list of leaderboard records.
    #[prost(message, repeated, tag = "1")]
    pub records: ::prost::alloc::vec::Vec<LeaderboardRecord>,
    /// A batched set of leaderboard records belonging to specified owners.
    #[prost(message, repeated, tag = "2")]
    pub owner_records: ::prost::alloc::vec::Vec<LeaderboardRecord>,
    /// The cursor to send when retrieving the next page, if any.
    #[prost(string, tag = "3")]
    pub next_cursor: ::prost::alloc::string::String,
    /// The cursor to send when retrieving the previous page, if any.
    #[prost(string, tag = "4")]
    pub prev_cursor: ::prost::alloc::string::String,
}
/// Leave a group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaveGroupRequest {
    /// The group ID to leave.
    #[prost(string, tag = "1")]
    pub group_id: ::prost::alloc::string::String,
}
/// Link Facebook to the current user's account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkFacebookRequest {
    /// The Facebook account details.
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<AccountFacebook>,
    /// Import Facebook friends for the user.
    #[prost(message, optional, tag = "4")]
    pub sync: ::core::option::Option<bool>,
}
/// List a channel's message history.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChannelMessagesRequest {
    /// The channel ID to list from.
    #[prost(string, tag = "1")]
    pub channel_id: ::prost::alloc::string::String,
    /// Max number of records to return. Between 1 and 100.
    #[prost(message, optional, tag = "2")]
    pub limit: ::core::option::Option<i32>,
    /// True if listing should be older messages to newer, false if reverse.
    #[prost(message, optional, tag = "3")]
    pub forward: ::core::option::Option<bool>,
    /// A pagination cursor, if any.
    #[prost(string, tag = "4")]
    pub cursor: ::prost::alloc::string::String,
}
/// List friends for a user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFriendsRequest {
    /// Max number of records to return. Between 1 and 100.
    #[prost(message, optional, tag = "1")]
    pub limit: ::core::option::Option<i32>,
    /// The friend state to list.
    #[prost(message, optional, tag = "2")]
    pub state: ::core::option::Option<i32>,
    /// An optional next page cursor.
    #[prost(string, tag = "3")]
    pub cursor: ::prost::alloc::string::String,
}
/// List groups based on given filters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGroupsRequest {
    /// List groups that contain this value in their names.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional pagination cursor.
    #[prost(string, tag = "2")]
    pub cursor: ::prost::alloc::string::String,
    /// Max number of groups to return. Between 1 and 100.
    #[prost(message, optional, tag = "3")]
    pub limit: ::core::option::Option<i32>,
}
/// List all users that are part of a group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGroupUsersRequest {
    /// The group ID to list from.
    #[prost(string, tag = "1")]
    pub group_id: ::prost::alloc::string::String,
    /// Max number of records to return. Between 1 and 100.
    #[prost(message, optional, tag = "2")]
    pub limit: ::core::option::Option<i32>,
    /// The group user state to list.
    #[prost(message, optional, tag = "3")]
    pub state: ::core::option::Option<i32>,
    /// An optional next page cursor.
    #[prost(string, tag = "4")]
    pub cursor: ::prost::alloc::string::String,
}
/// List leaerboard records from a given leaderboard around the owner.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLeaderboardRecordsAroundOwnerRequest {
    /// The ID of the tournament to list for.
    #[prost(string, tag = "1")]
    pub leaderboard_id: ::prost::alloc::string::String,
    /// Max number of records to return. Between 1 and 100.
    #[prost(message, optional, tag = "2")]
    pub limit: ::core::option::Option<u32>,
    /// The owner to retrieve records around.
    #[prost(string, tag = "3")]
    pub owner_id: ::prost::alloc::string::String,
    /// Expiry in seconds (since epoch) to begin fetching records from.
    #[prost(message, optional, tag = "4")]
    pub expiry: ::core::option::Option<i64>,
}
/// List leaderboard records from a given leaderboard.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLeaderboardRecordsRequest {
    /// The ID of the leaderboard to list for.
    #[prost(string, tag = "1")]
    pub leaderboard_id: ::prost::alloc::string::String,
    /// One or more owners to retrieve records for.
    #[prost(string, repeated, tag = "2")]
    pub owner_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Max number of records to return. Between 1 and 100.
    #[prost(message, optional, tag = "3")]
    pub limit: ::core::option::Option<i32>,
    /// A next or previous page cursor.
    #[prost(string, tag = "4")]
    pub cursor: ::prost::alloc::string::String,
    /// Expiry in seconds (since epoch) to begin fetching records from. Optional. 0 means from current time.
    #[prost(message, optional, tag = "5")]
    pub expiry: ::core::option::Option<i64>,
}
/// List realtime matches.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMatchesRequest {
    /// Limit the number of returned matches.
    #[prost(message, optional, tag = "1")]
    pub limit: ::core::option::Option<i32>,
    /// Authoritative or relayed matches.
    #[prost(message, optional, tag = "2")]
    pub authoritative: ::core::option::Option<bool>,
    /// Label filter.
    #[prost(message, optional, tag = "3")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
    /// Minimum user count.
    #[prost(message, optional, tag = "4")]
    pub min_size: ::core::option::Option<i32>,
    /// Maximum user count.
    #[prost(message, optional, tag = "5")]
    pub max_size: ::core::option::Option<i32>,
    /// Arbitrary label query.
    #[prost(message, optional, tag = "6")]
    pub query: ::core::option::Option<::prost::alloc::string::String>,
}
/// Get a list of unexpired notifications.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotificationsRequest {
    /// The number of notifications to get. Between 1 and 100.
    #[prost(message, optional, tag = "1")]
    pub limit: ::core::option::Option<i32>,
    /// A cursor to page through notifications. May be cached by clients to get from point in time forwards.
    ///
    /// value from NotificationList.cacheable_cursor.
    #[prost(string, tag = "2")]
    pub cacheable_cursor: ::prost::alloc::string::String,
}
/// List publicly readable storage objects in a given collection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStorageObjectsRequest {
    /// ID of the user.
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    /// The collection which stores the object.
    #[prost(string, tag = "2")]
    pub collection: ::prost::alloc::string::String,
    /// The number of storage objects to list. Between 1 and 100.
    #[prost(message, optional, tag = "3")]
    pub limit: ::core::option::Option<i32>,
    /// The cursor to page through results from.
    ///
    /// value from StorageObjectList.cursor.
    #[prost(string, tag = "4")]
    pub cursor: ::prost::alloc::string::String,
}
/// List tournament records from a given tournament around the owner.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTournamentRecordsAroundOwnerRequest {
    /// The ID of the tournament to list for.
    #[prost(string, tag = "1")]
    pub tournament_id: ::prost::alloc::string::String,
    /// Max number of records to return. Between 1 and 100.
    #[prost(message, optional, tag = "2")]
    pub limit: ::core::option::Option<u32>,
    /// The owner to retrieve records around.
    #[prost(string, tag = "3")]
    pub owner_id: ::prost::alloc::string::String,
    /// Expiry in seconds (since epoch) to begin fetching records from.
    #[prost(message, optional, tag = "4")]
    pub expiry: ::core::option::Option<i64>,
}
/// List tournament records from a given tournament.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTournamentRecordsRequest {
    /// The ID of the tournament to list for.
    #[prost(string, tag = "1")]
    pub tournament_id: ::prost::alloc::string::String,
    /// One or more owners to retrieve records for.
    #[prost(string, repeated, tag = "2")]
    pub owner_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Max number of records to return. Between 1 and 100.
    #[prost(message, optional, tag = "3")]
    pub limit: ::core::option::Option<i32>,
    /// A next or previous page cursor.
    #[prost(string, tag = "4")]
    pub cursor: ::prost::alloc::string::String,
    /// Expiry in seconds (since epoch) to begin fetching records from.
    #[prost(message, optional, tag = "5")]
    pub expiry: ::core::option::Option<i64>,
}
/// List active/upcoming tournaments based on given filters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTournamentsRequest {
    /// The start of the categories to include. Defaults to 0.
    #[prost(message, optional, tag = "1")]
    pub category_start: ::core::option::Option<u32>,
    /// The end of the categories to include. Defaults to 128.
    #[prost(message, optional, tag = "2")]
    pub category_end: ::core::option::Option<u32>,
    /// The start time for tournaments. Defaults to epoch.
    #[prost(message, optional, tag = "3")]
    pub start_time: ::core::option::Option<u32>,
    /// The end time for tournaments. Defaults to +1 year from current Unix time.
    #[prost(message, optional, tag = "4")]
    pub end_time: ::core::option::Option<u32>,
    /// Max number of records to return. Between 1 and 100.
    #[prost(message, optional, tag = "6")]
    pub limit: ::core::option::Option<i32>,
    /// A next page cursor for listings (optional).
    #[prost(string, tag = "8")]
    pub cursor: ::prost::alloc::string::String,
}
/// List the groups a user is part of, and their relationship to each.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUserGroupsRequest {
    /// ID of the user.
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    /// Max number of records to return. Between 1 and 100.
    #[prost(message, optional, tag = "2")]
    pub limit: ::core::option::Option<i32>,
    /// The user group state to list.
    #[prost(message, optional, tag = "3")]
    pub state: ::core::option::Option<i32>,
    /// An optional next page cursor.
    #[prost(string, tag = "4")]
    pub cursor: ::prost::alloc::string::String,
}
/// Represents a realtime match.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Match {
    /// The ID of the match, can be used to join.
    #[prost(string, tag = "1")]
    pub match_id: ::prost::alloc::string::String,
    /// True if it's an server-managed authoritative match, false otherwise.
    #[prost(bool, tag = "2")]
    pub authoritative: bool,
    /// Match label, if any.
    #[prost(message, optional, tag = "3")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
    /// Current number of users in the match.
    #[prost(int32, tag = "4")]
    pub size: i32,
    /// Tick Rate
    #[prost(int32, tag = "5")]
    pub tick_rate: i32,
    /// Handler name
    #[prost(string, tag = "6")]
    pub handler_name: ::prost::alloc::string::String,
}
/// A list of realtime matches.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchList {
    /// A number of matches corresponding to a list operation.
    #[prost(message, repeated, tag = "1")]
    pub matches: ::prost::alloc::vec::Vec<Match>,
}
/// A notification in the server.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Notification {
    /// ID of the Notification.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Subject of the notification.
    #[prost(string, tag = "2")]
    pub subject: ::prost::alloc::string::String,
    /// Content of the notification in JSON.
    #[prost(string, tag = "3")]
    pub content: ::prost::alloc::string::String,
    /// Category code for this notification.
    #[prost(int32, tag = "4")]
    pub code: i32,
    /// ID of the sender, if a user. Otherwise 'null'.
    #[prost(string, tag = "5")]
    pub sender_id: ::prost::alloc::string::String,
    /// The UNIX time when the notification was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// True if this notification was persisted to the database.
    #[prost(bool, tag = "7")]
    pub persistent: bool,
}
/// A collection of zero or more notifications.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationList {
    /// Collection of notifications.
    #[prost(message, repeated, tag = "1")]
    pub notifications: ::prost::alloc::vec::Vec<Notification>,
    /// Use this cursor to paginate notifications. Cache this to catch up to new notifications.
    #[prost(string, tag = "2")]
    pub cacheable_cursor: ::prost::alloc::string::String,
}
/// Promote a set of users in a group to the next role up.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PromoteGroupUsersRequest {
    /// The group ID to promote in.
    #[prost(string, tag = "1")]
    pub group_id: ::prost::alloc::string::String,
    /// The users to promote.
    #[prost(string, repeated, tag = "2")]
    pub user_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Demote a set of users in a group to the next role down.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DemoteGroupUsersRequest {
    /// The group ID to demote in.
    #[prost(string, tag = "1")]
    pub group_id: ::prost::alloc::string::String,
    /// The users to demote.
    #[prost(string, repeated, tag = "2")]
    pub user_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Storage objects to get.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadStorageObjectId {
    /// The collection which stores the object.
    #[prost(string, tag = "1")]
    pub collection: ::prost::alloc::string::String,
    /// The key of the object within the collection.
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    /// The user owner of the object.
    #[prost(string, tag = "3")]
    pub user_id: ::prost::alloc::string::String,
}
/// Batch get storage objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadStorageObjectsRequest {
    /// Batch of storage objects.
    #[prost(message, repeated, tag = "1")]
    pub object_ids: ::prost::alloc::vec::Vec<ReadStorageObjectId>,
}
/// Execute an Lua function on the server.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rpc {
    /// The identifier of the function.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The payload of the function which must be a JSON object.
    #[prost(string, tag = "2")]
    pub payload: ::prost::alloc::string::String,
    /// The authentication key used when executed as a non-client HTTP request.
    #[prost(string, tag = "3")]
    pub http_key: ::prost::alloc::string::String,
}
/// A user's session used to authenticate messages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Session {
    /// True if the corresponding account was just created, false otherwise.
    #[prost(bool, tag = "1")]
    pub created: bool,
    /// Authentication credentials.
    #[prost(string, tag = "2")]
    pub token: ::prost::alloc::string::String,
    /// Refresh token that can be used for session token renewal.
    #[prost(string, tag = "3")]
    pub refresh_token: ::prost::alloc::string::String,
}
/// An object within the storage engine.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageObject {
    /// The collection which stores the object.
    #[prost(string, tag = "1")]
    pub collection: ::prost::alloc::string::String,
    /// The key of the object within the collection.
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    /// The user owner of the object.
    #[prost(string, tag = "3")]
    pub user_id: ::prost::alloc::string::String,
    /// The value of the object.
    #[prost(string, tag = "4")]
    pub value: ::prost::alloc::string::String,
    /// The version hash of the object.
    #[prost(string, tag = "5")]
    pub version: ::prost::alloc::string::String,
    /// The read access permissions for the object.
    #[prost(int32, tag = "6")]
    pub permission_read: i32,
    /// The write access permissions for the object.
    #[prost(int32, tag = "7")]
    pub permission_write: i32,
    /// The UNIX time when the object was created.
    #[prost(message, optional, tag = "8")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The UNIX time when the object was last updated.
    #[prost(message, optional, tag = "9")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// A storage acknowledgement.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageObjectAck {
    /// The collection which stores the object.
    #[prost(string, tag = "1")]
    pub collection: ::prost::alloc::string::String,
    /// The key of the object within the collection.
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    /// The version hash of the object.
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
    /// The owner of the object.
    #[prost(string, tag = "4")]
    pub user_id: ::prost::alloc::string::String,
}
/// Batch of acknowledgements for the storage object write.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageObjectAcks {
    /// Batch of storage write acknowledgements.
    #[prost(message, repeated, tag = "1")]
    pub acks: ::prost::alloc::vec::Vec<StorageObjectAck>,
}
/// Batch of storage objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageObjects {
    /// The batch of storage objects.
    #[prost(message, repeated, tag = "1")]
    pub objects: ::prost::alloc::vec::Vec<StorageObject>,
}
/// List of storage objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageObjectList {
    /// The list of storage objects.
    #[prost(message, repeated, tag = "1")]
    pub objects: ::prost::alloc::vec::Vec<StorageObject>,
    /// The cursor for the next page of results, if any.
    #[prost(string, tag = "2")]
    pub cursor: ::prost::alloc::string::String,
}
/// A tournament on the server.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tournament {
    /// The ID of the tournament.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The title for the tournament.
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// The description of the tournament. May be blank.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// The category of the tournament. e.g. "vip" could be category 1.
    #[prost(uint32, tag = "4")]
    pub category: u32,
    /// ASC or DESC sort mode of scores in the tournament.
    #[prost(uint32, tag = "5")]
    pub sort_order: u32,
    /// The current number of players in the tournament.
    #[prost(uint32, tag = "6")]
    pub size: u32,
    /// The maximum number of players for the tournament.
    #[prost(uint32, tag = "7")]
    pub max_size: u32,
    /// The maximum score updates allowed per player for the current tournament.
    #[prost(uint32, tag = "8")]
    pub max_num_score: u32,
    /// True if the tournament is active and can enter. A computed value.
    #[prost(bool, tag = "9")]
    pub can_enter: bool,
    /// The UNIX time when the tournament stops being active until next reset. A computed value.
    #[prost(uint32, tag = "10")]
    pub end_active: u32,
    /// The UNIX time when the tournament is next playable. A computed value.
    #[prost(uint32, tag = "11")]
    pub next_reset: u32,
    /// Additional information stored as a JSON object.
    #[prost(string, tag = "12")]
    pub metadata: ::prost::alloc::string::String,
    /// The UNIX time when the tournament was created.
    #[prost(message, optional, tag = "13")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The UNIX time when the tournament will start.
    #[prost(message, optional, tag = "14")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The UNIX time when the tournament will be stopped.
    #[prost(message, optional, tag = "15")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Duration of the tournament in seconds.
    #[prost(uint32, tag = "16")]
    pub duration: u32,
    /// The UNIX time when the tournament start being active. A computed value.
    #[prost(uint32, tag = "17")]
    pub start_active: u32,
}
/// A list of tournaments.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TournamentList {
    /// The list of tournaments returned.
    #[prost(message, repeated, tag = "1")]
    pub tournaments: ::prost::alloc::vec::Vec<Tournament>,
    /// A pagination cursor (optional).
    #[prost(string, tag = "2")]
    pub cursor: ::prost::alloc::string::String,
}
/// A set of tournament records which may be part of a tournament records page or a batch of individual records.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TournamentRecordList {
    /// A list of tournament records.
    #[prost(message, repeated, tag = "1")]
    pub records: ::prost::alloc::vec::Vec<LeaderboardRecord>,
    /// A batched set of tournament records belonging to specified owners.
    #[prost(message, repeated, tag = "2")]
    pub owner_records: ::prost::alloc::vec::Vec<LeaderboardRecord>,
    /// The cursor to send when retireving the next page (optional).
    #[prost(string, tag = "3")]
    pub next_cursor: ::prost::alloc::string::String,
    /// The cursor to send when retrieving the previous page (optional).
    #[prost(string, tag = "4")]
    pub prev_cursor: ::prost::alloc::string::String,
}
/// Update a user's account details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAccountRequest {
    /// The username of the user's account.
    #[prost(message, optional, tag = "1")]
    pub username: ::core::option::Option<::prost::alloc::string::String>,
    /// The display name of the user.
    #[prost(message, optional, tag = "2")]
    pub display_name: ::core::option::Option<::prost::alloc::string::String>,
    /// A URL for an avatar image.
    #[prost(message, optional, tag = "3")]
    pub avatar_url: ::core::option::Option<::prost::alloc::string::String>,
    /// The language expected to be a tag which follows the BCP-47 spec.
    #[prost(message, optional, tag = "4")]
    pub lang_tag: ::core::option::Option<::prost::alloc::string::String>,
    /// The location set by the user.
    #[prost(message, optional, tag = "5")]
    pub location: ::core::option::Option<::prost::alloc::string::String>,
    /// The timezone set by the user.
    #[prost(message, optional, tag = "6")]
    pub timezone: ::core::option::Option<::prost::alloc::string::String>,
}
/// Update fields in a given group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGroupRequest {
    /// The ID of the group to update.
    #[prost(string, tag = "1")]
    pub group_id: ::prost::alloc::string::String,
    /// Name.
    #[prost(message, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Description string.
    #[prost(message, optional, tag = "3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// Lang tag.
    #[prost(message, optional, tag = "4")]
    pub lang_tag: ::core::option::Option<::prost::alloc::string::String>,
    /// Avatar URL.
    #[prost(message, optional, tag = "5")]
    pub avatar_url: ::core::option::Option<::prost::alloc::string::String>,
    /// Open is true if anyone should be allowed to join, or false if joins must be approved by a group admin.
    #[prost(message, optional, tag = "6")]
    pub open: ::core::option::Option<bool>,
}
/// A user in the server.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    /// The id of the user's account.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The username of the user's account.
    #[prost(string, tag = "2")]
    pub username: ::prost::alloc::string::String,
    /// The display name of the user.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// A URL for an avatar image.
    #[prost(string, tag = "4")]
    pub avatar_url: ::prost::alloc::string::String,
    /// The language expected to be a tag which follows the BCP-47 spec.
    #[prost(string, tag = "5")]
    pub lang_tag: ::prost::alloc::string::String,
    /// The location set by the user.
    #[prost(string, tag = "6")]
    pub location: ::prost::alloc::string::String,
    /// The timezone set by the user.
    #[prost(string, tag = "7")]
    pub timezone: ::prost::alloc::string::String,
    /// Additional information stored as a JSON object.
    #[prost(string, tag = "8")]
    pub metadata: ::prost::alloc::string::String,
    /// The Facebook id in the user's account.
    #[prost(string, tag = "9")]
    pub facebook_id: ::prost::alloc::string::String,
    /// The Google id in the user's account.
    #[prost(string, tag = "10")]
    pub google_id: ::prost::alloc::string::String,
    /// The Apple Game Center in of the user's account.
    #[prost(string, tag = "11")]
    pub gamecenter_id: ::prost::alloc::string::String,
    /// The Steam id in the user's account.
    #[prost(string, tag = "12")]
    pub steam_id: ::prost::alloc::string::String,
    /// Indicates whether the user is currently online.
    #[prost(bool, tag = "13")]
    pub online: bool,
    /// Number of related edges to this user.
    #[prost(int32, tag = "14")]
    pub edge_count: i32,
    /// The UNIX time when the user was created.
    #[prost(message, optional, tag = "15")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The UNIX time when the user was last updated.
    #[prost(message, optional, tag = "16")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The Facebook Instant Game ID in the user's account.
    #[prost(string, tag = "17")]
    pub facebook_instant_game_id: ::prost::alloc::string::String,
    /// The Apple Sign In ID in the user's account.
    #[prost(string, tag = "18")]
    pub apple_id: ::prost::alloc::string::String,
}
/// A list of groups belonging to a user, along with the user's role in each group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserGroupList {
    /// Group-role pairs for a user.
    #[prost(message, repeated, tag = "1")]
    pub user_groups: ::prost::alloc::vec::Vec<user_group_list::UserGroup>,
    /// Cursor for the next page of results, if any.
    #[prost(string, tag = "2")]
    pub cursor: ::prost::alloc::string::String,
}
/// Nested message and enum types in `UserGroupList`.
pub mod user_group_list {
    /// A single group-role pair.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UserGroup {
        /// Group.
        #[prost(message, optional, tag = "1")]
        pub group: ::core::option::Option<super::Group>,
        /// The user's relationship to the group.
        #[prost(message, optional, tag = "2")]
        pub state: ::core::option::Option<i32>,
    }
    /// Nested message and enum types in `UserGroup`.
    pub mod user_group {
        /// The group role status.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum State {
            /// The user is a superadmin with full control of the group.
            Superadmin = 0,
            /// The user is an admin with additional privileges.
            Admin = 1,
            /// The user is a regular member.
            Member = 2,
            /// The user has requested to join the group
            JoinRequest = 3,
        }
    }
}
/// A collection of zero or more users.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Users {
    /// The User objects.
    #[prost(message, repeated, tag = "1")]
    pub users: ::prost::alloc::vec::Vec<User>,
}
/// A request to submit a score to a leaderboard.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteLeaderboardRecordRequest {
    /// The ID of the leaderboard to write to.
    #[prost(string, tag = "1")]
    pub leaderboard_id: ::prost::alloc::string::String,
    /// Record input.
    #[prost(message, optional, tag = "2")]
    pub record: ::core::option::Option<write_leaderboard_record_request::LeaderboardRecordWrite>,
}
/// Nested message and enum types in `WriteLeaderboardRecordRequest`.
pub mod write_leaderboard_record_request {
    /// Record values to write.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LeaderboardRecordWrite {
        /// The score value to submit.
        #[prost(int64, tag = "1")]
        pub score: i64,
        /// An optional secondary value.
        #[prost(int64, tag = "2")]
        pub subscore: i64,
        /// Optional record metadata.
        #[prost(string, tag = "3")]
        pub metadata: ::prost::alloc::string::String,
    }
}
/// The object to store.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteStorageObject {
    /// The collection to store the object.
    #[prost(string, tag = "1")]
    pub collection: ::prost::alloc::string::String,
    /// The key for the object within the collection.
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    /// The value of the object.
    #[prost(string, tag = "3")]
    pub value: ::prost::alloc::string::String,
    /// The version hash of the object to check. Possible values are: ["", "*", "#hash#"].
    ///
    /// if-match and if-none-match
    #[prost(string, tag = "4")]
    pub version: ::prost::alloc::string::String,
    /// The read access permissions for the object.
    #[prost(message, optional, tag = "5")]
    pub permission_read: ::core::option::Option<i32>,
    /// The write access permissions for the object.
    #[prost(message, optional, tag = "6")]
    pub permission_write: ::core::option::Option<i32>,
}
/// Write objects to the storage engine.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteStorageObjectsRequest {
    /// The objects to store on the server.
    #[prost(message, repeated, tag = "1")]
    pub objects: ::prost::alloc::vec::Vec<WriteStorageObject>,
}
/// A request to submit a score to a tournament.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteTournamentRecordRequest {
    /// The tournament ID to write the record for.
    #[prost(string, tag = "1")]
    pub tournament_id: ::prost::alloc::string::String,
    /// Record input.
    #[prost(message, optional, tag = "2")]
    pub record: ::core::option::Option<write_tournament_record_request::TournamentRecordWrite>,
}
/// Nested message and enum types in `WriteTournamentRecordRequest`.
pub mod write_tournament_record_request {
    /// Record values to write.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TournamentRecordWrite {
        /// The score value to submit.
        #[prost(int64, tag = "1")]
        pub score: i64,
        /// An optional secondary value.
        #[prost(int64, tag = "2")]
        pub subscore: i64,
        /// A JSON object of additional properties (optional).
        #[prost(string, tag = "3")]
        pub metadata: ::prost::alloc::string::String,
    }
}
#[doc = r" Generated client implementations."]
pub mod nakama_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "*"]
    #[doc = " The Nakama RPC protocol service built with GRPC."]
    pub struct NakamaClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl NakamaClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> NakamaClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Add friends by ID or username to a user's account."]
        pub async fn add_friends(
            &mut self,
            request: impl tonic::IntoRequest<super::AddFriendsRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/AddFriends");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Add users to a group."]
        pub async fn add_group_users(
            &mut self,
            request: impl tonic::IntoRequest<super::AddGroupUsersRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/AddGroupUsers");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Refresh a user's session using a refresh token retrieved from a previous authentication request."]
        pub async fn session_refresh(
            &mut self,
            request: impl tonic::IntoRequest<super::SessionRefreshRequest>,
        ) -> Result<tonic::Response<super::Session>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/SessionRefresh");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Authenticate a user with an Apple ID against the server."]
        pub async fn authenticate_apple(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthenticateAppleRequest>,
        ) -> Result<tonic::Response<super::Session>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/AuthenticateApple");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Authenticate a user with a custom id against the server."]
        pub async fn authenticate_custom(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthenticateCustomRequest>,
        ) -> Result<tonic::Response<super::Session>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/nakama.api.Nakama/AuthenticateCustom");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Authenticate a user with a device id against the server."]
        pub async fn authenticate_device(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthenticateDeviceRequest>,
        ) -> Result<tonic::Response<super::Session>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/nakama.api.Nakama/AuthenticateDevice");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Authenticate a user with an email+password against the server."]
        pub async fn authenticate_email(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthenticateEmailRequest>,
        ) -> Result<tonic::Response<super::Session>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/AuthenticateEmail");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Authenticate a user with a Facebook OAuth token against the server."]
        pub async fn authenticate_facebook(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthenticateFacebookRequest>,
        ) -> Result<tonic::Response<super::Session>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/nakama.api.Nakama/AuthenticateFacebook");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Authenticate a user with a Facebook Instant Game token against the server."]
        pub async fn authenticate_facebook_instant_game(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthenticateFacebookInstantGameRequest>,
        ) -> Result<tonic::Response<super::Session>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/nakama.api.Nakama/AuthenticateFacebookInstantGame",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Authenticate a user with Apple's GameCenter against the server."]
        pub async fn authenticate_game_center(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthenticateGameCenterRequest>,
        ) -> Result<tonic::Response<super::Session>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/nakama.api.Nakama/AuthenticateGameCenter");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Authenticate a user with Google against the server."]
        pub async fn authenticate_google(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthenticateGoogleRequest>,
        ) -> Result<tonic::Response<super::Session>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/nakama.api.Nakama/AuthenticateGoogle");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Authenticate a user with Steam against the server."]
        pub async fn authenticate_steam(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthenticateSteamRequest>,
        ) -> Result<tonic::Response<super::Session>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/AuthenticateSteam");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Ban a set of users from a group."]
        pub async fn ban_group_users(
            &mut self,
            request: impl tonic::IntoRequest<super::BanGroupUsersRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/BanGroupUsers");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Block one or more users by ID or username."]
        pub async fn block_friends(
            &mut self,
            request: impl tonic::IntoRequest<super::BlockFriendsRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/BlockFriends");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Create a new group with the current user as the owner."]
        pub async fn create_group(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateGroupRequest>,
        ) -> Result<tonic::Response<super::Group>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/CreateGroup");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Delete one or more users by ID or username."]
        pub async fn delete_friends(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteFriendsRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/DeleteFriends");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Delete a group by ID."]
        pub async fn delete_group(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteGroupRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/DeleteGroup");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Delete a leaderboard record."]
        pub async fn delete_leaderboard_record(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteLeaderboardRecordRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/nakama.api.Nakama/DeleteLeaderboardRecord");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Delete one or more notifications for the current user."]
        pub async fn delete_notifications(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteNotificationsRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/nakama.api.Nakama/DeleteNotifications");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Delete one or more objects by ID or username."]
        pub async fn delete_storage_objects(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteStorageObjectsRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/nakama.api.Nakama/DeleteStorageObjects");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Submit an event for processing in the server's registered runtime custom events handler."]
        pub async fn event(
            &mut self,
            request: impl tonic::IntoRequest<super::Event>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/Event");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Fetch the current user's account."]
        pub async fn get_account(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> Result<tonic::Response<super::Account>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/GetAccount");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Fetch zero or more users by ID and/or username."]
        pub async fn get_users(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUsersRequest>,
        ) -> Result<tonic::Response<super::Users>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/GetUsers");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " A healthcheck which load balancers can use to check the service."]
        pub async fn healthcheck(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/Healthcheck");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Import Facebook friends and add them to a user's account."]
        pub async fn import_facebook_friends(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportFacebookFriendsRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/nakama.api.Nakama/ImportFacebookFriends");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Immediately join an open group, or request to join a closed one."]
        pub async fn join_group(
            &mut self,
            request: impl tonic::IntoRequest<super::JoinGroupRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/JoinGroup");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Attempt to join an open and running tournament."]
        pub async fn join_tournament(
            &mut self,
            request: impl tonic::IntoRequest<super::JoinTournamentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/JoinTournament");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Kick a set of users from a group."]
        pub async fn kick_group_users(
            &mut self,
            request: impl tonic::IntoRequest<super::KickGroupUsersRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/KickGroupUsers");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Leave a group the user is a member of."]
        pub async fn leave_group(
            &mut self,
            request: impl tonic::IntoRequest<super::LeaveGroupRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/LeaveGroup");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Add an Apple ID to the social profiles on the current user's account."]
        pub async fn link_apple(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountApple>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/LinkApple");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Add a custom ID to the social profiles on the current user's account."]
        pub async fn link_custom(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountCustom>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/LinkCustom");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Add a device ID to the social profiles on the current user's account."]
        pub async fn link_device(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountDevice>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/LinkDevice");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Add an email+password to the social profiles on the current user's account."]
        pub async fn link_email(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountEmail>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/LinkEmail");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Add Facebook to the social profiles on the current user's account."]
        pub async fn link_facebook(
            &mut self,
            request: impl tonic::IntoRequest<super::LinkFacebookRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/LinkFacebook");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Add Facebook Instant Game to the social profiles on the current user's account."]
        pub async fn link_facebook_instant_game(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountFacebookInstantGame>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/nakama.api.Nakama/LinkFacebookInstantGame");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Add Apple's GameCenter to the social profiles on the current user's account."]
        pub async fn link_game_center(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountGameCenter>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/LinkGameCenter");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Add Google to the social profiles on the current user's account."]
        pub async fn link_google(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountGoogle>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/LinkGoogle");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Add Steam to the social profiles on the current user's account."]
        pub async fn link_steam(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountSteam>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/LinkSteam");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List a channel's message history."]
        pub async fn list_channel_messages(
            &mut self,
            request: impl tonic::IntoRequest<super::ListChannelMessagesRequest>,
        ) -> Result<tonic::Response<super::ChannelMessageList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/nakama.api.Nakama/ListChannelMessages");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List all friends for the current user."]
        pub async fn list_friends(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFriendsRequest>,
        ) -> Result<tonic::Response<super::FriendList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/ListFriends");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List groups based on given filters."]
        pub async fn list_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGroupsRequest>,
        ) -> Result<tonic::Response<super::GroupList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/ListGroups");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List all users that are part of a group."]
        pub async fn list_group_users(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGroupUsersRequest>,
        ) -> Result<tonic::Response<super::GroupUserList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/ListGroupUsers");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List leaderboard records."]
        pub async fn list_leaderboard_records(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLeaderboardRecordsRequest>,
        ) -> Result<tonic::Response<super::LeaderboardRecordList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/nakama.api.Nakama/ListLeaderboardRecords");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List leaderboard records that belong to a user."]
        pub async fn list_leaderboard_records_around_owner(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLeaderboardRecordsAroundOwnerRequest>,
        ) -> Result<tonic::Response<super::LeaderboardRecordList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/nakama.api.Nakama/ListLeaderboardRecordsAroundOwner",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Fetch list of running matches."]
        pub async fn list_matches(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMatchesRequest>,
        ) -> Result<tonic::Response<super::MatchList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/ListMatches");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Fetch list of notifications."]
        pub async fn list_notifications(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNotificationsRequest>,
        ) -> Result<tonic::Response<super::NotificationList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/ListNotifications");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List publicly readable storage objects in a given collection."]
        pub async fn list_storage_objects(
            &mut self,
            request: impl tonic::IntoRequest<super::ListStorageObjectsRequest>,
        ) -> Result<tonic::Response<super::StorageObjectList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/nakama.api.Nakama/ListStorageObjects");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List current or upcoming tournaments."]
        pub async fn list_tournaments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTournamentsRequest>,
        ) -> Result<tonic::Response<super::TournamentList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/ListTournaments");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List tournament records."]
        pub async fn list_tournament_records(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTournamentRecordsRequest>,
        ) -> Result<tonic::Response<super::TournamentRecordList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/nakama.api.Nakama/ListTournamentRecords");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List tournament records for a given owner."]
        pub async fn list_tournament_records_around_owner(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTournamentRecordsAroundOwnerRequest>,
        ) -> Result<tonic::Response<super::TournamentRecordList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/nakama.api.Nakama/ListTournamentRecordsAroundOwner",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List groups the current user belongs to."]
        pub async fn list_user_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUserGroupsRequest>,
        ) -> Result<tonic::Response<super::UserGroupList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/ListUserGroups");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Promote a set of users in a group to the next role up."]
        pub async fn promote_group_users(
            &mut self,
            request: impl tonic::IntoRequest<super::PromoteGroupUsersRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/PromoteGroupUsers");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Demote a set of users in a group to the next role down."]
        pub async fn demote_group_users(
            &mut self,
            request: impl tonic::IntoRequest<super::DemoteGroupUsersRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/DemoteGroupUsers");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get storage objects."]
        pub async fn read_storage_objects(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadStorageObjectsRequest>,
        ) -> Result<tonic::Response<super::StorageObjects>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/nakama.api.Nakama/ReadStorageObjects");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Execute a Lua function on the server."]
        pub async fn rpc_func(
            &mut self,
            request: impl tonic::IntoRequest<super::Rpc>,
        ) -> Result<tonic::Response<super::Rpc>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/RpcFunc");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Remove the Apple ID from the social profiles on the current user's account."]
        pub async fn unlink_apple(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountApple>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/UnlinkApple");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Remove the custom ID from the social profiles on the current user's account."]
        pub async fn unlink_custom(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountCustom>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/UnlinkCustom");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Remove the device ID from the social profiles on the current user's account."]
        pub async fn unlink_device(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountDevice>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/UnlinkDevice");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Remove the email+password from the social profiles on the current user's account."]
        pub async fn unlink_email(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountEmail>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/UnlinkEmail");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Remove Facebook from the social profiles on the current user's account."]
        pub async fn unlink_facebook(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountFacebook>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/UnlinkFacebook");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Remove Facebook Instant Game profile from the social profiles on the current user's account."]
        pub async fn unlink_facebook_instant_game(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountFacebookInstantGame>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/nakama.api.Nakama/UnlinkFacebookInstantGame",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Remove Apple's GameCenter from the social profiles on the current user's account."]
        pub async fn unlink_game_center(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountGameCenter>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/UnlinkGameCenter");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Remove Google from the social profiles on the current user's account."]
        pub async fn unlink_google(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountGoogle>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/UnlinkGoogle");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Remove Steam from the social profiles on the current user's account."]
        pub async fn unlink_steam(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountSteam>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/UnlinkSteam");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Update fields in the current user's account."]
        pub async fn update_account(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAccountRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/UpdateAccount");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Update fields in a given group."]
        pub async fn update_group(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGroupRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nakama.api.Nakama/UpdateGroup");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Write a record to a leaderboard."]
        pub async fn write_leaderboard_record(
            &mut self,
            request: impl tonic::IntoRequest<super::WriteLeaderboardRecordRequest>,
        ) -> Result<tonic::Response<super::LeaderboardRecord>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/nakama.api.Nakama/WriteLeaderboardRecord");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Write objects into the storage engine."]
        pub async fn write_storage_objects(
            &mut self,
            request: impl tonic::IntoRequest<super::WriteStorageObjectsRequest>,
        ) -> Result<tonic::Response<super::StorageObjectAcks>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/nakama.api.Nakama/WriteStorageObjects");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Write a record to a tournament."]
        pub async fn write_tournament_record(
            &mut self,
            request: impl tonic::IntoRequest<super::WriteTournamentRecordRequest>,
        ) -> Result<tonic::Response<super::LeaderboardRecord>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/nakama.api.Nakama/WriteTournamentRecord");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for NakamaClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for NakamaClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "NakamaClient {{ ... }}")
        }
    }
}
