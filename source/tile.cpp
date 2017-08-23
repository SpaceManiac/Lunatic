#include "tile.h"
#include "options.h"

tile_t tiles[NUMTILES];
MGLDraw *tileMGL;

// --- RENDERING!
// Helper shenanigans for C stuff, see jamulspr.cpp
extern "C" byte SprModifyLight(byte color, char bright);

byte ModifyDiscoColor(byte color, byte disco)
{
	if (!opt.discoMode) return color;
	else return (color & 31) | (disco);
}

// Disco!

byte discoTab[] = {1, 3, 4, 5, 6, 7};

static inline byte PickDiscoColor()
{
	return discoTab[rand() % 6]*32;
}

// Rendering for real!

void RenderFloorTile(int x, int y, int t, char light)
{
	byte *dst, *src;
	int wid, hgt;
	byte disco = PickDiscoColor();

	if (light == 0 && !opt.discoMode)
	{
		return RenderFloorTileUnlit(x, y, t);
	}

	if (x < 0)
	{
		wid = TILE_WIDTH + x;
		if (wid < 1)
			return;

		dst = tileMGL->GetScreen() + y * 640;
		src = tiles[t] - x;
	}
	else if (x > 640 - TILE_WIDTH)
	{
		wid = TILE_WIDTH - (x - (640 - TILE_WIDTH));
		if (wid < 1)
			return;
		dst = tileMGL->GetScreen() + x + y * 640;
		src = tiles[t];
	}
	else
	{
		wid = TILE_WIDTH;
		dst = tileMGL->GetScreen() + x + y * 640;
		src = tiles[t];
	}

	if (y < 0)
	{
		dst -= y * 640;
		src -= y*TILE_WIDTH;

		hgt = TILE_HEIGHT + y;
	}
	else if (y > 480 - TILE_HEIGHT)
	{
		hgt = TILE_HEIGHT - (y - (480 - TILE_HEIGHT));
	}
	else
	{
		hgt = TILE_HEIGHT;
	}

	if (hgt <= 0)
		return;

	if (light<-28)
	{
		// just render a black box
		while (hgt > 0)
		{
			hgt--;
			memset(dst, 0, wid);
			dst += 640;
		}
		return;
	}
	else
	{
		while (hgt > 0)
		{
			hgt--;
			for (int i = 0; i < wid; ++i)
			{
				dst[i] = SprModifyLight(ModifyDiscoColor(src[i], disco), light);
			}
			dst += 640;
			src += 32;
		}
	}
}

void RenderFloorTileShadow(int x, int y, int t, char light)
{
	byte *dst, *src;
	int wid, hgt, darkpart;
	byte disco = PickDiscoColor();

	if (x < 0)
	{
		wid = TILE_WIDTH + x;
		if (wid < 1)
			return;

		darkpart = 8;
		dst = tileMGL->GetScreen() + y * 640;
		src = tiles[t] - x;
	}
	else if (x > 640 - TILE_WIDTH)
	{
		wid = TILE_WIDTH - (x - (640 - TILE_WIDTH));
		if (wid < 1)
			return;
		darkpart = 8 - (x - (640 - TILE_WIDTH));
		dst = tileMGL->GetScreen() + x + y * 640;
		src = tiles[t];
	}
	else
	{
		wid = TILE_WIDTH;
		dst = tileMGL->GetScreen() + x + y * 640;
		src = tiles[t];
		darkpart = 8; // shadows are 8 pixels wide I guess
	}

	if (y < 0)
	{
		dst -= y * 640;
		src -= y*TILE_WIDTH;

		hgt = TILE_HEIGHT + y;
	}
	else if (y > 480 - TILE_HEIGHT)
	{
		hgt = TILE_HEIGHT - (y - (480 - TILE_HEIGHT));
	}
	else
		hgt = TILE_HEIGHT;

	if (hgt <= 0)
		return;

	while (hgt > 0)
	{
		hgt--;
		for (int i = 0; i < wid; ++i)
		{
			dst[i] = SprModifyLight(ModifyDiscoColor(src[i], disco), light - 4 * (i > wid - darkpart));
		}
		dst += 640;
		src += 32;
	}
}

void RenderFloorTileUnlit(int x, int y, int t)
{
	byte *dst, *src;
	int wid, hgt;

	if (x < 0)
	{
		wid = TILE_WIDTH + x;
		if (wid < 1)
			return;

		dst = tileMGL->GetScreen() + y * 640;
		src = tiles[t] - x;
	}
	else if (x > 640 - TILE_WIDTH)
	{
		wid = TILE_WIDTH - (x - (640 - TILE_WIDTH));
		if (wid < 1)
			return;
		dst = tileMGL->GetScreen() + x + y * 640;
		src = tiles[t];
	}
	else
	{
		wid = TILE_WIDTH;
		dst = tileMGL->GetScreen() + x + y * 640;
		src = tiles[t];
	}

	if (y < 0)
	{
		dst -= y * 640;
		src -= y*TILE_WIDTH;

		hgt = TILE_HEIGHT + y;
	}
	else if (y > 480 - TILE_HEIGHT)
	{
		hgt = TILE_HEIGHT - (y - (480 - TILE_HEIGHT));
	}
	else
		hgt = TILE_HEIGHT;

	while (hgt > 0)
	{
		hgt--;
		memcpy(dst, src, wid);
		dst += 640;
		src += 32;
	}
}

void RenderFloorTileTrans(int x, int y, int t, char light)
{
	byte *dst, *src;
	int wid, hgt;
	byte disco = PickDiscoColor();

	if (x < 0)
	{
		wid = TILE_WIDTH + x;
		if (wid < 1)
			return;

		dst = tileMGL->GetScreen() + y * 640;
		src = tiles[t] - x;
	}
	else if (x > 640 - TILE_WIDTH)
	{
		wid = TILE_WIDTH - (x - (640 - TILE_WIDTH));
		if (wid < 1)
			return;
		dst = tileMGL->GetScreen() + x + y * 640;
		src = tiles[t];
	}
	else
	{
		wid = TILE_WIDTH;
		dst = tileMGL->GetScreen() + x + y * 640;
		src = tiles[t];
	}

	if (y < 0)
	{
		dst -= y * 640;
		src -= y*TILE_WIDTH;

		hgt = TILE_HEIGHT + y;
	}
	else if (y > 480 - TILE_HEIGHT)
	{
		hgt = TILE_HEIGHT - (y - (480 - TILE_HEIGHT));
	}
	else
		hgt = TILE_HEIGHT;

	if (hgt <= 0)
		return;

	while (hgt > 0)
	{
		hgt--;
		for (int i = 0; i < wid; ++i)
		{
			if (src[i]) dst[i] = SprModifyLight(ModifyDiscoColor(src[i], disco), light);
		}
		dst += 640;
		src += 32;
	}
}
