/**************************************************************************
 * TURO COMPILER
 * 
 * hashtable.c - Implementation of the hash table data structure (underlying structure for symbol
 * table). Implementation will use buckets for collisions; no load factor (temp)
 * 
 * Copyright (c) 2021. N. Nemenzo. All rights reserved.
 * May not be used, modified, or copied without permission.
 **************************************************************************/

#ifndef SYMBOLTABLE_H
#define SYMBOLTABLE_H

#include "turo.h"

#define CAPACITY 100

/* Table entry struct */
typedef struct {
    char *id;
    struct entry_t *next;
} entry_t, *eptr_t;

/* entry_init() - initialize a table entry
 * Parameters: 
 * Return: return a pointer to a new table entry
 */ 
extern entry_t *entry_init(void);

/* entry_destroy - free memory taken up by a table entry
 * Parameters: entry to destroy
 */
extern void entry_destroy(entry_t *);

/* Symbol table struct */
typedef struct {
    entry_t **entries; // pointer to array of entry pointers
} symbol_table_t, *stableptr_t;

/* table_init() - initialize a table
 * Return: a pointer to the newly initialized table
 */
extern symbol_table_t *table_init(void);

/* table_destroy() - free all memory allocated by a symbol table (including entries)
 * Parameters: symbol table to destroy
 */
extern void table_destroy(symbol_table_t *);

/* put() - put an entry into the symbol table
 * Parameters: symbol table to add to, entry to add
 * Side effect: changes the symbol table
 */
extern void put(symbol_table_t *, entry_t *);

/* get() - get an entry from the symbol table
 * Parameters: symbol table to add to
 * Return: entry if exists, NULL if doesn't exist
 */
extern entry_t *get(symbol_table_t *);

#endif // SYMBOLTABLE_H
