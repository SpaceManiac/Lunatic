#include "mgldraw.h"
#include "winpch.h"
#include "game.h"	// for SetGameIdle and GetGameIdle only
#include "sound.h"
#include "music.h"
#include "ctype.h"
#include "shlobj.h" // for SHGetFolderPath
#include <stdio.h>
#include <random>

extern "C" bool MGLDraw_Process(MGLDraw*);
bool MGLDraw::Process()
{
	return MGLDraw_Process(this);
}

extern "C" bool MGLDraw_Flip(MGLDraw*);
void MGLDraw::Flip()
{
	MGLDraw_Flip(this);
}

void MGLDraw::ClearScreen()
{
	memset(scrn, 0, xRes * yRes);
}

byte *MGLDraw::GetScreen()
{
	return scrn;
}

int MGLDraw::GetWidth()
{
	return pitch;
}

int MGLDraw::GetHeight()
{
	return yRes;
}

void MGLDraw::Quit()
{
	readyToQuit = true;
}

extern "C" void MGLDraw_SetPalette(MGLDraw*, const palette_t*);
void MGLDraw::SetPalette(const palette_t *pal2)
{
	MGLDraw_SetPalette(this, pal2);
}

// 8-bit graphics only

extern "C" void MGLDraw_Box(MGLDraw*, int, int, int, int, byte);
void MGLDraw::Box(int x, int y, int x2, int y2, byte c)
{
	MGLDraw_Box(this, x, y, x2, y2, c);
}

extern "C" void MGLDraw_FillBox(MGLDraw*, int, int, int, int, byte);
void MGLDraw::FillBox(int x, int y, int x2, int y2, byte c)
{
	MGLDraw_FillBox(this, x, y, x2, y2, c);
}

void MGLDraw::SetLastKey(char c)
{
	lastKeyPressed = c;
}

char MGLDraw::LastKeyPressed()
{
	char c = lastKeyPressed;
	lastKeyPressed = 0;
	return c;
}

void MGLDraw::SetMouseDown(byte w)
{
	mouseDown = w;
}

byte MGLDraw::MouseDown()
{
	return mouseDown;
}

void MGLDraw::SetMouse(int x, int y)
{
	mousex = x;
	mousey = y;
}

void MGLDraw::TeleportMouse(int x, int y)
{
	POINT pt = {x, y};
	ClientToScreen(win_get_window(), &pt);
	SetCursorPos(pt.x, pt.y);
	SetMouse(x, y);
}

void MGLDraw::GetMouse(int *x, int *y)
{
	*x = mousex;
	*y = mousey;
}

char MGLDraw::LastKeyPeek()
{
	return lastKeyPressed;
}

bool MGLDraw::LoadBMP(const char *name)
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
		pal[i] = makecol(pal2[i].rgbRed, pal2[i].rgbGreen, pal2[i].rgbBlue);
	}

	for (i = 0; i < bmpIHead.biHeight; i++)
	{
		scr = &scrn[(bmpIHead.biHeight - 1 - i) * pitch];
		fread(scr, 1, bmpIHead.biWidth, f);
	}
	fclose(f);
	return TRUE;
}

void MGLDraw::GammaCorrect(byte gamma)
{
	/*int i;
	int r, g, b;
	palette_t temp[256];

	memcpy(temp, pal, sizeof (palette_t)*256);
	for (i = 0; i < 256; i++)
	{
		r = pal[i].red;
		g = pal[i].green;
		b = pal[i].blue;
		r = (r * (gamma + 4)) / 4;
		g = (g * (gamma + 4)) / 4;
		b = (b * (gamma + 4)) / 4;
		if (r > 255)
			r = 255;
		if (g > 255)
			g = 255;
		if (b > 255)
			b = 255;
		pal[i].red = r;
		pal[i].green = g;
		pal[i].blue = b;
	}
	memcpy(pal, temp, sizeof (palette_t)*256);*/
}
