#ifndef PART1
#define PART1

struct Card {
        int id;
        int numbers[10];
        int winning_numbers[25];
};

struct Card lineToCard(char line[]);

#endif
