use crate::logic::player::Player;

macro_rules! dummy_handler {
    ($($type_name:ident;)*) => {

        $(::paste::paste! {
            use wicked_waifus_protocol::{[<$type_name Request>], [<$type_name Response>]};
        })*

        $(::paste::paste! {
            pub fn [<on_ $type_name:snake _request>](
                _player: &Player,
                _request: [<$type_name Request>],
                _response: &mut [<$type_name Response>],
            ) {}
        })*
    };
}

// TODO: implement this
dummy_handler! {
    RoleVisionRecommendData;
    RoleVisionRecommendAttr;
    PlayerMotion;
    GetFormationData;
    FishingData;
    EnergySync;
    TutorialInfo;
    MonthCard;
    InfluenceInfo;
    WebSign;
    PhotoMemory;
    WeaponSkin;
    VisionEquipGroupInfo;
    UpdatePlayStationBlockAccount;
    MapTraceInfo;
    Tower;
    ReportData;
    UpdateVoxelEnv;
    SimpleTrackReportAsync;
    TowerSeasonUpdate;
    FriendAll;
    NormalItem;
    WeaponItem;
    PhantomItem;
    ValidTimeItem;
    EntityPatrolStop;
    PayShopInfo;
    InitRange;
    Activity;
    BattlePass;
    SlashAndTowerInfo;
    Advice;
}