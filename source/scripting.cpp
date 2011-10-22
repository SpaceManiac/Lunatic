#include "scripting.h"
#include "cpptcl/cpptcl.h"
#include "display.h"
#include "jamulfont.h"
#include "player.h"
#include <string>
#include <vector>
using namespace std;

#define CONSOLE_SIZE (15)
#define LINE_HEIGHT (14)

struct console_t
{
	bool active, flash;

	string entry;
	string::iterator entryIter;

	vector<string> backlog;
	size_t backlogPos;

	vector<string> entryHistory;
	size_t historyPos;
} console;
tcl::interpreter* interp;

extern mfont_t *gameFont[2]; // the fonts, for FontWidth - see display.cpp

void LogConsole(std::string s)
{
	bool move = console.backlogPos >= console.backlog.size() - CONSOLE_SIZE - 1;

	for (size_t i = 1; i < s.length(); ++i)
	{
		int len = FontStrLen(s.substr(0, i).c_str(), gameFont[1]);
		if (len > 308)
		{
			// wrap if we need to
			console.backlog.push_back(s.substr(0, i - 1));
			s = s.substr(i - 1);
			i = 1;
		}
	}

	console.backlog.push_back(s);

	while (console.backlog.size() > CONSOLE_SIZE && console.backlog[0] == "")
	{
		console.backlog.erase(console.backlog.begin());
	}

	if (move) console.backlogPos = console.backlog.size() - CONSOLE_SIZE;
}

void InitScripting()
{
	interp = new tcl::interpreter();
	// safeify
	interp->def<void, std::string > ("puts", LogConsole);
	interp->eval("rename exit {}");

	// define lm_ functions
	interp->eval("namespace eval lm_ {}");
	interp->def<byte, byte, int, int>("lm_::PlayerGetItem", PlayerGetItem);

	// execute embedded init.tcl
	HRSRC resource = FindResource(NULL, "init_tcl", "SCRIPT");
	dword size = SizeofResource(NULL, resource);
	void* data = LoadResource(NULL, resource);
	interp->eval(string((char*) data, size));

	// configure console
	console.active = false;
	console.entryIter = console.entry.begin();

	for (int i = 0; i < CONSOLE_SIZE - 1; ++i) console.backlog.push_back("");
	console.backlog.push_back("Tcl console initialized");
	console.backlogPos = 0;

	console.entryHistory.push_back("# Top of history");
	console.historyPos = 1;
}

void ExitScripting()
{
	delete interp;
	interp = NULL;
}

byte ScriptKeyPressed(int keycode)
{
	char ch = keycode & 0xff;
	int scan = keycode >> 8;

	if (scan == KEY_TILDE || scan == KEY_BACKQUOTE)
	{
		console.active = !console.active;
	}
	else if (console.active)
	{
		if (scan == KEY_ESC)
		{
			console.active = false;
		}
		else if (scan == KEY_ENTER && console.entry != "")
		{
			LogConsole("%" + console.entry);
			std::string s;
			try
			{
				s = (std::string) interp->eval(console.entry);
			}
			catch (tcl::tcl_error& e)
			{
				s = e.what();
			}
			if (s != "") LogConsole(s);

			console.entryHistory.push_back(console.entry);
			console.historyPos = console.entryHistory.size();

			console.entry.clear();
			console.entryIter = console.entry.begin();
		}
		else if (scan == KEY_BACKSPACE)
		{
			if (console.entryIter != console.entry.begin())
			{
				--console.entryIter;
				console.entryIter = console.entry.erase(console.entryIter);
			}
		}
		else if (scan == KEY_DEL)
		{
			if (console.entryIter != console.entry.end()) console.entryIter = console.entry.erase(console.entryIter);
		}
		else if (scan == KEY_LEFT)
		{
			if (console.entryIter != console.entry.begin()) --console.entryIter;
		}
		else if (scan == KEY_RIGHT)
		{
			if (console.entryIter != console.entry.end()) ++console.entryIter;
		}
		else if (scan == KEY_UP)
		{
			if (key[KEY_LCONTROL] || key[KEY_RCONTROL])
			{
				if (console.backlogPos > 0) console.backlogPos--;
			}
			else
			{
				if (console.historyPos > 0) console.historyPos--;
				console.entry = console.entryHistory[console.historyPos];
				console.entryIter = console.entry.end();
			}
		}
		else if (scan == KEY_DOWN)
		{
			if (key[KEY_LCONTROL] || key[KEY_RCONTROL])
			{
				if (console.backlogPos < console.backlog.size() - CONSOLE_SIZE) console.backlogPos++;
			}
			else if (console.historyPos < console.entryHistory.size())
			{
				if (++console.historyPos == console.entryHistory.size())
				{
					console.entry = "";
				}
				else
				{
					console.entry = console.entryHistory[console.historyPos];
				}
				console.entryIter = console.entry.end();
			}
		}
		else if (ch >= ' ' && ch <= '~')
		{
			console.entryIter = console.entry.insert(console.entryIter, ch);
			++console.entryIter;
		}
	}

	return console.active;
}

void RenderConsole(MGLDraw* mgl)
{
	if (!console.active) return;
	console.flash = !console.flash;

	mgl->FillBox(322, 2, 638, 8 + (CONSOLE_SIZE + 1) * LINE_HEIGHT, 10);
	mgl->FillBox(324, 4, 636, 4 + CONSOLE_SIZE*LINE_HEIGHT, 20);
	mgl->FillBox(324, 6 + CONSOLE_SIZE*LINE_HEIGHT, 636, 6 + (CONSOLE_SIZE + 1) * LINE_HEIGHT, 20);
	Print(326, 7 + CONSOLE_SIZE*LINE_HEIGHT, console.entry.c_str(), 0, 1);

	if (console.flash)
	{
		// draw the caret
		int x = 325 + FontStrLen(console.entry.substr(0, console.entryIter - console.entry.begin()).c_str(), gameFont[1]);
		mgl->Box(x, 7 + CONSOLE_SIZE*LINE_HEIGHT, x, 5 + (CONSOLE_SIZE + 1) * LINE_HEIGHT, 31);
	}

	// draw up/down "arrows" if log is scrollable
	if (console.backlogPos > 0)
		mgl->FillBox(627, 7 + CONSOLE_SIZE * LINE_HEIGHT, 635, 10 + CONSOLE_SIZE * LINE_HEIGHT, 10);
	if (console.backlogPos < console.backlog.size() - CONSOLE_SIZE)
		mgl->FillBox(627, 16 + CONSOLE_SIZE * LINE_HEIGHT, 635, 19 + CONSOLE_SIZE * LINE_HEIGHT, 10);

	int y = 5;
	for (size_t i = console.backlogPos; i < console.backlogPos + CONSOLE_SIZE; ++i)
	{
		Print(326, y, console.backlog[i].c_str(), 0, 1);
		y += LINE_HEIGHT;
	}
}
