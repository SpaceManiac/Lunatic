#ifndef GUY_H
#define GUY_H

#include "winpch.h"
#include "map.h"
#include "monster.h"
#include "jamulspr.h"
#include "display.h"

enum {
	ACTION_IDLE = 0,
	ACTION_BUSY
};

class Guy
{
public:
	Guy(void);
	~Guy(void);

	void Update(Map *map, world_t *world);
	void EditorUpdate(Map *map);
	void Render(byte light);
	byte CanWalk(int xx, int yy, Map *map, world_t *world);
	void NextFrame(void);
	void SeqFinished(void);
	void MonsterControl(Map *map, world_t *world);
	byte CoconutBonk(int xx, int yy, Guy *him);
	byte AttackCheck(byte size, int xx, int yy, Guy *him);
	void AttackThem(void);
	void GetShot(int dx, int dy, byte damage, Map *map, world_t *world);
	void CalculateRect(void);

	int x, y, z;
	int oldx, oldy;
	int dx, dy, dz;
	byte mapx, mapy;
	byte facing;

	byte mind; // brain variables for AI
	byte mind1;
	byte mind2;
	byte mind3;

	byte reload;
	byte poison;

	byte ouch;
	byte action;
	word frmTimer;
	word frmAdvance;
	byte frm;
	byte seq;
	char bright;
	byte friendly;

	word mindControl;
	Guy *target;
	Guy *parent;
	int hp;
	byte type;
	int rectx, recty, rectx2, recty2; // for collision checks
	word ID; // just a copy of the guy's number
};

extern Guy *goodguy;

extern "C" {
void InitGuys(int max);
void ExitGuys(void);
void UpdateGuys(Map *map, world_t *world);
void EditorUpdateGuys(Map *map);
void RenderGuys(byte light);
Guy *AddGuy(int x, int y, int z, byte type);
Guy *GetGuy(word w);
void DeleteGuy(int x, int y, byte type);
void AddMapGuys(Map *map);
byte FindVictim(int x, int y, byte size, int dx, int dy, byte damage, Map *map, world_t *world, byte friendly);
byte FindVictims(int x, int y, byte size, int dx, int dy, byte damage, Map *map, world_t *world, byte friendly);
byte FindVictims2(int x, int y, byte size, int dx, int dy, byte damage, Map *map, world_t *world, byte friendly);
word LockOnEvil(int x, int y);
word LockOnEvil2(int x, int y);
word LockOnGood(int x, int y);
word LockOnGood2(int x, int y);
byte GetGuyPos(word guy, int *x, int *y);
byte MonsterExists(byte type);
void HealGoodguy(byte amt);
byte MossCheck(int x, int y);
void KillKids(Guy *g);
byte RaftNearby(void);
void GuySwap(int sx, int sy, int width, int height, int dx, int dy);
void ShiftGuys(char dx, char dy, Map *map);
void AddPygmy(Map *map, world_t *world);
void AddNinja(Map *map, world_t *world);
Guy *GetLastGuyHit(void);
byte ControlMind(Guy *me);
void KillAllMonsters(byte type);
void KillMonster(int x, int y);
void ChangeAllMonsters(byte type, byte newtype);
void ChangeMonster(int x, int y, byte type);
byte SwapMe(int x, int y, byte size, Map *map);

void MonsterTally(void);
}

#endif
