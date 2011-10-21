# common.py - part of the Loonymod project
# these structures and adapters are used by various formats.

from construct import *

# pretty much everything is little endian
# dword = ULInt32
# word = ULInt16
# byte = ULInt8

# constructs

class WithContext(Construct):
	def __init__(self, func):
		Construct.__init__(self, "WithContext")
		self.func = func
	def _parse(self, stream, context):
		self.func(context)

# adapters

class NullTerminateAdapter(Adapter):
	def _encode(self, obj, ctx):
		return obj
	def _decode(self, obj, ctx):
		return obj[:obj.find('\x00')]

# utilities

def echo(x): print x

def PrintContext():
	return WithContext(echo)

def PrintContextItem(field):
	return WithContext(lambda ctx: echo(ctx[field]))

def PackedString(name, len=32):
	return NullTerminateAdapter(String(name, len))

def CtxRepeater(name, obj):
	return MetaRepeater(lambda ctx: ctx[name], obj)
