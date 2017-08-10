#include "monster.h"
#include "player.h"
#include "options.h"

extern monsterType_t monsType[NUM_MONSTERS];

void InitMonsters(void)
{
	int i;

	for (i = 0; i < NUM_MONSTERS; i++)
	{
		monsType[i].spr = NULL;
	}
	// just keep bouapha perma-loaded
	monsType[MONS_BOUAPHA].spr = new sprite_set_t(monsType[MONS_BOUAPHA].sprName);
}

void ExitMonsters(void)
{
	int i;

	for (i = 1; i < NUM_MONSTERS; i++)
	{
		if (i == MONS_BJORN)
			monsType[i].spr = NULL; // he uses the same sprites as MONS_SVEN
		if (monsType[i].spr)
			delete monsType[i].spr;
		monsType[i].spr = NULL;
	}
}

void ChangeOffColor(byte type, byte from, byte to)
{
	monsType[type].fromCol = from;
	monsType[type].toCol = to;
}

void PurgeMonsterSprites(void)
{
	int i;

	// note this starts at 2, skipping bouapha
	for (i = 2; i < NUM_MONSTERS; i++)
	{
		// repeat graphics monsters do not delete their sprites
		if (monsType[i].sprName[0] == '!')
			monsType[i].spr = NULL;
		else if (monsType[i].spr)
			delete monsType[i].spr;
		monsType[i].spr = NULL;
	}
}

byte MonsterSize(byte type)
{
#ifdef EXPANDO
	if (type == MONS_BOUAPHA && player.weapon == WPN_PWRARMOR)
		return monsType[MONS_PWRBOUAPHA].size;
#endif
	return monsType[type].size;
}

byte *MonsterAnim(byte type, byte anim)
{
#ifdef EXPANDO
	if (type == MONS_BOUAPHA && player.weapon == WPN_PWRARMOR)
		return monsType[MONS_PWRBOUAPHA].anim[anim];
#endif
	return monsType[type].anim[anim];
}

word MonsterFlags(byte type)
{
#ifdef EXPANDO
	if (type == MONS_BOUAPHA && player.weapon == WPN_PWRARMOR)
		return monsType[MONS_PWRBOUAPHA].flags;
#endif
	return monsType[type].flags;
}

byte MonsterFrames(byte type)
{
#ifdef EXPANDO
	if (type == MONS_BOUAPHA && player.weapon == WPN_PWRARMOR)
		return monsType[MONS_PWRBOUAPHA].framesPerDir;
#endif
	return monsType[type].framesPerDir;
}

word MonsterPoints(byte type)
{
	return monsType[type].points;
}

word MonsterHP(byte type)
{
	return monsType[type].hp;
}

char *MonsterName(byte type)
{
	static char nullName[32] = "NULL";
	if (type >= NUM_MONSTERS)
		return nullName;
	return monsType[type].name;
}

monsterAi_t MonsterAi(byte type)
{
	if (type >= NUM_MONSTERS)
		return NULL;
	return monsType[type].aiFunc;
}

void SetMonsterFlags(byte type, word flags)
{
	monsType[type].flags = flags;
}

void LoadMySprite(byte type)
{
	int v;
	dword start, end;

	if (type == 0 || type >= NUM_MONSTERS)
		return;

	start = timeGetTime();
	if (monsType[type].spr == NULL)
	{
		if (monsType[type].sprName[0] == '!')
		{
			// it's a repeat of someone else's sprite
			v = atoi(&monsType[type].sprName[1]);
			if (!monsType[v].spr)
				monsType[v].spr = new sprite_set_t(monsType[v].sprName);

			monsType[type].spr = monsType[v].spr;
		}
		else
			monsType[type].spr = new sprite_set_t(monsType[type].sprName);

		if (monsType[type].spr == NULL)
			MGL_fatalError("Out of memory!");
	}
	end = timeGetTime();
	AddGarbageTime(end - start);
}

sprite_t *GetMonsterSprite(byte type, byte seq, byte frm, byte facing)
{
	int v;

	if (type == MONS_BOUAPHA)
	{
		if (player.weapon == WPN_PWRARMOR)
			type = MONS_PWRBOUAPHA;
		else if (opt.playAs == PLAYAS_LUNATIC)
			type = MONS_DRL;
		else if (opt.playAs == PLAYAS_HAPPY)
			type = MONS_STICKMAN;
	}

	// load if not loaded
	LoadMySprite(type);

	v = monsType[type].anim[seq][frm];

	if (v == 254)
		return NULL; // 254 means no sprite for this frame

	if (!(monsType[type].flags & MF_ONEFACE))
		v += facing * monsType[type].framesPerDir;

	if (type == MONS_BOUAPHA)
	{
		if (PlayerHasHammer())
			v += 8 * monsType[type].framesPerDir;
	}
	if (type == MONS_EVILCLONE)
		v += 8 * monsType[type].framesPerDir;

	if (monsType[type].flags & MF_FACECMD)
		v += facing;

	return monsType[type].spr->GetSprite(v);
}

void MonsterDraw(int x, int y, int z, byte type, byte seq, byte frm, byte facing, char bright, byte ouch, byte poison)
{
	sprite_t *curSpr;
	int v;
	byte shld, isBouapha;

	if (type == MONS_BOUAPHA)
	{
		if (player.weapon == WPN_PWRARMOR)
			type = MONS_PWRBOUAPHA;
		else if (opt.playAs == PLAYAS_LUNATIC)
			type = MONS_DRL;
		else if (opt.playAs == PLAYAS_HAPPY)
			type = MONS_STICKMAN;

		isBouapha = 1;
	}
	else
		isBouapha = 0;

	// load if not loaded
	LoadMySprite(type);

	v = monsType[type].anim[seq][frm];

	if (v == 254)
		return; // don't draw this frame

	if (!(monsType[type].flags & MF_ONEFACE))
		v += facing * monsType[type].framesPerDir;

	if (isBouapha)
	{
		if (type == MONS_BOUAPHA && PlayerHasHammer())
			v += 8 * monsType[type].framesPerDir;
		shld = PlayerShield();
		if ((shld < 16) && (shld & 2)) // it blinks when there is 1/2 second left
			shld = 0;
		curSpr = monsType[MONS_BOUAPHA].spr->GetSprite(464 + (shld & 7));
		if (shld)
			SprDraw(x >> FIXSHIFT, (y >> FIXSHIFT) + 1, 1, 255, bright, curSpr, DISPLAY_DRAWME | DISPLAY_GLOW);
		if (poison)
		{
			curSpr = monsType[type].spr->GetSprite(v);
			if (!curSpr)
				return;
			if (!(monsType[type].flags & MF_NOSHADOW))
				SprDraw(x >> FIXSHIFT, y >> FIXSHIFT, 0, 255, 0, curSpr, DISPLAY_DRAWME | DISPLAY_SHADOW);
			if (ouch == 0)
				SprDraw(x >> FIXSHIFT, y >> FIXSHIFT, z >> FIXSHIFT, 1, bright - 4, curSpr, DISPLAY_DRAWME); // green
			else
				SprDraw(x >> FIXSHIFT, y >> FIXSHIFT, z >> FIXSHIFT, 5, bright, curSpr, DISPLAY_DRAWME); // yellow
			return;
		}
		else if (player.invisibility)
		{
			curSpr = monsType[type].spr->GetSprite(v);
			if (!curSpr)
				return;
			SprDraw(x >> FIXSHIFT, y >> FIXSHIFT, z >> FIXSHIFT, 255, bright, curSpr, DISPLAY_DRAWME | DISPLAY_GLOW);
			return;
		}
	}

	if (type == MONS_EVILCLONE)
		v += 8 * monsType[type].framesPerDir;

	if (monsType[type].flags & MF_FACECMD)
		v += facing;

	curSpr = monsType[type].spr->GetSprite(v);
	if (!curSpr)
		return;

	if (!(monsType[type].flags & MF_NOSHADOW))
		SprDraw(x >> FIXSHIFT, y >> FIXSHIFT, 0, 255, 0, curSpr, DISPLAY_DRAWME | DISPLAY_SHADOW);

	if (ouch == 0)
	{
		if (poison)
			SprDraw(x >> FIXSHIFT, y >> FIXSHIFT, z >> FIXSHIFT, 1, bright, curSpr, DISPLAY_DRAWME);
		else if (!(monsType[type].flags & (MF_GHOST | MF_GLOW)))
		{
			if (monsType[type].fromCol == 255)
				SprDraw(x >> FIXSHIFT, y >> FIXSHIFT, z >> FIXSHIFT, 255, bright + monsType[type].brtChg, curSpr, DISPLAY_DRAWME);
			else
			{
				SprDrawOff(x >> FIXSHIFT, y >> FIXSHIFT, z >> FIXSHIFT, monsType[type].fromCol, monsType[type].toCol,
						bright + monsType[type].brtChg, curSpr, DISPLAY_DRAWME);
			}
		}
		else if (monsType[type].flags & MF_GHOST)
			SprDraw(x >> FIXSHIFT, y >> FIXSHIFT, z >> FIXSHIFT, 255, bright + monsType[type].brtChg, curSpr, DISPLAY_DRAWME | DISPLAY_GHOST);
		else if (monsType[type].flags & MF_GLOW)
			SprDraw(x >> FIXSHIFT, y >> FIXSHIFT, z >> FIXSHIFT, 255, bright + monsType[type].brtChg, curSpr, DISPLAY_DRAWME | DISPLAY_GLOW);
	}
	else
	{
		if (!poison)
			SprDraw(x >> FIXSHIFT, y >> FIXSHIFT, z >> FIXSHIFT, 4, bright, curSpr, DISPLAY_DRAWME);
		else
			SprDraw(x >> FIXSHIFT, y >> FIXSHIFT, z >> FIXSHIFT, 5, bright, curSpr, DISPLAY_DRAWME);
	}

}

void InstaRenderMonster(int x, int y, byte type, char bright, MGLDraw *mgl)
{
	sprite_t *curSpr;
	int v;

	// load if not loaded
	LoadMySprite(type);

	v = monsType[type].anim[ANIM_IDLE][0];
	if (!(monsType[type].flags & MF_ONEFACE))
		v += 2 * monsType[type].framesPerDir;

	curSpr = monsType[type].spr->GetSprite(v);
	if (!curSpr)
		return;

	if (monsType[type].fromCol == 255)
		curSpr->DrawBright(x, y, mgl, bright + monsType[type].brtChg);
	else
		curSpr->DrawOffColor(x, y, mgl, monsType[type].fromCol, monsType[type].toCol,
			bright + monsType[type].brtChg);
}

// AI auxiliary functions to make it simple
// ---------------------------------------------

inline void FaceGoodguy(Guy *me, Guy *goodguy)
{
	if (goodguy->x < me->x - FIXAMT * 16)
	{
		if (goodguy->y < me->y - FIXAMT * 16)
			me->facing = 5;
		else if (goodguy->y > me->y + FIXAMT * 16)
			me->facing = 3;
		else
			me->facing = 4;
	}
	else if (goodguy->x > me->x + FIXAMT * 16)
	{
		if (goodguy->y < me->y - FIXAMT * 16)
			me->facing = 7;
		else if (goodguy->y > me->y + FIXAMT * 16)
			me->facing = 1;
		else
			me->facing = 0;
	}
	else
	{
		if (goodguy->y < me->y - FIXAMT * 16)
			me->facing = 6;
		else if (goodguy->y > me->y + FIXAMT * 16)
			me->facing = 2;
	}
}

#ifndef __MINGW32__

inline // Mingw complains about an undefinied reference to this from guy.cpp
#endif
int RangeToTarget(Guy *me, Guy *goodguy)
{
	return abs(me->x - goodguy->x) + abs(me->y - goodguy->y);
}

// this version doesn't insta-face, it rotates toward the right facing, and it has much
// more leeway than the 16 pixels of the other (it's for bigger creatures)

inline void FaceGoodguy2(Guy *me, Guy *goodguy)
{
	int desired;
	int diff, dir;

	if (goodguy->x < me->x - FIXAMT * 32)
	{
		if (goodguy->y < me->y - FIXAMT * 32)
			desired = 5;
		else if (goodguy->y > me->y + FIXAMT * 32)
			desired = 3;
		else
			desired = 4;
	}
	else if (goodguy->x > me->x + FIXAMT * 32)
	{
		if (goodguy->y < me->y - FIXAMT * 32)
			desired = 7;
		else if (goodguy->y > me->y + FIXAMT * 32)
			desired = 1;
		else
			desired = 0;
	}
	else
	{
		if (goodguy->y < me->y - FIXAMT * 32)
			desired = 6;
		else
			desired = 2;
	}

	if (desired == me->facing)
		return;

	if (desired > me->facing)
	{
		diff = desired - me->facing;
		if (diff > 4)
		{
			dir = -1;
			diff = 8 - diff;
		}
		else
			dir = 1;
	}
	else
	{
		diff = me->facing - desired;
		if (diff > 4)
		{
			dir = 1;
			diff = 8 - diff;
		}
		else
			dir = -1;
	}
	me->facing = (me->facing + dir)&7;
}

inline void FaceGoodguy3(Guy *me, Guy *goodguy)
{
	int desired;
	int diff, dir;

	if (abs(me->x - goodguy->x) + abs(me->y - goodguy->y) > FIXAMT * 72)
	{
		FaceGoodguy2(me, goodguy);
		return;
	}

	if (goodguy->x < me->x - FIXAMT * 16)
	{
		if (goodguy->y < me->y - FIXAMT * 16)
			desired = 5;
		else if (goodguy->y > me->y + FIXAMT * 16)
			desired = 3;
		else
			desired = 4;
	}
	else if (goodguy->x > me->x + FIXAMT * 16)
	{
		if (goodguy->y < me->y - FIXAMT * 16)
			desired = 7;
		else if (goodguy->y > me->y + FIXAMT * 16)
			desired = 1;
		else
			desired = 0;
	}
	else
	{
		if (goodguy->y < me->y - FIXAMT * 16)
			desired = 6;
		else
			desired = 2;
	}

	if (desired == me->facing)
		return;

	if (desired > me->facing)
	{
		diff = desired - me->facing;
		if (diff > 4)
		{
			dir = -1;
			diff = 8 - diff;
		}
		else
			dir = 1;
	}
	else
	{
		diff = me->facing - desired;
		if (diff > 4)
		{
			dir = 1;
			diff = 8 - diff;
		}
		else
			dir = -1;
	}
	me->facing = (me->facing + dir)&7;
}

// this is only used for The Thing's tentacles, to keep their flailing within quadrants

void FlailLock(Guy *me)
{
	byte parentangle;
	byte diff;
	char dir;

	if (!me->parent || me->parent->type == MONS_NONE)
		return; // no good

	if (me->parent->type != MONS_THINGTENT)
	{
		switch (me->mind) // which quadrant of the tentacles is he?
		{
			case 0: // lower left
				parentangle = 6;
				break;
			case 1: // lower right
				parentangle = 2;
				break;
			case 2: // upper right
				parentangle = 14;
				break;
			case 3: // upper left
				parentangle = 10;
				break;
			default:
				parentangle = me->parent->facing;
		}
	}
	else
	{
		parentangle = me->parent->facing;
	}

	if (parentangle > me->facing)
	{
		diff = parentangle - me->facing;
		if (diff > 8)
		{
			dir = 1;
			diff = 16 - diff;
		}
		else
			dir = -1;
	}
	else
	{
		diff = me->facing - parentangle;
		if (diff > 8)
		{
			dir = -1;
			diff = 16 - diff;
		}
		else
			dir = 1;
	}
	if (diff > 2)
	{
		me->facing = (parentangle + dir * 2)&15;
	}
}

// here be the AIs for each monster type
// --------------------------------------------------------------------------------------

#include "monster1.inl"
#ifdef EXPANDO
#include "monster2.inl"
#include "monster3.inl"
#endif
#include "monsterDefs.inl"
