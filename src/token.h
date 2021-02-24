/**************************************************************************
 * TURO COMPILER
 * 
 * token.h - Contains token types and struct for lexeme
 * 
 * Copyright (c) 2021. N. Nemenzo. All rights reserved.
 * May not be used, modified, or copied without permission.
 **************************************************************************/ 

#ifndef TOKEN_H
#define TOKEN_H

/* Sets an upper bound for the max representation length of a token (ids can't be more than 100) */
#define MAX_LEXEME_LENGTH 100

/* Enum for all possible types that a token may have */
typedef enum {
    /* ----- keywords ----- */
    TOK_FUNCTION,
    TOK_MAIN,
    TOK_DEF,
    TOK_DECL,
    TOK_TRUE,
    TOK_FALSE,
    /* ----- type keywords ----- */
    TOK_INT,
    TOK_STRING,
    TOK_BOOL,
    /* ----- program flow ----- */
    TOK_IF,
    TOK_ELSE,
    TOK_WHILE,
    TOK_ASSIGN,
    /* ----- whitespace types ----- */
    TOK_PURENL,
    TOK_PUREWS,
    TOK_NLWS,
    /* ----- identifiers & literals ----- */
    TOK_ID,
    TOK_NUM,
    TOK_STR,
    /* ----- relational operators ----- */
    TOK_EQ,
    TOK_NE,
    TOK_GT,
    TOK_LT,
    TOK_GTE,
    TOK_LTE,
    /* ----- arithmetic operators ----- */
    TOK_PLUS,
    TOK_MINUS,
    TOK_MULT,
    TOK_DIV,
    /* ----- comments & punctuation ----- */
    TOK_LPAR,
    TOK_RPAR,
    TOK_SEMICOL,
    TOK_COMMENT,
    /* ----- other ----- */
    TOK_IDENTITY, // 'nothing' type
    TOK_EOF,
    TOK_INVALID = -1,
} token_type_t;

/* Struct for tokens found in input string */
typedef struct token {
    token_type_t ttype;
    char *lexeme;
} token_t, *tptr_t;

#endif // TOKEN_H