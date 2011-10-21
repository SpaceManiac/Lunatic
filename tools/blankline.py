# Lunatic blanklines.py
# Tad Hardesty, aka SpaceManiac, 2011
# Ensures every file in source has a blank line at the end
# usage: 'python tools/blankline.py'

from glob import glob
from sys import argv
from os import path
import os, re

def getFileList(dir, ext):
	result = []
	for file in map(lambda(x): x.replace('\\', '/'), glob(dir + '/*')):
		if path.isdir(file):
			result += getFileList(file, ext)
		else:
			if file.find("old/") < 0 and file.endswith(ext):
				result += [file]
	return result

files = getFileList('source', '.cpp') + getFileList('source', '.h')

for path in files:
	f = file(path)
	data = f.read()
	f.close()

	if data[-1] != '\n':
		f = file(path, 'w')
		f.write(data)
		f.write('\n')
		f.close()
		print 'Corrected ' + path
