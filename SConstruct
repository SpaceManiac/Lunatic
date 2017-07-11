# Dr. Lunatic SCons configuration script

import os
import platform
import subprocess

LIBS = ['winmm', 'ws2_32', 'advapi32', 'userenv', 'shell32', 'gcc_eh', 'pthread',
	'alleg44', 'logg', 'vorbisfile', 'vorbis', 'ogg', 'vorbisenc']

def getFileList(dir, ext):
	for dirpath, dirnames, filenames in os.walk(dir):
		for name in filenames:
			if 'old/' not in name and name.endswith(ext):
				yield os.path.join(dirpath, name)

class Cargo(object):
	def __init__(self, debug):
		self.input = list(getFileList('rust/', ''))
		self.output = 'build/' + ('debug' if debug else 'release') + '/lunatic.lib'
		self.command = ['cargo', 'build']
		if not debug:
			self.command.append('--release')

	def __call__(self, env, **kwargs):
		environ = dict(env['ENV'])
		environ['CARGO_TARGET_DIR'] = '../build'
		subprocess.check_call(self.command, cwd='rust/', env=environ)

def program(output, debug):
	# if we're on Windows, force Mingw use
	if platform.system() == 'Windows':
		env = Environment(ENV = os.environ, tools = ['mingw'])
	else:
		env = Environment(ENV = os.environ)

	# compiler
	env.Append(CCFLAGS = ['-Wall', '-Wextra', '-Wno-unused-parameter', '-std=c++11'])
	env.Append(CPPPATH = ['include'])
	env.Append(CPPDEFINES = ['ALLEGRO_MINGW32', 'EXPANDO'])
	if debug:
		env.Append(CPPDEFINES = ['_DEBUG', 'LOG'])
		env.Append(CCFLAGS = ['-g'])
	else:
		env.Append(CPPDEFINES = ['NDEBUG'])
		env.Append(CCFLAGS = ['-O2', '-s'])
		env.Append(LINKFLAGS = ['-O2', '-s', '-mwindows'])

	# linker
	env.Append(LINKFLAGS = ['-static-libgcc', '-static-libstdc++', '-std=c++11'])
	env.Append(LIBPATH = ['lib/i686-pc-windows-gnu'])
	env.Append(LIBS = LIBS)

	# output files
	objects = []
	for source in getFileList('source/', '.cpp'):
		object = 'build/' + output + '/' + source.replace('.cpp', '.o')
		objects.append(env.Object(target=object, source=source))

	# resources
	for source in getFileList('source/', '.rc'):
		object = 'build/' + output + '/' + source.replace('.rc', '.res')
		objects.append(env.Command(object, source, 'windres ' + source + ' -O coff -o ' + object))

	# cargo
	cargo = Cargo(debug)
	objects.append(env.Command(cargo.output, cargo.input, cargo))

	# finish
	outputExe = 'bin/' + output + '.exe'
	return Alias(output, [
		env.Program(target=outputExe, source=objects),
		env.Install('game/', outputExe),
	])

lunatic = program('lunatic', False)
lunatic_debug = program('lunatic_debug', True)
