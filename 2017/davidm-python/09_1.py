with open('09.input') as f:
    input = f.read().strip()

def garbage(seq):
    i = 0
    while i < len(seq):
        if '>' in seq[i]:
            return i + 1
        elif '!' in seq[i]:
            i += 1
        i += 1

def analyze(seq, depth):
    tot_score = 0
    i = 0
    while i < len(seq):
        if '{' in seq[i]:
            length, score = analyze(seq[i+1:], depth + 1)
            i += length
            tot_score += score
        elif '}' in seq[i]:
            tot_score += depth
            return i + 1, tot_score
        elif '<' in seq[i]:
            i += garbage(seq[i+1:])
        elif '!' in seq[i]:
            i += 1
        i += 1
    return i, tot_score

print(analyze(input[1:], 1))
