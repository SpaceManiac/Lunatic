#ifndef JAMULSPR_H
#define JAMULSPR_H

#include "winpch.h"
#include "mgldraw.h"

class sprite_t
{
public:
	sprite_t(void);
	sprite_t(byte *info);

	~sprite_t(void);

	void* operator new(size_t);
	void operator delete(void*, size_t);

	bool LoadData(FILE *f);
	bool SaveData(FILE *f);
	void GetHeader(byte *buffer);
	void Draw(int x, int y, MGLDraw *mgl);

	void DrawBright(int x, int y, MGLDraw *mgl, char bright);
	void DrawColored(int x, int y, MGLDraw *mgl, byte color, char bright);
	void DrawOffColor(int x, int y, MGLDraw *mgl, byte fromColor, byte toColor, char bright);
	void DrawGhost(int x, int y, MGLDraw *mgl, char bright);
	void DrawGlow(int x, int y, MGLDraw *mgl, char bright);

	// this makes half-height tilted black shadows (they darken by 4)
	void DrawShadow(int x, int y, MGLDraw *mgl);
	void GetCoords(int x, int y, int *rx, int *ry, int *rx2, int *ry2);

	word width;
	word height;
	short ofsx;
	short ofsy;
private:
	byte *data;
	dword size;
};

class sprite_set_t
{
public:
	sprite_set_t(const char *fname);
	~sprite_set_t(void);

	sprite_t *GetSprite(int which);

	void* operator new(size_t);
	void operator delete(void*, size_t);
private:
	bool Save(const char *fname);
	bool Load(const char *fname);
	void Free(void);

	sprite_t **spr;
	dword count;
};

#endif
