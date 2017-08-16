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

byte WorldPauseRun(int *lastTime)
{
	numRunsToMakeUp = 0;
	while (*lastTime >= TIME_PER_FRAME)
	{
		if (!gamemgl->Process())
		{
			mapToGoTo = 255;
			return WORLD_QUITGAME;
		}

		switch (UpdatePauseMenu(gamemgl)) {
			case 0:
				lastKey = 0;
				return LEVEL_WIN;
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
		*lastTime -= TIME_PER_FRAME;
		numRunsToMakeUp++;
		updFrameCount++;
	}
	HandleCDMusic();
	garbageTime = 0;
	JamulSoundUpdate();

	return LEVEL_PLAYING;
}

void WorldPauseDraw(void)
{
	dword d;

	gamemgl->ClearScreen();
	RenderPauseMenu();

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

byte WorldPickerPause(void)
{
	int lastTime = 1;
	byte exitcode = LEVEL_PLAYING;

	InitPauseMenu();
	SetGiveUpText(2);
	while (exitcode == LEVEL_PLAYING)
	{
		lastTime += TimeLength();
		StartClock();
		exitcode = WorldPauseRun(&lastTime);
		WorldPauseDraw();

		if (!gamemgl->Process())
		{
			exitcode = WORLD_QUITGAME;
			mapToGoTo = 255;
		}
		EndClock();
	}
	return exitcode;
}

void HandleKeyPresses(void)
{
	char k;

	k = gamemgl->LastKeyPressed();
	if (k)
	{
		lastKey = k;
		if ((lastKey >= 'a' && lastKey <= 'z') || (lastKey >= 'A' && lastKey <= 'Z'))
			CheatKey(lastKey);
	}
#ifdef _DEBUG
	// can't show stats unless in debug mode
	if (lastKey == 's')
	{
		showStats = 1 - showStats;
		lastKey = 0;
	}
#endif
	if (lastKey == 'g')
	{
		k = GetGamma();
		k++;
		if (k > 3)
			k = 0;
		gamemgl->GammaCorrect(k);
		SetGamma(k);
		lastKey = 0;
	}

}

byte PlayALevel(byte map)
{
	int lastTime = 1;
	byte exitcode = 0;

	if (!InitLevel(map))
	{
		mapToGoTo = 255;
		return LEVEL_ABORT;
	}

	exitcode = LEVEL_PLAYING;
	gameMode = GAMEMODE_PLAY;
	CDMessingTime = 0;
	garbageTime = 0;

	UpdateGuys(curMap, &curWorld); // this will force the camera into the right position
	// it also makes everybody animate by one frame, but no one will
	// ever notice
	while (exitcode == LEVEL_PLAYING)
	{
		lastTime += TimeLength() - CDMessingTime;
		StartClock();
		if (gameMode == GAMEMODE_PLAY)
			HandleKeyPresses();
		exitcode = LunaticRun(&lastTime);
		LunaticDraw();

		if (lastKey == 27 && gameMode == GAMEMODE_PLAY)
		{
			InitPauseMenu();
			gameMode = GAMEMODE_MENU;
		}

		if (!gamemgl->Process())
		{
			exitcode = LEVEL_ABORT;
			mapToGoTo = 255;
		}
		EndClock();
	}

	ExitLevel();
	return exitcode;
}

byte LunaticWorld(byte world, const char *worldName)
{
	byte result;

	InitPlayer(INIT_WORLD, world, 0);
	if (!LoadWorld(&curWorld, worldName))
		return WORLD_ABORT;

	worldNum = world;
	InitWorld(&curWorld, worldNum);

	mapNum = 0;
	while (1)
	{
		result = PlayALevel(mapNum);
		if (result == LEVEL_ABORT)
		{
			PlayerResetScore();
			if (mapToGoTo < 255)
				mapNum = mapToGoTo;
			else
				break;
		}
		else if (result == LEVEL_RESET)
		{
			PlayerResetScore();
			// don't do anything, play the same level
		}
		else if (result == LEVEL_WIN)
		{
			PlayerWinLevel(world, mapNum, curMapFlags & MAP_SECRET);
			mapNum = mapToGoTo;
		}
		else if (result == LEVEL_LOADING)
		{
			FreeWorld(&curWorld);
			PlayerResetScore();
			return WORLD_LOAD;
		}
		else if (result == WORLD_QUITGAME)
		{
			FreeWorld(&curWorld);
			PlayerResetScore();
			return WORLD_QUITGAME;
		}
	}
	FreeWorld(&curWorld);
	return WORLD_ABORT;
}

void LunaticGame(MGLDraw *mgl, byte load)
{
	char custName[64];
	byte b, worldResult;

	if (!load) // don't do this if loading a game, it was already done and the player was filled with values
		InitPlayer(INIT_GAME, 0, 0);

	if (load > 0) // continuing a saved game
		worldResult = WORLD_LOAD;
	else
		worldResult = 0;
	while (1)
	{
		if (worldResult == WORLD_LOAD)
		{
			ReScanWorldNames();
			b = GetPlayerWorld();
		}
		else
		{
			b = 255;
			while (1)
			{
				msgFromOtherModules = 0;
				b = WorldPicker(mgl);
				if (b == 253)
				{
					b = WorldPickerPause();
					if (b == WORLD_QUITGAME)
					{
						b = 255;
						break;
					}
					if (msgFromOtherModules == MSG_LOADGAME) // load a game
					{
						ReScanWorldNames();
						b = GetPlayerWorld();
						break;
					}
				}
				else
					break;
			}
			if (b == 255)
				break;
			player.worldNum = b;
		}

		if (b < 5)
		{
			if (b == 4 && worldResult != WORLD_LOAD)
				// the asylum requires that you have all four loony keys to enter
				// unless you're loading a saved game saved in there, in which case watching
				// the animation would be a waste
			{

				if (PlayerHasLunacyKey(0) && PlayerHasLunacyKey(1) &&
						PlayerHasLunacyKey(2) && PlayerHasLunacyKey(3))
				{
					ShowVictoryAnim(12);
					garbageTime = 0;
					sprintf(custName, "worlds\\%s", GetCustomName());
					worldResult = LunaticWorld(b, custName);
				}
				else
				{
					ShowVictoryAnim(11);
					garbageTime = 0;
					worldResult = 0; // not allowed to enter
				}
			}
			else
			{
				sprintf(custName, "worlds\\%s", GetCustomName());
				worldResult = LunaticWorld(b, custName);
			}
		}
		else
		{
			sprintf(custName, "worlds\\%s", GetCustomName());
			worldResult = LunaticWorld(b, custName);
		}
		if (worldResult == WORLD_QUITGAME)
		{
			mgl->LastKeyPressed(); // just to clear key buffer
			break;
		}
	}
	ExitPlayer();
}
