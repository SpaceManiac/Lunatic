#ifndef SCRIPTING_H
#define	SCRIPTING_H

#include "jamultypes.h"
#include "mgldraw.h"

void InitScripting();
void ExitScripting();

byte ScriptKeyPressed(int key);
void RenderConsole(MGLDraw* mgl);

#endif
