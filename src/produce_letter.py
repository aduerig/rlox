
final = []
for i in range(26):
    final.append(chr(ord('a') + i))

for i in range(26):
    final.append(chr(ord('A') + i))


print('|'.join(map(lambda x: "'" + x + "'", final)))