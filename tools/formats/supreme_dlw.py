# supreme_dlw.py - part of the Loonymod project
# import this file and use the supreme_dlw construct.

from common import *

# constructs and adapters

class RleBitmap(Construct):
	def isEncoded(self, method, row):
		return bool((ord(method[row / 8]) >> (row % 8)) & 1)
		
	def _parse(self, stream, context):
		rows = []
		for row in range(24):
			if self.isEncoded(context["method"], row):
				cols = []
				while len(cols) < 32:
					cols += map(ord, ord(stream.read(1)) * stream.read(1))
				rows.append(cols)
			else:
				rows.append(map(ord, stream.read(32)))
		return rows
		
	def _build(self, obj, stream, context):
		for row in range(24):
			cols = obj[row]
			if self.isEncoded(context["method"], row):
				value, run = cols[0], 1
				for pixel in cols[1:]:
					if pixel == value:
						run += 1
					else:
						stream.write(chr(run) + value.build())
						value, run = pixel, 1
				stream.write(chr(run) + value.build())
			else:
				stream.write(''.join(map(chr, cols)))
		
	def _sizeof(self, context):
		raise SizeofError

class RleLevel(Construct):
	def _parse(self, stream, context):
		width, height = context["width"], context["height"]
		rawTiles = []
		while len(rawTiles) < width * height:
			run = SLInt8("run").parse(stream.read(1))
			if run < 0:
				tile = levelTile.parse(stream.read(levelTile.sizeof()))
				for i in range(-run):
					rawTiles.append(tile)
			else:
				for i in range(run):
					rawTiles.append(levelTile.parse(stream.read(levelTile.sizeof())))
		rows = []
		for row in range(height):
			cols = []
			for col in range(width):
				cols.append(rawTiles[row * width + col])
			rows.append(cols)
		return rows
		
	def _build(self, obj, stream, context):
		tiles = []
		for row in obj:
			for col in row:
				tiles.append(col)
		value, run = tiles[0], run
		for tile in tiles[1:]:
			if tile == value and run < 127:
				run += 1
			else:
				stream.write(chr(run) + levelTile.build(value))
				value, run = tile, 1
		stream.write(chr(run) + levelTile.build(value))
		
	def _sizeof(self, context):
		raise SizeofError

class ItemContainer(Construct):
	def _parse(self, stream, context):
		result = []
		itemId = 0
		for i in range(context["itemCount"]):
			if itemId != 255:
				itemId = ord(stream.read(1))
				data = item.parse(stream.read(item.sizeof()))
				data.itemId = itemId
				result.append(data)
			else:
				data = item.parse(stream.read(item.sizeof()))
				data.itemId = 255
				result.append(data)
		return result
		
	def _build(self, obj, stream, context):
		part = 0
		for data in obj:
			if data.itemId == 255:
				if part == 0:
					stream.write(chr(255))
					part = 1
				stream.write(item.build(data))
			else:
				stream.write(chr(data.itemId))
				stream.write(item.build(data))
		
	def _sizeof(self, context):
		raise SizeofError

class ItemDropAdapter(Adapter):
	def _encode(self, obj, ctx):
		return chr(256 * (obj - int(obj))) + chr(obj)
	def _decode(self, obj, ctx):
		return ord(obj[1]) + ord(obj[0]) / 256.0

# structures

monster = Struct("monster",
	ULInt8("x"),
	ULInt8("y"),
	ULInt8("type"),
	ULInt8("item"),
)

trigger = Struct("trigger",
	ULInt8("parameter"),
	ULInt8("type"),
	ULInt8("x"),
	ULInt8("y"),
	ULInt32("index1"),
	ULInt32("index2"),
)

effect = Struct("effect",
	Embed(trigger),
	PackedString("text"),
)

special = Struct("special",
	ULInt8("x"),
	ULInt8("y"),
	ULInt8("uses"),
	BitStruct("length",
		BitField("effects", 5),
		BitField("triggers", 3)
	),
	MetaRepeater(lambda ctx: ctx["length"]["triggers"], trigger),
	MetaRepeater(lambda ctx: ctx["length"]["effects"], effect),
)

levelTile = Struct("levelTile",
	ULInt16("floor"),
	ULInt16("wall"),
	ULInt8("item"),
	SLInt8("light"),
)

level = Struct("level",
	ULInt8("width"),
	ULInt8("height"),
	PackedString("name"),
	PackedString("song"),
	ULInt8("monsterCount"),
	MetaRepeater(lambda ctx: ctx["monsterCount"], monster),
	ULInt8("specialCount"),
	MetaRepeater(lambda ctx: ctx["specialCount"], special),
	BitStruct("flags",
		Flag("underwater"),
		Flag("starry"),
		Flag("lantern"),
		Flag("torch"),
		Flag("secret"),
		Flag("hub"),
		Flag("raining"),
		Flag("snowing"),
		Flag("reserved5"),
		Flag("reserved4"),
		Flag("reserved3"),
		Flag("reserved2"),
		Flag("reserved1"),
		Flag("reserved0"),
		Flag("stealth"),
		Flag("underlava"),
	),
	ULInt16("brains"),
	ULInt16("candles"),
	ItemDropAdapter(Bytes("itemDrop", 2)),
	RleLevel("tiles"),
)

tileImage = Struct("tileImage",
	Bytes("method", 3),
	RleBitmap("bitmap"),
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
		Flag("bouncy"),
		Flag("enemyProof"),
		Flag("ghostProof"),
		Flag("bunnyPath"),
		Flag("minecartPath"),
		Flag("transparentRoof"),
		Flag("animateHit"),
		Flag("animateStep"),
	),
	ULInt16("nextTile"),
)

item = Struct("item",
	PackedString("name"),
	SLInt8("offsetX"),
	SLInt8("offsetY"),
	ULInt16("sprite"),
	ULInt8("fromColor"),
	ULInt8("toColor"),
	SLInt8("light"),
	ULInt8("rarity"),
	BitStruct("flags",
		Padding(1),
		Flag("useTileGraphic"),
		Flag("loonyColor"),
		Flag("pickup"),
		Flag("bulletproof"),
		Flag("impassible"),
		Flag("glowing"),
		Flag("shadow"),
		Padding(8)
	),
	BitStruct("themes",
		Flag("crate"),
		Flag("rock"),
		Flag("tree"),
		Flag("door"),
		Flag("bulletproof"),
		Flag("obstacle"),
		Flag("decoration"),
		Flag("pickup"),
		Flag("chair"),
		Flag("entrance"),
		Flag("food"),
		Flag("collectible"),
		Flag("key"),
		Flag("powerup"),
		Flag("weapon"),
		Flag("sign"),
		Padding(7),
		Flag("custom"),
		Padding(8)
	),
	BitStruct("trigger",
		Flag("always"),
		Flag("minecart"),
		Flag("machete"),
		Flag("friendbump"),
		Flag("enemybump"),
		Flag("playerbump"),
		Flag("shoot"),
		Flag("pickup"),
		Padding(8)
	),
	ULInt8("effect"),
	SLInt16("effectData"),
	PackedString("message", 64),
	ULInt16("sound")
)

sound = Struct("sound",
	ULInt16("soundId"),
	PackedString("name"),
	BitStruct("theme",
		Padding(2),
		Flag("custom"),
		Flag("vocal"),
		Flag("effect"),
		Flag("monster"),
		Flag("player"),
		Flag("intface")
	),
	SLInt32("dataSize"),
	MetaField("data", lambda ctx: ctx["dataSize"])
)

supreme_dlw = Struct("world",
	String("gameid", 8),
	PackedString("author"),
	PackedString("name"),
	ULInt8("levelCount"),
	ULInt32("totalPoints"),
	ULInt16("tileCount"),
	MetaRepeater(lambda ctx: ctx["tileCount"], tileImage),
	MetaRepeater(lambda ctx: ctx["tileCount"], tileData),
	MetaRepeater(lambda ctx: ctx["levelCount"], level),
	ULInt16("itemCount"),
	ItemContainer("items"),
	SLInt16("soundCount"),
	MetaRepeater(lambda ctx: ctx["soundCount"], sound)
)
