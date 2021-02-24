/**************************************************************************
 * TURO COMPILER
 * 
 * err_handler.h - Contains functions for error handling.
 * ERROR HANDLING CODE IS COPIED FROM A 2021 UTCS ASSIGNMENT BY S. CHATTERJEE, X. SHEN, AND T. BYRD.
 * 
 * Copyright (c) 2021. N. Nemenzo. All rights reserved.
 * May not be used, modified, or copied without permission.
 **************************************************************************/ 

#include "turo.h"

// Boolean dictating the state of the program regarding reading input and terminating on failure
bool ignore_input = false;
bool terminate = false;

// Print buffer for printing logging and error messages
static char printbuf[100];

// Used for printing
static char *sevnames[LOG_FATAL+1] = {
    "INFO",
    "WARNING",
    "ERROR",
    "FATAL"
};

// Used for printing
static char *errnames[ERR_UNDEFINED+1] = {
    "Failed Lexical Analysis",
    "Failed Syntactic Analysis",
    "Failed Type Inference",
    "Failed Evaluation",
    "Undefined Variable"
};

/* format_log_message() - function used in formatting of log messages
 * Parameters: log_level_t - signify log level
 *             char * - pass in the message for logging
 * Side effect: none
 * Return: return filled string
 */
static char* format_log_message(log_lev_t sev, char *msg) {
    sprintf(printbuf, "[%s] %s", sevnames[sev], msg);
    return printbuf;
}

extern int logmsg(log_lev_t level, char *msg) {
    switch (level) {
        case LOG_ERROR:
            if (ignore_input) return 0;
            ignore_input = true;
            break;
        case LOG_FATAL:
            terminate = true;
            break;
        default:
            ;
    }
    if (outfile != stdout && level == LOG_ERROR) {
        fprintf(outfile, "\t[ERROR]\n");
    }
    return fprintf(errfile, "%s\n", format_log_message(level, msg));
}

extern int handle_error(err_type_t err) {
    if (ignore_input) return 0;
    ignore_input = true;
    if (outfile != stdout) {
        return fprintf(outfile, "\tERROR: %s\n", errnames[err]);
    }
    return fprintf(outfile, "\tERROR: %s\n", errnames[err]);
}