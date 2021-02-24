/**************************************************************************
 * TURO COMPILER
 * 
 * interface.c - Initialization and finalization of the Turo Compiler.
 * 
 * Copyright (c) 2021. N. Nemenzo. All rights reserved.
 * May not be used, modified, or copied without permission.
 **************************************************************************/

#include "turo.h"

const char author[] = "Nathaniel Nemenzo";

/* print_init_msg() - function that prints the initial messages when the program first begins
 */
static void print_init_msg(void) {
    time_t t;
    fprintf(outfile, "\nWelcome to the Turo Compiler!\n");
    fprintf(outfile, "Author: %s\n\n", author);
    assert(time(&t) != -1);
    fprintf(outfile, "Compilation begun at: %s\n\n", ctime(&t));
}

/* print_final_msg90 - function that prints the final messages then the program finalizes and ends
 */
static void print_final_msg(void) {
    time_t t;
    assert(time(&t) != -1);
    fprintf(outfile, "\nCompilation ended at: %s\n", ctime(&t));
}

void init_interface(void) {
    if (!terminate) {
        print_init_msg();
    }
}

void finalize(void) {
    if (!terminate) {
        print_final_msg();
    }
}