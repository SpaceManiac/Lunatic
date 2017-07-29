#include "pause.h"
#include "player.h"
#include "options.h"

enum {
	SUBMODE_NONE = 0,
	SUBMODE_SLOTPICK
};

byte cursor = 0;
byte subcursor = 0;
static char lastKey = 0;
byte subMode;
float percent[3]; // the percentages in each save slot
byte giveUp = 0; // which text should be shown for "Give Up"

void HandlePauseKeyPresses(MGLDraw *mgl)
{
	char k;

	k = mgl->LastKeyPressed();
	if (k)
		lastKey = k;
}

void InitPauseMenu(void)
{
	FILE *f;
	player_t p;

	lastKey = 0;
	subMode = 0;

	f = AppdataOpen("loony.sav", "rb");
	if (!f)
	{
		percent[0] = 0.0;
		percent[1] = 0.0;
		percent[2] = 0.0;
	}
	else
	{
		fread(&p, sizeof (player_t), 1, f);
		percent[0] = CalcTotalPercent(&p)*100;
		fread(&p, sizeof (player_t), 1, f);
		percent[1] = CalcTotalPercent(&p)*100;
		fread(&p, sizeof (player_t), 1, f);
		percent[2] = CalcTotalPercent(&p)*100;
		fclose(f);
	}
	MakeNormalSound(SND_PAUSE);
}

byte UpdatePauseMenu(MGLDraw *mgl)
{
	byte c;
	static byte oldc = 0;
	static byte reptCounter = 0;

	if (giveUp == 2 && cursor == 4) // not allowed in world picker pause
		cursor = 0;

	c = GetControls() | GetArrows();

	reptCounter++;
	if ((!oldc) || (reptCounter > 10))
		reptCounter = 0;

	if (subMode == SUBMODE_NONE) // not in any submenu
	{
		if ((c & CONTROL_UP) && (!reptCounter))
		{
			cursor--;
			if (cursor == 255)
				cursor = 5;

			if (giveUp == 2 && cursor == 4) // world picker pause has no option 4
				cursor = 3;

			MakeNormalSound(SND_MENUCLICK);
		}
		if ((c & CONTROL_DN) && (!reptCounter))
		{
			cursor++;
			if (cursor == 6)
				cursor = 0;
			if (giveUp == 2 && cursor == 4) // world picker pause has no option 4
				cursor = 5;

			MakeNormalSound(SND_MENUCLICK);
		}
		if (((c & CONTROL_B1) && (!(oldc & CONTROL_B1))) ||
				((c & CONTROL_B2) && (!(oldc & CONTROL_B2))))
		{
			MakeNormalSound(SND_MENUSELECT);
			switch (cursor) {
				case 0: // cancel
					return 0;
				case 1: // Load
					subMode = SUBMODE_SLOTPICK;
					break;
				case 2: // Save
					subMode = SUBMODE_SLOTPICK;
					break;
				case 3: // music
					CDNeedsUpdating();
					CDStop();
					PlayerSetMusicSettings((PlayerGetMusicSettings() + 1) % 3);
					if (PlayerGetMusicSettings() == MUSIC_ON)
						CDPlay(GetCurSong());
					opt.music = PlayerGetMusicSettings();
					break;
				case 4: // give up
					return 2;
				case 5: // quit game
					return 3;
			}
		}
	}
	else if (subMode == SUBMODE_SLOTPICK)
	{
		if ((c & CONTROL_UP) && (!reptCounter))
		{
			MakeNormalSound(SND_MENUCLICK);
			subcursor--;
			if (subcursor == 255)
				subcursor = 2;
		}
		if ((c & CONTROL_DN) && (!reptCounter))
		{
			MakeNormalSound(SND_MENUCLICK);
			subcursor++;
			if (subcursor == 3)
				subcursor = 0;
		}
		if (((c & CONTROL_B1) && (!(oldc & CONTROL_B1))) ||
				((c & CONTROL_B2) && (!(oldc & CONTROL_B2))))
		{
			MakeNormalSound(SND_MENUSELECT);
			if (cursor == 1) // Load
			{
				SendMessageToGame(MSG_LOADGAME, 0);
				PlayerLoadGame(subcursor);
				MakeNormalSound(SND_LOADGAME);
				return 0;
			}
			else if (cursor == 2) // Save
			{
				PlayerSaveGame(subcursor);
				MakeNormalSound(SND_SAVEGAME);
				return 0;
			}
			subMode = SUBMODE_NONE;
		}
	}
	oldc = c;

	HandlePauseKeyPresses(mgl);
	if (lastKey == 27) // hit ESC to exit pause menu
	{
		MakeNormalSound(SND_MENUSELECT);
		if (subMode == SUBMODE_NONE)
			return 0;
		else
			subMode = SUBMODE_NONE;
		lastKey = 0;
	}
	return 1;
}
