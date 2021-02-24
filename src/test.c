/**************************************************************************
 * TURO COMPILER
 * 
 * test.c - Testing module for the Turo Compiler
 * 
 * Copyright (c) 2021. N. Nemenzo. All rights reserved.
 * May not be used, modified, or copied without permission.
 **************************************************************************/

#include "turo.h"


char printbuff[100];

 /*test_format() - formatting of test output
 * Parameters: module being tested, test number, passed or not
 * Return: formatted output
 */
 
static char *test_format(int testnum, char *module, bool passed) {
    if (passed) {
        sprintf(printbuff, "[%s] Test %d passed.\n", module, testnum);
    } else {
        sprintf(printbuff, "[%s] Test %d failed. **\n", module, testnum);
    }
    return printbuff;
}

/* test_log() - log function for testing
 * Parameters: test number, module of program to be tested, passed or not
 */
 
static int test_log(int testnum, char *module, bool passed) {
    if (outfile != stdout) {
        fprintf(outfile, "[TESTING]\n");
    }
    return fprintf(outfile, "%s\n", test_format(testnum, module, passed));
}

void test(symbol_table_t *tptr) {

}


