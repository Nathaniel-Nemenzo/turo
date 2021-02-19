/**************************************************************************
 * TURO COMPILER
 * 
 * lex.c - Lexical analysis functions.
 * 
 * Copyright (c) 2021. N. Nemenzo. All rights reserved.
 * May not be used, modified, or copied without permission.
 **************************************************************************/ 

#include "turo.h"

char *lexemeBegin = NULL;
char *forward = NULL;

/* load_buffer() - function that loads a buffer
 * Parameters: primary - denotes whether to load the primary or secondary buffer
 * Side effects: loads a buffer (rewrites a current one or writes to a new one)
 */
static void load_buffer(bool primary) {
    char *buff = (primary) ? primary_buff : secondary_buff;
    // read characters from file
    char c;
    size_t i = 0;
    while ((c = fgetc(infile)) != EOF && i < BUFFER_SIZE - 1) {
        buff[i++] = (char) c;
    }
    assert(buff[BUFFER_SIZE - 1] == EOF);
}

void init_lexer(void) {
    if (!terminate) {
        // place sentinels (should never be overwritten)
        primary_buff[BUFFER_SIZE - 1] = EOF;
        secondary_buff[BUFFER_SIZE - 1] = EOF;
        // set pointers
        lexemeBegin = primary_buff;
        forward = primary_buff;
        load_buffer(true);
    }
}

token_t *get_token(void) {
    return NULL;
}