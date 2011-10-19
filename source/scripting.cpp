#include "scripting.h"
#include "cpptcl/cpptcl.h"
#include "display.h"
#include "jamulfont.h"
#include "player.h"
#include <string>
#include <vector>
using namespace std;

struct console_t {
	bool active, flash;
	vector<string> backlog;
	
	string entry;
	string::iterator entryIter;
	
	vector<string> entryHistory;
	size_t historyPos;
} console;
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
			console.backlog.push_back(s.substr(0, i - 1));
			s = s.substr(i - 1);
			i = 1;
		}
	}
	console.backlog.push_back(s);
}

void InitScripting()
{
	interp = new tcl::interpreter();
	// safeify
	interp->def<void, std::string>("puts", LogConsole);
	interp->eval("rename exit {}");
	
	// define lm_ functions
	interp->eval("namespace eval lm_ {}");
	interp->def<byte, byte, int, int>("lm_::PlayerGetItem", PlayerGetItem);
	
	// execute embedded init.tcl
	HRSRC resource = FindResource(NULL, "init_tcl", "SCRIPT");
	dword size = SizeofResource(NULL, resource);
	void* data = LoadResource(NULL, resource);
	interp->eval(string((char*)data, size));
	
	console.active = false;
	console.entryIter = console.entry.begin();
	console.backlog.push_back("Tcl console initialized");
	console.entryHistory.push_back("# Top of history");
	console.historyPos = 1;
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
			if (console.historyPos > 0) console.historyPos--;
			console.entry = console.entryHistory[console.historyPos];
			console.entryIter = console.entry.end();
		}
		else if (scan == KEY_DOWN)
		{
			if (console.historyPos < console.entryHistory.size())
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

	mgl->FillBox(322, 2, 638, 162, 10);
	mgl->FillBox(324, 4, 636, 144, 20);
	mgl->FillBox(324, 146, 636, 160, 20);
	Print(326, 147, console.entry.c_str(), 0, 1);
	
	if (console.flash)
	{
		// draw the caret
		int x = 325 + FontStrLen(console.entry.substr(0, console.entryIter - console.entry.begin()).c_str(), gameFont[1]);
		mgl->Box(x, 147, x, 147+12, 31);
	}

	int start = console.backlog.size() - 10;
	if (start < 0) start = 0;
	int y = 5 + 14 * (10 - console.backlog.size());
	if (y < 5) y = 5;

	for (size_t i = start; i < console.backlog.size(); ++i)
	{
		Print(326, y, console.backlog[i].c_str(), 0, 1);
		y += 14;
	}
}
