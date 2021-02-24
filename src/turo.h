/**************************************************************************
 * TURO COMPILER
 * 
 * turo.h - Contains functions and variables necessary for program function.
 * TOP-LEVEL CODE IS BASED ON A 2021 UTCS ASSIGNMENT BY S. CHATTERJEE, X. SHEN, AND T. BYRD.
 * 
 * Copyright (c) 2021. N. Nemenzo. All rights reserved.
 * May not be used, modified, or copied without permission.
 **************************************************************************/ 

#ifndef TURO_H
#define TURO_H

#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <assert.h>
#include <unistd.h>
#include <ctype.h>
#include <string.h>
#include <time.h>
#include "err_handler.h"
#include "lex.h"
#include "symboltable.h"

/* handle_args() - function to handle command-line arguments given to the program
 *
 */
extern void handle_args(int argc, char *argv[]);

/* init() - initialization procedures
 * Side effects:
 */
extern void init_interface(void);

/* finalize() - function to print the final statements of the compilation
 * Side effects: 
 */
extern void finalize(void);

/* test() - unit tests
 * Parameters: parts of program to test
 */
extern void test(symbol_table_t *);

/* Program variables */

/* File variables
 * infile - input file to read from
 * outfile - output file to output to
 * errfile - error file
 */
extern FILE *infile;
extern FILE *outfile;
extern FILE *errfile;

/* Variables that control program execution:
 * ignore_input - ignores input but doesn't terminate the program
 * terminate - ends the program
 */
extern bool ignore_input, terminate;

#endif // TURO_H