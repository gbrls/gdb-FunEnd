#include <iostream>
#include <cstdint>
#include <string>
#define TRUE 1
#define FALSE 0

using namespace std;

struct node
{
    int data;
    struct node *next;
};

struct cell
{
    string kmer;
    struct node *vector;
};

    struct cell *
InitializeTable (int tableSize)
{
    struct cell *table = new struct cell[tableSize];
    for (int i = 0; i < tableSize; i++)
    {
        (table + i)->vector = NULL;
    }
    return table;
}

    int
InitializeVector (struct cell *newCell)
{
    newCell->vector = new struct node;
    newCell->vector->next = NULL;
    return 0;
}

    int
CalculateKey (string slice, int tableSize)
{
    int stringSize = slice.size();
    uint32_t key = 0;
    for (int i = 0; i < stringSize; i++)
    {
        key = key*128 + (uint32_t)(slice[i]);
    }
    key %= tableSize;
    return key;
}

    int
Append (struct node *head, int new_data)
{
    struct node *current = head;
    while (TRUE)
    {
        if (current->next!=NULL)
        {
            current = current->next;
            continue;
        }
        else
        {
            current->next = new struct node;
            current = current->next;
            current->data = new_data;
            current->next = NULL;
            break;
        }
    }
    return 0;
}

    struct cell *
AddToTable (struct cell *table, string kmer, int data, int tableSize)
{
    int position = CalculateKey(kmer, tableSize);
    struct cell * ptrToCell;
    while (TRUE)
    {
        struct cell *current = table + position;
        ptrToCell = current;
        if (current->vector==NULL)
        {
            InitializeVector(current);
            Append(current->vector, data);
            current->kmer = kmer;
            break;
        }
        else if (current->kmer==kmer)
        {
            Append(current->vector, data);
            break;
        }
        position = (position + 1) % tableSize;
    }
    return ptrToCell;
}

    struct node *
FindInTable (struct cell *table, string kmer, int tableSize)
{
    int position = CalculateKey(kmer, tableSize);
    while (TRUE)
    {
        struct cell *current = table + position;
        if (current->kmer==kmer)
        {
            return current->vector;
        }
        else if (current->vector==NULL)
        {
            return NULL;
        }
        position = (position + 1) % tableSize;
    }
}

    struct cell *
Rehash (struct cell *oldTable, int oldTableSize)
{
    int newTableSize = 2*oldTableSize + 1;
    struct cell *newTable = InitializeTable(newTableSize);
    for (int i = 0; i < oldTableSize; i++)
    {
        struct cell *current = oldTable + i;
        if (current->vector!=NULL)
        {
            struct cell *newCurrent = AddToTable(newTable, current->kmer, -1, newTableSize);
            delete newCurrent->vector;
            newCurrent->vector = current->vector;
        }
    }
    delete[] oldTable;
    return newTable;
}

    int
main ()
{
    int kmerSize = 10, tableSize = 12;
    struct cell *table = InitializeTable(tableSize);
    //cin >> kmerSize >> tableSize;
    string placeholderString = "t", text = "YOELEOELS";
    int lines;
    //cin >> placeholderString >> lines;
    for (int i = 0; i < lines; i++)
    {
        getline(cin >> ws, placeholderString);
        text += ' ' + placeholderString;
    }
    int tableLoad = 0;
    for (int i = 0; i < text.size()-kmerSize; i++)
    {
        string kmer = text.substr(i, kmerSize);
        float loadFactor = (float)tableLoad/(float)tableSize;
        if (loadFactor>=0.5)
        {
            table = Rehash(table, tableSize);
            tableSize *= 2;
            tableSize++;
        }
        if (FindInTable(table, kmer, tableSize)==NULL)
        {
            tableLoad++;
        }
        AddToTable(table, kmer, i, tableSize);
    }
    return 0;
}
