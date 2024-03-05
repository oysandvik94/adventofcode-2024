#include "part1.h"
#include <stdbool.h>
#include <assert.h>

bool arraysAreEqual(int arr1[], int arr2[], int length);

int main(int argc, char *argv[]) {
        struct Card expectedCard = {
                1, 
                {41, 48, 83, 86, 17}, 
                {83, 86, 6, 31, 17, 9, 48, 53}
        };

        struct Card returnedCard = lineToCard("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");

        assert(expectedCard.id == returnedCard.id);

        int numbersLength = sizeof(expectedCard.numbers) / sizeof(expectedCard.numbers[0]);
        assert(arraysAreEqual(expectedCard.numbers, returnedCard.numbers, numbersLength));

        int winningNumbersLength = sizeof(expectedCard.winning_numbers) / sizeof(expectedCard.winning_numbers[0]);
        assert(arraysAreEqual(expectedCard.winning_numbers, returnedCard.winning_numbers, winningNumbersLength));
        return 0;
}

bool arraysAreEqual(int arr1[], int arr2[], int length) {
        for (int i = 0; i < length; i++) {
                if (arr1[i] != arr2[i]) {
                        return false;
                }
        }
        return true;
}
