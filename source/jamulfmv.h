#ifndef JAMULFMV_H
#define JAMULFMV_H

#include "jamultypes.h"
#include "mgldraw.h"
#include <stdio.h>

extern "C" {
void FLI_play(const char *name, byte loop, word wait, MGLDraw *mgl);
}

#endif
