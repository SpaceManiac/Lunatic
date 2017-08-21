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

extern "C" sprite_t* Sprite_Alloc();
extern "C" void Sprite_Destruct(sprite_t*);
extern "C" void Sprite_Dealloc(void*);

extern "C" void Sprite_Draw(sprite_t*, int, int, MGLDraw*);
extern "C" void Sprite_DrawBright(sprite_t*, int, int, MGLDraw*, char);

sprite_t::~sprite_t(void) {
	Sprite_Destruct(this);
}

void* sprite_t::operator new(size_t) {
	return Sprite_Alloc();
}

void sprite_t::operator delete(void* p, size_t) {
	Sprite_Dealloc(p);
}

// REGULAR MEMBER FUNCTIONS

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

extern "C" sprite_set_t* SpriteSet_Alloc();
extern "C" void SpriteSet_Destruct(void*);
extern "C" void SpriteSet_Dealloc(void*);
extern "C" void SpriteSet_Load(sprite_set_t*, const char*);

sprite_set_t::sprite_set_t(const char *fname) {
	SpriteSet_Load(this, fname);
}

sprite_set_t::~sprite_set_t(void) {
	SpriteSet_Destruct(this);
}

void* sprite_set_t::operator new(size_t) {
	return SpriteSet_Alloc();
}

void sprite_set_t::operator delete(void* p, size_t) {
	SpriteSet_Dealloc(p);
}

// REGULAR MEMBER FUNCTIONS

sprite_t *sprite_set_t::GetSprite(int which)
{
	if (spr && which <= count && spr[which])
		return spr[which];
	return NULL;
}
