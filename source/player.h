#ifndef PLAYER_H
#define PLAYER_H

#include "jamultypes.h"
#include "guy.h"
#include "control.h"
#include "game.h"
#include "display.h"
#include "bullet.h"
#include "world.h"
#include "intface.h"

// secondary weapon defines
enum {
	WPN_NONE,
	WPN_MISSILES,
	WPN_AK8087,
	WPN_BOMBS,
	WPN_FLAME,
	WPN_PWRARMOR,
	WPN_BIGAXE,
	WPN_LIGHTNING,
	WPN_SPEAR,
	WPN_MACHETE,
	WPN_MINES,
	WPN_TURRET,
	WPN_MINDCONTROL,
	WPN_REFLECTOR,
	WPN_JETPACK,
	WPN_SWAPGUN
};

// initializing constants (pass to InitPlayer)
enum {
	INIT_GAME = 2,
	INIT_WORLD = 1,
	INIT_LEVEL = 0
};

// vehicles you could be on
enum {
	VE_NONE = 0,
	VE_MINECART = 1,
	VE_RAFT = 2
};

// the most custom worlds it will handle
const int MAX_CUSTOM = 128;

struct player_t
{
	// values for the overall game
	byte musicSettings;
	int prevScore; // so you can lose all your points when you die
	int score;
	byte levelPassed[MAX_CUSTOM][MAX_MAPS];
	byte keychain[MAX_CUSTOM][4];
	// total completion is how many "points" the world has in it
	int totalCompletion[MAX_CUSTOM];
	// complete is how many of those points the player has, to create a percentage complete display
	int complete[MAX_CUSTOM];
	char customName[MAX_CUSTOM][32];
	byte lunacyKey[MAX_CUSTOM];
	// values reset for each world
	byte levelsPassed;
	byte worldNum;
	// values reset for each level
	byte shield;
	byte levelNum;
	byte keys[4];
	int boredom;
	byte hammers;
	byte hamSpeed;
	byte weapon;
	int ammo;
	byte reload;
	byte wpnReload;
	byte life;
	int brains;
	byte pushPower; // for pushing pushy blocks
	byte hammerFlags;
	byte vehicle;
	byte garlic;
	byte speed; // accelerated
	byte rageClock;
	word rage;
	byte invisibility;
	byte jetting;
};

extern player_t player;

void InitPlayer(byte initWhat, byte world, byte level);
void ExitPlayer(void);
void PlayerControlMe(Guy *me, mapTile_t *mapTile, world_t *world);
void PlayerControlPowerArmor(Guy *me, mapTile_t *mapTile, world_t *world);
byte PlayerHasHammer(void);
extern "C" byte PlayerGetItem(byte itm, int x, int y);
void PlayerWinLevel(byte w, byte l, byte isSecret);
void PlayerRenderInterface(MGLDraw *mgl);
int PlayerBrains(void);
void SetPlayerHP(int hp);
byte PlayerLevelsPassed(void);
byte PlayerPassedLevel(byte world, byte map);
byte PlayerKeyChain(byte w);
byte PlayerKeys(byte w);
void PlayerLoseKey(byte w);
extern "C" void PlayerSetWorldWorth(byte world, int amt);
float PlayerGetPercent(byte world);
float PlayerGetGamePercent(void);
void SetCustomName(const char *name);
char *GetCustomName(void);
byte PlayerHasLunacyKey(byte w);
extern "C" void PlayerHeal(byte amt);
void PlayerLoadGame(byte which);
void PlayerSaveGame(byte which);
byte GetPlayerWorld(void);
byte PlayerShield(void);
void PlayerGetPoints(int amt);
void PlayerResetScore(void);
byte PlayerGetMusicSettings(void);
void PlayerSetMusicSettings(byte m);
void SetPlayerGlow(byte v);
byte GetPlayerGlow(void);
byte PlayerPushMore(void);
void PoisonVictim(Guy *me, byte amt);
byte PlayerIsPoisoned(void);
extern "C" void ToggleWaterwalk(void);
byte PlayerCanWaterwalk(void);
void SetTportClock(byte tp);
byte GetTportClock(void);
byte StealWeapon(void);

#endif
