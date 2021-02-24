/**************************************************************************
 * TURO COMPILER
 * 
 * lex.c - Lexical analysis functions.
 * 
 * Copyright (c) 2021. N. Nemenzo. All rights reserved.
 * May not be used, modified, or copied without permission.
 **************************************************************************/ 

// #include "turo.h"

// char *lexemeBegin = NULL;
// char *forward = NULL;
// char *current_buff = NULL;

// char lexeme_buf[MAX_LEXEME_LENGTH];
// static char printbuff[100];

// /* load_buffer() - function that loads a buffer
//  * Parameters: primary - denotes whether to load the primary or secondary buffer
//  * Side effects: loads a buffer (rewrites a current one or writes to a new one)
//  */
// static void load_buffer(bool primary) {
//     char *buff = (primary) ? primary_buff : secondary_buff;
//     // read characters from file
//     char c;
//     size_t i = 0;
//     while ((c = fgetc(infile)) != EOF && i < BUFFER_SIZE - 1) {
//         buff[i++] = (char) c;
//     }
//     assert(buff[BUFFER_SIZE - 1] == EOF);
// }

// void init_lexer(void) {
//     if (!terminate) {
//         // place sentinels (should never be overwritten)
//         primary_buff[BUFFER_SIZE - 1] = EOF;
//         secondary_buff[BUFFER_SIZE - 1] = EOF;
//         // read into primary
//         current_buff = primary_buff;
//         // set pointers
//         lexemeBegin = primary_buff;
//         forward = lexemeBegin + 1;
//         load_buffer(true);
//     }
// }

// /* create_token() - allocate memory for a token
//  * Side effects: none
//  * Return: return a 'base' token
//  */
// token_t *create_token(char lexeme[]) {
//     token_t *ret = (token_t *)calloc(1, sizeof(token_t));
//     ret->ttype = TOK_IDENTITY;
//     strcpy(ret->lexeme, lexeme);
//     return ret;
// }

// /* check_whitespace() - check if a given character is the initial state for whitespace
//  * Parameters: c - given character
//  * Side effects: none
//  * Return: true if whitespace, false if not
//  */
// bool check_whitespace(char c) {
//     return c == '\n' || c == ' ' || c == '\t';
// }

// /* check_relop_assn() - check if a given character is the initial state for relational operators
//                         or assignment
//  * Parameters: c - given character
//  * Side effects: none
//  * Return: true if so, false if not
//  */
// bool check_relop_assn(char c) {
//     return c == '<' || c == '=' || c == '!' || c == '>'; 
// }

// /* check_relop_assn() - check if a given character is the initial state for arithmetic operators
//                         or comments
//  * Parameters: c - given character
//  * Side effects: none
//  * Return: true if so, false if not
//  */
// bool check_arithop_comm(char c) {
//     return c == '+' || c == '-' || c == '/' || c == '*';   
// }

// /* check_relop_assn() - check if a given character is punctuation
//  * Parameters: c - given character
//  * Side effects: none
//  * Return: true if so, false if not
//  */
// bool check_punctuation(char c) {
//     return c == '(' || c == ')' || c == ';' || c == ',';  
// }

// /* install_id() - installs an id into the symbol table or returns the id associated with the found id
//  * Parameters: lexeme - passed in the lexeme to check the symbol table for
//  * Side effects: mutates the symbol table
//  * Return: a token with the corresponding lexeme OR an existing token (could possibly be a keyword)
//  */
// token_t *install_id(char lexeme[]) {

// }

// /* get_identifier() - finds an identifier in the given input stream
//  * Side effects: moves lexemeBegin and forward pointers
//  * Return: the identifier token with data corresponding to found id
//  */
// token_t *get_identifier(void) {
//     int state = 0;
//     int i = 0;
//     while (i < MAX_LEXEME_LENGTH) {
//         switch(state) {
//             case 0:
//                 lexeme_buf[i++] = *lexemeBegin;
//                 state = 1;
//                 break;
//             case 1:
//                 if (isalpha(*forward) || isdigit(*forward) || *forward == '_') {
//                     lexeme_buf[i++] = *forward++;
//                 } else {
//                     // check if identifier and return
//                 }
//         }
//     }
//     // warning when lexeme buff max length reached
//     if (i == MAX_LEXEME_LENGTH) {
//         sprintf(printbuff, "Found lexeme '%s' is greater than 100 characters.\n", lexeme_buf);
//         logmsg(LOG_WARNING, printbuff);
//     }
// }

// /* get_whitespace() - finds whitespace in the given input stream
//  * Side effects: moves lexemeBegin and forward pointers
//  * Return: the whitespace token with data corresponding to found whitespace
//  */
// token_t *get_whitespace(void) {

// }

// /* get_number() - finds a number in the given input stream
//  * Side effects: moves lexemeBegin and forward pointers
//  * Return: the number token with data corresponding to found number
//  */
// token_t *get_number(void) {

// }

// /* get_relop_assign() - finds a relop or assign lexeme in the given input stream
//  * Side effects: moves lexemeBegin and forward pointers
//  * Return: the relop or assign token
//  */
// token_t *get_relop_assign(void) {

// }

// /* get_punctuation() - finds a punctuation lexeme in the given input stream
//  * Side effects: moves lexemeBegin and forward pointers
//  * Return: the punctuation token
//  */
// token_t *get_punctuation(void) {

// }

// /* get_arithop_comment() - finds an arithop or comment lexeme in the given input stream
//  * Side effects: moves lexemeBegin and forward pointers
//  * Return: the respective token
//  */
// token_t *get_arithop_comment(void) {

// }

// token_t *get_token(void) {
//     if (isalpha(*lexemeBegin) || *lexemeBegin == '_') {
//         // need to differentiate between keywords and identifiers
//     } else if (isdigit(*lexemeBegin)) {

//     } else if (check_whitespace(*lexemeBegin)) {

//     } else if (check_relop_assn(*lexemeBegin)) {

//     } else if (check_arithop_comm(*lexemeBegin)) {
        
//     } else if (check_punctuation(*lexemeBegin)) {

//     } else if (*lexemeBegin == EOF) {

//     } else { // declare error (didn't find a proper initial state)
//         handle_error(ERR_LEX);
//         return NULL;
//     }
// }