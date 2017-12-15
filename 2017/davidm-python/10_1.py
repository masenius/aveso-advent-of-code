lengths = [206,63,255,131,65,80,238,157,254,24,133,2,16,0,1,3]
list_ = list(range(256))
#lengths = [3,4,1,5]
#list_ = list(range(5))
current = 0
skip = 0

for length in lengths:
    l = list_[current:] + list_[0:current]
    l = list(reversed(l[0:length])) + l[length:]
    list_ = l[len(list_) - current:] + l[0:len(list_) - current]
    current = (current + length + skip) % len(list_)
    skip += 1

print(list_[0] * list_[1])
