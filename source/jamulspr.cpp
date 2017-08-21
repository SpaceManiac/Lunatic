#include "jamulspr.h"

// -------------------------------------------------------------------------
// ****************************** SPRITE_T *********************************
// -------------------------------------------------------------------------

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
void sprite_t::Draw(int x, int y, MGLDraw *mgl) {
	Sprite_Draw(this, x, y, mgl);
}
void sprite_t::DrawBright(int x, int y, MGLDraw *mgl, char bright) {
	Sprite_DrawBright(this, x, y, mgl, bright);
}

// -------------------------------------------------------------------------
// ***************************** SPRITE_SET_T ******************************
// -------------------------------------------------------------------------

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

sprite_t *sprite_set_t::GetSprite(int which) {
	if (spr && which >= 0 && (dword)which < count)
		return spr[which];
	return NULL;
}
