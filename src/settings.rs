/* V Rising Server Settings Editor
 * Copyright (C) 2025 LordEvilthe666
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy, EnumIter, Display)]
#[repr(u8)]
pub enum GameDifficulty {
    Relaxed = 0,
    Normal = 1,
    Hard = 2,
}
impl Default for GameDifficulty {
    fn default() -> Self {
        GameDifficulty::Normal
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy, EnumIter, Display)]
#[repr(u8)]
pub enum GameModeType {
    PvE = 0,
    PvP = 1,
}
impl Default for GameModeType {
    fn default() -> Self {
        GameModeType::PvE
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy, EnumIter, Display)]
#[repr(u8)]
pub enum CastleDamageMode {
    Never = 0,
    Always = 1,
    TimeRestricted = 2,
}
impl Default for CastleDamageMode {
    fn default() -> Self {
        CastleDamageMode::Never
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy, EnumIter, Display)]
#[repr(u8)]
pub enum SiegeWeaponHealth {
    VeryLow = 0,
    Low = 1,
    Normal = 2,
    High = 3,
    VeryHigh = 4,
    MegaHigh = 5,
    UltraHigh = 6,
    CrazyHigh = 7,
    Max = 8,
}
impl Default for SiegeWeaponHealth {
    fn default() -> Self {
        SiegeWeaponHealth::Normal
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy, EnumIter, Display)]
#[repr(u8)]
pub enum PlayerDamageMode {
    Always = 0,
    TimeRestricted = 1,
}
impl Default for PlayerDamageMode {
    fn default() -> Self {
        PlayerDamageMode::Always
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy, EnumIter, Display)]
#[repr(u8)]
pub enum CastleHeartDamageMode {
    CanBeDestroyedOnlyWhenDecaying = 0,
    CanBeDestroyedByPlayers = 1,
    CanBeSeizedOrDestroyedByPlayers = 2,
}
impl Default for CastleHeartDamageMode {
    fn default() -> Self {
        CastleHeartDamageMode::CanBeDestroyedOnlyWhenDecaying
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy, EnumIter, Display)]
#[repr(u8)]
pub enum PvPProtectionMode {
    Disabled = 0,
    VeryShort = 1,
    Short = 2,
    Medium = 3,
    Long = 4,
}
impl Default for PvPProtectionMode {
    fn default() -> Self {
        PvPProtectionMode::Medium
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy, EnumIter, Display)]
#[repr(u8)]
pub enum DeathContainerPermission {
    Anyone = 0,
    ClanMembers = 1,
    OnlySelf = 2,
}
impl Default for DeathContainerPermission {
    fn default() -> Self {
        DeathContainerPermission::ClanMembers
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy, EnumIter, Display)]
#[repr(u8)]
pub enum RelicSpawnType {
    Unique = 0,
    Plentiful = 1,
}
impl Default for RelicSpawnType {
    fn default() -> Self {
        RelicSpawnType::Unique
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy, EnumIter, Display)]
#[repr(i32)]
pub enum StarterEquipmentId {
    None = 0,
    Copper = 742198603,
    MercilessCopper = -663535879,
    Iron = 688096336,
    MercilessIron = -1502721803,
    DarkSilver = 28431735,
    Sanguine = -983090495,
    Dracula = -1466803079,
}
impl Default for StarterEquipmentId {
    fn default() -> Self {
        StarterEquipmentId::None
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy, EnumIter, Display)]
#[repr(i32)]
pub enum StarterResourcesId {
    None = 0,
    Level30 = 1982471388,
    Level40 = 1504234317,
    Level50 = 548330870,
    Level60 = 815373441,
    Level70 = -1370930855,
    Level80 = -1394108841,
}
impl Default for StarterResourcesId {
    fn default() -> Self {
        StarterResourcesId::None
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Copy, EnumIter, Display)]
#[repr(i32)]
pub enum VBloodUnitId {
    #[strum(to_string = "Alpha the White Wolf")]
    AlphaWolf = -1905691330,
    #[strum(to_string = "Keely the Frost Archer")]
    KeelyFrostArcher = 1124739990,
    #[strum(to_string = "Errol the Stonebreaker")]
    ErrolStonebreaker = -2025101517,
    #[strum(to_string = "Rufus the Foreman")]
    RufusForeman = 2122229952,
    #[strum(to_string = "Grayson the Armourer")]
    GraysonArmourer = 1106149033,
    #[strum(to_string = "Goreswine the Ravager")]
    GoreswineRavager = 577478542,
    #[strum(to_string = "Lidia the Chaos Archer")]
    LidiaChaosArcher = 763273073,
    #[strum(to_string = "Clive the Firestarter")]
    CliveFirestarter = 1896428751,
    #[strum(to_string = "Nibbles the Putrid Rat")]
    NibblesPutridRat = -2039908510,
    #[strum(to_string = "Finn the Fisherman")]
    FinnFisherman = -2122682556,
    #[strum(to_string = "Polora the Feywalker")]
    PoloraFeywalker = -484556888,
    #[strum(to_string = "Kodia the Ferocious Bear")]
    KodiaFerociousBear = -1391546313,
    #[strum(to_string = "Nicholaus the Fallen")]
    NicholausFallen = 153390636,
    #[strum(to_string = "Quincey the Bandit King")]
    QuinceyBanditKing = -1659822956,
    #[strum(to_string = "Beatrice the Tailor")]
    BeatriceTailor = -1942352521,
    #[strum(to_string = "Vincent the Frostbringer")]
    VincentFrostbringer = -29797003,
    #[strum(to_string = "Christina the Sun Priestess")]
    ChristinaSunPriestess = -99012450,
    #[strum(to_string = "Tristan the Vampire Hunter")]
    TristanVampireHunter = -1449631170,
    #[strum(to_string = "Sir Erwin the Gallant Cavalier")]
    SirErwinGallantCavalier = 619948378,
    #[strum(to_string = "Kriig the Undead General")]
    KriigUndeadGeneral = -1365931036,
    #[strum(to_string = "Leandra the Shadow Priestess")]
    LeandraShadowPriestess = 939467639,
    #[strum(to_string = "Maja the Dark Savant")]
    MajaDarkSavant = 1945956671,
    #[strum(to_string = "Bane the Shadowblade")]
    BaneShadowblade = 613251918,
    #[strum(to_string = "Grethel the Glassblower")]
    GrethelGlassblower = 910988233,
    #[strum(to_string = "Meredith the Bright Archer")]
    MeredithBrightArcher = 850622034,
    #[strum(to_string = "Terah the Geomancer")]
    TerahGeomancer = -1065970933,
    #[strum(to_string = "Frostmaw the Mountain Terror")]
    FrostmawMountainTerror = 24378719,
    #[strum(to_string = "General Elena the Hollow")]
    GeneralElenaHollow = 795262842,
    #[strum(to_string = "Gaius the Cursed Champion")]
    GaiusCursedChampion = -753453016,
    #[strum(to_string = "General Cassius the Betrayer")]
    GeneralCassiusBetrayer = -496360395,
    #[strum(to_string = "Jade the Vampire Hunter")]
    JadeVampireHunter = -1968372384,
    #[strum(to_string = "Raziel the Shepherd")]
    RazielShepherd = -680831417,
    #[strum(to_string = "Octavian the Militia Captain")]
    OctavianMilitiaCaptain = 1688478381,
    #[strum(to_string = "Ziva the Engineer")]
    ZivaEngineer = 172235178,
    #[strum(to_string = "Domina the Blade Dancer")]
    DominaBladeDancer = -1101874342,
    #[strum(to_string = "Angram the Purifier")]
    AngramPurifier = 106480588,
    #[strum(to_string = "Ungora the Spider Queen")]
    UngoraSpiderQueen = -548489519,
    #[strum(to_string = "Ben the Old Wanderer")]
    BenOldWanderer = 109969450,
    #[strum(to_string = "Foulrot the Soultaker")]
    FoulrotSoultaker = -1208888966,
    #[strum(to_string = "Albert the Duke of Balaton")]
    AlbertDukeOfBalaton = -203043163,
    #[strum(to_string = "Willfred the Village Elder")]
    WillfredVillageElder = -1505705712,
    #[strum(to_string = "Cyril the Cursed Smith")]
    CyrilCursedSmith = 326378955,
    #[strum(to_string = "Sir Magnus the Overseer")]
    SirMagnusOverseer = -26105228,
    #[strum(to_string = "Baron du Bouchon the Sommelier")]
    BaronDuBouchonSommelier = 192051202,
    #[strum(to_string = "Morian the Stormwing Matriarch")]
    MorianStormwingMatriarch = 685266977,
    #[strum(to_string = "Mairwyn the Elementalist")]
    MairwynElementalist = -2013903325,
    #[strum(to_string = "Henry Blackbrew the Doctor")]
    HenryBlackbrewDoctor = 814083983,
    #[strum(to_string = "Jakira the Shadow Huntress")]
    JakiraShadowHuntress = -1383529374,
    #[strum(to_string = "Stavros the Carver")]
    StavrosCarver = -1669199769,
    #[strum(to_string = "Lucile the Venom Alchemist")]
    LucileVenomAlchemist = 1295855316,
    #[strum(to_string = "Matka the Curse Weaver")]
    MatkaCurseWeaver = -910296704,
    #[strum(to_string = "Terrorclaw the Ogre")]
    TerrorclawOgre = -1347412392,
    #[strum(to_string = "Azariel the Sunbringer")]
    AzarielSunbringer = 114912615,
    #[strum(to_string = "Voltatia the Power Master")]
    VoltatiaPowerMaster = 2054432370,
    #[strum(to_string = "Simon Belmont the Vampire Hunter")]
    SimonBelmontVampireHunter = 336560131,
    #[strum(to_string = "Dantos the Forgebinder")]
    DantosForgebinder = 173259239,
    #[strum(to_string = "Lord Styx the Night Champion")]
    LordStyxNightChampion = 1112948824,
    #[strum(to_string = "Gorecrusher the Behemoth")]
    GorecrusherBehemoth = -1936575244,
    #[strum(to_string = "General Valencia the Hunter")]
    GeneralValenciaHunter = 495971434,
    #[strum(to_string = "Solarus the Immaculate")]
    SolarusImmaculate = -740796338,
    #[strum(to_string = "Talzur the Winged Horror")]
    TalzurWingedHorror = -393555055,
    #[strum(to_string = "Megara the Serpent Queen")]
    MegaraSerpentQueen = 591725925,
    #[strum(to_string = "Adam the Firstborn")]
    AdamFirstborn = 1233988687,
    #[strum(to_string = "Dracula the Immortal King")]
    DraculaImmortalKing = -327335305,
}
impl Default for VBloodUnitId {
    fn default() -> Self {
        VBloodUnitId::AlphaWolf
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Copy, EnumIter, Display)]
#[repr(i32)]
pub enum AchievementId {
    #[strum(to_string = "Collecting the Remains")]
    CollectingTheRemains = -1770927128,
    #[strum(to_string = "Wielding the Sword")]
    WieldingTheSword = 436375429,
    #[strum(to_string = "Mastering Magic")]
    MasteringMagic = -1400391027,
    #[strum(to_string = "Defensive Measures")]
    DefensiveMeasures = -2102083739,
    #[strum(to_string = "Hides of the Wild")]
    HidesOfTheWild = 1566228114,
    #[strum(to_string = "Into the Woods")]
    IntoTheWoods = 1695239324,
    #[strum(to_string = "Gathering")]
    Gathering = -54280488,
    #[strum(to_string = "Lord of Shadows")]
    LordOfShadows = 1694767961,
    #[strum(to_string = "Fortify")]
    Fortify = -1899098914,
    #[strum(to_string = "Shelter")]
    Shelter = -122882616,
    #[strum(to_string = "Getting Ready for the Hunt")]
    GettingReadyForTheHunt = 560247139,
    #[strum(to_string = "Blood Hunt")]
    BloodHunt = -1995132640,
    #[strum(to_string = "Thirst for Power")]
    ThirstForPower = -302458684,
    #[strum(to_string = "The first book in the Library")]
    TheFirstBookInTheLibrary = -1434604634,
    #[strum(to_string = "Expanding my Domain")]
    ExpandingMyDomain = 1668809517,
    #[strum(to_string = "Building a Castle")]
    BuildingACastle = 334973636,
    #[strum(to_string = "Waygate")]
    Waygate = 134993992,
    #[strum(to_string = "Lord of the Manor")]
    LordOfTheManor = 606418711,
    #[strum(to_string = "Servants")]
    Servants = -892747762,
    #[strum(to_string = "Army of Darkness")]
    ArmyOfDarkness = -437605270,
    #[strum(to_string = "Broaden Horizons")]
    BroadenHorizons = -1472413073,
    #[strum(to_string = "Blood on Tap")]
    BloodOnTap = 1248242594,
    #[strum(to_string = "Throne of Command")]
    ThroneOfCommand = -327597689,
    #[strum(to_string = "Reign Supreme")]
    ReignSupreme = 149111189,
    #[strum(to_string = "An Eye into Mortium")]
    AnEyeIntoMortium = -452204266,
    #[strum(to_string = "A Castle reaching the Sky")]
    ACastleReachingTheSky = 1805684941,
    #[strum(to_string = "Nightfall Steed")]
    NightfallSteed = -699165894,
    #[strum(to_string = "Vampire Empire")]
    VampireEmpire = 1861267375,
    #[strum(to_string = "Soul Stones")]
    SoulStones = -2104585843,
    #[strum(to_string = "Lord of the Night")]
    LordOfTheNight = 1762480233,
}
impl Default for AchievementId {
    fn default() -> Self {
        AchievementId::CollectingTheRemains
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Copy, EnumIter, Display)]
#[repr(i32)]
pub enum ResearchId {
    #[strum(to_string = "Tier 1")]
    Tier1 = -495424062,
    #[strum(to_string = "Tier 2")]
    Tier2 = -1292809886,
    #[strum(to_string = "Tier 3")]
    Tier3 = -1262194203,
}
impl Default for ResearchId {
    fn default() -> Self {
        ResearchId::Tier1
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy, EnumIter, Display)]
#[serde(rename_all = "PascalCase")]
#[repr(u8)]
pub enum CastleHeartLimitType {
    User,
    Clan,
}
impl Default for CastleHeartLimitType {
    fn default() -> Self {
        CastleHeartLimitType::User
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy, EnumIter, Display)]
#[serde(rename_all = "PascalCase")]
pub enum TimeZone {
    Local,
    UTC,
    PST,
    EST,
    CET,
    CST,
}
impl Default for TimeZone {
    fn default() -> Self {
        TimeZone::UTC
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy, EnumIter, Display)]
pub enum WarEventInterval {
    Minimum,
    VeryShort,
    Short,
    Medium,
    Long,
    VeryLong,
    Extensive,
    Maximum,
}
impl Default for WarEventInterval {
    fn default() -> Self {
        WarEventInterval::Medium
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy, EnumIter, Display)]
pub enum WarEventDuration {
    Minimum,
    VeryShort,
    Short,
    Medium,
    Long,
    VeryLong,
    Extensive,
    Maximum,
}
impl Default for WarEventDuration {
    fn default() -> Self {
        WarEventDuration::Medium
    }
}

// --- Вложенные структуры ---

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct VBloodUnitSetting {
    #[serde(rename = "UnitId")]
    pub unit_id: VBloodUnitId,
    pub unit_level: u8,
    pub default_unlocked: bool,
}

impl Default for VBloodUnitSetting {
    fn default() -> Self {
        Self {
            unit_id: VBloodUnitId::default(),
            unit_level: 0,
            default_unlocked: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
#[serde(rename_all = "PascalCase")]
pub struct GameTimeModifiers {
    pub day_duration_in_seconds: f32,
    pub day_start_hour: u8,
    pub day_start_minute: u8,
    pub day_end_hour: u8,
    pub day_end_minute: u8,
    pub blood_moon_frequency_min: u8,
    pub blood_moon_frequency_max: u8,
    pub blood_moon_buff: f32,
}
impl Default for GameTimeModifiers {
    fn default() -> Self {
        Self {
            day_duration_in_seconds: 1080.0,
            day_start_hour: 9,
            day_start_minute: 0,
            day_end_hour: 17,
            day_end_minute: 0,
            blood_moon_frequency_min: 10,
            blood_moon_frequency_max: 18,
            blood_moon_buff: 0.2,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
#[serde(rename_all = "PascalCase")]
pub struct VampireStatModifiers {
    pub max_health_modifier: f32,
    pub physical_power_modifier: f32,
    pub spell_power_modifier: f32,
    pub resource_power_modifier: f32,
    pub siege_power_modifier: f32,
    pub damage_received_modifier: f32,
    pub revive_cancel_delay: f32,
}
impl Default for VampireStatModifiers {
    fn default() -> Self {
        Self {
            max_health_modifier: 1.0,
            physical_power_modifier: 1.0,
            spell_power_modifier: 1.0,
            resource_power_modifier: 1.0,
            siege_power_modifier: 1.0,
            damage_received_modifier: 1.0,
            revive_cancel_delay: 5.0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
#[serde(rename_all = "PascalCase")]
pub struct UnitStatModifiers {
    pub max_health_modifier: f32,
    pub power_modifier: f32,
    pub level_increase: u8,
}
impl Default for UnitStatModifiers {
    fn default() -> Self {
        Self {
            max_health_modifier: 1.0,
            power_modifier: 1.0,
            level_increase: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
#[serde(rename_all = "PascalCase")]
pub struct EquipmentStatModifiers {
    pub max_health_modifier: f32,
    pub resource_yield_modifier: f32,
    pub physical_power_modifier: f32,
    pub spell_power_modifier: f32,
    pub siege_power_modifier: f32,
    pub movement_speed_modifier: f32,
}
impl Default for EquipmentStatModifiers {
    fn default() -> Self {
        Self {
            max_health_modifier: 1.0,
            resource_yield_modifier: 1.0,
            physical_power_modifier: 1.0,
            spell_power_modifier: 1.0,
            siege_power_modifier: 1.0,
            movement_speed_modifier: 1.0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
#[serde(rename_all = "PascalCase")]
pub struct HeartLevelLimit {
    pub floor_limit: i16,
    pub servant_limit: u8,
    #[serde(default = "default_build_limits")]
    pub build_limits: u8,
    pub height_limit: u8,
}
fn default_build_limits() -> u8 {
    2
}
impl Default for HeartLevelLimit {
    fn default() -> Self {
        Self {
            floor_limit: 40,
            servant_limit: 4,
            build_limits: default_build_limits(),
            height_limit: 3,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
#[serde(rename_all = "PascalCase")]
pub struct HeartLimits {
    #[serde(rename = "Level1")]
    pub level1: HeartLevelLimit,
    #[serde(rename = "Level2")]
    pub level2: HeartLevelLimit,
    #[serde(rename = "Level3")]
    pub level3: HeartLevelLimit,
    #[serde(rename = "Level4")]
    pub level4: HeartLevelLimit,
    #[serde(rename = "Level5")]
    pub level5: HeartLevelLimit,
}
impl Default for HeartLimits {
    fn default() -> Self {
        Self {
            level1: HeartLevelLimit {
                floor_limit: 40,
                servant_limit: 4,
                build_limits: 2,
                height_limit: 3,
            },
            level2: HeartLevelLimit {
                floor_limit: 100,
                servant_limit: 5,
                build_limits: 2,
                height_limit: 3,
            },
            level3: HeartLevelLimit {
                floor_limit: 180,
                servant_limit: 6,
                build_limits: 2,
                height_limit: 3,
            },
            level4: HeartLevelLimit {
                floor_limit: 260,
                servant_limit: 7,
                build_limits: 2,
                height_limit: 3,
            },
            level5: HeartLevelLimit {
                floor_limit: 420,
                servant_limit: 8,
                build_limits: 2,
                height_limit: 3,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
#[serde(rename_all = "PascalCase")]
pub struct CastleStatModifiers {
    pub tick_period: f32,
    pub safety_box_limit: u8,
    pub tomb_limit: u8,
    pub eye_structures_limit: u8,
    pub vermin_nest_limit: u8,
    pub prison_cell_limit: u8,
    pub heart_limits: HeartLimits,
    pub castle_limit: u8,
    pub nether_gate_limit: u8,
    pub throne_of_darkness_limit: u8,
    #[serde(default)]
    pub castle_heart_limit_type: Option<CastleHeartLimitType>,
}
impl Default for CastleStatModifiers {
    fn default() -> Self {
        Self {
            tick_period: 5.0,
            safety_box_limit: 1,
            tomb_limit: 12,
            eye_structures_limit: 1,
            vermin_nest_limit: 4,
            prison_cell_limit: 16,
            heart_limits: HeartLimits::default(),
            castle_limit: 2,
            nether_gate_limit: 1,
            throne_of_darkness_limit: 1,
            castle_heart_limit_type: Some(CastleHeartLimitType::default()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
#[serde(rename_all = "PascalCase")]
pub struct StartEndTimeData {
    pub start_hour: u8,
    pub start_minute: u8,
    pub end_hour: u8,
    pub end_minute: u8,
}
impl Default for StartEndTimeData {
    fn default() -> Self {
        Self {
            start_hour: 0,
            start_minute: 0,
            end_hour: 23,
            end_minute: 59,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
#[serde(rename_all = "PascalCase")]
pub struct PlayerInteractionSettings {
    pub time_zone: TimeZone,
    #[serde(rename = "VSPlayerWeekdayTime")]
    pub vs_player_weekday_time: StartEndTimeData,
    #[serde(rename = "VSPlayerWeekendTime")]
    pub vs_player_weekend_time: StartEndTimeData,
    #[serde(rename = "VSCastleWeekdayTime")]
    pub vs_castle_weekday_time: StartEndTimeData,
    #[serde(rename = "VSCastleWeekendTime")]
    pub vs_castle_weekend_time: StartEndTimeData,
}
impl Default for PlayerInteractionSettings {
    fn default() -> Self {
        Self {
            time_zone: TimeZone::default(),
            vs_player_weekday_time: StartEndTimeData {
                start_hour: 17,
                start_minute: 0,
                end_hour: 23,
                end_minute: 0,
            },
            vs_player_weekend_time: StartEndTimeData {
                start_hour: 17,
                start_minute: 0,
                end_hour: 23,
                end_minute: 0,
            },
            vs_castle_weekday_time: StartEndTimeData {
                start_hour: 17,
                start_minute: 0,
                end_hour: 23,
                end_minute: 0,
            },
            vs_castle_weekend_time: StartEndTimeData {
                start_hour: 17,
                start_minute: 0,
                end_hour: 23,
                end_minute: 0,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
#[serde(rename_all = "PascalCase")]
pub struct TraderModifiers {
    pub stock_modifier: f32,
    #[serde(rename = "PriceModifier.")]
    pub price_modifier: f32,
    pub restock_timer_modifier: f32,
}
impl Default for TraderModifiers {
    fn default() -> Self {
        Self {
            stock_modifier: 1.0,
            price_modifier: 1.0,
            restock_timer_modifier: 1.0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
#[serde(rename_all = "PascalCase")]
pub struct WarEventGameSettings {
    pub interval: WarEventInterval,
    pub major_duration: WarEventDuration,
    pub minor_duration: WarEventDuration,
    pub week_day_time: StartEndTimeData,
    pub weekend_time: StartEndTimeData,
    #[serde(default = "default_true")]
    pub enable_war_events: bool,
    #[serde(default = "default_true")]
    pub enable_incursions: bool,
    #[serde(default = "default_true")]
    pub enable_major_incursions: bool,
    #[serde(default = "default_true")]
    pub enable_minor_incursions: bool,
}
impl Default for WarEventGameSettings {
    fn default() -> Self {
        Self {
            interval: WarEventInterval::default(),
            major_duration: WarEventDuration::default(),
            minor_duration: WarEventDuration::default(),
            week_day_time: StartEndTimeData::default(),
            weekend_time: StartEndTimeData::default(),
            enable_war_events: true,
            enable_incursions: true,
            enable_major_incursions: true,
            enable_minor_incursions: true,
        }
    }
}

// --- Основная структура настроек ---

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Settings {
    pub game_difficulty: GameDifficulty,
    pub game_mode_type: GameModeType,
    pub castle_damage_mode: CastleDamageMode,
    pub siege_weapon_health: SiegeWeaponHealth,
    pub player_damage_mode: PlayerDamageMode,
    pub castle_heart_damage_mode: CastleHeartDamageMode,
    #[serde(rename = "PvPProtectionMode")]
    pub pvp_protection_mode: PvPProtectionMode,
    pub death_container_permission: DeathContainerPermission,
    pub relic_spawn_type: RelicSpawnType,
    pub can_loot_enemy_containers: bool,
    pub blood_bound_equipment: bool,
    pub teleport_bound_items: bool,
    #[serde(default)]
    pub bat_bound_items: bool,
    pub allow_global_chat: bool,
    pub all_waypoints_unlocked: bool,
    pub free_castle_raid: bool,
    pub free_castle_claim: bool,
    pub free_castle_destroy: bool,
    #[serde(default = "default_true")]
    pub castle_relocation_enabled: bool,
    pub inactivity_kill_enabled: bool,
    pub inactivity_kill_time_min: i32,
    pub inactivity_kill_time_max: i32,
    pub inactivity_kill_safe_time_addition: i32,
    pub inactivity_kill_timer_max_item_level: u8,
    pub disable_disconnected_dead_enabled: bool,
    pub disable_disconnected_dead_timer: i32,
    #[serde(default = "default_disconnected_sun_immunity")]
    pub disconnected_sun_immunity_time: f32,
    pub inventory_stacks_modifier: f32,
    #[serde(rename = "DropTableModifier_General")]
    pub drop_table_modifier_general: f32,
    #[serde(rename = "DropTableModifier_Missions")]
    pub drop_table_modifier_missions: f32,
    #[serde(default = "default_one_f32")]
    #[serde(rename = "DropTableModifier_StygianShards")]
    pub drop_table_modifier_stygian_shards: f32,
    #[serde(default = "default_one_f32")]
    #[serde(rename = "SoulShard_DurabilityLossRate")]
    pub soul_shard_durability_loss_rate: f32,
    #[serde(rename = "MaterialYieldModifier_Global")]
    pub material_yield_modifier_global: f32,
    pub blood_essence_yield_modifier: f32,
    #[serde(rename = "JournalVBloodSourceUnitMaxDistance")]
    pub journal_v_blood_source_unit_max_distance: f32,
    #[serde(rename = "PvPVampireRespawnModifier")]
    pub pvp_vampire_respawn_modifier: f32,
    pub castle_minimum_distance_in_floors: u8,
    pub clan_size: u8,
    pub blood_drain_modifier: f32,
    pub durability_drain_modifier: f32,
    pub garlic_area_strength_modifier: f32,
    pub holy_area_strength_modifier: f32,
    pub silver_strength_modifier: f32,
    pub sun_damage_modifier: f32,
    pub castle_decay_rate_modifier: f32,
    pub castle_blood_essence_drain_modifier: f32,
    pub castle_siege_timer: f32,
    pub castle_under_attack_timer: f32,
    pub castle_raid_timer: f32,
    #[serde(default = "default_castle_raid_protection")]
    pub castle_raid_protection_time: f32,
    #[serde(default = "default_castle_exposed_free_claim")]
    pub castle_exposed_free_claim_timer: f32,
    #[serde(default = "default_castle_relocation_cooldown")]
    pub castle_relocation_cooldown: f32,
    pub announce_siege_weapon_spawn: bool,
    pub show_siege_weapon_map_icon: bool,
    pub build_cost_modifier: f32,
    pub recipe_cost_modifier: f32,
    pub craft_rate_modifier: f32,
    pub research_cost_modifier: f32,
    pub refinement_cost_modifier: f32,
    pub refinement_rate_modifier: f32,
    pub research_time_modifier: f32,
    pub dismantle_resource_modifier: f32,
    pub servant_convert_rate_modifier: f32,
    pub repair_cost_modifier: f32,
    #[serde(rename = "Death_DurabilityFactorLoss")]
    pub death_durability_factor_loss: f32,
    #[serde(rename = "Death_DurabilityLossFactorAsResources")]
    pub death_durability_loss_factor_as_resources: f32,
    #[serde(rename = "StarterEquipmentId")]
    pub starter_equipment_id: StarterEquipmentId,
    #[serde(rename = "StarterResourcesId")]
    pub starter_resources_id: StarterResourcesId,
    #[serde(default)]
    pub starting_progression_level: u8,
    #[serde(rename = "VBloodUnitSettings")]
    #[serde(default)]
    pub v_blood_unit_settings: Vec<VBloodUnitSetting>,
    #[serde(rename = "UnlockedAchievements")]
    #[serde(default)]
    pub unlocked_achievements: Vec<AchievementId>,
    #[serde(rename = "UnlockedResearchs")]
    #[serde(default)]
    pub unlocked_researchs: Vec<ResearchId>,
    #[serde(rename = "GameTimeModifiers")]
    pub game_time_modifiers: GameTimeModifiers,
    #[serde(rename = "VampireStatModifiers")]
    pub vampire_stat_modifiers: VampireStatModifiers,
    #[serde(rename = "UnitStatModifiers_Global")]
    pub unit_stat_modifiers_global: UnitStatModifiers,
    #[serde(rename = "UnitStatModifiers_VBlood")]
    pub unit_stat_modifiers_v_blood: UnitStatModifiers,
    #[serde(rename = "EquipmentStatModifiers_Global")]
    pub equipment_stat_modifiers_global: EquipmentStatModifiers,
    #[serde(rename = "CastleStatModifiers_Global")]
    pub castle_stat_modifiers_global: CastleStatModifiers,
    #[serde(default)]
    #[serde(rename = "PlayerInteractionSettings")]
    pub player_interaction_settings: Option<PlayerInteractionSettings>,
    #[serde(rename = "TraderModifiers")]
    pub trader_modifiers: TraderModifiers,
    #[serde(default)]
    #[serde(rename = "WarEventGameSettings")]
    pub war_event_game_settings: Option<WarEventGameSettings>,
}

fn default_true() -> bool {
    true
}
fn default_one_f32() -> f32 {
    1.0
}
fn default_disconnected_sun_immunity() -> f32 {
    300.0
}
fn default_castle_raid_protection() -> f32 {
    1800.0
}
fn default_castle_exposed_free_claim() -> f32 {
    300.0
}
fn default_castle_relocation_cooldown() -> f32 {
    10800.0
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            game_difficulty: Default::default(),
            game_mode_type: Default::default(),
            castle_damage_mode: Default::default(),
            siege_weapon_health: Default::default(),
            player_damage_mode: Default::default(),
            castle_heart_damage_mode: Default::default(),
            pvp_protection_mode: Default::default(),
            death_container_permission: Default::default(),
            relic_spawn_type: Default::default(),
            can_loot_enemy_containers: true,
            blood_bound_equipment: true,
            teleport_bound_items: true,
            bat_bound_items: false,
            allow_global_chat: true,
            all_waypoints_unlocked: false,
            free_castle_raid: false,
            free_castle_claim: false,
            free_castle_destroy: false,
            castle_relocation_enabled: default_true(),
            inactivity_kill_enabled: true,
            inactivity_kill_time_min: 3600,
            inactivity_kill_time_max: 604800,
            inactivity_kill_safe_time_addition: 172800,
            inactivity_kill_timer_max_item_level: 84,
            disable_disconnected_dead_enabled: true,
            disable_disconnected_dead_timer: 60,
            disconnected_sun_immunity_time: default_disconnected_sun_immunity(),
            inventory_stacks_modifier: 1.0,
            drop_table_modifier_general: 1.0,
            drop_table_modifier_missions: 1.0,
            drop_table_modifier_stygian_shards: default_one_f32(),
            soul_shard_durability_loss_rate: default_one_f32(),
            material_yield_modifier_global: 1.0,
            blood_essence_yield_modifier: 1.0,
            journal_v_blood_source_unit_max_distance: 25.0,
            pvp_vampire_respawn_modifier: 1.0,
            castle_minimum_distance_in_floors: 2,
            clan_size: 4,
            blood_drain_modifier: 1.0,
            durability_drain_modifier: 1.0,
            garlic_area_strength_modifier: 1.0,
            holy_area_strength_modifier: 1.0,
            silver_strength_modifier: 1.0,
            sun_damage_modifier: 1.0,
            castle_decay_rate_modifier: 1.0,
            castle_blood_essence_drain_modifier: 1.0,
            castle_siege_timer: 420.0,
            castle_under_attack_timer: 60.0,
            castle_raid_timer: 600.0,
            castle_raid_protection_time: default_castle_raid_protection(),
            castle_exposed_free_claim_timer: default_castle_exposed_free_claim(),
            castle_relocation_cooldown: default_castle_relocation_cooldown(),
            announce_siege_weapon_spawn: true,
            show_siege_weapon_map_icon: false,
            build_cost_modifier: 1.0,
            recipe_cost_modifier: 1.0,
            craft_rate_modifier: 1.0,
            research_cost_modifier: 1.0,
            refinement_cost_modifier: 1.0,
            refinement_rate_modifier: 1.0,
            research_time_modifier: 1.0,
            dismantle_resource_modifier: 0.75,
            servant_convert_rate_modifier: 1.0,
            repair_cost_modifier: 1.0,
            death_durability_factor_loss: 0.125,
            death_durability_loss_factor_as_resources: 1.0,
            starter_equipment_id: Default::default(),
            starter_resources_id: Default::default(),
            starting_progression_level: 0,
            v_blood_unit_settings: Vec::new(),
            unlocked_achievements: Vec::new(),
            unlocked_researchs: Vec::new(),
            game_time_modifiers: Default::default(),
            vampire_stat_modifiers: Default::default(),
            unit_stat_modifiers_global: Default::default(),
            unit_stat_modifiers_v_blood: Default::default(),
            equipment_stat_modifiers_global: Default::default(),
            castle_stat_modifiers_global: Default::default(),
            player_interaction_settings: Some(Default::default()),
            trader_modifiers: Default::default(),
            war_event_game_settings: Some(Default::default()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ServerGameSettings {
    pub name: String,
    pub description: String,
    pub settings: Settings,
}

impl Default for ServerGameSettings {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            settings: Settings::default(),
        }
    }
}
