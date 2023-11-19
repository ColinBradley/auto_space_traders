/*
 * SpaceTraders API
 *
 * SpaceTraders is an open-universe game and learning platform that offers a set of HTTP endpoints to control a fleet of ships and explore a multiplayer universe.  The API is documented using [OpenAPI](https://github.com/SpaceTradersAPI/api-docs). You can send your first request right here in your browser to check the status of the game server.  ```json http {   \"method\": \"GET\",   \"url\": \"https://api.spacetraders.io/v2\", } ```  Unlike a traditional game, SpaceTraders does not have a first-party client or app to play the game. Instead, you can use the API to build your own client, write a script to automate your ships, or try an app built by the community.  We have a [Discord channel](https://discord.com/invite/jh6zurdWk5) where you can share your projects, ask questions, and get help from other players.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: joel@spacetraders.io
 * Generated by: https://openapi-generator.tech
 */

/// TradeSymbol : The good's symbol.

/// The good's symbol.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TradeSymbol {
    #[serde(rename = "PRECIOUS_STONES")]
    PreciousStones,
    #[serde(rename = "QUARTZ_SAND")]
    QuartzSand,
    #[serde(rename = "SILICON_CRYSTALS")]
    SiliconCrystals,
    #[serde(rename = "AMMONIA_ICE")]
    AmmoniaIce,
    #[serde(rename = "LIQUID_HYDROGEN")]
    LiquidHydrogen,
    #[serde(rename = "LIQUID_NITROGEN")]
    LiquidNitrogen,
    #[serde(rename = "ICE_WATER")]
    IceWater,
    #[serde(rename = "EXOTIC_MATTER")]
    ExoticMatter,
    #[serde(rename = "ADVANCED_CIRCUITRY")]
    AdvancedCircuitry,
    #[serde(rename = "GRAVITON_EMITTERS")]
    GravitonEmitters,
    #[serde(rename = "IRON")]
    Iron,
    #[serde(rename = "IRON_ORE")]
    IronOre,
    #[serde(rename = "COPPER")]
    Copper,
    #[serde(rename = "COPPER_ORE")]
    CopperOre,
    #[serde(rename = "ALUMINUM")]
    Aluminum,
    #[serde(rename = "ALUMINUM_ORE")]
    AluminumOre,
    #[serde(rename = "SILVER")]
    Silver,
    #[serde(rename = "SILVER_ORE")]
    SilverOre,
    #[serde(rename = "GOLD")]
    Gold,
    #[serde(rename = "GOLD_ORE")]
    GoldOre,
    #[serde(rename = "PLATINUM")]
    Platinum,
    #[serde(rename = "PLATINUM_ORE")]
    PlatinumOre,
    #[serde(rename = "DIAMONDS")]
    Diamonds,
    #[serde(rename = "URANITE")]
    Uranite,
    #[serde(rename = "URANITE_ORE")]
    UraniteOre,
    #[serde(rename = "MERITIUM")]
    Meritium,
    #[serde(rename = "MERITIUM_ORE")]
    MeritiumOre,
    #[serde(rename = "HYDROCARBON")]
    Hydrocarbon,
    #[serde(rename = "ANTIMATTER")]
    Antimatter,
    #[serde(rename = "FAB_MATS")]
    FabMats,
    #[serde(rename = "FERTILIZERS")]
    Fertilizers,
    #[serde(rename = "FABRICS")]
    Fabrics,
    #[serde(rename = "FOOD")]
    Food,
    #[serde(rename = "JEWELRY")]
    Jewelry,
    #[serde(rename = "MACHINERY")]
    Machinery,
    #[serde(rename = "FIREARMS")]
    Firearms,
    #[serde(rename = "ASSAULT_RIFLES")]
    AssaultRifles,
    #[serde(rename = "MILITARY_EQUIPMENT")]
    MilitaryEquipment,
    #[serde(rename = "EXPLOSIVES")]
    Explosives,
    #[serde(rename = "LAB_INSTRUMENTS")]
    LabInstruments,
    #[serde(rename = "AMMUNITION")]
    Ammunition,
    #[serde(rename = "ELECTRONICS")]
    Electronics,
    #[serde(rename = "SHIP_PLATING")]
    ShipPlating,
    #[serde(rename = "SHIP_PARTS")]
    ShipParts,
    #[serde(rename = "EQUIPMENT")]
    Equipment,
    #[serde(rename = "FUEL")]
    Fuel,
    #[serde(rename = "MEDICINE")]
    Medicine,
    #[serde(rename = "DRUGS")]
    Drugs,
    #[serde(rename = "CLOTHING")]
    Clothing,
    #[serde(rename = "MICROPROCESSORS")]
    Microprocessors,
    #[serde(rename = "PLASTICS")]
    Plastics,
    #[serde(rename = "POLYNUCLEOTIDES")]
    Polynucleotides,
    #[serde(rename = "BIOCOMPOSITES")]
    Biocomposites,
    #[serde(rename = "QUANTUM_STABILIZERS")]
    QuantumStabilizers,
    #[serde(rename = "NANOBOTS")]
    Nanobots,
    #[serde(rename = "AI_MAINFRAMES")]
    AiMainframes,
    #[serde(rename = "QUANTUM_DRIVES")]
    QuantumDrives,
    #[serde(rename = "ROBOTIC_DRONES")]
    RoboticDrones,
    #[serde(rename = "CYBER_IMPLANTS")]
    CyberImplants,
    #[serde(rename = "GENE_THERAPEUTICS")]
    GeneTherapeutics,
    #[serde(rename = "NEURAL_CHIPS")]
    NeuralChips,
    #[serde(rename = "MOOD_REGULATORS")]
    MoodRegulators,
    #[serde(rename = "VIRAL_AGENTS")]
    ViralAgents,
    #[serde(rename = "MICRO_FUSION_GENERATORS")]
    MicroFusionGenerators,
    #[serde(rename = "SUPERGRAINS")]
    Supergrains,
    #[serde(rename = "LASER_RIFLES")]
    LaserRifles,
    #[serde(rename = "HOLOGRAPHICS")]
    Holographics,
    #[serde(rename = "SHIP_SALVAGE")]
    ShipSalvage,
    #[serde(rename = "RELIC_TECH")]
    RelicTech,
    #[serde(rename = "NOVEL_LIFEFORMS")]
    NovelLifeforms,
    #[serde(rename = "BOTANICAL_SPECIMENS")]
    BotanicalSpecimens,
    #[serde(rename = "CULTURAL_ARTIFACTS")]
    CulturalArtifacts,
    #[serde(rename = "FRAME_PROBE")]
    FrameProbe,
    #[serde(rename = "FRAME_DRONE")]
    FrameDrone,
    #[serde(rename = "FRAME_INTERCEPTOR")]
    FrameInterceptor,
    #[serde(rename = "FRAME_RACER")]
    FrameRacer,
    #[serde(rename = "FRAME_FIGHTER")]
    FrameFighter,
    #[serde(rename = "FRAME_FRIGATE")]
    FrameFrigate,
    #[serde(rename = "FRAME_SHUTTLE")]
    FrameShuttle,
    #[serde(rename = "FRAME_EXPLORER")]
    FrameExplorer,
    #[serde(rename = "FRAME_MINER")]
    FrameMiner,
    #[serde(rename = "FRAME_LIGHT_FREIGHTER")]
    FrameLightFreighter,
    #[serde(rename = "FRAME_HEAVY_FREIGHTER")]
    FrameHeavyFreighter,
    #[serde(rename = "FRAME_TRANSPORT")]
    FrameTransport,
    #[serde(rename = "FRAME_DESTROYER")]
    FrameDestroyer,
    #[serde(rename = "FRAME_CRUISER")]
    FrameCruiser,
    #[serde(rename = "FRAME_CARRIER")]
    FrameCarrier,
    #[serde(rename = "REACTOR_SOLAR_I")]
    ReactorSolarI,
    #[serde(rename = "REACTOR_FUSION_I")]
    ReactorFusionI,
    #[serde(rename = "REACTOR_FISSION_I")]
    ReactorFissionI,
    #[serde(rename = "REACTOR_CHEMICAL_I")]
    ReactorChemicalI,
    #[serde(rename = "REACTOR_ANTIMATTER_I")]
    ReactorAntimatterI,
    #[serde(rename = "ENGINE_IMPULSE_DRIVE_I")]
    EngineImpulseDriveI,
    #[serde(rename = "ENGINE_ION_DRIVE_I")]
    EngineIonDriveI,
    #[serde(rename = "ENGINE_ION_DRIVE_II")]
    EngineIonDriveIi,
    #[serde(rename = "ENGINE_HYPER_DRIVE_I")]
    EngineHyperDriveI,
    #[serde(rename = "MODULE_MINERAL_PROCESSOR_I")]
    ModuleMineralProcessorI,
    #[serde(rename = "MODULE_GAS_PROCESSOR_I")]
    ModuleGasProcessorI,
    #[serde(rename = "MODULE_CARGO_HOLD_I")]
    ModuleCargoHoldI,
    #[serde(rename = "MODULE_CARGO_HOLD_II")]
    ModuleCargoHoldIi,
    #[serde(rename = "MODULE_CARGO_HOLD_III")]
    ModuleCargoHoldIii,
    #[serde(rename = "MODULE_CREW_QUARTERS_I")]
    ModuleCrewQuartersI,
    #[serde(rename = "MODULE_ENVOY_QUARTERS_I")]
    ModuleEnvoyQuartersI,
    #[serde(rename = "MODULE_PASSENGER_CABIN_I")]
    ModulePassengerCabinI,
    #[serde(rename = "MODULE_MICRO_REFINERY_I")]
    ModuleMicroRefineryI,
    #[serde(rename = "MODULE_SCIENCE_LAB_I")]
    ModuleScienceLabI,
    #[serde(rename = "MODULE_JUMP_DRIVE_I")]
    ModuleJumpDriveI,
    #[serde(rename = "MODULE_JUMP_DRIVE_II")]
    ModuleJumpDriveIi,
    #[serde(rename = "MODULE_JUMP_DRIVE_III")]
    ModuleJumpDriveIii,
    #[serde(rename = "MODULE_WARP_DRIVE_I")]
    ModuleWarpDriveI,
    #[serde(rename = "MODULE_WARP_DRIVE_II")]
    ModuleWarpDriveIi,
    #[serde(rename = "MODULE_WARP_DRIVE_III")]
    ModuleWarpDriveIii,
    #[serde(rename = "MODULE_SHIELD_GENERATOR_I")]
    ModuleShieldGeneratorI,
    #[serde(rename = "MODULE_SHIELD_GENERATOR_II")]
    ModuleShieldGeneratorIi,
    #[serde(rename = "MODULE_ORE_REFINERY_I")]
    ModuleOreRefineryI,
    #[serde(rename = "MODULE_FUEL_REFINERY_I")]
    ModuleFuelRefineryI,
    #[serde(rename = "MOUNT_GAS_SIPHON_I")]
    MountGasSiphonI,
    #[serde(rename = "MOUNT_GAS_SIPHON_II")]
    MountGasSiphonIi,
    #[serde(rename = "MOUNT_GAS_SIPHON_III")]
    MountGasSiphonIii,
    #[serde(rename = "MOUNT_SURVEYOR_I")]
    MountSurveyorI,
    #[serde(rename = "MOUNT_SURVEYOR_II")]
    MountSurveyorIi,
    #[serde(rename = "MOUNT_SURVEYOR_III")]
    MountSurveyorIii,
    #[serde(rename = "MOUNT_SENSOR_ARRAY_I")]
    MountSensorArrayI,
    #[serde(rename = "MOUNT_SENSOR_ARRAY_II")]
    MountSensorArrayIi,
    #[serde(rename = "MOUNT_SENSOR_ARRAY_III")]
    MountSensorArrayIii,
    #[serde(rename = "MOUNT_MINING_LASER_I")]
    MountMiningLaserI,
    #[serde(rename = "MOUNT_MINING_LASER_II")]
    MountMiningLaserIi,
    #[serde(rename = "MOUNT_MINING_LASER_III")]
    MountMiningLaserIii,
    #[serde(rename = "MOUNT_LASER_CANNON_I")]
    MountLaserCannonI,
    #[serde(rename = "MOUNT_MISSILE_LAUNCHER_I")]
    MountMissileLauncherI,
    #[serde(rename = "MOUNT_TURRET_I")]
    MountTurretI,
    #[serde(rename = "SHIP_PROBE")]
    ShipProbe,
    #[serde(rename = "SHIP_MINING_DRONE")]
    ShipMiningDrone,
    #[serde(rename = "SHIP_SIPHON_DRONE")]
    ShipSiphonDrone,
    #[serde(rename = "SHIP_INTERCEPTOR")]
    ShipInterceptor,
    #[serde(rename = "SHIP_LIGHT_HAULER")]
    ShipLightHauler,
    #[serde(rename = "SHIP_COMMAND_FRIGATE")]
    ShipCommandFrigate,
    #[serde(rename = "SHIP_EXPLORER")]
    ShipExplorer,
    #[serde(rename = "SHIP_HEAVY_FREIGHTER")]
    ShipHeavyFreighter,
    #[serde(rename = "SHIP_LIGHT_SHUTTLE")]
    ShipLightShuttle,
    #[serde(rename = "SHIP_ORE_HOUND")]
    ShipOreHound,
    #[serde(rename = "SHIP_REFINING_FREIGHTER")]
    ShipRefiningFreighter,
    #[serde(rename = "SHIP_SURVEYOR")]
    ShipSurveyor,
}

impl ToString for TradeSymbol {
    fn to_string(&self) -> String {
        match self {
            Self::PreciousStones => String::from("PRECIOUS_STONES"),
            Self::QuartzSand => String::from("QUARTZ_SAND"),
            Self::SiliconCrystals => String::from("SILICON_CRYSTALS"),
            Self::AmmoniaIce => String::from("AMMONIA_ICE"),
            Self::LiquidHydrogen => String::from("LIQUID_HYDROGEN"),
            Self::LiquidNitrogen => String::from("LIQUID_NITROGEN"),
            Self::IceWater => String::from("ICE_WATER"),
            Self::ExoticMatter => String::from("EXOTIC_MATTER"),
            Self::AdvancedCircuitry => String::from("ADVANCED_CIRCUITRY"),
            Self::GravitonEmitters => String::from("GRAVITON_EMITTERS"),
            Self::Iron => String::from("IRON"),
            Self::IronOre => String::from("IRON_ORE"),
            Self::Copper => String::from("COPPER"),
            Self::CopperOre => String::from("COPPER_ORE"),
            Self::Aluminum => String::from("ALUMINUM"),
            Self::AluminumOre => String::from("ALUMINUM_ORE"),
            Self::Silver => String::from("SILVER"),
            Self::SilverOre => String::from("SILVER_ORE"),
            Self::Gold => String::from("GOLD"),
            Self::GoldOre => String::from("GOLD_ORE"),
            Self::Platinum => String::from("PLATINUM"),
            Self::PlatinumOre => String::from("PLATINUM_ORE"),
            Self::Diamonds => String::from("DIAMONDS"),
            Self::Uranite => String::from("URANITE"),
            Self::UraniteOre => String::from("URANITE_ORE"),
            Self::Meritium => String::from("MERITIUM"),
            Self::MeritiumOre => String::from("MERITIUM_ORE"),
            Self::Hydrocarbon => String::from("HYDROCARBON"),
            Self::Antimatter => String::from("ANTIMATTER"),
            Self::FabMats => String::from("FAB_MATS"),
            Self::Fertilizers => String::from("FERTILIZERS"),
            Self::Fabrics => String::from("FABRICS"),
            Self::Food => String::from("FOOD"),
            Self::Jewelry => String::from("JEWELRY"),
            Self::Machinery => String::from("MACHINERY"),
            Self::Firearms => String::from("FIREARMS"),
            Self::AssaultRifles => String::from("ASSAULT_RIFLES"),
            Self::MilitaryEquipment => String::from("MILITARY_EQUIPMENT"),
            Self::Explosives => String::from("EXPLOSIVES"),
            Self::LabInstruments => String::from("LAB_INSTRUMENTS"),
            Self::Ammunition => String::from("AMMUNITION"),
            Self::Electronics => String::from("ELECTRONICS"),
            Self::ShipPlating => String::from("SHIP_PLATING"),
            Self::ShipParts => String::from("SHIP_PARTS"),
            Self::Equipment => String::from("EQUIPMENT"),
            Self::Fuel => String::from("FUEL"),
            Self::Medicine => String::from("MEDICINE"),
            Self::Drugs => String::from("DRUGS"),
            Self::Clothing => String::from("CLOTHING"),
            Self::Microprocessors => String::from("MICROPROCESSORS"),
            Self::Plastics => String::from("PLASTICS"),
            Self::Polynucleotides => String::from("POLYNUCLEOTIDES"),
            Self::Biocomposites => String::from("BIOCOMPOSITES"),
            Self::QuantumStabilizers => String::from("QUANTUM_STABILIZERS"),
            Self::Nanobots => String::from("NANOBOTS"),
            Self::AiMainframes => String::from("AI_MAINFRAMES"),
            Self::QuantumDrives => String::from("QUANTUM_DRIVES"),
            Self::RoboticDrones => String::from("ROBOTIC_DRONES"),
            Self::CyberImplants => String::from("CYBER_IMPLANTS"),
            Self::GeneTherapeutics => String::from("GENE_THERAPEUTICS"),
            Self::NeuralChips => String::from("NEURAL_CHIPS"),
            Self::MoodRegulators => String::from("MOOD_REGULATORS"),
            Self::ViralAgents => String::from("VIRAL_AGENTS"),
            Self::MicroFusionGenerators => String::from("MICRO_FUSION_GENERATORS"),
            Self::Supergrains => String::from("SUPERGRAINS"),
            Self::LaserRifles => String::from("LASER_RIFLES"),
            Self::Holographics => String::from("HOLOGRAPHICS"),
            Self::ShipSalvage => String::from("SHIP_SALVAGE"),
            Self::RelicTech => String::from("RELIC_TECH"),
            Self::NovelLifeforms => String::from("NOVEL_LIFEFORMS"),
            Self::BotanicalSpecimens => String::from("BOTANICAL_SPECIMENS"),
            Self::CulturalArtifacts => String::from("CULTURAL_ARTIFACTS"),
            Self::FrameProbe => String::from("FRAME_PROBE"),
            Self::FrameDrone => String::from("FRAME_DRONE"),
            Self::FrameInterceptor => String::from("FRAME_INTERCEPTOR"),
            Self::FrameRacer => String::from("FRAME_RACER"),
            Self::FrameFighter => String::from("FRAME_FIGHTER"),
            Self::FrameFrigate => String::from("FRAME_FRIGATE"),
            Self::FrameShuttle => String::from("FRAME_SHUTTLE"),
            Self::FrameExplorer => String::from("FRAME_EXPLORER"),
            Self::FrameMiner => String::from("FRAME_MINER"),
            Self::FrameLightFreighter => String::from("FRAME_LIGHT_FREIGHTER"),
            Self::FrameHeavyFreighter => String::from("FRAME_HEAVY_FREIGHTER"),
            Self::FrameTransport => String::from("FRAME_TRANSPORT"),
            Self::FrameDestroyer => String::from("FRAME_DESTROYER"),
            Self::FrameCruiser => String::from("FRAME_CRUISER"),
            Self::FrameCarrier => String::from("FRAME_CARRIER"),
            Self::ReactorSolarI => String::from("REACTOR_SOLAR_I"),
            Self::ReactorFusionI => String::from("REACTOR_FUSION_I"),
            Self::ReactorFissionI => String::from("REACTOR_FISSION_I"),
            Self::ReactorChemicalI => String::from("REACTOR_CHEMICAL_I"),
            Self::ReactorAntimatterI => String::from("REACTOR_ANTIMATTER_I"),
            Self::EngineImpulseDriveI => String::from("ENGINE_IMPULSE_DRIVE_I"),
            Self::EngineIonDriveI => String::from("ENGINE_ION_DRIVE_I"),
            Self::EngineIonDriveIi => String::from("ENGINE_ION_DRIVE_II"),
            Self::EngineHyperDriveI => String::from("ENGINE_HYPER_DRIVE_I"),
            Self::ModuleMineralProcessorI => String::from("MODULE_MINERAL_PROCESSOR_I"),
            Self::ModuleGasProcessorI => String::from("MODULE_GAS_PROCESSOR_I"),
            Self::ModuleCargoHoldI => String::from("MODULE_CARGO_HOLD_I"),
            Self::ModuleCargoHoldIi => String::from("MODULE_CARGO_HOLD_II"),
            Self::ModuleCargoHoldIii => String::from("MODULE_CARGO_HOLD_III"),
            Self::ModuleCrewQuartersI => String::from("MODULE_CREW_QUARTERS_I"),
            Self::ModuleEnvoyQuartersI => String::from("MODULE_ENVOY_QUARTERS_I"),
            Self::ModulePassengerCabinI => String::from("MODULE_PASSENGER_CABIN_I"),
            Self::ModuleMicroRefineryI => String::from("MODULE_MICRO_REFINERY_I"),
            Self::ModuleScienceLabI => String::from("MODULE_SCIENCE_LAB_I"),
            Self::ModuleJumpDriveI => String::from("MODULE_JUMP_DRIVE_I"),
            Self::ModuleJumpDriveIi => String::from("MODULE_JUMP_DRIVE_II"),
            Self::ModuleJumpDriveIii => String::from("MODULE_JUMP_DRIVE_III"),
            Self::ModuleWarpDriveI => String::from("MODULE_WARP_DRIVE_I"),
            Self::ModuleWarpDriveIi => String::from("MODULE_WARP_DRIVE_II"),
            Self::ModuleWarpDriveIii => String::from("MODULE_WARP_DRIVE_III"),
            Self::ModuleShieldGeneratorI => String::from("MODULE_SHIELD_GENERATOR_I"),
            Self::ModuleShieldGeneratorIi => String::from("MODULE_SHIELD_GENERATOR_II"),
            Self::ModuleOreRefineryI => String::from("MODULE_ORE_REFINERY_I"),
            Self::ModuleFuelRefineryI => String::from("MODULE_FUEL_REFINERY_I"),
            Self::MountGasSiphonI => String::from("MOUNT_GAS_SIPHON_I"),
            Self::MountGasSiphonIi => String::from("MOUNT_GAS_SIPHON_II"),
            Self::MountGasSiphonIii => String::from("MOUNT_GAS_SIPHON_III"),
            Self::MountSurveyorI => String::from("MOUNT_SURVEYOR_I"),
            Self::MountSurveyorIi => String::from("MOUNT_SURVEYOR_II"),
            Self::MountSurveyorIii => String::from("MOUNT_SURVEYOR_III"),
            Self::MountSensorArrayI => String::from("MOUNT_SENSOR_ARRAY_I"),
            Self::MountSensorArrayIi => String::from("MOUNT_SENSOR_ARRAY_II"),
            Self::MountSensorArrayIii => String::from("MOUNT_SENSOR_ARRAY_III"),
            Self::MountMiningLaserI => String::from("MOUNT_MINING_LASER_I"),
            Self::MountMiningLaserIi => String::from("MOUNT_MINING_LASER_II"),
            Self::MountMiningLaserIii => String::from("MOUNT_MINING_LASER_III"),
            Self::MountLaserCannonI => String::from("MOUNT_LASER_CANNON_I"),
            Self::MountMissileLauncherI => String::from("MOUNT_MISSILE_LAUNCHER_I"),
            Self::MountTurretI => String::from("MOUNT_TURRET_I"),
            Self::ShipProbe => String::from("SHIP_PROBE"),
            Self::ShipMiningDrone => String::from("SHIP_MINING_DRONE"),
            Self::ShipSiphonDrone => String::from("SHIP_SIPHON_DRONE"),
            Self::ShipInterceptor => String::from("SHIP_INTERCEPTOR"),
            Self::ShipLightHauler => String::from("SHIP_LIGHT_HAULER"),
            Self::ShipCommandFrigate => String::from("SHIP_COMMAND_FRIGATE"),
            Self::ShipExplorer => String::from("SHIP_EXPLORER"),
            Self::ShipHeavyFreighter => String::from("SHIP_HEAVY_FREIGHTER"),
            Self::ShipLightShuttle => String::from("SHIP_LIGHT_SHUTTLE"),
            Self::ShipOreHound => String::from("SHIP_ORE_HOUND"),
            Self::ShipRefiningFreighter => String::from("SHIP_REFINING_FREIGHTER"),
            Self::ShipSurveyor => String::from("SHIP_SURVEYOR"),
        }
    }
}

impl Default for TradeSymbol {
    fn default() -> TradeSymbol {
        Self::PreciousStones
    }
}
