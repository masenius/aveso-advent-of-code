with open('09.input') as f:
    input = f.read().strip()

def garbage(seq):
    i = 0
    j = 0
    while i < len(seq):
        if '>' in seq[i]:
            return i + 1, j
        elif '!' in seq[i]:
            i += 2
        else:
            i += 1
            j += 1

def analyze(seq):
    tot_score = 0
    i = 0
    while i < len(seq):
        if '{' in seq[i]:
            length, score = analyze(seq[i+1:])
            i += length
            tot_score += score
        elif '}' in seq[i]:
            return i + 1, tot_score
        elif '<' in seq[i]:
            length, score = garbage(seq[i+1:])
            i += length
            tot_score += score
        elif '!' in seq[i]:
            i += 1
        i += 1
    return i, tot_score

print(analyze(input[1:]))
