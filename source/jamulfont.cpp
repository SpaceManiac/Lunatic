#include "jamulfont.h"


MGLDraw *fontmgl;
// this is a sort of palette translation table for the font
byte fontPal[256];

void FontPrintChar(int x, int y, char c, mfont_t *font)
{
	byte *dst, *src;
	int scrWidth, scrHeight, chrWidth;
	int i, j;

	scrWidth = fontmgl->GetWidth();
	scrHeight = fontmgl->GetHeight();
	dst = fontmgl->GetScreen() + x + y*scrWidth;

	if (c < font->firstChar || c >= (font->firstChar + font->numChars))
		return; // unprintable

	c -= (char) font->firstChar;

	// c -> (int)c to prevent warning: array subscript has type 'char'
	chrWidth = *(font->chars[(int) c]);
	src = font->chars[(int) c] + 1;
	for (j = 0; j < font->height; j++)
	{
		for (i = 0; i < (*font->chars[(int) c]); i++)
		{
			if (*src && (x > 0) && (x < scrWidth) && (y > 0) && (y < scrHeight))
				*dst = fontPal[*src];
			dst++;
			src++;
			x++;
		}
		y++;
		x -= chrWidth;
		dst += (scrWidth - chrWidth);
	}
}

extern "C" void FontPrintCharColor(int x, int y, char c, byte color, mfont_t *font)
{
	byte *dst, *src;
	int scrWidth, scrHeight, chrWidth;
	int i, j;

	scrWidth = fontmgl->GetWidth();
	scrHeight = fontmgl->GetHeight();
	dst = fontmgl->GetScreen() + x + y*scrWidth;

	if (c < font->firstChar || c >= (font->firstChar + font->numChars))
		return; // unprintable

	c -= (char) font->firstChar;

	chrWidth = *(font->chars[(int) c]);
	src = font->chars[(int) c] + 1;
	color *= 32;
	for (j = 0; j < font->height; j++)
	{
		for (i = 0; i < (*font->chars[(int) c]); i++)
		{
			if (*src && (x > 0) && (x < scrWidth) && (y > 0) && (y < scrHeight))
			{
				if ((*src >= 64 && *src < 64 + 32) || (*src >= 128 && *src < 128 + 32))
					*dst = ((*src)&31) + color;
				else
					*dst = *src;
			}
			dst++;
			src++;
			x++;
		}
		y++;
		x -= chrWidth;
		dst += (scrWidth - chrWidth);
	}
}

void FontPrintCharBright(int x, int y, char c, char bright, mfont_t *font)
{
	byte *dst, *src;
	int scrWidth, scrHeight, chrWidth;
	int i, j;

	scrWidth = fontmgl->GetWidth();
	scrHeight = fontmgl->GetHeight();
	dst = fontmgl->GetScreen() + x + y*scrWidth;

	if (c < font->firstChar || c >= (font->firstChar + font->numChars))
		return; // unprintable

	c -= (char) font->firstChar;

	chrWidth = *(font->chars[(int) c]);
	src = font->chars[(int) c] + 1;

	for (j = 0; j < font->height; j++)
	{
		for (i = 0; i < (*font->chars[(int) c]); i++)
		{
			if (*src && (x > 0) && (x < scrWidth) && (y > 0) && (y < scrHeight))
			{
				*dst = *src + bright;
				if (*dst > (*src & (~31)) + 31)
					*dst = (*src & (~31)) + 31;
				else if (*dst < (*src & (~31)))
					*dst = *src & (~31);
			}
			dst++;
			src++;
			x++;
		}
		y++;
		x -= chrWidth;
		dst += (scrWidth - chrWidth);
	}
}

void FontPrintCharSolid(int x, int y, char c, mfont_t *font, byte color)
{
	byte *dst, *src;
	int scrWidth, scrHeight, chrWidth;
	int i, j;

	scrWidth = fontmgl->GetWidth();
	scrHeight = fontmgl->GetHeight();
	dst = fontmgl->GetScreen() + x + y*scrWidth;

	if (c < font->firstChar || c >= (font->firstChar + font->numChars))
		return; // unprintable

	c -= (char) font->firstChar;

	chrWidth = *(font->chars[(int) c]);
	src = font->chars[(int) c] + 1;
	for (j = 0; j < font->height; j++)
	{
		for (i = 0; i < (*font->chars[(int) c]); i++)
		{
			if (*src && (x > 0) && (x < scrWidth) && (y > 0) && (y < scrHeight))
				*dst = color;
			dst++;
			src++;
			x++;
		}
		y++;
		x -= chrWidth;
		dst += (scrWidth - chrWidth);
	}
}

bool FontInputText(char *prompt, char *buffer, int len, void (*renderScrn)(mfont_t *), mfont_t *font)
{
	int pos = 0;
	bool done = 0;
	char c;

	while (buffer[pos] && pos < len)
		pos++;
	while (!done)
	{
		renderScrn(font);
		fontmgl->FillBox(0, 200, 639, 250, 0);
		fontmgl->Box(0, 200, 639, 250, 255);
		FontPrintString(2, 202, prompt, font);
		buffer[pos] = '_';
		buffer[pos + 1] = '\0';
		FontPrintString(2, 202 + font->height + 2, buffer, font);
		buffer[pos] = '\0';
		fontmgl->Flip();
		if (!fontmgl->Process())
			return FALSE;
		if ((c = fontmgl->LastKeyPressed())) // extra pair of parentheses for a warning about assignment in truth value
		{
			if (c == 8) // backspace
			{
				if (pos > 0)
				{
					pos--;
					buffer[pos] = '\0';
				}
			}
			else if (c == 27)
			{
				done = TRUE;
				buffer[0] = '\0';
			}
			else if (c == 13)
			{
				done = TRUE;
				buffer[pos] = '\0';
			}
			else if (pos < len)
			{
				buffer[pos++] = c;
				buffer[pos] = '\0';
			}
		}
	}
	return TRUE;
}
