#include "monster.h"
#include "player.h"
#include "options.h"

extern monsterType_t monsType[NUM_MONSTERS];

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
