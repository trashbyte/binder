text = open('output.txt', 'rt').read()
lines = filter(lambda l: len(l) > 0, text.split('\n'))
xml_stripped = map(lambda l: l.replace('<?xml version=\"1.0\" encoding=\"UTF-8\"?><testsuites>','').replace('</testsuites>',''), lines)

with open('tests.xml', 'wt') as f:
    f.write('<?xml version=\"1.0\" encoding=\"UTF-8\"?><testsuites>' + "".join(list(xml_stripped)) + '</testsuites>')