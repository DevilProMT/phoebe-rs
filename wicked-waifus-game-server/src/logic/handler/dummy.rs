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
    GetDetectionLabelInfo;
    TutorialInfo;
    MonthCard;
    InfluenceInfo;
    ForgeInfo;
    AchievementInfo;
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
    FriendAll;
    NormalItem;
    WeaponItem;
    PhantomItem;
    ValidTimeItem;
    ItemExchangeInfo;
}