#include "game.h"
#include "title.h"
#include "jamulfmv.h"
#include "rage.h"
#include "options.h"

byte showStats = 0;
dword gameStartTime, visFrameCount, updFrameCount;
dword tickerTime;
dword CDMessingTime; // how long the CD messing with took, take it out of the time budget, because
// it can bog the game, but it should just freeze the action
dword garbageTime = 0;

int visFrms;
float frmRate;
word numRunsToMakeUp;

char lastKey = 0;

MGLDraw *gamemgl;
Map *game_curMap;
#define curMap game_curMap
byte gameMode = GAMEMODE_PLAY;
byte mapToGoTo;
byte worldNum;
byte mapNum;
byte curMapFlags;
world_t curWorld;

byte msgFromOtherModules = 0;
byte msgContent;

word windingDown;
byte windingUp;
byte windDownReason;
byte game_idleGame = 0;
#define idleGame game_idleGame
FILE *logFile;

byte LunaticRun(int *lastTime)
{
	numRunsToMakeUp = 0;
	if (*lastTime > TIME_PER_FRAME * 30)
		*lastTime = TIME_PER_FRAME * 30;

	while (*lastTime >= TIME_PER_FRAME)
	{
		if (!gamemgl->Process())
		{
			mapToGoTo = 255;
			return LEVEL_ABORT;
		}

		if (gameMode == GAMEMODE_PLAY)
		{
			// update everything here
			if (!windingDown)
			{
				if (windingUp)
				{
					curMap->Update(UPDATE_FADEIN, &curWorld);
					EditorUpdateGuys(curMap);
					windingUp--;
				}
				else
				{
					curMap->Update(UPDATE_GAME, &curWorld);
					UpdateGuys(curMap, &curWorld);
					UpdateBullets(curMap, &curWorld);
					SpecialAnytimeCheck(curMap);
				}
			}
			else
			{
				curMap->Update(UPDATE_FADE, &curWorld);
				EditorUpdateGuys(curMap);
			}
			UpdateParticles(curMap);
			UpdateMessage();

			if (curMap->flags & MAP_SNOWING)
				MakeItSnow(curMap);

			if (windingDown)
			{
				windingDown--;
				if (!windingDown)
					return windDownReason;
			}
		}
		else if (gameMode == GAMEMODE_MENU)
		{
			switch (UpdatePauseMenu(gamemgl)) {
				case 0:
					lastKey = 0;
					gameMode = GAMEMODE_PLAY;
					break;
				case 1:
					break;
				case 2:
					if (mapNum)
						mapToGoTo = 0;
					else
						mapToGoTo = 255;
					lastKey = 0;
					return LEVEL_ABORT;
					break;
				case 3:
					mapToGoTo = 255;
					lastKey = 0;
					return WORLD_QUITGAME; // dump out altogether
					break;
			}
		}
		else if (gameMode == GAMEMODE_PIC) // gamemode_pic
		{
			if (GetTaps()&(CONTROL_B1 | CONTROL_B2))
			{
				gameMode = GAMEMODE_PLAY;
				// restore the palette
				gamemgl->LoadBMP("graphics\\title.bmp");
			}
		}
		else // gamemode_rage
		{
			UpdateRage(gamemgl);
			if (player.rageClock)
				player.rageClock--;
			else
			{
				gameMode = GAMEMODE_PLAY;
				StartRaging();
			}
			if (goodguy)
				goodguy->facing = (goodguy->facing + 1)&7;
		}

		if (msgFromOtherModules == MSG_NEWFEATURE)
		{
			NewMessage("** NEW FEATURE ADDED!! **", 120, 1);
			msgFromOtherModules = MSG_NONE;
		}
		else if (msgFromOtherModules == MSG_GOTOMAP)
		{
			mapToGoTo = msgContent;
			windingDown = 30;
			windDownReason = LEVEL_ABORT;
			msgFromOtherModules = MSG_NONE;
		}
		else if (msgFromOtherModules == MSG_WINLEVEL)
		{
			mapToGoTo = msgContent;
			windingDown = 40;
			windDownReason = LEVEL_WIN;
			msgFromOtherModules = MSG_NONE;
			if (player.worldNum == 4 && player.levelNum == 6)
			{

				ShowVictoryAnim(4); // you killed him.
				SendMessageToGame(MSG_WINGAME, 0);
			}
			player.boredom = 0;
		}
		else if (msgFromOtherModules == MSG_RESET)
		{
			if (opt.youSuck)
				NewBigMessage("You Suck", 30);
			else
				NewBigMessage("Try Again!", 30);
			windingDown = 30;
			windDownReason = LEVEL_RESET;
			msgFromOtherModules = MSG_NONE;
		}
		else if (msgFromOtherModules == MSG_LOADGAME)
		{
			NewBigMessage("Loading Game", 30);
			windingDown = 30;
			windDownReason = LEVEL_LOADING;
			msgFromOtherModules = MSG_NONE;
		}
		else if (msgFromOtherModules == MSG_WINGAME)
		{
			dword CDtime;

			mapToGoTo = 0;
			windingDown = 1;
			windDownReason = LEVEL_WIN;
			msgFromOtherModules = MSG_NONE;
			CDtime = timeGetTime();
			VictoryText(gamemgl);
			Credits(gamemgl);
			garbageTime += timeGetTime() - CDtime;
			player.boredom = 0;
		}
		*lastTime -= TIME_PER_FRAME;
		numRunsToMakeUp++;
		updFrameCount++;
	}
	HandleCDMusic();
	garbageTime = 0;
	JamulSoundUpdate();

	return LEVEL_PLAYING;
}

void LunaticDraw(void)
{
	char s[32];
	dword d;

	// add all the sprites to the list
	if (gameMode != GAMEMODE_PIC)
	{
		RenderGuys(1);
		RenderBullets();
		RenderParticles();
		RenderItAll(&curWorld, curMap, MAP_SHOWLIGHTS | MAP_SHOWITEMS | MAP_SHOWWALLS);
		RenderSpecialXes(gamemgl, curMap, worldNum);
		RenderMessage();
		PlayerRenderInterface(gamemgl);
		if (gameMode == GAMEMODE_MENU)
			RenderPauseMenu();
		if (gameMode == GAMEMODE_RAGE)
			ShowRage(gamemgl);
	}
	else
	{
		// nothing to do
	}

	if (showStats)
	{
		sprintf(s, "QFPS %02.2f", frmRate);
		Print(0, 180, s, 6, 0);
		sprintf(s, "VFPS %02.2f", ((float) visFrameCount / (float) ((timeGetTime() - gameStartTime) / 1000)));
		Print(0, 10, s, 6, 0);
		sprintf(s, "GFPS %02.2f", ((float) updFrameCount / (float) ((timeGetTime() - gameStartTime) / 1000)));
		Print(0, 50, s, 6, 0);
		sprintf(s, "Runs %d", numRunsToMakeUp);
		Print(0, 100, s, 6, 0);
	}
	// update statistics
	d = timeGetTime();
	if (d - tickerTime > 999)
	{
		frmRate = (frmRate * 3 + ((float) visFrms / ((float) (d - tickerTime) / 1000.0f))) / 4;
		visFrms = 0;
		tickerTime = d;
	}

	gamemgl->Flip();
	CDMessingTime += garbageTime;
	garbageTime = 0;

	visFrameCount++;
	visFrms++;
}
