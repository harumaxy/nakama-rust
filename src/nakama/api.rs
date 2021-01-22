
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<User>,
    
    #[prost(string, tag = "2")]
    pub wallet: ::prost::alloc::string::String,
    
    #[prost(string, tag = "3")]
    pub email: ::prost::alloc::string::String,
    
    #[prost(message, repeated, tag = "4")]
    pub devices: ::prost::alloc::vec::Vec<AccountDevice>,
    
    #[prost(string, tag = "5")]
    pub custom_id: ::prost::alloc::string::String,
    
    #[prost(message, optional, tag = "6")]
    pub verify_time: ::core::option::Option<::prost_types::Timestamp>,
    
    #[prost(message, optional, tag = "7")]
    pub disable_time: ::core::option::Option<::prost_types::Timestamp>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountRefresh {
    
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    
    #[prost(map = "string, string", tag = "2")]
    pub vars:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountApple {
    
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    
    #[prost(map = "string, string", tag = "2")]
    pub vars:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountCustom {
    
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    
    #[prost(map = "string, string", tag = "2")]
    pub vars:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountDevice {
    
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    
    #[prost(map = "string, string", tag = "2")]
    pub vars:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountEmail {
    
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
    
    
    
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
    
    #[prost(map = "string, string", tag = "3")]
    pub vars:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountFacebook {
    
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    
    #[prost(map = "string, string", tag = "2")]
    pub vars:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountFacebookInstantGame {
    
    #[prost(string, tag = "1")]
    pub signed_player_info: ::prost::alloc::string::String,
    
    #[prost(map = "string, string", tag = "2")]
    pub vars:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}



#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountGameCenter {
    
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    
    #[prost(string, tag = "2")]
    pub bundle_id: ::prost::alloc::string::String,
    
    #[prost(int64, tag = "3")]
    pub timestamp_seconds: i64,
    
    #[prost(string, tag = "4")]
    pub salt: ::prost::alloc::string::String,
    
    #[prost(string, tag = "5")]
    pub signature: ::prost::alloc::string::String,
    
    #[prost(string, tag = "6")]
    pub public_key_url: ::prost::alloc::string::String,
    
    #[prost(map = "string, string", tag = "7")]
    pub vars:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountGoogle {
    
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    
    #[prost(map = "string, string", tag = "2")]
    pub vars:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountSteam {
    
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    
    #[prost(map = "string, string", tag = "2")]
    pub vars:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddFriendsRequest {
    
    #[prost(string, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    
    #[prost(string, repeated, tag = "2")]
    pub usernames: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddGroupUsersRequest {
    
    #[prost(string, tag = "1")]
    pub group_id: ::prost::alloc::string::String,
    
    #[prost(string, repeated, tag = "2")]
    pub user_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionRefreshRequest {
    
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    
    #[prost(map = "string, string", tag = "2")]
    pub vars:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateAppleRequest {
    
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<AccountApple>,
    
    #[prost(message, optional, tag = "2")]
    pub create: ::core::option::Option<bool>,
    
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateCustomRequest {
    
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<AccountCustom>,
    
    #[prost(message, optional, tag = "2")]
    pub create: ::core::option::Option<bool>,
    
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateDeviceRequest {
    
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<AccountDevice>,
    
    #[prost(message, optional, tag = "2")]
    pub create: ::core::option::Option<bool>,
    
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateEmailRequest {
    
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<AccountEmail>,
    
    #[prost(message, optional, tag = "2")]
    pub create: ::core::option::Option<bool>,
    
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateFacebookRequest {
    
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<AccountFacebook>,
    
    #[prost(message, optional, tag = "2")]
    pub create: ::core::option::Option<bool>,
    
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
    
    #[prost(message, optional, tag = "4")]
    pub sync: ::core::option::Option<bool>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateFacebookInstantGameRequest {
    
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<AccountFacebookInstantGame>,
    
    #[prost(message, optional, tag = "2")]
    pub create: ::core::option::Option<bool>,
    
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateGameCenterRequest {
    
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<AccountGameCenter>,
    
    #[prost(message, optional, tag = "2")]
    pub create: ::core::option::Option<bool>,
    
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateGoogleRequest {
    
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<AccountGoogle>,
    
    #[prost(message, optional, tag = "2")]
    pub create: ::core::option::Option<bool>,
    
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateSteamRequest {
    
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<AccountSteam>,
    
    #[prost(message, optional, tag = "2")]
    pub create: ::core::option::Option<bool>,
    
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BanGroupUsersRequest {
    
    #[prost(string, tag = "1")]
    pub group_id: ::prost::alloc::string::String,
    
    #[prost(string, repeated, tag = "2")]
    pub user_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockFriendsRequest {
    
    #[prost(string, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    
    #[prost(string, repeated, tag = "2")]
    pub usernames: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelMessage {
    
    #[prost(string, tag = "1")]
    pub channel_id: ::prost::alloc::string::String,
    
    #[prost(string, tag = "2")]
    pub message_id: ::prost::alloc::string::String,
    
    #[prost(message, optional, tag = "3")]
    pub code: ::core::option::Option<i32>,
    
    #[prost(string, tag = "4")]
    pub sender_id: ::prost::alloc::string::String,
    
    #[prost(string, tag = "5")]
    pub username: ::prost::alloc::string::String,
    
    #[prost(string, tag = "6")]
    pub content: ::prost::alloc::string::String,
    
    #[prost(message, optional, tag = "7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    
    #[prost(message, optional, tag = "8")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    
    #[prost(message, optional, tag = "9")]
    pub persistent: ::core::option::Option<bool>,
    
    #[prost(string, tag = "10")]
    pub room_name: ::prost::alloc::string::String,
    
    #[prost(string, tag = "11")]
    pub group_id: ::prost::alloc::string::String,
    
    #[prost(string, tag = "12")]
    pub user_id_one: ::prost::alloc::string::String,
    
    #[prost(string, tag = "13")]
    pub user_id_two: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelMessageList {
    
    #[prost(message, repeated, tag = "1")]
    pub messages: ::prost::alloc::vec::Vec<ChannelMessage>,
    
    #[prost(string, tag = "2")]
    pub next_cursor: ::prost::alloc::string::String,
    
    #[prost(string, tag = "3")]
    pub prev_cursor: ::prost::alloc::string::String,
    
    #[prost(string, tag = "4")]
    pub cacheable_cursor: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGroupRequest {
    
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    
    #[prost(string, tag = "3")]
    pub lang_tag: ::prost::alloc::string::String,
    
    #[prost(string, tag = "4")]
    pub avatar_url: ::prost::alloc::string::String,
    
    #[prost(bool, tag = "5")]
    pub open: bool,
    
    #[prost(int32, tag = "6")]
    pub max_count: i32,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFriendsRequest {
    
    #[prost(string, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    
    #[prost(string, repeated, tag = "2")]
    pub usernames: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteGroupRequest {
    
    #[prost(string, tag = "1")]
    pub group_id: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteLeaderboardRecordRequest {
    
    #[prost(string, tag = "1")]
    pub leaderboard_id: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteNotificationsRequest {
    
    #[prost(string, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteStorageObjectId {
    
    #[prost(string, tag = "1")]
    pub collection: ::prost::alloc::string::String,
    
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteStorageObjectsRequest {
    
    #[prost(message, repeated, tag = "1")]
    pub object_ids: ::prost::alloc::vec::Vec<DeleteStorageObjectId>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    
    #[prost(map = "string, string", tag = "2")]
    pub properties:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    
    #[prost(message, optional, tag = "3")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    
    #[prost(bool, tag = "4")]
    pub external: bool,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Friend {
    
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<User>,
    
    
    
    #[prost(message, optional, tag = "2")]
    pub state: ::core::option::Option<i32>,
    
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}

pub mod friend {
    
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        
        Friend = 0,
        
        InviteSent = 1,
        
        InviteReceived = 2,
        
        Blocked = 3,
    }
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendList {
    
    #[prost(message, repeated, tag = "1")]
    pub friends: ::prost::alloc::vec::Vec<Friend>,
    
    #[prost(string, tag = "2")]
    pub cursor: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUsersRequest {
    
    #[prost(string, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    
    #[prost(string, repeated, tag = "2")]
    pub usernames: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    
    #[prost(string, repeated, tag = "3")]
    pub facebook_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Group {
    
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    
    #[prost(string, tag = "2")]
    pub creator_id: ::prost::alloc::string::String,
    
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    
    #[prost(string, tag = "5")]
    pub lang_tag: ::prost::alloc::string::String,
    
    #[prost(string, tag = "6")]
    pub metadata: ::prost::alloc::string::String,
    
    #[prost(string, tag = "7")]
    pub avatar_url: ::prost::alloc::string::String,
    
    #[prost(message, optional, tag = "8")]
    pub open: ::core::option::Option<bool>,
    
    #[prost(int32, tag = "9")]
    pub edge_count: i32,
    
    #[prost(int32, tag = "10")]
    pub max_count: i32,
    
    #[prost(message, optional, tag = "11")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    
    #[prost(message, optional, tag = "12")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupList {
    
    #[prost(message, repeated, tag = "1")]
    pub groups: ::prost::alloc::vec::Vec<Group>,
    
    #[prost(string, tag = "2")]
    pub cursor: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupUserList {
    
    #[prost(message, repeated, tag = "1")]
    pub group_users: ::prost::alloc::vec::Vec<group_user_list::GroupUser>,
    
    #[prost(string, tag = "2")]
    pub cursor: ::prost::alloc::string::String,
}

pub mod group_user_list {
    
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GroupUser {
        
        #[prost(message, optional, tag = "1")]
        pub user: ::core::option::Option<super::User>,
        
        #[prost(message, optional, tag = "2")]
        pub state: ::core::option::Option<i32>,
    }
    
    pub mod group_user {
        
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum State {
            
            Superadmin = 0,
            
            Admin = 1,
            
            Member = 2,
            
            JoinRequest = 3,
        }
    }
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportFacebookFriendsRequest {
    
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<AccountFacebook>,
    
    #[prost(message, optional, tag = "2")]
    pub reset: ::core::option::Option<bool>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinGroupRequest {
    
    #[prost(string, tag = "1")]
    pub group_id: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinTournamentRequest {
    
    #[prost(string, tag = "1")]
    pub tournament_id: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KickGroupUsersRequest {
    
    #[prost(string, tag = "1")]
    pub group_id: ::prost::alloc::string::String,
    
    #[prost(string, repeated, tag = "2")]
    pub user_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaderboardRecord {
    
    #[prost(string, tag = "1")]
    pub leaderboard_id: ::prost::alloc::string::String,
    
    #[prost(string, tag = "2")]
    pub owner_id: ::prost::alloc::string::String,
    
    #[prost(message, optional, tag = "3")]
    pub username: ::core::option::Option<::prost::alloc::string::String>,
    
    #[prost(int64, tag = "4")]
    pub score: i64,
    
    #[prost(int64, tag = "5")]
    pub subscore: i64,
    
    #[prost(int32, tag = "6")]
    pub num_score: i32,
    
    #[prost(string, tag = "7")]
    pub metadata: ::prost::alloc::string::String,
    
    #[prost(message, optional, tag = "8")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    
    #[prost(message, optional, tag = "9")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    
    #[prost(message, optional, tag = "10")]
    pub expiry_time: ::core::option::Option<::prost_types::Timestamp>,
    
    #[prost(int64, tag = "11")]
    pub rank: i64,
    
    #[prost(uint32, tag = "12")]
    pub max_num_score: u32,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaderboardRecordList {
    
    #[prost(message, repeated, tag = "1")]
    pub records: ::prost::alloc::vec::Vec<LeaderboardRecord>,
    
    #[prost(message, repeated, tag = "2")]
    pub owner_records: ::prost::alloc::vec::Vec<LeaderboardRecord>,
    
    #[prost(string, tag = "3")]
    pub next_cursor: ::prost::alloc::string::String,
    
    #[prost(string, tag = "4")]
    pub prev_cursor: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaveGroupRequest {
    
    #[prost(string, tag = "1")]
    pub group_id: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkFacebookRequest {
    
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<AccountFacebook>,
    
    #[prost(message, optional, tag = "4")]
    pub sync: ::core::option::Option<bool>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChannelMessagesRequest {
    
    #[prost(string, tag = "1")]
    pub channel_id: ::prost::alloc::string::String,
    
    #[prost(message, optional, tag = "2")]
    pub limit: ::core::option::Option<i32>,
    
    #[prost(message, optional, tag = "3")]
    pub forward: ::core::option::Option<bool>,
    
    #[prost(string, tag = "4")]
    pub cursor: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFriendsRequest {
    
    #[prost(message, optional, tag = "1")]
    pub limit: ::core::option::Option<i32>,
    
    #[prost(message, optional, tag = "2")]
    pub state: ::core::option::Option<i32>,
    
    #[prost(string, tag = "3")]
    pub cursor: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGroupsRequest {
    
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    
    #[prost(string, tag = "2")]
    pub cursor: ::prost::alloc::string::String,
    
    #[prost(message, optional, tag = "3")]
    pub limit: ::core::option::Option<i32>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGroupUsersRequest {
    
    #[prost(string, tag = "1")]
    pub group_id: ::prost::alloc::string::String,
    
    #[prost(message, optional, tag = "2")]
    pub limit: ::core::option::Option<i32>,
    
    #[prost(message, optional, tag = "3")]
    pub state: ::core::option::Option<i32>,
    
    #[prost(string, tag = "4")]
    pub cursor: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLeaderboardRecordsAroundOwnerRequest {
    
    #[prost(string, tag = "1")]
    pub leaderboard_id: ::prost::alloc::string::String,
    
    #[prost(message, optional, tag = "2")]
    pub limit: ::core::option::Option<u32>,
    
    #[prost(string, tag = "3")]
    pub owner_id: ::prost::alloc::string::String,
    
    #[prost(message, optional, tag = "4")]
    pub expiry: ::core::option::Option<i64>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLeaderboardRecordsRequest {
    
    #[prost(string, tag = "1")]
    pub leaderboard_id: ::prost::alloc::string::String,
    
    #[prost(string, repeated, tag = "2")]
    pub owner_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    
    #[prost(message, optional, tag = "3")]
    pub limit: ::core::option::Option<i32>,
    
    #[prost(string, tag = "4")]
    pub cursor: ::prost::alloc::string::String,
    
    #[prost(message, optional, tag = "5")]
    pub expiry: ::core::option::Option<i64>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMatchesRequest {
    
    #[prost(message, optional, tag = "1")]
    pub limit: ::core::option::Option<i32>,
    
    #[prost(message, optional, tag = "2")]
    pub authoritative: ::core::option::Option<bool>,
    
    #[prost(message, optional, tag = "3")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
    
    #[prost(message, optional, tag = "4")]
    pub min_size: ::core::option::Option<i32>,
    
    #[prost(message, optional, tag = "5")]
    pub max_size: ::core::option::Option<i32>,
    
    #[prost(message, optional, tag = "6")]
    pub query: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotificationsRequest {
    
    #[prost(message, optional, tag = "1")]
    pub limit: ::core::option::Option<i32>,
    
    
    
    #[prost(string, tag = "2")]
    pub cacheable_cursor: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStorageObjectsRequest {
    
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    
    #[prost(string, tag = "2")]
    pub collection: ::prost::alloc::string::String,
    
    #[prost(message, optional, tag = "3")]
    pub limit: ::core::option::Option<i32>,
    
    
    
    #[prost(string, tag = "4")]
    pub cursor: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTournamentRecordsAroundOwnerRequest {
    
    #[prost(string, tag = "1")]
    pub tournament_id: ::prost::alloc::string::String,
    
    #[prost(message, optional, tag = "2")]
    pub limit: ::core::option::Option<u32>,
    
    #[prost(string, tag = "3")]
    pub owner_id: ::prost::alloc::string::String,
    
    #[prost(message, optional, tag = "4")]
    pub expiry: ::core::option::Option<i64>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTournamentRecordsRequest {
    
    #[prost(string, tag = "1")]
    pub tournament_id: ::prost::alloc::string::String,
    
    #[prost(string, repeated, tag = "2")]
    pub owner_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    
    #[prost(message, optional, tag = "3")]
    pub limit: ::core::option::Option<i32>,
    
    #[prost(string, tag = "4")]
    pub cursor: ::prost::alloc::string::String,
    
    #[prost(message, optional, tag = "5")]
    pub expiry: ::core::option::Option<i64>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTournamentsRequest {
    
    #[prost(message, optional, tag = "1")]
    pub category_start: ::core::option::Option<u32>,
    
    #[prost(message, optional, tag = "2")]
    pub category_end: ::core::option::Option<u32>,
    
    #[prost(message, optional, tag = "3")]
    pub start_time: ::core::option::Option<u32>,
    
    #[prost(message, optional, tag = "4")]
    pub end_time: ::core::option::Option<u32>,
    
    #[prost(message, optional, tag = "6")]
    pub limit: ::core::option::Option<i32>,
    
    #[prost(string, tag = "8")]
    pub cursor: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUserGroupsRequest {
    
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    
    #[prost(message, optional, tag = "2")]
    pub limit: ::core::option::Option<i32>,
    
    #[prost(message, optional, tag = "3")]
    pub state: ::core::option::Option<i32>,
    
    #[prost(string, tag = "4")]
    pub cursor: ::prost::alloc::string::String,
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
    
    #[prost(int32, tag = "5")]
    pub tick_rate: i32,
    
    #[prost(string, tag = "6")]
    pub handler_name: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchList {
    
    #[prost(message, repeated, tag = "1")]
    pub matches: ::prost::alloc::vec::Vec<Match>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Notification {
    
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    
    #[prost(string, tag = "2")]
    pub subject: ::prost::alloc::string::String,
    
    #[prost(string, tag = "3")]
    pub content: ::prost::alloc::string::String,
    
    #[prost(int32, tag = "4")]
    pub code: i32,
    
    #[prost(string, tag = "5")]
    pub sender_id: ::prost::alloc::string::String,
    
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    
    #[prost(bool, tag = "7")]
    pub persistent: bool,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationList {
    
    #[prost(message, repeated, tag = "1")]
    pub notifications: ::prost::alloc::vec::Vec<Notification>,
    
    #[prost(string, tag = "2")]
    pub cacheable_cursor: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PromoteGroupUsersRequest {
    
    #[prost(string, tag = "1")]
    pub group_id: ::prost::alloc::string::String,
    
    #[prost(string, repeated, tag = "2")]
    pub user_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DemoteGroupUsersRequest {
    
    #[prost(string, tag = "1")]
    pub group_id: ::prost::alloc::string::String,
    
    #[prost(string, repeated, tag = "2")]
    pub user_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadStorageObjectId {
    
    #[prost(string, tag = "1")]
    pub collection: ::prost::alloc::string::String,
    
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    
    #[prost(string, tag = "3")]
    pub user_id: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadStorageObjectsRequest {
    
    #[prost(message, repeated, tag = "1")]
    pub object_ids: ::prost::alloc::vec::Vec<ReadStorageObjectId>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rpc {
    
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    
    #[prost(string, tag = "2")]
    pub payload: ::prost::alloc::string::String,
    
    #[prost(string, tag = "3")]
    pub http_key: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Session {
    
    #[prost(bool, tag = "1")]
    pub created: bool,
    
    #[prost(string, tag = "2")]
    pub token: ::prost::alloc::string::String,
    
    #[prost(string, tag = "3")]
    pub refresh_token: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageObject {
    
    #[prost(string, tag = "1")]
    pub collection: ::prost::alloc::string::String,
    
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    
    #[prost(string, tag = "3")]
    pub user_id: ::prost::alloc::string::String,
    
    #[prost(string, tag = "4")]
    pub value: ::prost::alloc::string::String,
    
    #[prost(string, tag = "5")]
    pub version: ::prost::alloc::string::String,
    
    #[prost(int32, tag = "6")]
    pub permission_read: i32,
    
    #[prost(int32, tag = "7")]
    pub permission_write: i32,
    
    #[prost(message, optional, tag = "8")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    
    #[prost(message, optional, tag = "9")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageObjectAck {
    
    #[prost(string, tag = "1")]
    pub collection: ::prost::alloc::string::String,
    
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
    
    #[prost(string, tag = "4")]
    pub user_id: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageObjectAcks {
    
    #[prost(message, repeated, tag = "1")]
    pub acks: ::prost::alloc::vec::Vec<StorageObjectAck>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageObjects {
    
    #[prost(message, repeated, tag = "1")]
    pub objects: ::prost::alloc::vec::Vec<StorageObject>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageObjectList {
    
    #[prost(message, repeated, tag = "1")]
    pub objects: ::prost::alloc::vec::Vec<StorageObject>,
    
    #[prost(string, tag = "2")]
    pub cursor: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tournament {
    
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    
    #[prost(uint32, tag = "4")]
    pub category: u32,
    
    #[prost(uint32, tag = "5")]
    pub sort_order: u32,
    
    #[prost(uint32, tag = "6")]
    pub size: u32,
    
    #[prost(uint32, tag = "7")]
    pub max_size: u32,
    
    #[prost(uint32, tag = "8")]
    pub max_num_score: u32,
    
    #[prost(bool, tag = "9")]
    pub can_enter: bool,
    
    #[prost(uint32, tag = "10")]
    pub end_active: u32,
    
    #[prost(uint32, tag = "11")]
    pub next_reset: u32,
    
    #[prost(string, tag = "12")]
    pub metadata: ::prost::alloc::string::String,
    
    #[prost(message, optional, tag = "13")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    
    #[prost(message, optional, tag = "14")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    
    #[prost(message, optional, tag = "15")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    
    #[prost(uint32, tag = "16")]
    pub duration: u32,
    
    #[prost(uint32, tag = "17")]
    pub start_active: u32,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TournamentList {
    
    #[prost(message, repeated, tag = "1")]
    pub tournaments: ::prost::alloc::vec::Vec<Tournament>,
    
    #[prost(string, tag = "2")]
    pub cursor: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TournamentRecordList {
    
    #[prost(message, repeated, tag = "1")]
    pub records: ::prost::alloc::vec::Vec<LeaderboardRecord>,
    
    #[prost(message, repeated, tag = "2")]
    pub owner_records: ::prost::alloc::vec::Vec<LeaderboardRecord>,
    
    #[prost(string, tag = "3")]
    pub next_cursor: ::prost::alloc::string::String,
    
    #[prost(string, tag = "4")]
    pub prev_cursor: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAccountRequest {
    
    #[prost(message, optional, tag = "1")]
    pub username: ::core::option::Option<::prost::alloc::string::String>,
    
    #[prost(message, optional, tag = "2")]
    pub display_name: ::core::option::Option<::prost::alloc::string::String>,
    
    #[prost(message, optional, tag = "3")]
    pub avatar_url: ::core::option::Option<::prost::alloc::string::String>,
    
    #[prost(message, optional, tag = "4")]
    pub lang_tag: ::core::option::Option<::prost::alloc::string::String>,
    
    #[prost(message, optional, tag = "5")]
    pub location: ::core::option::Option<::prost::alloc::string::String>,
    
    #[prost(message, optional, tag = "6")]
    pub timezone: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGroupRequest {
    
    #[prost(string, tag = "1")]
    pub group_id: ::prost::alloc::string::String,
    
    #[prost(message, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    
    #[prost(message, optional, tag = "3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    
    #[prost(message, optional, tag = "4")]
    pub lang_tag: ::core::option::Option<::prost::alloc::string::String>,
    
    #[prost(message, optional, tag = "5")]
    pub avatar_url: ::core::option::Option<::prost::alloc::string::String>,
    
    #[prost(message, optional, tag = "6")]
    pub open: ::core::option::Option<bool>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    
    #[prost(string, tag = "2")]
    pub username: ::prost::alloc::string::String,
    
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    
    #[prost(string, tag = "4")]
    pub avatar_url: ::prost::alloc::string::String,
    
    #[prost(string, tag = "5")]
    pub lang_tag: ::prost::alloc::string::String,
    
    #[prost(string, tag = "6")]
    pub location: ::prost::alloc::string::String,
    
    #[prost(string, tag = "7")]
    pub timezone: ::prost::alloc::string::String,
    
    #[prost(string, tag = "8")]
    pub metadata: ::prost::alloc::string::String,
    
    #[prost(string, tag = "9")]
    pub facebook_id: ::prost::alloc::string::String,
    
    #[prost(string, tag = "10")]
    pub google_id: ::prost::alloc::string::String,
    
    #[prost(string, tag = "11")]
    pub gamecenter_id: ::prost::alloc::string::String,
    
    #[prost(string, tag = "12")]
    pub steam_id: ::prost::alloc::string::String,
    
    #[prost(bool, tag = "13")]
    pub online: bool,
    
    #[prost(int32, tag = "14")]
    pub edge_count: i32,
    
    #[prost(message, optional, tag = "15")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    
    #[prost(message, optional, tag = "16")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    
    #[prost(string, tag = "17")]
    pub facebook_instant_game_id: ::prost::alloc::string::String,
    
    #[prost(string, tag = "18")]
    pub apple_id: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserGroupList {
    
    #[prost(message, repeated, tag = "1")]
    pub user_groups: ::prost::alloc::vec::Vec<user_group_list::UserGroup>,
    
    #[prost(string, tag = "2")]
    pub cursor: ::prost::alloc::string::String,
}

pub mod user_group_list {
    
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UserGroup {
        
        #[prost(message, optional, tag = "1")]
        pub group: ::core::option::Option<super::Group>,
        
        #[prost(message, optional, tag = "2")]
        pub state: ::core::option::Option<i32>,
    }
    
    pub mod user_group {
        
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum State {
            
            Superadmin = 0,
            
            Admin = 1,
            
            Member = 2,
            
            JoinRequest = 3,
        }
    }
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Users {
    
    #[prost(message, repeated, tag = "1")]
    pub users: ::prost::alloc::vec::Vec<User>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteLeaderboardRecordRequest {
    
    #[prost(string, tag = "1")]
    pub leaderboard_id: ::prost::alloc::string::String,
    
    #[prost(message, optional, tag = "2")]
    pub record: ::core::option::Option<write_leaderboard_record_request::LeaderboardRecordWrite>,
}

pub mod write_leaderboard_record_request {
    
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LeaderboardRecordWrite {
        
        #[prost(int64, tag = "1")]
        pub score: i64,
        
        #[prost(int64, tag = "2")]
        pub subscore: i64,
        
        #[prost(string, tag = "3")]
        pub metadata: ::prost::alloc::string::String,
    }
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteStorageObject {
    
    #[prost(string, tag = "1")]
    pub collection: ::prost::alloc::string::String,
    
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    
    #[prost(string, tag = "3")]
    pub value: ::prost::alloc::string::String,
    
    
    
    #[prost(string, tag = "4")]
    pub version: ::prost::alloc::string::String,
    
    #[prost(message, optional, tag = "5")]
    pub permission_read: ::core::option::Option<i32>,
    
    #[prost(message, optional, tag = "6")]
    pub permission_write: ::core::option::Option<i32>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteStorageObjectsRequest {
    
    #[prost(message, repeated, tag = "1")]
    pub objects: ::prost::alloc::vec::Vec<WriteStorageObject>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteTournamentRecordRequest {
    
    #[prost(string, tag = "1")]
    pub tournament_id: ::prost::alloc::string::String,
    
    #[prost(message, optional, tag = "2")]
    pub record: ::core::option::Option<write_tournament_record_request::TournamentRecordWrite>,
}

pub mod write_tournament_record_request {
    
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TournamentRecordWrite {
        
        #[prost(int64, tag = "1")]
        pub score: i64,
        
        #[prost(int64, tag = "2")]
        pub subscore: i64,
        
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
