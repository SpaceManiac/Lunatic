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

extern "C" void Sprite_Draw(sprite_t*, int, int, MGLDraw*);
extern "C" void Sprite_DrawBright(sprite_t*, int, int, MGLDraw*, char);
void sprite_t::Draw(int x, int y, MGLDraw *mgl) {
	Sprite_Draw(this, x, y, mgl);
}
void sprite_t::DrawBright(int x, int y, MGLDraw *mgl, char bright) {
	Sprite_DrawBright(this, x, y, mgl, bright);
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
