# lunatic_dlw.py - part of the Loonymod project
# import this file and use the lunatic_dlw construct.

from common import *

# structures

monster = Struct("monster",
	ULInt8("x"),
	ULInt8("y"),
	ULInt8("type")
)

special = Struct("special",
	BitStruct("trigger",
		Flag("shoot"),
		Flag("haveBrains"),
		Flag("killAll"),
		Flag("haveKeychains"),
		Flag("passedLevels"),
		Flag("near"),
		Flag("enemyStep"),
		Flag("step"),
		Flag("floorAt"),
		Flag("killMonster"),
		Flag("hasLoonyKey"),
		Flag("randomChance"),
		Flag("timer"),
		Flag("chainAdjacent"),
		Flag("showMessage"),
		Flag("canRepeat")
	),
	ULInt8("triggerValue"),
	ULInt8("effect"),
	ULInt8("x"),
	ULInt8("y"),
	ULInt8("effectX"),
	ULInt8("effectY"),
	PackedString("message")
)

levelTile = Struct("levelTile",
	ULInt8("floor"),
	ULInt8("wall"),
	ULInt8("item"),
	SLInt8("light"),
	SLInt8("tempLight"),
	ULInt8("opaque")
)

level = Struct("level",
	SLInt16("width"),
	SLInt16("height"),
	PackedString("name"),
	Repeater(128, 128, monster),
	Repeater(32, 32, special),
	ULInt8("song"),
	ULInt8("flags"),
	MetaRepeater(lambda ctx: ctx["height"],
		MetaRepeater(lambda ctx: ctx["width"], levelTile)
	)
)

tileData = Struct("tileData",
	BitStruct("flags",
		Flag("animate"),
		Flag("canpushon"),
		Flag("pushable"),
		Flag("lava"),
		Flag("water"),
		Flag("muddy"),
		Flag("icy"),
		Flag("impassible"),
		Padding(3),
		Flag("bunnyPath"),
		Flag("minecartPath"),
		Flag("transparentRoof"),
		Flag("animateHit"),
		Flag("animateStep"),
	),
	ULInt8("nextTile")
)

lunatic_dlw = Struct("world",
	ULInt8("levelCount"),
	SLInt16("totalPoints"),
	Repeater(400, 400, Field("tileImage", 32*24)),
	Repeater(200, 200, tileData),
	MetaRepeater(lambda ctx: ctx["levelCount"], level)
)
