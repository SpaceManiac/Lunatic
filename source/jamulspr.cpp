#include "jamulspr.h"

// the sprites are 12 bytes, not including the data itself
// note that the value here is 16 - there are four bytes of
// garbage between each sprite header
const int SPRITE_INFO_SIZE = 16;

/*
Jamul Sprite - JSP

header:
count		1 word	how many frames in this sprite
data:
count structures:
	width	1 word		width of sprite in pixels
	height	1 word		height of sprite in pixels
	ofsX	1 short		x-coord of hotspot relative to left
	ofsY	1 short		y-coord of hotspot relative to top
	size	1 dword		how big the sprite data is in bytes

count data chunks:
	data	size bytes	transparency RLE'd sprite data

	The RLE format is as follows:

	count	1 byte	if count is positive, this is considered
			a run of data, negative is a run of
			transparency.  If the run is data, it is
			followed by count bytes of data.  If
			it is transparent, the next RLE tag
			simply follows it.
			Runs do not cross line boundaries.
 */

// -------------------------------------------------------------------------
// ****************************** SPRITE_T *********************************
// -------------------------------------------------------------------------

// Helper shenanigans for C stuff
static const int constrainX = 0, constrainY = 0, constrainX2 = 639, constrainY2 = 479;

extern "C" {
	byte SprModifyColor(byte color, byte hue);
	byte SprGetColor(byte color);
	byte SprModifyLight(byte color, char bright);
	byte SprModifyGhost(byte src, byte dst, char bright);
	byte SprModifyGlow(byte src, byte dst, char bright);
}

// CONSTRUCTORS & DESTRUCTORS

sprite_t::sprite_t(void)
{
	width = 0;
	height = 0;
	ofsx = 0;
	ofsy = 0;
	size = 0;
	data = NULL;
}

sprite_t::sprite_t(byte *info)
{
	memcpy(&width, &info[0], 2);
	memcpy(&height, &info[2], 2);
	memcpy(&ofsx, &info[4], 2);
	memcpy(&ofsy, &info[6], 2);
	memcpy(&size, &info[8], 4);
}

sprite_t::~sprite_t(void)
{
	if (data)
		free(data);
}

// REGULAR MEMBER FUNCTIONS

bool sprite_t::LoadData(FILE *f)
{
	if (size == 0)
		return TRUE;

	data = (byte *) malloc(size);
	if (!data)
		return FALSE;

	if (fread(data, 1, size, f) != size)
	{
		return FALSE;
	}
	return TRUE;
}

bool sprite_t::SaveData(FILE *f)
{
	if (size == 0)
		return TRUE;

	if (!data)
		return TRUE;

	if (fwrite(data, 1, size, f) != size)
	{
		return FALSE;
	}
	return TRUE;
}

void sprite_t::GetHeader(byte *buffer)
{
	memcpy(&buffer[0], &width, 2);
	memcpy(&buffer[2], &height, 2);
	memcpy(&buffer[4], &ofsx, 2);
	memcpy(&buffer[6], &ofsy, 2);
	memcpy(&buffer[8], &size, 4);
}

void sprite_t::GetCoords(int x, int y, int *rx, int *ry, int *rx2, int *ry2)
{
	*rx = x - ofsx;
	*ry = y - ofsy;
	*rx2 = *rx + width;
	*ry2 = *ry + height;
}

void sprite_t::Draw(int x, int y, MGLDraw *mgl)
{
	byte *src, *dst, b, skip;
	dword pitch;
	int srcx, srcy;
	byte noDraw;

	x -= ofsx;
	y -= ofsy;
	if (x > constrainX2 || y > constrainY2)
		return; // whole sprite is offscreen

	pitch = mgl->GetWidth();
	src = data;
	dst = mgl->GetScreen() + x + y*pitch;

	srcx = x;
	srcy = y;
	if (srcy < constrainY)
		noDraw = 1;
	else
		noDraw = 0;
	while (srcy < height + y)
	{
		if ((*src)&128) // transparent run
		{
			b = (*src)&127;
			srcx += b;
			dst += b;
			src++;
		}
		else // solid run
		{
			b = *src;
			src++;
			if (srcx < constrainX - b || srcx > constrainX2)
			{
				// don't draw this line
				src += b;
				srcx += b;
				dst += b;
			}
			else if (srcx < constrainX)
			{
				// skip some of the beginning
				skip = (constrainX - srcx);
				src += skip;
				srcx += skip;
				dst += skip;
				b -= skip;
				if (srcx > constrainX2 - b)
				{
					skip = (b - (constrainX2 - srcx)) - 1;
					if (!noDraw)
						memcpy(dst, src, b - skip);
					src += b;
					srcx += b;
					dst += b;
				}
				else
				{
					if (!noDraw)
						memcpy(dst, src, b);
					src += b;
					srcx += b;
					dst += b;
				}
			}
			else if (srcx > constrainX2 - b)
			{
				// skip some of the end
				skip = (srcx - (constrainX2 - b)) - 1;
				if (!noDraw)
					memcpy(dst, src, b - skip);
				src += b;
				srcx += b;
				dst += b;
			}
			else
			{
				// do it all!
				if (!noDraw)
					memcpy(dst, src, b);
				srcx += b;
				src += b;
				dst += b;
			}
		}
		if (srcx >= width + x)
		{
			srcx = x;
			srcy++;
			dst += pitch - width;
			if (srcy >= constrainY)
				noDraw = 0;
			if (srcy > constrainY2)
				return;
		}
	}
}

//   bright: how much to darken or lighten the whole thing (-16 to +16 reasonable)

void sprite_t::DrawBright(int x, int y, MGLDraw *mgl, char bright)
{
	byte *src, *dst, b, skip;
	dword pitch;
	int srcx, srcy;
	byte noDraw;
	int i;

	if (bright == 0)
	{ // don't waste time!
		Draw(x, y, mgl);
		return;
	}

	x -= ofsx;
	y -= ofsy;
	if (x > constrainX2 || y > constrainY2)
		return; // whole sprite is offscreen

	pitch = mgl->GetWidth();
	src = data;
	dst = mgl->GetScreen() + x + y*pitch;

	srcx = x;
	srcy = y;
	if (srcy < constrainY)
		noDraw = 1;
	else
		noDraw = 0;
	while (srcy < height + y)
	{
		if ((*src)&128) // transparent run
		{
			b = (*src)&127;
			srcx += b;
			dst += b;
			src++;
		}
		else // solid run
		{
			b = *src;
			src++;
			if (srcx < constrainX - b || srcx > constrainX2)
			{
				// don't draw this line
				src += b;
				srcx += b;
				dst += b;
			}
			else if (srcx < constrainX)
			{
				// skip some of the beginning
				skip = (constrainX - srcx);
				src += skip;
				srcx += skip;
				dst += skip;
				b -= skip;
				if (srcx > constrainX2 - b)
				{
					skip = (b - (constrainX2 - srcx)) - 1;
					if (!noDraw)
						for (i = 0; i < b - skip; ++i)
							dst[i] = SprModifyLight(src[i], bright);
					src += b;
					srcx += b;
					dst += b;
				}
				else
				{
					if (!noDraw)
						for (i = 0; i < b; ++i)
							dst[i] = SprModifyLight(src[i], bright);
					src += b;
					srcx += b;
					dst += b;
				}
			}
			else if (srcx > constrainX2 - b)
			{
				// skip some of the end
				skip = (srcx - (constrainX2 - b)) - 1;
				if (!noDraw)
					for (i = 0; i < b - skip; ++i)
						dst[i] = SprModifyLight(src[i], bright);
				src += b;
				srcx += b;
				dst += b;
			}
			else
			{
				// do it all!
				if (!noDraw)
					for (i = 0; i < b; ++i)
						dst[i] = SprModifyLight(src[i], bright);
				srcx += b;
				src += b;
				dst += b;
			}
		}
		if (srcx >= width + x)
		{
			srcx = x;
			srcy++;
			dst += pitch - width;
			if (srcy >= constrainY)
				noDraw = 0;
			if (srcy > constrainY2)
				return;
		}
	}
}

//	 color:  which hue (0-7) to use for the entire thing, ignoring its real hue
//   bright: how much to darken or lighten the whole thing (-16 to +16 reasonable)

void sprite_t::DrawColored(int x, int y, MGLDraw *mgl, byte color, char bright)
{
	byte *src, *dst, b, skip;
	dword pitch;
	int srcx, srcy;
	byte noDraw;
	int i;

	x -= ofsx;
	y -= ofsy;
	if (x > constrainX2 || y > constrainY2)
		return; // whole sprite is offscreen

	pitch = mgl->GetWidth();
	src = data;
	dst = mgl->GetScreen() + x + y*pitch;

	srcx = x;
	srcy = y;
	if (srcy < constrainY)
		noDraw = 1;
	else
		noDraw = 0;
	while (srcy < height + y)
	{
		if ((*src)&128) // transparent run
		{
			b = (*src)&127;
			srcx += b;
			dst += b;
			src++;
		}
		else // solid run
		{
			b = *src;
			src++;
			if (srcx < constrainX - b || srcx > constrainX2)
			{
				// don't draw this line
				src += b;
				srcx += b;
				dst += b;
			}
			else if (srcx < constrainX)
			{
				// skip some of the beginning
				skip = (constrainX - srcx);
				src += skip;
				srcx += skip;
				dst += skip;
				b -= skip;
				if (srcx > constrainX2 - b)
				{
					skip = (b - (constrainX2 - srcx)) - 1;
					if (!noDraw)
						for (i = 0; i < b - skip; ++i)
							dst[i] = SprModifyLight(SprModifyColor(src[i], color), bright);
					src += b;
					srcx += b;
					dst += b;
				}
				else
				{
					if (!noDraw)
						for (i = 0; i < b; ++i)
							dst[i] = SprModifyLight(SprModifyColor(src[i], color), bright);
					src += b;
					srcx += b;
					dst += b;
				}
			}
			else if (srcx > constrainX2 - b)
			{
				// skip some of the end
				skip = (srcx - (constrainX2 - b)) - 1;
				if (!noDraw)
					for (i = 0; i < b - skip; ++i)
						dst[i] = SprModifyLight(SprModifyColor(src[i], color), bright);
				src += b;
				srcx += b;
				dst += b;
			}
			else
			{
				// do it all!
				if (!noDraw)
					for (i = 0; i < b; ++i)
						dst[i] = SprModifyLight(SprModifyColor(src[i], color), bright);
				srcx += b;
				src += b;
				dst += b;
			}
		}
		if (srcx >= width + x)
		{
			srcx = x;
			srcy++;
			dst += pitch - width;
			if (srcy >= constrainY)
				noDraw = 0;
			if (srcy > constrainY2)
				return;
		}
	}
}

void sprite_t::DrawOffColor(int x, int y, MGLDraw *mgl, byte fromColor, byte toColor, char bright)
{
	byte *src, *dst, b, skip;
	dword pitch;
	int srcx, srcy;
	byte noDraw;
	int i;

	x -= ofsx;
	y -= ofsy;
	if (x > constrainX2 || y > constrainY2)
		return; // whole sprite is offscreen

	pitch = mgl->GetWidth();
	src = data;
	dst = mgl->GetScreen() + x + y*pitch;

	srcx = x;
	srcy = y;
	if (srcy < constrainY)
		noDraw = 1;
	else
		noDraw = 0;
	while (srcy < height + y)
	{
		if ((*src)&128) // transparent run
		{
			b = (*src)&127;
			srcx += b;
			dst += b;
			src++;
		}
		else // solid run
		{
			b = *src;
			src++;
			if (srcx < constrainX - b || srcx > constrainX2)
			{
				// don't draw this line
				src += b;
				srcx += b;
				dst += b;
			}
			else if (srcx < constrainX)
			{
				// skip some of the beginning
				skip = (constrainX - srcx);
				src += skip;
				srcx += skip;
				dst += skip;
				b -= skip;
				if (srcx > constrainX2 - b)
				{
					skip = (b - (constrainX2 - srcx)) - 1;
					if (!noDraw)
						for (i = 0; i < b - skip; ++i)
							dst[i] = SprModifyLight(SprGetColor(src[i]) == fromColor ? SprModifyColor(src[i], toColor) : src[i], bright);
					src += b;
					srcx += b;
					dst += b;
				}
				else
				{
					if (!noDraw)
						for (i = 0; i < b; ++i)
							dst[i] = SprModifyLight(SprGetColor(src[i]) == fromColor ? SprModifyColor(src[i], toColor) : src[i], bright);
					src += b;
					srcx += b;
					dst += b;
				}
			}
			else if (srcx > constrainX2 - b)
			{
				// skip some of the end
				skip = (srcx - (constrainX2 - b)) - 1;
				if (!noDraw)
					for (i = 0; i < b - skip; ++i)
						dst[i] = SprModifyLight(SprGetColor(src[i]) == fromColor ? SprModifyColor(src[i], toColor) : src[i], bright);
				src += b;
				srcx += b;
				dst += b;
			}
			else
			{
				// do it all!
				if (!noDraw)
					for (i = 0; i < b; ++i)
						dst[i] = SprModifyLight(SprGetColor(src[i]) == fromColor ? SprModifyColor(src[i], toColor) : src[i], bright);
				srcx += b;
				src += b;
				dst += b;
			}
		}
		if (srcx >= width + x)
		{
			srcx = x;
			srcy++;
			dst += pitch - width;
			if (srcy >= constrainY)
				noDraw = 0;
			if (srcy > constrainY2)
				return;
		}
	}
}

// a ghost sprite is rather special.  It is drawn normally (except lightened
// or darkened according to the brightness parameter), except where it is grey
// (color 1-31).  Wherever those colors occur, they are instead used as the
// degree to which the background should be brightened instead of drawn over.
//   bright: how much to darken or lighten the whole thing (-16 to +16 reasonable)

void sprite_t::DrawGhost(int x, int y, MGLDraw *mgl, char bright)
{
	byte *src, *dst, b, skip;
	dword pitch;
	int srcx, srcy;
	byte noDraw;
	int i;

	x -= ofsx;
	y -= ofsy;
	if (x > constrainX2 || y > constrainY2)
		return; // whole sprite is offscreen

	pitch = mgl->GetWidth();
	src = data;
	dst = mgl->GetScreen() + x + y*pitch;

	srcx = x;
	srcy = y;
	if (srcy < constrainY)
		noDraw = 1;
	else
		noDraw = 0;
	while (srcy < height + y)
	{
		if ((*src)&128) // transparent run
		{
			b = (*src)&127;
			srcx += b;
			dst += b;
			src++;
		}
		else // solid run
		{
			b = *src;
			src++;
			if (srcx < constrainX - b || srcx > constrainX2)
			{
				// don't draw this line
				src += b;
				srcx += b;
				dst += b;
			}
			else if (srcx < constrainX)
			{
				// skip some of the beginning
				skip = (constrainX - srcx);
				src += skip;
				srcx += skip;
				dst += skip;
				b -= skip;
				if (srcx > constrainX2 - b)
				{
					skip = (b - (constrainX2 - srcx)) - 1;
					if (!noDraw)
						for (i = 0; i < b - skip; ++i)
							dst[i] = SprModifyGhost(src[i], dst[i], bright);
					src += b;
					srcx += b;
					dst += b;
				}
				else
				{
					if (!noDraw)
						for (i = 0; i < b; ++i)
							dst[i] = SprModifyGhost(src[i], dst[i], bright);
					src += b;
					srcx += b;
					dst += b;
				}
			}
			else if (srcx > constrainX2 - b)
			{
				// skip some of the end
				skip = (srcx - (constrainX2 - b)) - 1;
				if (!noDraw)
					for (i = 0; i < b - skip; ++i)
						dst[i] = SprModifyGhost(src[i], dst[i], bright);
				src += b;
				srcx += b;
				dst += b;
			}
			else
			{
				// do it all!
				if (!noDraw)
					for (i = 0; i < b; ++i)
						dst[i] = SprModifyGhost(src[i], dst[i], bright);
				srcx += b;
				src += b;
				dst += b;
			}
		}
		if (srcx >= width + x)
		{
			srcx = x;
			srcy++;
			dst += pitch - width;
			if (srcy >= constrainY)
				noDraw = 0;
			if (srcy > constrainY2)
				return;
		}
	}
}

void sprite_t::DrawGlow(int x, int y, MGLDraw *mgl, char bright)
{
	byte *src, *dst, b, skip;
	dword pitch;
	int srcx, srcy;
	byte noDraw;
	int i;

	x -= ofsx;
	y -= ofsy;
	if (x > constrainX2 || y > constrainY2)
		return; // whole sprite is offscreen

	pitch = mgl->GetWidth();
	src = data;
	dst = mgl->GetScreen() + x + y*pitch;

	srcx = x;
	srcy = y;
	if (srcy < constrainY)
		noDraw = 1;
	else
		noDraw = 0;
	while (srcy < height + y)
	{
		if ((*src)&128) // transparent run
		{
			b = (*src)&127;
			srcx += b;
			dst += b;
			src++;
		}
		else // solid run
		{
			b = *src;
			src++;
			if (srcx < constrainX - b || srcx > constrainX2)
			{
				// don't draw this line
				src += b;
				srcx += b;
				dst += b;
			}
			else if (srcx < constrainX)
			{
				// skip some of the beginning
				skip = (constrainX - srcx);
				src += skip;
				srcx += skip;
				dst += skip;
				b -= skip;
				if (srcx > constrainX2 - b)
				{
					skip = (b - (constrainX2 - srcx)) - 1;
					if (!noDraw)
						for (i = 0; i < b - skip; ++i)
							dst[i] = SprModifyGlow(src[i], dst[i], bright);
					src += b;
					srcx += b;
					dst += b;
				}
				else
				{
					if (!noDraw)
						for (i = 0; i < b; ++i)
							dst[i] = SprModifyGlow(src[i], dst[i], bright);
					src += b;
					srcx += b;
					dst += b;
				}
			}
			else if (srcx > constrainX2 - b)
			{
				// skip some of the end
				skip = (srcx - (constrainX2 - b)) - 1;
				if (!noDraw)
					for (i = 0; i < b - skip; ++i)
						dst[i] = SprModifyGlow(src[i], dst[i], bright);
				src += b;
				srcx += b;
				dst += b;
			}
			else
			{
				// do it all!
				if (!noDraw)
					for (i = 0; i < b; ++i)
						dst[i] = SprModifyGlow(src[i], dst[i], bright);
				srcx += b;
				src += b;
				dst += b;
			}
		}
		if (srcx >= width + x)
		{
			srcx = x;
			srcy++;
			dst += pitch - width;
			if (srcy >= constrainY)
				noDraw = 0;
			if (srcy > constrainY2)
				return;
		}
	}
}

void sprite_t::DrawShadow(int x, int y, MGLDraw *mgl)
{
	byte *src, *dst, b, skip;
	dword pitch;
	int srcx, srcy, x2;
	byte noDraw;
	byte alternate;
	int i;

	x -= ofsx + height / 2;
	y -= ofsy / 2;
	if (x > constrainX2 || y > constrainY2)
		return; // whole sprite is offscreen

	pitch = mgl->GetWidth();
	src = data;
	dst = mgl->GetScreen() + x + y*pitch;

	srcx = x;
	srcy = y;
	if (srcy < constrainY)
		noDraw = 1;
	else
		noDraw = 0;

	alternate = 1;
	x2 = x;

	while (srcy < height / 2 + y)
	{
		if ((*src)&128) // transparent run
		{
			b = (*src)&127;
			srcx += b;
			dst += b;
			src++;
		}
		else // solid run
		{
			b = *src;
			src++;
			if (srcx < constrainX - b || srcx > constrainX2)
			{
				// don't draw this line
				src += b;
				srcx += b;
				dst += b;
			}
			else if (srcx < constrainX)
			{
				// skip some of the beginning
				skip = (constrainX - srcx);
				src += skip;
				srcx += skip;
				dst += skip;
				b -= skip;
				if (srcx > constrainX2 - b)
				{
					skip = (b - (constrainX2 - srcx)) - 1;
					if (!noDraw && alternate)
						for (i = 0; i < b - skip; ++i)
							dst[i] = SprModifyLight(dst[i], -4);
					src += b;
					srcx += b;
					dst += b;
				}
				else
				{
					if (!noDraw && alternate)
						for (i = 0; i < b; ++i)
							dst[i] = SprModifyLight(dst[i], -4);
					src += b;
					srcx += b;
					dst += b;
				}
			}
			else if (srcx > constrainX2 - b)
			{
				// skip some of the end
				skip = (srcx - (constrainX2 - b)) - 1;
				if (!noDraw && alternate)
					for (i = 0; i < b - skip; ++i)
						dst[i] = SprModifyLight(dst[i], -4);
				src += b;
				srcx += b;
				dst += b;
			}
			else
			{
				// do it all!
				if (!noDraw && alternate)
					for (i = 0; i < b; ++i)
						dst[i] = SprModifyLight(dst[i], -4);
				srcx += b;
				src += b;
				dst += b;
			}
		}
		if (srcx >= width + x2)
		{
			alternate = 1 - alternate;
			x2 += alternate;
			srcx -= width - alternate;
			srcy += alternate;
			dst += (alternate ? pitch : 1) - width;
			if (srcy >= constrainY)
				noDraw = 0;
			if (srcy > constrainY2)
				return;
		}
	}
}

// -------------------------------------------------------------------------
// ***************************** SPRITE_SET_T ******************************
// -------------------------------------------------------------------------

// CONSTRUCTORS & DESTRUCTORS

sprite_set_t::sprite_set_t(const char *fname)
{
	count = 0;
	spr = NULL;
	Load(fname);
}

sprite_set_t::~sprite_set_t(void)
{
	Free();
}

// REGULAR MEMBER FUNCTIONS

bool sprite_set_t::Load(const char *fname)
{
	FILE *f;
	int i;
	byte *buffer;

	if (spr)
		Free();

	f = fopen(fname, "rb");
	if (!f)
		return FALSE;
	// read the count
	fread(&count, 2, 1, f);

#ifndef NDEBUG
	printf("loading %s, count = %d\n", fname, count);
#endif

	spr = (sprite_t **) malloc(sizeof (sprite_t *) * count);
	if (!spr)
	{
		fclose(f);
		return FALSE;
	}

	// allocate a buffer to load sprites into
	buffer = (byte *) malloc(SPRITE_INFO_SIZE * count);
	if (!buffer)
	{
		fclose(f);
		free(spr);
		return FALSE;
	}

	// read in the sprite headers
	if (fread(buffer, SPRITE_INFO_SIZE, count, f) != count)
	{
		fclose(f);
		free(spr);
		free(buffer);
		return FALSE;
	}

	// allocate the sprites and read in the data for them
	for (i = 0; i < count; i++)
	{
		spr[i] = new sprite_t(&buffer[i * SPRITE_INFO_SIZE]);
		if (!spr[i])
		{
			fclose(f);
			return FALSE;
		}
		if (!spr[i]->LoadData(f))
		{
			fclose(f);
			return FALSE;
		}
	}
	free(buffer);
	fclose(f);

	return TRUE;
}

bool sprite_set_t::Save(const char *fname)
{
	FILE *f;
	int i;
	byte *buffer;

	f = fopen(fname, "wb");
	if (!f)
		return FALSE;
	// write the count
	fwrite(&count, 2, 1, f);

	// allocate a buffer to copy sprites into
	buffer = (byte *) malloc(SPRITE_INFO_SIZE * count);
	if (!buffer)
	{
		fclose(f);
		return FALSE;
	}

	for (i = 0; i < count; i++)
		spr[i]->GetHeader(&buffer[i * SPRITE_INFO_SIZE]);

	// write the sprites out
	if (fwrite(buffer, SPRITE_INFO_SIZE, count, f) != count)
	{
		fclose(f);
		free(buffer);
		return FALSE;
	}

	// write the sprite data
	for (i = 0; i < count; i++)
	{
		if (!spr[i]->SaveData(f))
		{
			fclose(f);
			return FALSE;
		}
	}
	fclose(f);
	return TRUE;

}

void sprite_set_t::Free(void)
{
	int i;
	for (i = 0; i < count; i++)
		delete spr[i];
	free(spr);
}

sprite_t *sprite_set_t::GetSprite(int which)
{
	if (spr && which <= count && spr[which])
		return spr[which];
	return NULL;
}
