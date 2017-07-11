#ifndef CLOCK_H
#define CLOCK_H

#include "winpch.h"

extern "C" {
    void StartClock(void);
    void EndClock(void);
    dword TimeLength(void);
}

#endif
