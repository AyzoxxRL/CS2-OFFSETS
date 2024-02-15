/*
 * All offsets dumped with: https://frezzyhook.com
 * Thu, 15 Feb 2024 02:25:16 +0000
 */

#pragma once

#include <cstddef>

namespace client_dll { // client.dll
    constexpr std::ptrdiff_t dwEntityList = 0x18B3018;
    constexpr std::ptrdiff_t dwForceAttack = 0x1721140;
    constexpr std::ptrdiff_t dwForceAttack2 = 0x17211D0;
    constexpr std::ptrdiff_t dwForceBackward = 0x1721410;
    constexpr std::ptrdiff_t dwForceCrouch = 0x17216E0;
    constexpr std::ptrdiff_t dwForceForward = 0x1721380;
    constexpr std::ptrdiff_t dwForceJump = 0x1721650;
    constexpr std::ptrdiff_t dwForceLeft = 0x17214A0;
    constexpr std::ptrdiff_t dwForceRight = 0x1721530;
    constexpr std::ptrdiff_t dwGameEntitySystem = 0x19CE850;
    constexpr std::ptrdiff_t dwGameEntitySystem_getHighestEntityIndex = 0x1510;
    constexpr std::ptrdiff_t dwGameRules = 0x1910010;
    constexpr std::ptrdiff_t dwGlobalVars = 0x171BE20;
    constexpr std::ptrdiff_t dwGlowManager = 0x1910428;
    constexpr std::ptrdiff_t dwInterfaceLinkList = 0x19FF8A8;
    constexpr std::ptrdiff_t dwLocalPlayerController = 0x1902988;
    constexpr std::ptrdiff_t dwLocalPlayerPawn = 0x17282B8;
    constexpr std::ptrdiff_t dwPlantedC4 = 0x1918238;
    constexpr std::ptrdiff_t dwPrediction = 0x1728180;
    constexpr std::ptrdiff_t dwSensitivity = 0x1910D48;
    constexpr std::ptrdiff_t dwSensitivity_sensitivity = 0x40;
    constexpr std::ptrdiff_t dwViewAngles = 0x1920150;
    constexpr std::ptrdiff_t dwViewMatrix = 0x1912320;
    constexpr std::ptrdiff_t dwViewRender = 0x1912BA0;
}

namespace engine2_dll { // engine2.dll
    constexpr std::ptrdiff_t dwBuildNumber = 0x512554;
    constexpr std::ptrdiff_t dwNetworkGameClient = 0x511AC8;
    constexpr std::ptrdiff_t dwNetworkGameClient_getLocalPlayer = 0xF0;
    constexpr std::ptrdiff_t dwNetworkGameClient_maxClients = 0x250;
    constexpr std::ptrdiff_t dwNetworkGameClient_signOnState = 0x240;
    constexpr std::ptrdiff_t dwWindowHeight = 0x5C9A2C;
    constexpr std::ptrdiff_t dwWindowWidth = 0x5C9A28;
}

namespace game_info { // Some additional information about the game at dump time
    constexpr std::ptrdiff_t buildNumber = 0x36A8; // Game build number
}

namespace inputsystem_dll { // inputsystem.dll
    constexpr std::ptrdiff_t dwInputSystem = 0x367A0;
}