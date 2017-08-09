#include "mgldraw.h"

// Functions with Rust implementations
extern "C" bool MGLDraw_Process(MGLDraw*);
extern "C" bool MGLDraw_Flip(MGLDraw*);
extern "C" void MGLDraw_SetPalette(MGLDraw*, const palette_t*);
extern "C" void MGLDraw_Box(MGLDraw*, int, int, int, int, byte);
extern "C" void MGLDraw_FillBox(MGLDraw*, int, int, int, int, byte);
extern "C" void MGLDraw_TeleportMouse(MGLDraw*, int, int);
extern "C" bool MGLDraw_LoadBMP(MGLDraw*, const char*);
extern "C" void MGLDraw_GammaCorrect(MGLDraw*, byte);

bool MGLDraw::Process() {
	return MGLDraw_Process(this);
}
void MGLDraw::Flip() {
	MGLDraw_Flip(this);
}
void MGLDraw::SetPalette(const palette_t *pal2) {
	MGLDraw_SetPalette(this, pal2);
}
void MGLDraw::Box(int x, int y, int x2, int y2, byte c) {
	MGLDraw_Box(this, x, y, x2, y2, c);
}
void MGLDraw::FillBox(int x, int y, int x2, int y2, byte c) {
	MGLDraw_FillBox(this, x, y, x2, y2, c);
}
void MGLDraw::TeleportMouse(int x, int y) {
	MGLDraw_TeleportMouse(this, x, y);
}
bool MGLDraw::LoadBMP(const char *name) {
	return MGLDraw_LoadBMP(this, name);
}
void MGLDraw::GammaCorrect(byte gamma) {
	MGLDraw_GammaCorrect(this, gamma);
}
