pub use achievement::*;
pub use calabash::*;
pub use chat::*;
pub use combat::*;
pub use dummy::*;
pub use entity::*;
pub use friend::*;
pub use gacha::*;
pub use guide::*;
pub use lord_gym::*;
pub use mail::*;
pub use map::*;
pub use misc::*;
pub use role::*;
pub use scene::*;
use wicked_waifus_protocol::message::Message;
pub use skill::*;
pub use teleport::*;
pub use tutorial::*;

mod achievement;
mod calabash;
mod chat;
mod combat;
mod dummy;
mod entity;
mod friend;
mod gacha;
mod guide;
mod lord_gym;
mod mail;
mod map;
mod misc;
mod role;
mod scene;
mod skill;
mod teleport;
mod tutorial;

macro_rules! handle_request {
    ($($name:ident $(, $inner_package:ident)?;)*) => {
        fn handle_request(player: &mut super::player::Player, mut msg: Message) {
            use ::wicked_waifus_protocol::{MessageID, Protobuf};

            ::paste::paste! {
                match msg.get_message_id() {
                    $(
                        ::wicked_waifus_protocol::$($inner_package::)?[<$name Request>]::MESSAGE_ID => {
                            let Ok(request) = ::wicked_waifus_protocol::$($inner_package::)?[<$name Request>]::decode(&*msg.remove_payload()) else {
                                tracing::debug!("failed to decode {}, player_id: {}", stringify!($($inner_package::)?[<$name Request>]), player.basic_info.id);
                                return;
                            };

                            tracing::debug!("logic: processing request {}", stringify!($($inner_package::)?[<$name Request>]));

                            let mut response = ::wicked_waifus_protocol::$($inner_package::)?[<$name Response>]::default();
                            [<on_ $($inner_package:snake _)? $name:snake _request>](player, request, &mut response);

                            player.respond(response, msg.get_rpc_id());
                        },
                    )*
                    unhandled => {
                         ::tracing::warn!("can't find handler for request with message_id={unhandled}");
                         let tmp = &*msg.remove_payload();
                         let (name, value) = wicked_waifus_protocol::proto_dumper::get_debug_info(
                             unhandled, tmp,
                         ).unwrap_or_else(|err| ("Error", err.to_string()));
                        tracing::debug!("trying to log unhandled data for message {name} with:\n{value}")
                    }
                }
            }
        }
    };
}

macro_rules! handle_push {
    ($($name:ident $(, $inner_package:ident)?;)*) => {
        fn handle_push(player: &mut super::player::Player, mut msg: Message) {
            use ::wicked_waifus_protocol::{MessageID, Protobuf};

            ::paste::paste! {
                match msg.get_message_id() {
                    $(
                        ::wicked_waifus_protocol::$($inner_package::)?[<$name Push>]::MESSAGE_ID => {
                            let Ok(push) = ::wicked_waifus_protocol::$($inner_package::)?[<$name Push>]::decode(&*msg.remove_payload()) else {
                                tracing::debug!("failed to decode {}, player_id: {}", stringify!($($inner_package::)?[<$name Push>]), player.basic_info.id);
                                return;
                            };

                            tracing::debug!("logic: processing push {}", stringify!($($inner_package::)?[<$name Push>]));

                            [<on_ $($inner_package:snake _)? $name:snake _push>](player, push);
                        },
                    )*
                    unhandled => {
                         ::tracing::warn!("can't find handler for push with message_id={unhandled}");
                         let tmp = &*msg.remove_payload();
                         let (name, value) = wicked_waifus_protocol::proto_dumper::get_debug_info(
                             unhandled, tmp,
                         ).unwrap_or_else(|err| ("Error", err.to_string()));
                        tracing::debug!("trying to log unhandled data for message {name} with:\n{value}")
                    }
                }
            }
        }
    };
}

handle_request! {

    // Achievement
    AchievementInfo;
    UpdateAchievementInfo;

    // Calabash
    IllustratedInfo;

    // Chat  (TODO: Review TODOs)
    PrivateChat;
    PrivateChatOperate;
    PrivateChatHistory;

    // Combat (TODO: Review this on_..., port some from go)
    CombatSendPack, combat_message;
    // CombatMessagePostInfo, combat_message; // TODO: Review this niggerianism, Encrypted shadow data

    // Friend (TODO: Implement them)
    FriendApplySend;
    FriendRecentlyTeam;
    PlayerBasicInfoGet;

    // Entity (TODO: Review this on_..., port some from go)
    EntityActive;
    EntityOnLanded;
    EntityPosition;
    EntityLoadComplete;

    // Gacha
    Gacha;
    GachaInfo;
    GachaUsePool;

    // Guide
    GuideInfo;
    GuideTrigger;

    // Lord Gym (TODO: Review this on_..., port some from go)
    LordGymInfo;

    // Mail
    MailBindInfo;

    // Map
    DarkCoastDelivery;
    MapUnlockFieldInfo;
    // LevelPlayStateListAsyncRequest // Example: "x9l": [{"inst_id": 902,"level_play_ids": [166700009,157700000]}]

    // Misc (TODO: Review this on_..., port some from go)
    // Advice;
    InputSetting;
    InputSettingUpdate;
    LanguageSettingUpdate;
    ServerPlayStationPlayOnlyState;

    // Player (TODO: Review this on_..., port some from go)
    // PlayerMotion;
    // ModifySignature;
    // ModifyName;
    // ChangeHeadPhoto;

    // Role (TODO: Review this on_..., port some from go)
    RoleShowListUpdate;
    ClientCurrentRoleReport;
    RoleFavorList;
    FormationAttr;
    UpdateFormation;

    // Scene (TODO: Review this on_..., port some from go)
    SceneTrace;
    SceneLoadingFinish;
    UpdateSceneDate;
    AccessPathTimeServerConfig;
    UnlockRoleSkinList;
    PlayerHeadData;

    // Shop (TODO: Review this on_..., port some from go)
    // PayInfo;
    // PayGiftInfo;
    // PayShopItemUpdate;
    // PayShopInfo;
    // PayShopUpdate;
    // MonthCard;

    // Skill (TODO: Review this on_..., port some from go)
    VisionExploreSkillSet;
    ExploreSkillRouletteSet;
    // RoleActivateSkill;

    // Teleport
    TeleportData;
    TeleportTransfer;
    TeleportFinish;

    // Tower (TODO: Review this on_..., port some from go)
    // TowerSeasonUpdate;
    // Tower;

    // Tutorial
    TutorialInfo;
    // TutorialUnlock;

    // TODO: Implement all this properly, workaround for game enter
    // Role
    RoleVisionRecommendData;
    RoleVisionRecommendAttr;
    PlayerMotion;

    // Formation
    GetFormationData;

    // Misc
    FishingData;
    EnergySync;
    GetDetectionLabelInfo;
    MonthCard;
    InfluenceInfo;
    ForgeInfo;
    ExchangeReward;
    Liveness;
    WebSign;
    PhotoMemory;
    WeaponSkin;
    VisionEquipGroupInfo;
    UpdatePlayStationBlockAccount;
    AdventureManual;
    MapTraceInfo;
    Tower;
    ExploreProgress;
    ReportData;
    UpdateVoxelEnv;
    SimpleTrackReportAsync;
    TowerSeasonUpdate;
    
    EntityPatrolStop;
    PayShopInfo;
    InitRange;
    Activity;
    BattlePass;
    SlashAndTowerInfo;
    Advice;

    // Friend
    FriendAll;

    // Inventory
    NormalItem;
    WeaponItem;
    PhantomItem;
    ValidTimeItem;
    ItemExchangeInfo;
}

handle_push! {
    // Entity
    MovePackage;

    // Misc
    VersionInfo;
}

pub fn handle_logic_message(player: &mut super::player::Player, msg: Message) {
    match msg {
        Message::Request { .. } => handle_request(player, msg),
        Message::Push { .. } => handle_push(player, msg),
        _ => tracing::warn!(
            "handle_logic_message: wrong message type: {}, message_id: {}, player_id: {}",
            msg.get_message_type(),
            msg.get_message_id(),
            player.basic_info.id,
        ),
    }
}
