#ifndef MESSAGE_H
#define MESSAGE_H

#include "winpch.h"
#include "display.h"

struct message_t
{
	int x, y;
	int dy;
	int timer;
	int bright;
	char brightDir;
	char msg[32];
	byte priority;
};

void InitMessage(void);
extern "C" void NewMessage(const char *txt, int time, byte priority);
void NewBigMessage(const char *txt, int time);
void UpdateMessage(void);
void RenderMessage(void);
void NoRepeatNewMessage(const char *txt, int time, byte priority);

#endif
