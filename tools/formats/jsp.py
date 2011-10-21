# jsp.py - part of the Loonymod project
# import this file and use the jsp construct.

from common import *

# Official documentation:
# Jamul Sprite - JSP
#
# header:
# count		1 word	how many frames in this sprite
# data:
# count structures:
# 	width	1 word		width of sprite in pixels
# 	height	1 word		height of sprite in pixels
# 	ofsX	1 short		x-coord of hotspot relative to left
# 	ofsY	1 short		y-coord of hotspot relative to top
# 	size	1 dword		how big the sprite data is in bytes
# 	data	size bytes	transparency RLE'd sprite data
# 
# 	The RLE format is as follows:
# 
# 	count	1 byte	if count is positive, this is considered
# 			a run of data, negative is a run of
# 			transparency.  If the run is data, it is
# 			followed by count bytes of data.  If
# 			it is transparent, the next RLE tag
# 			simply follows it.
# 			Runs do not cross line boundaries.

# constructs

class RleSprite(Construct):
	def _parse(self, stream, context):
		if "_curSprite" in context:
			info = context["frameInfo"][context["_curSprite"]]
			context["_curSprite"] += 1
		else:
			info = context["frameInfo"][0]
			context["_curSprite"] = 1
		
		return stream.read(info.dataSize)
		
	def _build(self, obj, stream, context):
		return obj
		
	def _sizeof(self, context):
		raise SizeofError

# structures

frameInfo = Struct("frameInfo",
	ULInt16("width"),
	ULInt16("height"),
	SLInt16("offsetX"),
	SLInt16("offsetY"),
	ULInt32("dataSize"),
	Field("garbage", 4)
)

jsp = Struct("jsp",
	ULInt16("count"),
	CtxRepeater("count", frameInfo),
	CtxRepeater("count", RleSprite("frameData"))
)
