#ifndef TITLE_H
#define TITLE_H

#include "winpch.h"
#include "display.h"

extern "C" {
byte WorldPicker(MGLDraw *mgl);
byte MainMenu(MGLDraw *mgl);
void Credits(MGLDraw *mgl);
void SplashScreen(MGLDraw *mgl, const char *fname, int delay, byte sound);
void VictoryText(MGLDraw *mgl);
void HelpScreens(MGLDraw *mgl);
void DemoSplashScreens(MGLDraw *mgl);

void ScanWorldNames(void);
void ReScanWorldNames(void);
}

#endif
