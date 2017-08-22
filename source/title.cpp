#include "title.h"
#include "game.h"
#include <io.h>
#include "pause.h"
#include "options.h"

#define VERSION "3.1"
#define COPYRIGHT "Copyright 1998-2011, Hamumu Software"

// once the credits have scrolled to END_OF_CREDITS pixels, they end
const int END_OF_CREDITS = 480 * 4 + 180;
const int END_OF_VICTORY = 480 * 2;

struct title_t
{
	int bouaphaX, doctorX;
	int blueY;
	char titleBright;
	char titleDir;
	int expando;
	char dexpando;
	byte cursor;
	byte savecursor;
	float percent[3];
};

sprite_set_t *planetSpr;

byte title_oldc = 0;
#define oldc title_oldc

byte GameSlotPicker(MGLDraw *mgl, title_t *title);

void ScanWorldNames(void)
{
	int i;
	long hFile;
	struct _finddata_t filedata;
	int numFiles;

	for (i = 5; i < MAX_CUSTOM; i++)
		player.customName[i][0] = '\0';

	hFile = _findfirst("worlds\\*.dlw", &filedata);

	if (hFile != -1) // there's at least one
	{
		// rule out the regular game worlds, so they don't show up as custom worlds
		if ((strcmp(filedata.name, "forest.dlw")) &&
				(strcmp(filedata.name, "desert.dlw")) &&
				(strcmp(filedata.name, "icymount.dlw")) &&
				(strcmp(filedata.name, "caverns.dlw")) &&
				(strcmp(filedata.name, "asylum.dlw")) &&
				(strcmp(filedata.name, "backup_load.dlw")) &&
				(strcmp(filedata.name, "backup_exit.dlw")))
		{
			strncpy(player.customName[5], filedata.name, 32);
			numFiles = 1;
		}
		else
			numFiles = 0;

		while (numFiles < MAX_CUSTOM)
		{
			if (_findnext(hFile, &filedata) == 0)
			{
				// rule out the regular game worlds, so they don't show up as custom worlds
				if ((strcmp(filedata.name, "forest.dlw")) &&
						(strcmp(filedata.name, "desert.dlw")) &&
						(strcmp(filedata.name, "icymount.dlw")) &&
						(strcmp(filedata.name, "caverns.dlw")) &&
						(strcmp(filedata.name, "asylum.dlw")) &&
						(strcmp(filedata.name, "backup_load.dlw")) &&
						(strcmp(filedata.name, "backup_exit.dlw")))
				{
					strncpy(player.customName[numFiles + 5], filedata.name, 32);
					numFiles++;
				}
			}
			else // no more files
				break;
		}
	}
	_findclose(hFile);
}

void ReScanWorldNames(void)
{
	int i;
	long hFile;
	struct _finddata_t filedata;
	byte okay[MAX_CUSTOM];

	for (i = 5; i < MAX_CUSTOM; i++)
	{
		if (player.customName[i][0] == '\0')
			okay[i] = 1;
		else
			okay[i] = 0;
	}

	hFile = _findfirst("worlds\\*.dlw", &filedata);

	while (hFile != -1) // there's at least one
	{
		// rule out the regular game worlds, so they don't show up as custom worlds
		if ((strcmp(filedata.name, "forest.dlw")) &&
				(strcmp(filedata.name, "desert.dlw")) &&
				(strcmp(filedata.name, "icymount.dlw")) &&
				(strcmp(filedata.name, "caverns.dlw")) &&
				(strcmp(filedata.name, "asylum.dlw")) &&
				(strcmp(filedata.name, "backup_load.dlw")) &&
				(strcmp(filedata.name, "backup_exit.dlw")))
		{
			for (i = 5; i < MAX_CUSTOM; i++)
			{
				if (!strcmp(filedata.name, player.customName[i]))
				{
					okay[i] = 1;
					break;
				}
			}
			if (i == MAX_CUSTOM) // none of the files matched, this is a new one
			{
				// add it in, if there's room
				for (i = 5; i < MAX_CUSTOM; i++)
				{
					if (player.customName[i][0] == '\0')
					{
						strncpy(player.customName[i], filedata.name, 32);
						break;
					}
				}
			}
		}
		if (_findnext(hFile, &filedata) != 0)
			break;
	}
	_findclose(hFile);

	// remove any that aren't valid
	for (i = 5; i < MAX_CUSTOM; i++)
	{
		if (okay[i] == 0)
			player.customName[i][0] = '\0';
	}
}

void CommonMenuDisplay(MGLDraw* mgl, title_t title)
{
	int i, color, deltaColor;
	byte *scrn;

	color = 0;
	deltaColor = (12 * 65536) / (480 - title.blueY);
	scrn = mgl->GetScreen();
	if (title.blueY > 0)
		memset(scrn, 0, 640 * title.blueY);
	scrn += 640 * title.blueY;
	for (i = title.blueY; i < 480; i++)
	{
		memset(scrn, color / 65536 + 96, 640);
		scrn += 640;
		color += deltaColor;
	}

	// draw Dr. L & Bouapha
	planetSpr->GetSprite(0)->Draw(640 - title.doctorX, 480, mgl);
	planetSpr->GetSprite(1)->Draw(title.bouaphaX, 480, mgl);

	// draw the title parts
	planetSpr->GetSprite(2)->DrawBright(240, 30, mgl, title.titleBright); // SPISPOPD II:
	planetSpr->GetSprite(3)->DrawBright(290, 125, mgl, title.titleBright); // DR. LUNATIC

	// LoonyMod and version number
	CenterPrint(320, 120, "LoonyMod", 0, 0);
	CenterPrint(321, 171, "Version " VERSION, 1, 1);
	CenterPrint(320, 170, "Version " VERSION, 0, 1);

	// Copyright
	Print(3, 467, COPYRIGHT, 1, 1);
	Print(2, 466, COPYRIGHT, 0, 1);
}

void MainMenuDisplay(MGLDraw *mgl, title_t title)
{
	CommonMenuDisplay(mgl, title);

	// now the menu options
	planetSpr->GetSprite(9 + (title.cursor == 0))->Draw(240, 270, mgl);
	planetSpr->GetSprite(11 + (title.cursor == 1))->Draw(260, 300, mgl);
	planetSpr->GetSprite(13 + (title.cursor == 2))->Draw(280, 330, mgl);
	planetSpr->GetSprite(15 + (title.cursor == 3))->Draw(300, 360, mgl);
	planetSpr->GetSprite(19 + (title.cursor == 5))->Draw(340, 420, mgl);
	planetSpr->GetSprite(21 + (title.cursor == 6))->Draw(360, 450, mgl);
}

byte MainMenuUpdate(MGLDraw *mgl, title_t *title)
{
	byte c;
	static byte reptCounter = 0;

	// update graphics
	title->titleBright += title->titleDir;
	if (title->titleBright > 31)
	{
		title->titleDir = -1;
		title->titleBright = 31;
	}
	if (title->titleDir < 0 && title->titleBright == 0)
		title->titleDir = 0;

	if (title->bouaphaX < 0)
		title->bouaphaX += 8;
	if (title->doctorX < 0)
		title->doctorX += 8;

	if (title->blueY > 0)
		title->blueY -= 8;
	if (title->blueY < 0)
		title->blueY = 0;

	title->expando += title->dexpando;
	if (title->expando > 79)
	{
		title->dexpando = -title->dexpando * 13 / 16;
		title->expando = 79;
	}
	else
		title->dexpando++;

	// now real updating
	c = GetControls() | GetArrows();

	reptCounter++;
	if ((!oldc) || (reptCounter > 10))
		reptCounter = 0;

	if ((c & CONTROL_UP) && (!reptCounter))
	{
		(title->cursor)--;
		if (title->cursor == 255)
			title->cursor = 6;
#ifndef DEMO
		if (title->cursor == 4) // ordering is not a viable option in the non-shareware
			title->cursor = 3;
#endif
		MakeNormalSound(SND_MENUCLICK);
	}
	if ((c & CONTROL_DN) && (!reptCounter))
	{
		(title->cursor)++;
		if (title->cursor == 7)
			title->cursor = 0;
#ifndef DEMO
		if (title->cursor == 4) // ordering is not a viable option in the non-shareware
			title->cursor = 5;
#endif
		MakeNormalSound(SND_MENUCLICK);
	}
	if (((c & CONTROL_B1) && (!(oldc & CONTROL_B1))) ||
			((c & CONTROL_B2) && (!(oldc & CONTROL_B2))))
	{
		MakeNormalSound(SND_MENUSELECT);
		return 1;
	}
	oldc = c;

	c = mgl->LastKeyPressed();
	if (c == 27)
	{
		MakeNormalSound(SND_MENUSELECT);
		return 2;
	}

	HandleCDMusic();

	return 0;
}

byte MainMenu(MGLDraw *mgl)
{
	dword startTime, now;
	dword runStart, runEnd;

	byte b = 0;
	title_t title;

	if (opt.music == MUSIC_ON)
		CDPlay(2); // the title theme
	CDNeedsUpdating();

	mgl->LoadBMP("graphics\\title.bmp");
	mgl->LastKeyPressed();
	mgl->ClearScreen();
	oldc = CONTROL_B1 | CONTROL_B2;
	planetSpr = new sprite_set_t("graphics\\titlespr.jsp");

	title.bouaphaX = -320;
	title.doctorX = -320;
	title.titleBright = -32;
	title.titleDir = 4;
	title.cursor = 0;
	title.blueY = 479;
	title.expando = 0;
	title.dexpando = 0;
	startTime = timeGetTime();
	while (b == 0)
	{
		runStart = timeGetTime();
		b = MainMenuUpdate(mgl, &title);
		MainMenuDisplay(mgl, title);
		mgl->Flip();
		runEnd = timeGetTime();

		if (runEnd - runStart < (1000 / 50))
			Sleep((1000 / 50)-(runEnd - runStart));

		if (!mgl->Process())
		{
			CDStop();
			delete planetSpr;
			return 255;
		}
		if (b == 1 && title.cursor == 1) // selected Load Game
		{
			if (!GameSlotPicker(mgl, &title)) // pressed ESC on the slot picker
			{
				b = 0;
			}
			startTime = timeGetTime();
		}
		if (b == 1 && title.cursor == 2) // options
		{
			OptionsMenu(mgl);
			startTime = timeGetTime();
		}
		if (b == 1 && title.cursor == 5) // help
		{
			HelpScreens(mgl);
			startTime = timeGetTime();
		}
		now = timeGetTime();
		if (now - startTime > 1000 * 20)
		{
			Credits(mgl);
			startTime = timeGetTime();
		}
	}
	delete planetSpr;
	if (b == 1) // something was selected
	{
		if (title.cursor == 6) // exit
			return 255;
		else
			return title.cursor;
	}
	else
		return 255; // ESC was pressed
}

void GameSlotPickerDisplay(MGLDraw *mgl, title_t title)
{
	int i;
	char txt[18];

	CommonMenuDisplay(mgl, title);

	// now the game slots
	for (i = 0; i < 3; i++)
	{
		if (title.percent[i] > 99.9)
			sprintf(txt, "Slot %d - 100%%", i + 1);
		else
			sprintf(txt, "Slot %d - %03.1f%%", i + 1, title.percent[i]);

		Print(180 + 30 * i, 220 + 70 * i, txt, -6 + 12 * (title.savecursor == i), 0);
	}
}

byte GameSlotPickerUpdate(MGLDraw *mgl, title_t *title)
{
	byte c;
	static byte reptCounter = 0;

	// update graphics
	title->titleBright += title->titleDir;
	if (title->titleBright > 31)
	{
		title->titleDir = -1;
		title->titleBright = 31;
	}
	if (title->titleDir < 0 && title->titleBright == 0)
		title->titleDir = 0;

	if (title->bouaphaX<-60)
		title->bouaphaX += 8;
	if (title->bouaphaX>-60)
		title->bouaphaX -= 8;
	if (title->doctorX<-40)
		title->doctorX += 8;
	if (title->doctorX>-40)
		title->doctorX -= 8;
	if (title->blueY > 0)
		title->blueY -= 8;
	if (title->blueY < 0)
		title->blueY = 0;

	title->expando += title->dexpando;
	if (title->expando > 79)
	{
		title->dexpando = -title->dexpando * 13 / 16;
		title->expando = 79;
	}
	else
		title->dexpando++;

	// now real updating
	c = GetControls() | GetArrows();

	reptCounter++;
	if ((!oldc) || (reptCounter > 10))
		reptCounter = 0;

	if ((c & CONTROL_UP) && (!reptCounter))
	{
		(title->savecursor)--;
		if (title->savecursor == 255)
			title->savecursor = 2;
		MakeNormalSound(SND_MENUCLICK);
	}
	if ((c & CONTROL_DN) && (!reptCounter))
	{
		(title->savecursor)++;
		if (title->savecursor == 3)
			title->savecursor = 0;
		MakeNormalSound(SND_MENUCLICK);
	}
	if (((c & CONTROL_B1) && (!(oldc & CONTROL_B1))) ||
			((c & CONTROL_B2) && (!(oldc & CONTROL_B2))))
	{
		MakeNormalSound(SND_MENUSELECT);
		return 1;
	}
	oldc = c;

	if (mgl->LastKeyPressed() == 27)
	{
		MakeNormalSound(SND_MENUSELECT);
		return 2;
	}

	HandleCDMusic();

	return 0;
}

void InitGameSlotPicker(MGLDraw *mgl, title_t *title)
{
	FILE *f;
	player_t p;

	f = AppdataOpen("loony.sav", "rb");
	if (!f)
	{
		title->percent[0] = 0.0;
		title->percent[1] = 0.0;
		title->percent[2] = 0.0;
	}
	else
	{
		fread(&p, sizeof (player_t), 1, f);
		title->percent[0] = CalcTotalPercent(&p)*100;
		fread(&p, sizeof (player_t), 1, f);
		title->percent[1] = CalcTotalPercent(&p)*100;
		fread(&p, sizeof (player_t), 1, f);
		title->percent[2] = CalcTotalPercent(&p)*100;
		fclose(f);
	}
	mgl->LastKeyPressed();
	oldc = CONTROL_B1 | CONTROL_B2;
}

byte GameSlotPicker(MGLDraw *mgl, title_t *title)
{
	byte b = 0;
	dword runEnd, runStart;

	title->savecursor = 0;
	InitGameSlotPicker(mgl, title);

	while (b == 0)
	{
		runStart = timeGetTime();

		b = GameSlotPickerUpdate(mgl, title);
		GameSlotPickerDisplay(mgl, *title);
		mgl->Flip();
		runEnd = timeGetTime();

		if (runEnd - runStart < (1000 / 50))
			Sleep((1000 / 50)-(runEnd - runStart));

		if (!mgl->Process())
			return 0;
	}
	if (b == 1) // something was selected
	{
		InitPlayer(INIT_GAME, 0, 0);
		PlayerLoadGame(title->savecursor);
		// make it remember which was picked so the pause menu will start on the same
		SetSubCursor(title->savecursor);
		return 1;
	}
	else
		return 0;
}

byte SpecialLoadBMP(const char *name, MGLDraw *mgl, palette_t *pal)
{
	FILE *f;
	BITMAPFILEHEADER bmpFHead;
	BITMAPINFOHEADER bmpIHead;
	RGBQUAD pal2[256];

	int i;
	byte *scr;

	f = fopen(name, "rb");
	if (!f)
		return FALSE;

	fread(&bmpFHead, sizeof (BITMAPFILEHEADER), 1, f);
	fread(&bmpIHead, sizeof (BITMAPINFOHEADER), 1, f);

	// 8-bit BMPs only
	if (bmpIHead.biBitCount != 8)
		return FALSE;

	// Non-RLE BMPs only
	if (bmpIHead.biCompression != 0)
	{
		printf("bitmap %s is compressed (%lu)\n", name, bmpIHead.biCompression);
		return FALSE;
	}

	fread(pal2, sizeof (pal2), 1, f);
	for (i = 0; i < 256; i++)
	{
		pal[i].red = pal2[i].rgbRed;
		pal[i].green = pal2[i].rgbGreen;
		pal[i].blue = pal2[i].rgbBlue;
	}

	for (i = 0; i < bmpIHead.biHeight; i++)
	{
		scr = mgl->GetScreen() + (bmpIHead.biHeight - 1 - i) * mgl->GetWidth();
		fread(scr, 1, bmpIHead.biWidth, f);
	}
	fclose(f);
	return TRUE;
}

byte SpeedSplash(MGLDraw *mgl, const char *fname)
{
	int i, j, clock;
	palette_t desiredpal[256], curpal[256];
	byte mode, done;
	byte c, oldc;


	for (i = 0; i < 256; i++)
	{
		curpal[i].red = 0;
		curpal[i].green = 0;
		curpal[i].blue = 0;
	}
	mgl->SetPalette(curpal);

	mgl->LastKeyPressed();
	oldc = GetControls() | GetArrows();

	SpecialLoadBMP(fname, mgl, desiredpal);

	mode = 0;
	clock = 0;
	done = 0;
	while (!done)
	{
		mgl->Flip();
		if (!mgl->Process())
			return 0;
		c = mgl->LastKeyPressed();

		if (c == 27)
			return 0;
		else if (c)
			mode = 2;

		HandleCDMusic();

		c = GetControls() | GetArrows();
		if ((c & (CONTROL_B1 | CONTROL_B2)) && (!(oldc & (CONTROL_B1 | CONTROL_B2))))
			mode = 2;
		oldc = c;

		clock++;
		switch (mode) {
			case 0: // fading in
				for (j = 0; j < 16; j++)
					for (i = 0; i < 256; i++)
					{
						if (curpal[i].red < desiredpal[i].red)
							curpal[i].red++;
						if (curpal[i].green < desiredpal[i].green)
							curpal[i].green++;
						if (curpal[i].blue < desiredpal[i].blue)
							curpal[i].blue++;
					}
				mgl->SetPalette(curpal);
				if (clock > 16)
				{
					mode = 1;
					clock = 0;
				}
				break;
			case 1:
				// sit around
				break;
			case 2: // fading out
				clock = 0;
				for (j = 0; j < 16; j++)
					for (i = 0; i < 256; i++)
					{
						if (curpal[i].red > 0)
							curpal[i].red--;
						else
							clock++;
						if (curpal[i].green > 0)
							curpal[i].green--;
						else
							clock++;
						if (curpal[i].blue > 0)
							curpal[i].blue--;
						else
							clock++;
					}
				mgl->SetPalette(curpal);
				if (clock == 256 * 3 * 16)
					done = 1;
				break;
		}
	}
	mgl->ClearScreen();
	mgl->Flip();
	return 1;
}

void HelpScreens(MGLDraw *mgl)
{
	int i;
	char name[32];

	for (i = 0; i < 5; i++)
	{
		sprintf(name, "docs\\help%d.bmp", i + 1);
		if (!SpeedSplash(mgl, name))
			return;
	}
}

void SplashScreen(MGLDraw *mgl, const char *fname, int delay, byte sound)
{
	int i, j, clock;
	palette_t desiredpal[256], curpal[256];
	byte mode, done;

	for (i = 0; i < 256; i++)
	{
		curpal[i].red = 0;
		curpal[i].green = 0;
		curpal[i].blue = 0;
	}
	mgl->SetPalette(curpal);

	mgl->LastKeyPressed();

	SpecialLoadBMP(fname, mgl, desiredpal);

	mode = 0;
	clock = 0;
	done = 0;
	while (!done)
	{
		mgl->Flip();
		if (!mgl->Process())
			return;
		if (mgl->LastKeyPressed())
			mode = 2;

		clock++;
		switch (mode) {
			case 0: // fading in
				for (j = 0; j < 8; j++)
					for (i = 0; i < 256; i++)
					{
						if (curpal[i].red < desiredpal[i].red)
							curpal[i].red++;
						if (curpal[i].green < desiredpal[i].green)
							curpal[i].green++;
						if (curpal[i].blue < desiredpal[i].blue)
							curpal[i].blue++;
					}
				mgl->SetPalette(curpal);
				if (clock == 32)
				{
					if (sound == 2)
						MakeNormalSound(SND_HAMUMU);
				}
				if (clock > 64)
				{
					mode = 1;
					clock = 0;
				}
				break;
			case 1:
				if (clock > delay)
				{
					mode = 2;
					clock = 0;
				}
				break;
			case 2: // fading out
				clock = 0;
				for (j = 0; j < 8; j++)
					for (i = 0; i < 256; i++)
					{
						if (curpal[i].red > 0)
							curpal[i].red--;
						else
							clock++;
						if (curpal[i].green > 0)
							curpal[i].green--;
						else
							clock++;
						if (curpal[i].blue > 0)
							curpal[i].blue--;
						else
							clock++;
					}
				mgl->SetPalette(curpal);
				if (clock == 256 * 3 * 8)
					done = 1;
				break;
		}
	}
	mgl->ClearScreen();
	mgl->Flip();
}
