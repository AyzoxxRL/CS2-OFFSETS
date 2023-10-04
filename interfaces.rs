#![allow(non_snake_case, non_upper_case_globals)]

// Dumped With FrezzyHook.com
// 2023-10-04 23:32:11.917694100 UTC

pub mod animationsystem_dll {
    pub const AnimationSystemUtils_001: usize = 0x64510;
    pub const AnimationSystem_001: usize = 0x5F070;
}

pub mod client_dll {
    pub const LegacyGameUI001: usize = 0x88DF10;
    pub const Source2ClientUI001: usize = 0x87C900;
    pub const Source2ClientPrediction001: usize = 0x79BC60;
    pub const ClientToolsInfo_001: usize = 0x727A90;
    pub const Source2Client002: usize = 0x727AB0;
    pub const GameClientExports001: usize = 0x727AA0;
    pub const EmptyWorldService001_Client: usize = 0x48AB60;
    pub const Source2ClientConfig001: usize = 0x471EC0;
}

pub mod engine2_dll {
    pub const SimpleEngineLoopService_001: usize = 0x1E51D0;
    pub const ClientServerEngineLoopService_001: usize = 0x1DAEA0;
    pub const KeyValueCache001: usize = 0x1D7240;
    pub const HostStateMgr001: usize = 0x1D4C50;
    pub const GameEventSystemServerV001: usize = 0x1D0070;
    pub const GameEventSystemClientV001: usize = 0x1D0060;
    pub const EngineServiceMgr001: usize = 0x1CB980;
    pub const VProfService_001: usize = 0x1B8F70;
    pub const ToolService_001: usize = 0x1B7CD0;
    pub const StatsService_001: usize = 0x1B2F50;
    pub const SplitScreenService_001: usize = 0x1B01C0;
    pub const SoundService_001: usize = 0x1AAD10;
    pub const ScreenshotService001: usize = 0x1A6BE0;
    pub const RenderService_001: usize = 0x1A3F40;
    pub const NetworkService_001: usize = 0x1A3A50;
    pub const NetworkServerService_001: usize = 0x19EAF0;
    pub const NetworkP2PService_001: usize = 0x19A500;
    pub const NetworkClientService_001: usize = 0x194130;
    pub const MapListService_001: usize = 0x18D100;
    pub const InputService_001: usize = 0x17AE50;
    pub const GameUIService_001: usize = 0x175910;
    pub const GameResourceServiceServerV001: usize = 0x173670;
    pub const GameResourceServiceClientV001: usize = 0x173660;
    pub const BugService001: usize = 0x16C690;
    pub const BenchmarkService001: usize = 0x16AE60;
    pub const VENGINE_GAMEUIFUNCS_VERSION005: usize = 0x118580;
    pub const EngineGameUI001: usize = 0x117840;
    pub const INETSUPPORT_001: usize = 0xE6E50;
    pub const Source2EngineToServerStringTable001: usize = 0xA18B0;
    pub const Source2EngineToServer001: usize = 0x8B6E0;
    pub const Source2EngineToClientStringTable001: usize = 0x840E0;
    pub const Source2EngineToClient001: usize = 0x5A740;
}

pub mod filesystem_stdio_dll {
    pub const VAsyncFileSystem2_001: usize = 0x66D40;
    pub const VFileSystem017: usize = 0x66D30;
}

pub mod host_dll {
    pub const Source2Host001: usize = 0x18E50;
    pub const SinglePlayerSharedMemory001: usize = 0x18AD0;
    pub const SaveRestoreDataVersion001: usize = 0x18AC0;
    pub const PredictionDiffManager001: usize = 0x16E60;
    pub const HostUtils001: usize = 0x123F0;
    pub const GameSystem2HostHook: usize = 0x11C40;
    pub const GameModelInfo001: usize = 0x11B60;
    pub const DebugDrawQueueManager001: usize = 0x115B0;
}

pub mod imemanager_dll {
    pub const IMEManager001: usize = 0xC470;
}

pub mod inputsystem_dll {
    pub const InputSystemVersion001: usize = 0x28D0;
    pub const InputStackSystemVersion001: usize = 0x16F0;
}

pub mod localize_dll {
    pub const Localize_001: usize = 0x3830;
}

pub mod matchmaking_dll {
    pub const GameTypes001: usize = 0x50110;
    pub const MATCHFRAMEWORK_001: usize = 0x101180;
}

pub mod materialsystem2_dll {
    pub const MaterialUtils_001: usize = 0x4DDC0;
    pub const TextLayout_001: usize = 0x4A510;
    pub const PostProcessingSystem_001: usize = 0x42C90;
    pub const FontManager_001: usize = 0x37A70;
    pub const VMaterialSystem2_001: usize = 0x25FD0;
}

pub mod meshsystem_dll {
    pub const MeshSystem001: usize = 0x7110;
}

pub mod navsystem_dll {
    pub const NavSystem001: usize = 0x7590;
}

pub mod networksystem_dll {
    pub const SerializedEntitiesVersion001: usize = 0xD2A60;
    pub const NetworkSystemVersion001: usize = 0xBCD00;
    pub const NetworkMessagesVersion001: usize = 0x9CD30;
    pub const FlattenedSerializersVersion001: usize = 0x7C4E0;
}

pub mod panorama_dll {
    pub const PanoramaUIEngine001: usize = 0x5E650;
}

pub mod panorama_text_pango_dll {
    pub const PanoramaTextServices001: usize = 0x4CCB0;
}

pub mod panoramauiclient_dll {
    pub const PanoramaUIClient001: usize = 0x12620;
}

pub mod particles_dll {
    pub const ParticleSystemMgr003: usize = 0x52D00;
}

pub mod pulse_system_dll {
    pub const IPulseSystem_001: usize = 0x5A20;
}

pub mod resourcesystem_dll {
    pub const ResourceSystem013: usize = 0x106B0;
}

pub mod scenefilecache_dll {
    pub const SceneFileCache002: usize = 0x68E0;
    pub const ResponseRulesCache001: usize = 0x3190;
}

pub mod scenesystem_dll {
    pub const SceneUtils_001: usize = 0x13D620;
    pub const SceneSystem_002: usize = 0xCAAC0;
    pub const RenderingPipelines_001: usize = 0x8EA20;
}

pub mod schemasystem_dll {
    pub const SchemaSystem_001: usize = 0xA840;
}

pub mod server_dll {
    pub const NavGameTest001: usize = 0xA2DCD0;
    pub const ServerToolsInfo_001: usize = 0x8323B0;
    pub const Source2GameClients001: usize = 0x8323C0;
    pub const Source2GameEntities001: usize = 0x8323D0;
    pub const Source2Server001: usize = 0x8323E0;
    pub const EmptyWorldService001_Server: usize = 0x586F20;
    pub const Source2ServerConfig001: usize = 0x566EC0;
    pub const EntitySubclassUtilsV001: usize = 0x2C8200;
    pub const customnavsystem001: usize = 0x244CE0;
    pub const Source2GameDirector001: usize = 0x13E8A0;
}

pub mod soundsystem_dll {
    pub const SoundOpSystem001: usize = 0x1560D0;
    pub const SoundOpSystemEdit001: usize = 0x8B4B0;
    pub const VMixEditTool001: usize = 0x71710;
    pub const SoundSystem001: usize = 0x46510;
}

pub mod steamaudio_dll {
    pub const SteamAudio001: usize = 0x12D90;
}

pub mod steamclient64_dll {
    pub const IVALIDATE001: usize = 0x833640;
    pub const CLIENTENGINE_INTERFACE_VERSION005: usize = 0x82F4C0;
    pub const SteamClient020: usize = 0x62CAF0;
    pub const SteamClient019: usize = 0x62CAE0;
    pub const SteamClient018: usize = 0x62CAD0;
    pub const SteamClient017: usize = 0x62CAC0;
    pub const SteamClient016: usize = 0x62CAB0;
    pub const SteamClient015: usize = 0x62CAA0;
    pub const SteamClient014: usize = 0x62CA90;
    pub const SteamClient013: usize = 0x62CA80;
    pub const SteamClient012: usize = 0x62CA70;
    pub const SteamClient011: usize = 0x62CA60;
    pub const SteamClient010: usize = 0x62CA50;
    pub const SteamClient009: usize = 0x62CA40;
    pub const SteamClient008: usize = 0x62CA30;
    pub const SteamClient007: usize = 0x62CA20;
    pub const SteamClient006: usize = 0x62CA10;
    pub const p2pvoice002: usize = 0xD8EA0;
    pub const p2pvoicesingleton002: usize = 0xD5840;
}

pub mod tier0_dll {
    pub const VStringTokenSystem001: usize = 0x18D680;
    pub const TestScriptMgr001: usize = 0x13F7B0;
    pub const VProcessUtils002: usize = 0x12F7C0;
    pub const VEngineCvar007: usize = 0x61640;
}

pub mod v8system_dll {
    pub const Source2V8System001: usize = 0x1670;
}

pub mod valve_avi_dll {
    pub const VAvi001: usize = 0x2F90;
}

pub mod valve_wmf_dll {
    pub const VMediaFoundation001: usize = 0x12D0;
}

pub mod vphysics2_dll {
    pub const VPhysics2_Handle_Interface_001: usize = 0x5F8F0;
    pub const VPhysics2_Interface_001: usize = 0x5B690;
}

pub mod vscript_dll {
    pub const VScriptManager010: usize = 0x31C40;
}

pub mod vstdlib_s64_dll {
    pub const IVALIDATE001: usize = 0x24FF0;
    pub const VEngineCvar002: usize = 0x5750;
}

pub mod worldrenderer_dll {
    pub const WorldRendererMgr001: usize = 0x21320;
}