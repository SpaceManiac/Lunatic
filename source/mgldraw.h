#ifndef MGLDRAW_H
#define MGLDRAW_H

#include "allegro.h"
#include <winalleg.h>
#include "winpch.h"
#include "jamulsound.h"
#include "control.h"
#include <stdio.h>
#include <memory>

// For appdata storage of stuff
extern "C" FILE* AppdataOpen(const char* filename, const char* mode);

// Replacement for missing palette_t
struct palette_t
{
	byte alpha, red, green, blue;
};

// Replacement for missing MGL functions
extern "C" {
int MGL_random(int max);
void MGL_srand(int seed);
long MGL_randoml(long max);
void MGL_fatalError(const char* txt);
}

class MGLDraw
{
public:
	// handle windows messages and such
	bool Process();

	// get a pointer to the screen memory
	byte *GetScreen() { return scrn; }
	int GetWidth() { return pitch; }
	int GetHeight() { return yRes; }
	void ClearScreen() { memset(scrn, 0, xRes * yRes); }
	void Flip();
	void Quit() { readyToQuit = true; }

	bool LoadPalette(const char *name);
	void SetPalette(const palette_t *pal2);

	bool LoadBMP(const char *name);

	char LastKeyPressed() { char c = lastKeyPressed; lastKeyPressed = 0; return c; }
	char LastKeyPeek() { return lastKeyPressed; }
	void SetLastKey(char c) { lastKeyPressed = c; }

	void GammaCorrect(byte gamma);

	// handy little drawing routines
	void Box(int x, int y, int x2, int y2, byte c);
	void FillBox(int x, int y, int x2, int y2, byte c);

	// mouse functions
	byte MouseDown() { return mouseDown; }
	void SetMouseDown(byte w) { mouseDown = w; }
	void SetMouse(int x, int y) { mousex = x; mousey = y; }
	void TeleportMouse(int x, int y);
	void GetMouse(int *x, int *y) { *x = mousex; *y = mousey; }

protected:
	MGLDraw(const char *name, int xRes, int yRes, bool window);
	~MGLDraw();

	struct bitmap_deleter {
		void operator()(BITMAP* buffer) {
			destroy_bitmap(buffer);
		}
	};

	int xRes, yRes, pitch;
	int mousex, mousey;
	byte* scrn;
	BITMAP* buffer;
	int pal[256];
	bool readyToQuit;
	char lastKeyPressed;
	byte mouseDown;
};

#endif
