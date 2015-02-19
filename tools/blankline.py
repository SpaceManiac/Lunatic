# Lunatic blanklines.py
# Tad Hardesty, aka SpaceManiac, 2011
# Ensures every file in source has a blank line at the end
# usage: 'python tools/blankline.py'

import os
import itertools

def getFileList(dir, ext):
	for dirpath, dirnames, filenames in os.walk(dir):
		for name in filenames:
			if 'old/' not in name and name.endswith(ext):
				yield os.path.join(dirpath, name)

files = itertools.chain(
	getFileList('source', '.cpp'),
	getFileList('source', '.h'),
	getFileList('tools', '.py'))

for path in files:
	f = file(path)
	data = f.read()
	f.close()

	lines = 0
	while len(data) > lines and data[-1 - lines] == '\n':
		lines += 1

	if lines != 1:
		if lines > 0:
			data = data[:-lines] + '\n'
		else:
			data = data + '\n'
		f = file(path, 'w')
		f.write(data)
		f.write('\n')
		f.close()
		print 'Corrected ' + path
