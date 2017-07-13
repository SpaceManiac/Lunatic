#ifndef JAMULFONT_H
#define JAMULFONT_H

#include "winpch.h"
#include "mgldraw.h"

const int FONT_MAX_CHARS = 128;

struct mfont_t
{
	byte numChars; // # of characters in the font
	byte firstChar; // the first character's ASCII value (they ascend from there)
	byte height; // height in pixels of the font
	byte spaceSize; // # of pixels wide to make spaces
	byte gapSize; // # of pixels between adjacent letters
	byte gapHeight; // # of pixels to descend for a carriage return
	long dataSize; // the size in bytes of the data of the characters themselves
	byte *data; // pointer to the character data
	byte * chars[FONT_MAX_CHARS]; // pointers to each character's data (can't have more than FONT_MAX_CHARS)
};

// each character in the font is stored as:
// width    1 byte       width of the character in pixels
// data     width*height bytes of actual data

// error codes
enum {
	FONT_OK = 0,
	FONT_FILENOTFOUND,
	FONT_CANTALLOC,
	FONT_INVALIDFILE
};

extern "C" {
void FontInit(MGLDraw *mgl);
void FontExit(void);

void FontFree(mfont_t *font);


int FontLoad(const char *fname, mfont_t *font);
int FontSave(const char *fname, mfont_t *font);

void FontPrintChar(int x, int y, char c, mfont_t *font);
void FontPrintCharSolid(int x, int y, char c, mfont_t *font, byte color);
void FontPrintCharBright(int x, int y, char c, char bright, mfont_t *font);
void FontPrintString(int x, int y, const char *s, mfont_t *font);
void FontPrintStringSolid(int x, int y, const char *s, mfont_t *font, byte color);
void FontPrintStringDropShadow(int x, int y, const char *s, mfont_t *font, byte shadowColor, byte shadowOffset);
void FontPrintStringColor(int x, int y, const char *s, mfont_t *font, byte color);
void FontPrintStringBright(int x, int y, const char *s, mfont_t *font, char bright);

int FontStrLen(const char *s, mfont_t *font);
void FontSetColors(byte first, byte count, byte *data);
bool FontInputText(char *prompt, char *buffer, int len, void (*renderScrn)(mfont_t *), mfont_t *font);
}

#endif
