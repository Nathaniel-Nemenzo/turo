/**************************************************************************
 * TURO COMPILER
 * 
 * lex.h - Contains functions and variables for lexical analysis.
 * 
 * Copyright (c) 2021. N. Nemenzo. All rights reserved.
 * May not be used, modified, or copied without permission.
 **************************************************************************/ 

#include "token.h"

#define BUFFER_SIZE 2048

/* Input buffers */
char primary_buff[BUFFER_SIZE];
char secondary_buff[BUFFER_SIZE];

/* Lexeme pointers */
char *lexemeBegin, *forward;

/* init_lexer() - initialization of lexer
 * Side effects: fills input buffers and sets lexeme pointers
 */
void init_lexer(void);

/* get_token() - gets a token from the input stream
 * Side effects: advances lexeme pointers and (possibly) refills input buffers
 */
token_t *get_token(void);