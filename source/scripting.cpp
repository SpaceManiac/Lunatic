#include "scripting.h"
#include "cpptcl/cpptcl.h"
#include "display.h"
#include "jamulfont.h"
#include "player.h"
#include <string>
#include <vector>

bool consoleActive;
bool cursorFlash;
std::string consoleEntry;
std::vector<std::string> consoleBacklog;
tcl::interpreter* interp;

extern mfont_t *gameFont[2]; // the fonts, for FontWidth - see display.cpp

void LogConsole(std::string s)
{
	for (size_t i = 1; i < s.length(); ++i)
	{
		int len = FontStrLen(s.substr(0, i).c_str(), gameFont[1]);
		if (len > 308)
		{
			// wrap if we need to
			consoleBacklog.push_back(s.substr(0, i - 1));
			s = s.substr(i - 1);
			i = 1;
		}
	}
	consoleBacklog.push_back(s);
}

void InitScripting()
{
	consoleActive = false;
	
	interp = new tcl::interpreter();
	interp->def<void, std::string>("puts", LogConsole);
	interp->def<byte, byte, int, int>("_PlayerGetItem", PlayerGetItem);
	interp->eval("rename exit {}");
	
	consoleBacklog.push_back("Tcl console initialized");
}

void ExitScripting()
{
	delete interp;
	interp = NULL;
}

byte ScriptKeyPressed(int key)
{
	char ch = key & 0xff;
	int scan = key >> 8;

	if (scan == KEY_TILDE || scan == KEY_BACKQUOTE)
	{
		consoleActive = !consoleActive;
	}
	else if (consoleActive)
	{
		if (scan == KEY_ESC)
		{
			consoleActive = false;
		}
		else if (scan == KEY_ENTER)
		{
			consoleBacklog.push_back("%" + consoleEntry);
			std::string s;
			try
			{
				s = (std::string) interp->eval(consoleEntry);
			}
			catch (tcl::tcl_error& e)
			{
				s = e.what();
			}
			if (s != "") LogConsole(s);
			consoleEntry = "";
		}
		else if (scan == KEY_BACKSPACE || scan == KEY_DEL)
		{
			if (consoleEntry.length() > 0)
			{
				consoleEntry = consoleEntry.substr(0, consoleEntry.length() - 1);
			}
		}
		else if (ch >= ' ' && ch <= '~')
		{
			consoleEntry += ch;
		}
	}

	return consoleActive;
}

void RenderConsole(MGLDraw* mgl)
{
	if (!consoleActive) return;
	cursorFlash = !cursorFlash;

	mgl->FillBox(322, 2, 638, 162, 10);
	mgl->FillBox(324, 4, 636, 144, 20);
	mgl->FillBox(324, 146, 636, 160, 20);
	Print(326, 147, (consoleEntry + (cursorFlash ? "|" : "")).c_str(), 0, 1);

	int start = consoleBacklog.size() - 10;
	if (start < 0) start = 0;
	int y = 5 + 14 * (10 - consoleBacklog.size());
	if (y < 5) y = 5;

	for (size_t i = start; i < consoleBacklog.size(); ++i)
	{
		Print(326, y, consoleBacklog[i].c_str(), 0, 1);
		y += 14;
	}
}
