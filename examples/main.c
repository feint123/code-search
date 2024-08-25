#include <stdio.h>
#include <string.h>

// Define a struct for a person
struct Person {
    char name[50];
    int age;
    float height;
};

// Function to print person details
void printPerson(struct Person p) {
    printf("Name: %s\n", p.name);
    printf("Age: %d\n", p.age);
    printf("Height: %.2f\n", p.height);
}

// Function to update person's age
void updateAge(struct Person *p, int newAge) {
    p->age = newAge;
}

int main() {
    struct Person person1;

    strcpy(person1.name, "John Doe");
    person1.age = 30;
    person1.height = 1.75;

    printf("Initial details:\n");
    printPerson(person1);

    updateAge(&person1, 31);

    printf("\nAfter updating age:\n");
    printPerson(person1);

    return 0;
}
