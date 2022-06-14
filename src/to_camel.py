a = """
// Single-character tokens.
TOKEN_LEFT_PAREN, TOKEN_RIGHT_PAREN,
TOKEN_LEFT_BRACE, TOKEN_RIGHT_BRACE,
TOKEN_COMMA, TOKEN_DOT, TOKEN_MINUS, TOKEN_PLUS,
TOKEN_SEMICOLON, TOKEN_SLASH, TOKEN_STAR,
// One or two character tokens.
TOKEN_BANG, TOKEN_BANG_EQUAL,
TOKEN_EQUAL, TOKEN_EQUAL_EQUAL,
TOKEN_GREATER, TOKEN_GREATER_EQUAL,
TOKEN_LESS, TOKEN_LESS_EQUAL,
// Literals.
TOKEN_IDENTIFIER, TOKEN_STRING, TOKEN_NUMBER,
// Keywords.
TOKEN_AND, TOKEN_CLASS, TOKEN_ELSE, TOKEN_FALSE,
TOKEN_FOR, TOKEN_FUN, TOKEN_IF, TOKEN_NIL, TOKEN_OR,
TOKEN_PRINT, TOKEN_RETURN, TOKEN_SUPER, TOKEN_THIS,
TOKEN_TRUE, TOKEN_VAR, TOKEN_WHILE,
TOKEN_ERROR,
TOKEN_EOF
"""


all_lines = []
for line in a.split('\n'):
    line = line.strip()
    if line:
        if not line.startswith('//'):
            new_words = []
            words = line.split(',')
            for word in words:
                word = word.strip()
                word = word.lower().replace('_', ' ')
                thingy = word.split()
                new_words.append(''.join(map(lambda x: x.capitalize(), thingy)))
            all_lines.append(', '.join(new_words))
        else:
            all_lines.append(line)

for i in all_lines:
    print(i)