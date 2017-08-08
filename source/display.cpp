#include "display.h"
#include "particle.h"
#include "jamulfmv.h"
#include "game.h"
#include "options.h"

mfont_t *gameFont[2] = {NULL, NULL};
MGLDraw *mgl = NULL;

int scrx = 320, scry = 240, scrdx = 0, scrdy = 0;
int rscrx = 320 << FIXSHIFT, rscry = 240 << FIXSHIFT;

byte shakeTimer = 0;

DisplayList *dispList;

extern "C" void LoadText(char *nm);

void ShowImageOrFlic(char *str)
{
	dword start, end;
	int speed;

	char *fname;
	char *other;
	char nm[64];

	fname = strtok(str, ",\n");
	if (!fname)
		return;

	other = strtok(NULL, ",\n");

	// BMP loading
	if ((fname[strlen(fname) - 3] == 'b' || fname[strlen(fname) - 3] == 'B') &&
			(fname[strlen(fname) - 2] == 'm' || fname[strlen(fname) - 2] == 'M') &&
			(fname[strlen(fname) - 1] == 'p' || fname[strlen(fname) - 1] == 'P'))
	{
		EnterPictureDisplay();
		MakeNormalSound(SND_MESSAGE);
		sprintf(nm, "graphics\\%s", fname);
		GetDisplayMGL()->LoadBMP(nm);
		return;
	}
	if ((fname[strlen(fname) - 3] == 't' || fname[strlen(fname) - 3] == 'T') &&
			(fname[strlen(fname) - 2] == 'x' || fname[strlen(fname) - 2] == 'X') &&
			(fname[strlen(fname) - 1] == 't' || fname[strlen(fname) - 1] == 'T'))
	{
		EnterPictureDisplay();
		MakeNormalSound(SND_MESSAGE);
		sprintf(nm, "graphics\\%s", fname);
		LoadText(nm);
		return;
	}

	if (other)
		speed = atoi(other);
	else
		speed = 60;

	sprintf(nm, "graphics\\%s", fname);

	start = timeGetTime();
	FLI_play(nm, 0, speed, mgl);
	mgl->LoadBMP("graphics\\title.bmp");
	end = timeGetTime();
	AddGarbageTime(end - start);
}

void UpdateCamera(int x, int y, byte facing, Map *map)
{
	int desiredX, desiredY;

	desiredX = ((x << FIXSHIFT) + Cosine(facing)*80) >> FIXSHIFT;
	desiredY = ((y << FIXSHIFT) + Sine(facing)*60) >> FIXSHIFT;

	rscrx += scrdx;
	rscry += scrdy;

	if (rscrx < 320 << FIXSHIFT)
		rscrx = 320 << FIXSHIFT;
	if (rscrx > ((map->width * TILE_WIDTH - 320) << FIXSHIFT))
		rscrx = (map->width * TILE_WIDTH - 320) << FIXSHIFT;
	if (rscry < (240 - TILE_HEIGHT) << FIXSHIFT)
		rscry = (240 - TILE_HEIGHT) << FIXSHIFT;
	if (rscry > ((map->height * TILE_HEIGHT - 240) << FIXSHIFT))
		rscry = (map->height * TILE_HEIGHT - 240) << FIXSHIFT;

	if (scrx > desiredX + 20)
		scrdx = -((scrx - (desiredX + 20)) * FIXAMT / 16);
	if (scrx < desiredX - 20)
		scrdx = (((desiredX - 20) - scrx) * FIXAMT / 16);
	if (scry > desiredY + 20)
		scrdy = -((scry - (desiredY + 20)) * FIXAMT / 16);
	if (scry < desiredY - 20)
		scrdy = (((desiredY - 20) - scry) * FIXAMT / 16);

	Dampen(&scrdx, 1 << FIXSHIFT);
	Dampen(&scrdy, 1 << FIXSHIFT);

	scrx = (rscrx >> FIXSHIFT);
	scry = (rscry >> FIXSHIFT);
}

void RenderItAll(world_t *world, Map *map, byte flags)
{
	if (shakeTimer)
	{
		shakeTimer--;
		scrx -= 2 + MGL_random(5);
		scry -= 2 + MGL_random(5);
	}
	map->Render(world, scrx, scry, flags);

	scrx -= 320;
	scry -= 240;
	dispList->Render();
	dispList->ClearList();
	scrx += 320;
	scry += 240;
}

// ---------------------------------------------------------------------------------------
// from here on out it's class DISPLAYLIST

DisplayList::DisplayList(void)
{
	ClearList();
}

DisplayList::~DisplayList(void)
{
	// nothin
}

int DisplayList::GetOpenSlot(void)
{
	int i;

	for (i = 0; i < MAX_DISPLAY_OBJS; i++)
	{
		if (dispObj[i].flags == 0)
			return i;
	}

	return -1;
}

void DisplayList::HookIn(int me)
{
	int i;

	if (head == -1)
	{
		head = me;
		dispObj[me].prev = -1;
		dispObj[me].next = -1;
		return;
	}
	else
	{
		// shadows go on the head of the list always, drawn before anything else
		// (and the order of shadows doesn't matter, of course)
		if (dispObj[me].flags & DISPLAY_SHADOW)
		{
			dispObj[me].next = head;
			dispObj[head].prev = me;
			dispObj[me].prev = -1;
			head = me;
			return;
		}

		i = head;
		while (i != -1)
		{
			if ((!(dispObj[i].flags & DISPLAY_SHADOW)) &&
					(dispObj[i].y > dispObj[me].y || (dispObj[i].y == dispObj[me].y && dispObj[i].z > dispObj[me].z)))
			{
				dispObj[me].prev = dispObj[i].prev;
				dispObj[me].next = i;
				if (dispObj[me].prev != -1)
					dispObj[dispObj[me].prev].next = me;
				dispObj[i].prev = me;
				if (head == i)
					head = me;
				return;
			}
			if (dispObj[i].next == -1)
			{
				dispObj[i].next = me;
				dispObj[me].prev = i;
				dispObj[me].next = -1;
				return;
			}
			i = dispObj[i].next;
		}
		return; // this would be bad, but hopefully can't occur
	}
}

bool DisplayList::DrawSprite(int x, int y, int z, int z2, byte hue, char bright, sprite_t *spr, word flags)
{
	int i;

	if ((x - scrx + 320)<-DISPLAY_XBORDER || (x - scrx + 320) > 640 + DISPLAY_XBORDER ||
			(y - scry + 240)<-DISPLAY_YBORDER || (y - scry + 240) > 480 + DISPLAY_YBORDER)
		return true;
	i = GetOpenSlot();
	if (i == -1)
		return false;

	dispObj[i].hue = hue;
	dispObj[i].bright = bright;
	dispObj[i].flags = flags;
	dispObj[i].spr = spr;
	dispObj[i].x = x;
	dispObj[i].y = y;
	dispObj[i].z = z;
	dispObj[i].z2 = z2;
	HookIn(i);
	return true;
}

void DisplayList::ClearList(void)
{
	int i;

	for (i = 0; i < MAX_DISPLAY_OBJS; i++)
	{
		dispObj[i].prev = -1;
		dispObj[i].next = -1;
		dispObj[i].flags = 0;
	}
	head = -1;
	nextfree = 0;
}

void DisplayList::Render(void)
{
	int i;

	i = head;

	while (i != -1)
	{
		if ((dispObj[i].flags & DISPLAY_DRAWME) && (dispObj[i].spr))
		{
			if (dispObj[i].flags & DISPLAY_WALLTILE)
			{
				// for tiles, DISPLAY_GHOST means lighting is disabled
				char* bright = ((Map*) dispObj[i].spr)->MakeSmoothLighting(dispObj[i].flags&DISPLAY_GHOST, dispObj[i].x / 32, dispObj[i].y / 24);
				RenderWallTileFancy(dispObj[i].x - scrx, dispObj[i].y - scry, 199 + dispObj[i].z2, bright);
				RenderRoofTileFancy(dispObj[i].x - scrx, dispObj[i].y - scry - TILE_HEIGHT, dispObj[i].hue, dispObj[i].flags&DISPLAY_TRANSTILE, 0, bright);
			}
			else if (dispObj[i].flags & DISPLAY_ROOFTILE)
			{
				char* bright = ((Map*) dispObj[i].spr)->MakeSmoothLighting(dispObj[i].flags&DISPLAY_GHOST, dispObj[i].x / 32, dispObj[i].y / 24);
				RenderRoofTileFancy(dispObj[i].x - scrx, dispObj[i].y - scry - TILE_HEIGHT, dispObj[i].hue, dispObj[i].flags&DISPLAY_TRANSTILE, 0, bright);
			}
			else if (dispObj[i].flags & DISPLAY_SHADOW)
			{
				dispObj[i].spr->DrawShadow(dispObj[i].x - scrx, dispObj[i].y - scry - dispObj[i].z, mgl);
			}
			else if (dispObj[i].flags & DISPLAY_PARTICLE)
			{
				RenderParticle(dispObj[i].x - scrx, dispObj[i].y - scry - dispObj[i].z, mgl->GetScreen(),
						dispObj[i].hue, dispObj[i].bright);
			}
			else if (dispObj[i].flags & DISPLAY_LIGHTNING)
			{
				RenderLightningParticle(dispObj[i].x - scrx, dispObj[i].y - scry, dispObj[i].z - scrx,
						dispObj[i].z2 - scry, dispObj[i].bright, dispObj[i].hue, mgl->GetScreen());
			}
			else
			{
				if (dispObj[i].flags & DISPLAY_GHOST)
				{
					dispObj[i].spr->DrawGhost(dispObj[i].x - scrx, dispObj[i].y - scry - dispObj[i].z, mgl,
							dispObj[i].bright);
				}
				else if (dispObj[i].flags & DISPLAY_GLOW)
				{
					dispObj[i].spr->DrawGlow(dispObj[i].x - scrx, dispObj[i].y - scry - dispObj[i].z, mgl,
							dispObj[i].bright);
				}
				else if (dispObj[i].flags & DISPLAY_OFFCOLOR)
				{
					dispObj[i].spr->DrawOffColor(dispObj[i].x - scrx, dispObj[i].y - scry - dispObj[i].z, mgl,
							dispObj[i].z2, dispObj[i].hue, dispObj[i].bright);
				}
				else
				{
					if (dispObj[i].hue == 255) // no special coloring
					{
						dispObj[i].spr->DrawBright(dispObj[i].x - scrx, dispObj[i].y - scry - dispObj[i].z, mgl,
								dispObj[i].bright);
					}
					else // draw special color
					{
						dispObj[i].spr->DrawColored(dispObj[i].x - scrx, dispObj[i].y - scry - dispObj[i].z, mgl,
								dispObj[i].hue, dispObj[i].bright);
					}
				}
			}
		}
		i = dispObj[i].next;
	}
}
