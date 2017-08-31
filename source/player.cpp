#include "player.h"
#include "music.h"
#include "rage.h"
#include "options.h"
#include "title.h"

const int PLYR_ACCEL = FIXAMT;
const int PLYR_DECEL = FIXAMT * 3 / 4;
const int PLYR_MAXSPD = FIXAMT * 6;

player_t player;
byte playerGlow; // for torch-lit levels, and for exciting moments
byte tportclock;

extern "C" void KeyChainAllCheck(void);

byte PlayerGetItem(byte itm, int x, int y)
{
	int cx, cy;

	GetCamera(&cx, &cy);
	cx <<= FIXSHIFT;
	cy <<= FIXSHIFT;

	// won't pick up other weapons when in power armor
	if (player.weapon == WPN_PWRARMOR && (itm == ITM_MISSILES || itm == ITM_AK8087 ||
			itm == ITM_FLAME || itm == ITM_BOMBS || itm == ITM_BIGAXE || itm == ITM_SPEAR ||
			itm == ITM_MACHETE || itm == ITM_MINES || itm == ITM_LIGHTNING || itm == ITM_TURRETWPN ||
			itm == ITM_JETPACK || itm == ITM_MINDCONTROL || itm == ITM_REFLECTOR || itm == ITM_SWAPGUN))
		return 1;

	switch (itm) {
		case ITM_HAMMERUP:
			if (player.hammers < 5)
				player.hammers++;
			MakeNormalSound(SND_HAMMERUP);
			NewMessage("HAMMER UP!", 75, 0);
			player.score += 150;
			return 0;
			break;
		case ITM_PANTS:
			if (player.hamSpeed > 0)
				player.hamSpeed -= 4;
			NewMessage("PANTS OF POWER!", 75, 0);
			MakeNormalSound(SND_PANTS);
			player.score += 100;
			return 0;
			break;
		case ITM_REVERSE:
			player.hammerFlags |= HMR_REVERSE;
			NewMessage("REVERSE HAMMER!", 75, 0);
			MakeNormalSound(SND_REVERSE);
			player.score += 100;
			return 0;
			break;
		case ITM_REFLECT:
			player.hammerFlags |= HMR_REFLECT;
			NewMessage("REFLECT HAMMER!", 75, 0);
			MakeNormalSound(SND_REFLECT);
			player.score += 100;
			return 0;
			break;
		case ITM_MISSILES:
			if (player.weapon == WPN_MISSILES && player.ammo == 20) // don't get it
				return 1;
			MakeNormalSound(SND_WEAPON);
			NewMessage("MISSILE PACK!", 75, 0);
			player.weapon = WPN_MISSILES;
			player.ammo = 20;
			player.score += 50;
			return 0;
			break;
		case ITM_FLAME:
			if (player.weapon == WPN_FLAME && player.ammo == 50) // don't get it
				return 1;
			MakeNormalSound(SND_WEAPON);
			NewMessage("FLAMETHROWER!", 75, 0);
			player.weapon = WPN_FLAME;
			player.ammo = 50;
			player.score += 50;
			return 0;
			break;
		case ITM_BOMBS:
			if (player.weapon == WPN_BOMBS && player.ammo == 5) // don't get it
				return 1;
			MakeNormalSound(SND_WEAPON);
			NewMessage("CHERRY BOMBS!", 75, 0);
			player.weapon = WPN_BOMBS;
			player.ammo = 5;
			player.score += 50;
			return 0;
			break;
		case ITM_AK8087:
			if (player.weapon == WPN_AK8087 && player.ammo == 99) // don't get it
				return 1;
			MakeNormalSound(SND_WEAPON);
			NewMessage("AK-8087!", 75, 0);
			player.weapon = WPN_AK8087;
			player.ammo = 99;
			player.score += 50;
			return 0;
			break;
		case ITM_TAKEOUT:
			if (player.life == 128)
				return 1; // don't need the food

			MakeNormalSound(SND_FOOD);
			goodguy->poison = 0;
			switch (MGL_random(8)) {
				case 0:
					NewMessage("Just fried rice", 75, 0);
					PlayerHeal(5);
					break;
				case 1:
					NewMessage("Mmm, Kung Pao!", 75, 0);
					PlayerHeal(25);
					break;
				case 2:
					NewMessage("Szechwan Shredded Pork!!", 75, 0);
					PlayerHeal(50);
					break;
				case 3:
					NewMessage("Moo Shi Beef!", 75, 0);
					PlayerHeal(25);
					break;
				case 4:
					NewMessage("Ick, mostly veggies", 75, 0);
					PlayerHeal(1);
					break;
				case 5:
					NewMessage("Hmm, tastes kind of citrusy", 75, 0);
					PlayerHeal(15);
					break;
				case 6:
					NewMessage("Joy! Wontons!", 75, 0);
					PlayerHeal(20);
					break;
				case 7:
					NewMessage("GeneralTsao has outdone himself", 75, 0);
					PlayerHeal(40);
					break;
			}
			player.score += 50;
			return 0;
			break;
		case ITM_SHIELD:
			MakeNormalSound(SND_SHIELD);
			player.shield = 240;
			player.score += 50;
			NewMessage("Energy Barrier!", 30, 0);
			return 0;
			break;
		case ITM_BRAIN:
			MakeNormalSound(SND_GETBRAIN);
			player.brains++;
			if (player.brains >= TotalBrains())
			{
				player.brains = TotalBrains();
				if (opt.playAs == PLAYAS_BOUAPHA)
					MakeNormalSound(SND_KOOLKAT);
				else if (opt.playAs == PLAYAS_LUNATIC)
					MakeNormalSound(SND_DRLLAUGH);
				else
					MakeNormalSound(SND_HAPPYDIE);

				playerGlow = 127;
			}
			player.score += 25;
			return 0;
			break;
		case ITM_KEY:
			if (player.keys[0] < 3)
			{
				player.keys[0]++;
				MakeNormalSound(SND_GETKEY);
				return 0;
			}
			else
				return 1; // don't pick it up
			break;
		case ITM_KEYR:
			MakeNormalSound(SND_GETKEY);
			player.keys[1] = 1;
			return 0;
			break;
		case ITM_KEYG:
			MakeNormalSound(SND_GETKEY);
			player.keys[2] = 1;
			return 0;
			break;
		case ITM_KEYB:
			MakeNormalSound(SND_GETKEY);
			player.keys[3] = 1;
			return 0;
			break;
		case ITM_KEYCH1:
			MakeNormalSound(SND_GETKEYCHAIN);
			player.keychain[player.worldNum][0] = 1;
			KeyChainAllCheck();
			player.score += 50000;
			player.prevScore += 50000;
			NewMessage("Cool! A pumpkin keychain!", 75, 0);
			return 0;
			break;
		case ITM_KEYCH2:
			MakeNormalSound(SND_GETKEYCHAIN);
			player.keychain[player.worldNum][1] = 1;
			KeyChainAllCheck();
			player.score += 50000;
			player.prevScore += 50000;
			NewMessage("Cool! A hammer keychain!", 75, 0);
			return 0;
			break;
		case ITM_KEYCH3:
			MakeNormalSound(SND_GETKEYCHAIN);
			player.keychain[player.worldNum][2] = 1;
			KeyChainAllCheck();
			player.score += 50000;
			player.prevScore += 50000;
			NewMessage("Cool! A rocket keychain!", 75, 0);
			return 0;
			break;
		case ITM_KEYCH4:
			MakeNormalSound(SND_GETKEYCHAIN);
			player.keychain[player.worldNum][3] = 1;
			KeyChainAllCheck();
			player.score += 50000;
			player.prevScore += 50000;
			NewMessage("Cool! A squash keychain!", 75, 0);
			return 0;
			break;
		case ITM_LOONYKEY:
			MakeNormalSound(SND_LOONYKEY);
			player.lunacyKey[player.worldNum] = 1;
			player.score += 100000;
			player.prevScore += 100000;
			return 0;
			break;
#ifdef EXPANDO
		case ITM_PWRARMOR:
			MakeNormalSound(SND_ROBOBOUAPHAON);
			NewMessage("POWER ARMOR!!", 75, 0);
			player.weapon = WPN_PWRARMOR;
			player.ammo = 1000;
			player.score += 150;
			player.shield = 0;
			goodguy->seq = ANIM_A2;
			goodguy->frm = 0;
			goodguy->frmTimer = 0;
			goodguy->frmAdvance = 64;
			goodguy->action = ACTION_BUSY;
			return 0;
			break;
		case ITM_BIGAXE:
			if (player.weapon == WPN_BIGAXE && player.ammo == 15) // don't get it
				return 1;
			MakeNormalSound(SND_WEAPON);
			NewMessage("BIG AXE!", 75, 0);
			player.weapon = WPN_BIGAXE;
			player.ammo = 15;
			player.score += 50;
			return 0;
			break;
		case ITM_LIGHTNING:
			if (player.weapon == WPN_LIGHTNING && player.ammo == 40) // don't get it
				return 1;
			MakeNormalSound(SND_WEAPON);
			NewMessage("LIGHTNING ROD!", 75, 0);
			player.weapon = WPN_LIGHTNING;
			player.ammo = 40;
			player.score += 50;
			return 0;
			break;
		case ITM_SPEAR:
			if (player.weapon == WPN_SPEAR && player.ammo == 20) // don't get it
				return 1;
			MakeNormalSound(SND_WEAPON);
			NewMessage("SPEARS!", 75, 0);
			player.weapon = WPN_SPEAR;
			player.ammo = 20;
			player.score += 50;
			return 0;
			break;
		case ITM_MACHETE:
			if (player.weapon == WPN_MACHETE && player.ammo == 30) // don't get it
				return 1;
			MakeNormalSound(SND_WEAPON);
			NewMessage("MACHETE!", 75, 0);
			player.weapon = WPN_MACHETE;
			player.ammo = 30;
			player.score += 50;
			return 0;
			break;
		case ITM_MINES:
			if (player.weapon == WPN_MINES && player.ammo == 8) // don't get it
				return 1;
			MakeNormalSound(SND_WEAPON);
			NewMessage("MINES!", 75, 0);
			player.weapon = WPN_MINES;
			player.ammo = 8;
			player.score += 50;
			return 0;
			break;
		case ITM_GARLIC:
			NewMessage("Fresh Garlic!", 75, 0);
			player.garlic = 255;
			return 0;
			break;
		case ITM_ORBITER:
			FireBullet(goodguy->x, goodguy->y, goodguy->facing, BLT_ORBITER, 1);
			NewMessage("ORBITER!", 75, 0);
			return 0;
			break;
		case ITM_ACCEL:
			MakeNormalSound(SND_SPEEDUP);
			NewMessage("PARTICLE ACCELERATOR!", 75, 0);
			player.speed = 255;
			return 0;
			break;
		case ITM_TURRETWPN:
			if (player.weapon == WPN_TURRET && player.ammo == 3) // don't get it
				return 1;
			MakeNormalSound(SND_WEAPON);
			NewMessage("TURRETS!", 75, 0);
			player.weapon = WPN_TURRET;
			player.ammo = 3;
			player.score += 50;
			return 0;
			break;
		case ITM_MINDCONTROL:
			if (player.weapon == WPN_MINDCONTROL && player.ammo == 1) // don't get it
				return 1;
			MakeNormalSound(SND_WEAPON);
			NewMessage("MIND CONTROL RAY!", 75, 0);
			player.weapon = WPN_MINDCONTROL;
			player.ammo = 1;
			player.score += 50;
			return 0;
			break;
		case ITM_REFLECTOR:
			if (player.weapon == WPN_REFLECTOR && player.ammo == 40) // don't get it
				return 1;
			MakeNormalSound(SND_WEAPON);
			NewMessage("REFLECTOR SHIELD!", 75, 0);
			player.weapon = WPN_REFLECTOR;
			player.ammo = 40;
			player.score += 50;
			return 0;
			break;
		case ITM_INVIS:
			MakeNormalSound(SND_ROBOBOUAPHAON);
			NewMessage("THE CLOAK INVISIBLE!", 75, 0);
			player.score += 50;
			player.invisibility = 255;
			return 0;
			break;
		case ITM_JETPACK:
			if (player.weapon == WPN_JETPACK && player.ammo == 10) // don't get it
				return 1;
			MakeNormalSound(SND_WEAPON);
			NewMessage("JETPACK!", 75, 0);
			player.weapon = WPN_JETPACK;
			player.ammo = 10;
			player.score += 50;
			return 0;
			break;
		case ITM_SWAPGUN:
			if (player.weapon == WPN_SWAPGUN && player.ammo == 3) // don't get it
				return 1;
			MakeNormalSound(SND_WEAPON);
			NewMessage("SWAPGUN!", 75, 0);
			player.weapon = WPN_SWAPGUN;
			player.ammo = 3;
			player.score += 50;
			return 0;
			break;
		case ITM_UNHAMMER:
			if (player.hammers > 0)
			{
				player.hammers--;
				player.score -= 150;
			}
			MakeNormalSound(SND_HAMMERDN);
			NewMessage("HAMMER DOWN!", 75, 0);
			return 0;
			break;
		case ITM_UNPANTS:
			if (player.hamSpeed < 16)
			{
				player.hamSpeed += 4;
				player.score -= 100;
			}
			NewMessage("PANTS OF SLOWNESS!", 75, 0);
			MakeNormalSound(SND_PANTSDN);
			return 0;
			break;
		case ITM_BADCHINESE:
			MakeNormalSound(SND_FOOD);
			goodguy->poison = 0;
			switch (MGL_random(6)) {
				case 0:
					NewMessage("Radioactive Energy!!", 75, 0);
					PlayerHeal(30);
					player.speed = 255;
					break;
				case 1:
					NewMessage("Szechwan Surprise!", 75, 0);
					PlayerHeal(128);
					break;
				case 2:
					NewMessage("Filling... with... RAGE!!", 75, 0);
					PlayerHeal(50);
					player.rage = 127 * 256;
					break;
				case 3:
					NewMessage("Toxic Mu Shu!", 75, 0);
					goodguy->poison = 255;
					break;
				case 4:
					NewMessage("Thermonuclear Heartburn!", 75, 0);
					goodguy->GetShot(0, 0, 50, GameCurrentMap(), &curWorld);
					break;
				case 5:
					NewMessage("Atomic Inviso-Power!", 75, 0);
					player.invisibility = 255;
					break;
			}
			player.score += 50;
			return 0;
			break;
#endif
	}
	return 1;
}

extern "C" void PlayerThrowHammer(Guy *me);

extern "C" void DoPlayerFacing(byte c, Guy *me);

extern "C" void PlayerFireWeapon(Guy *me);

extern "C" void PlayerFirePowerArmor(Guy *me, byte mode);

void PlayerControlMe(Guy *me, mapTile_t *mapTile, world_t *world)
{
	byte c;
	int x, y, i;

	player.life = me->hp;

	if (player.rage)
	{
		if (player.rage > 5)
			player.rage -= 6;
		else
			player.rage = 0;
	}
	if (player.rageClock)
		DoRage(me);

	if (player.invisibility)
		player.invisibility--;

	if (player.jetting && me->seq != ANIM_DIE && me->seq != ANIM_A3)
	{
		me->dx += Cosine(me->facing * 32)*6;
		me->dy += Sine(me->facing * 32)*6;
		Clamp(&me->dx, FIXAMT * 20);
		Clamp(&me->dy, FIXAMT * 20);

		if (me->z < FIXAMT * 20)
			me->z += FIXAMT * 4;
		me->dz = 0;

		MakeSound(SND_FLAMEGO, me->x, me->y, SND_CUTOFF, 1200);
		for (i = 0; i < 3; i++)
		{
			c = ((me->facing + 4)&7)*32;
			x = me->x + Cosine(c)*10 - FIXAMT * 10 + MGL_randoml(FIXAMT * 20);
			y = me->y + Sine(c)*10 - FIXAMT * 10 + MGL_randoml(FIXAMT * 20);
			FireBullet(x, y, (me->facing + 4)&7, BLT_FLAME, 1);
		}
		player.jetting--;
	}

	if (player.weapon == WPN_PWRARMOR)
	{
		PlayerControlPowerArmor(me, mapTile, world);
		return;
	}

	if (player.reload)
		player.reload--;
	if (player.wpnReload)
		player.wpnReload--;

	if (player.garlic)
	{
		player.garlic--;
		StinkySteam(me->x - FIXAMT * 20 + MGL_randoml(FIXAMT * 40), me->y - FIXAMT * 20 + MGL_randoml(FIXAMT * 40),
				me->z + FIXAMT * 40, FIXAMT * 2);
	}

	if (player.shield)
		player.shield--;

	if (player.pushPower)
		player.pushPower--;

	if (tportclock)
		tportclock--;

	// ice is slippery
	if (!(world->terrain[mapTile->floor].flags & TF_ICE))
	{
		if (player.jetting && me->mind1)
		{
			if (me->mind1 & 1)
			{
				me->dx = -me->dx;
				switch (me->facing) {
					case 0:
						me->facing = 4;
						break;
					case 1:
						me->facing = 3;
						break;
					case 2:
					case 6:
						break;
					case 3:
						me->facing = 1;
						break;
					case 4:
						me->facing = 0;
						break;
					case 5:
						me->facing = 7;
						break;
					case 7:
						me->facing = 5;
						break;
				}
			}
			if (me->mind1 & 2)
			{
				me->dy = -me->dy;
				switch (me->facing) {
					case 0:
					case 4:
						break;
					case 1:
						me->facing = 7;
						break;
					case 2:
						me->facing = 6;
						break;
					case 3:
						me->facing = 5;
						break;
					case 5:
						me->facing = 3;
						break;
					case 6:
						me->facing = 2;
						break;
					case 7:
						me->facing = 1;
						break;
				}
			}
		}
		Dampen(&me->dx, PLYR_DECEL);
		Dampen(&me->dy, PLYR_DECEL);
	}
	else
	{
		if (me->mind1) // bumped a wall while on ice
		{
			if (me->mind1 & 1)
				me->dx = -me->dx / 8;
			if (me->mind1 & 2)
				me->dy = -me->dy / 8;
		}
	}
	me->mind1 = 0;

	if (me->ouch == 4)
	{
		if (opt.playAs == PLAYAS_BOUAPHA)
		{
			if (me->hp > 0)
				MakeSound(SND_BOUAPHAOUCH, me->x, me->y, SND_CUTOFF | SND_ONE, 2000);
			else if (me->seq == ANIM_DIE) // so it doesn't do this if you're drowning
				MakeSound(SND_BOUAPHADIE, me->x, me->y, SND_CUTOFF | SND_ONE, 2000);
		}
		else if (opt.playAs == PLAYAS_LUNATIC)
		{
			if (me->hp > 0)
				MakeSound(SND_DRLOUCH, me->x, me->y, SND_CUTOFF | SND_ONE, 2000);
			else if (me->seq == ANIM_DIE) // so it doesn't do this if you're drowning
				MakeSound(SND_DRLDIE, me->x, me->y, SND_CUTOFF | SND_ONE, 2000);
		}
		else
		{
			if (me->hp > 0)
				MakeSound(SND_HAPPYOUCH, me->x, me->y, SND_CUTOFF | SND_ONE, 2000);
			else if (me->seq == ANIM_DIE) // so it doesn't do this if you're drowning
				MakeSound(SND_HAPPYDIE, me->x, me->y, SND_CUTOFF | SND_ONE, 2000);
		}
	}

	if (me->parent) // being grabbed by a Super Zombie or something
	{
		if (me->parent->type == MONS_SUPERZOMBIE)
		{
			me->dz = 0;
			if (me->parent->frm < 4)
				me->z += FIXAMT * 3;
			else if (me->parent->frm > 18)
			{
				me->z -= FIXAMT * 4;
				if (me->parent->frm == 21)
				{
					me->z = 0;
					me->parent = NULL;
					me->action = ACTION_IDLE;
					if (me->hp == 0)
					{
						me->seq = ANIM_DIE;
						me->frm = 0;
						me->frmTimer = 0;
						me->frmAdvance = 64;
						me->action = ACTION_BUSY;
					}
					return;
				}
			}
			if (me->seq != ANIM_MOVE)
			{
				me->seq = ANIM_MOVE;
				me->frm = 0;
				me->frmTimer = 0;
				me->frmAdvance = 512;
			}
			return;
		}
		else if (me->parent->type == MONS_MINECART)
		{
			me->x = me->parent->x;
			me->y = me->parent->y + 1;
			me->z = FIXAMT * 8;
		}
		else
		{
			me->parent = NULL;
		}
	}

	// triggering stuff
	if (me->action == ACTION_BUSY)
	{
		// throw hammer if need be, use item if need be
		if (me->seq == ANIM_A1 && me->frm == 2 && player.wpnReload == 0)
		{
			PlayerFireWeapon(me);
			return;
		}

		if (me->seq == ANIM_A3)
		{
			if (me->frm < 11)
			{
				me->z = FIXAMT * 8; // hover while spinning feet in the air before plunging into water
				me->dz = FIXAMT;
			}
			else
			{
				ExplodeParticles(PART_WATER, me->x, me->y, 0, 16);
			}
			return;
		}
		if (me->seq == ANIM_DIE)
		{
			me->facing = (me->facing + 1)&7;
			return;
		}
		if (me->seq == ANIM_A1)
			return;
	}

	// not busy, let's see if you want to do something
	c = GetControls();

	if (!player.jetting)
		DoPlayerFacing(c, me);

	if (me->action == ACTION_IDLE)
	{
		if ((c & (CONTROL_B1 | CONTROL_B2)) == (CONTROL_B1 | CONTROL_B2) && (player.rage / 256) >= player.life)
		{
			// RAGE!!!!!!!
			player.rage = 0;
			player.rageClock = 15;
			if (player.shield == 0)
				player.shield = 30;
			EnterRage();
		}
		if ((c & CONTROL_B1) && player.reload == 0) // pushed hammer throw button
		{
			me->action = ACTION_IDLE;
			if (!(c & (CONTROL_UP | CONTROL_DN | CONTROL_LF | CONTROL_RT)))
			{
				me->seq = ANIM_ATTACK; // even if unarmed
				me->frm = 0;
				me->frmTimer = 0;
				me->frmAdvance = 255;
				me->frm += 4 - (player.hamSpeed >> 2);
			}
			player.boredom = 0;
			if (player.hammers > 0)
				PlayerThrowHammer(me);
			player.reload += (10 - (4 - (player.hamSpeed >> 2)));
		}
		if ((c & CONTROL_B2) && player.wpnReload == 0 && player.weapon) // pushed wpn use button
		{
			me->action = ACTION_BUSY;
			me->seq = ANIM_A1;
			me->frm = 0;
			me->frmTimer = 0;
			me->frmAdvance = 200;
			player.boredom = 0;
			return;
		}
	}

	// if you are moving indeed
	if ((c & (CONTROL_UP | CONTROL_DN | CONTROL_LF | CONTROL_RT)) && !player.jetting)
	{
		if (!(world->terrain[mapTile->floor].flags & TF_ICE))
		{
			me->dx += Cosine(me->facing * 32)*3 / 2;
			me->dy += Sine(me->facing * 32)*3 / 2;
			if ((Cosine(me->facing * 32) < 0 && me->dx > 0) || (Cosine(me->facing * 32) > 0 && me->dx < 0))
				me->dx = 0;
			if ((Sine(me->facing * 32) < 0 && me->dy > 0) || (Sine(me->facing * 32) > 0 && me->dy < 0))
				me->dy = 0;
		}
		else // ice is slippery
		{
			me->dx += Cosine(me->facing * 32) / 4;
			me->dy += Sine(me->facing * 32) / 4;
		}

		Clamp(&me->dx, PLYR_MAXSPD);
		Clamp(&me->dy, PLYR_MAXSPD);

		if (player.reload > 0)
		{
			Clamp(&me->dx, PLYR_MAXSPD / 2);
			Clamp(&me->dy, PLYR_MAXSPD / 2);
		}

		if (me->seq != ANIM_MOVE)
		{
			me->seq = ANIM_MOVE;
			me->frm = 0;
			me->frmTimer = 0;
			me->frmAdvance = 128;
		}
		player.boredom = 0;
	}
	else // you aren't trying to move
	{
		// ice is slippery
		if (!(world->terrain[mapTile->floor].flags & TF_ICE))
		{
			Dampen(&me->dx, PLYR_DECEL / 2);
			Dampen(&me->dy, PLYR_DECEL / 2);
		}
		if (me->seq == ANIM_MOVE)
		{
			me->seq = ANIM_IDLE;
			me->frm = 0;
			me->frmTimer = 0;
			me->frmAdvance = 128;
		}
	}

	// boredom handler
	if (me->action == ACTION_IDLE && !player.jetting)
	{
		player.boredom++;
		if (player.boredom > 200 + MGL_random(3200))
		{
			MakeSound(SND_BOUAPHABORED, me->x, me->y, SND_CUTOFF | SND_ONE, 2000);
			me->seq = ANIM_A2;
			me->frm = 0;
			me->frmTimer = 0;
			me->frmAdvance = 64;
			player.boredom = 0;
		}
	}
}

void PlayerControlPowerArmor(Guy *me, mapTile_t *mapTile, world_t *world)
{
	byte c;

	if (player.reload)
		player.reload--;
	if (player.wpnReload)
		player.wpnReload--;

	if (player.garlic)
	{
		player.garlic--;
		StinkySteam(me->x - FIXAMT * 20 + MGL_randoml(FIXAMT * 40), me->y - FIXAMT * 20 + MGL_randoml(FIXAMT * 40),
				me->z + FIXAMT * 40, FIXAMT * 2);
	}

	if (player.shield)
		player.shield = 0;

	if (player.pushPower)
		player.pushPower--;

	if (tportclock)
		tportclock--;

	if (me->poison)
		// can't be poisoned in armor
		me->poison = 0;

	// ice is not slippery for armor
	Dampen(&me->dx, PLYR_DECEL);
	Dampen(&me->dy, PLYR_DECEL);
	me->mind1 = 0;
	player.boredom = 0;

	if (player.ammo)
		player.ammo--;

	if (player.ammo == 0 && me->seq != ANIM_DIE)
	{
		me->seq = ANIM_DIE;
		me->frm = 0;
		me->frmAdvance = 64;
		me->frmTimer = 0;
		me->action = ACTION_BUSY;
	}

	if (me->ouch == 4)
	{
		// switch these to robotic bouapha noises someday
		if (me->hp > 0)
			MakeSound(SND_ROBOBOUAPHAOUCH, me->x, me->y, SND_CUTOFF | SND_ONE, 2000);
	}

	if (me->parent) // being grabbed by a Super Zombie or something
	{
		if (me->parent->type == MONS_MINECART)
		{
			me->x += me->parent->dx;
			me->y += me->parent->dy;
		}
		else // note Super Zombie can't grab you
		{
			me->parent = NULL;
		}
	}

	// not busy, let's see if you want to do something
	c = GetControls();

	// triggering stuff
	if (me->action == ACTION_BUSY)
	{
		// throw hammer or fire missile
		if (me->seq == ANIM_ATTACK && me->frm == 3 && me->frmTimer < 64 && player.reload == 0)
		{
			PlayerFirePowerArmor(me, 1);
		}
		if (me->seq == ANIM_ATTACK && me->frm == 6 && player.ammo > 0)
		{
			if (c & CONTROL_B1)
			{
				me->frm = 2;
				me->frmTimer = 128;
				player.reload = 0;
			}
			else
				player.reload = 16; // if you stop firing, it takes a while to start again
		}
		if (me->seq == ANIM_A1 && me->frm == 2 && player.wpnReload == 0)
		{
			PlayerFirePowerArmor(me, 2);
			player.wpnReload = 40;
		}

		if (me->seq == ANIM_DIE)
		{
			me->frmAdvance = 64;
			if (me->frmTimer < 65)
				FireBullet(me->x - FIXAMT * 64 + MGL_randoml(FIXAMT * 128), me->y - FIXAMT * 64 + MGL_randoml(FIXAMT * 128),
					0, BLT_BOOM, 1);
		}

		return;
	}

	DoPlayerFacing(c, me);

	if ((c & CONTROL_B1) && player.reload == 0) // pushed fire button
	{
		me->action = ACTION_BUSY;
		me->seq = ANIM_ATTACK;
		me->frm = 0;
		me->frmTimer = 0;
		me->frmAdvance = 128;
		return;
	}
	if ((c & CONTROL_B2) && player.wpnReload == 0) // pushed missile button
	{
		me->action = ACTION_BUSY;
		me->seq = ANIM_A1;
		me->frm = 0;
		me->frmTimer = 0;
		me->frmAdvance = 200;
		return;
	}

	// if you are moving indeed
	if (c & (CONTROL_UP | CONTROL_DN | CONTROL_LF | CONTROL_RT))
	{
		me->dx += Cosine(me->facing * 32)*3 / 2;
		me->dy += Sine(me->facing * 32)*3 / 2;
		if ((Cosine(me->facing * 32) < 0 && me->dx > 0) || (Cosine(me->facing * 32) > 0 && me->dx < 0))
			me->dx = 0;
		if ((Sine(me->facing * 32) < 0 && me->dy > 0) || (Sine(me->facing * 32) > 0 && me->dy < 0))
			me->dy = 0;

		// move half speed in armor
		Clamp(&me->dx, PLYR_MAXSPD / 2);
		Clamp(&me->dy, PLYR_MAXSPD / 2);

		if (me->seq != ANIM_MOVE)
		{
			me->seq = ANIM_MOVE;
			me->frm = 0;
			me->frmTimer = 0;
			me->frmAdvance = 128;
		}
		player.boredom = 0;
	}
	else // you aren't trying to move
	{
		Dampen(&me->dx, PLYR_DECEL / 2);
		Dampen(&me->dy, PLYR_DECEL / 2);
		if (me->seq == ANIM_MOVE)
		{
			me->seq = ANIM_IDLE;
			me->frm = 0;
			me->frmTimer = 0;
			me->frmAdvance = 128;
		}
	}
}
