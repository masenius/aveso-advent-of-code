line = list(map(chr, range(97,113)))

with open('16.input') as f:
    input = f.read().strip().split(',')

results = []
for i in range(1000000000):
    l = ''.join(line)
    if l in results:
        print(results[1000000000 % i])
        break
    results.append(l)
    for move in input:
        if move[0] == 's':
            line = line[-int(move[1:]):] + line[:-int(move[1:])]
        elif move[0] == 'x':
            a, b = map(int, move[1:].split('/'))
            line[a], line[b] = line[b], line[a]
        elif move[0] == 'p':
            a, b = map(lambda x: line.index(x), move[1:].split('/'))
            line[a], line[b] = line[b], line[a]
