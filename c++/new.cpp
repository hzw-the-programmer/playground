#include <iostream>
#include <stdlib.h>

using namespace std;

void* operator new(size_t size) {
    cout << "global new called. size = " << size << endl;
    return malloc(size);
}

void operator delete(void *p) {
    cout << "global delete called." << endl;
    free(p);
}

class Student {
private:
    string name;
    int age;
public:
    Student() {
        cout << "default constructor called." << endl;
    }
    Student(string name, int age) {
        cout << "custom constructor called." << endl;
        this->name = name;
        this->age = age;
    }
    ~Student() {
        cout << "destructor called." << endl;
    }
    void* operator new(size_t size) {
        cout << "new called. size = " << size << endl;
        //return malloc(size);
        return ::new Student();
    }
    void operator delete(void *p) {
        cout << "delete called." << endl;
        free(p);
        //::delete p;
    }
    void display() {
        cout << "Name: " << this->name << endl;
        cout << "Age : " << this->age << endl;
    }
};

int main(int argc, char **argv) {
    Student *p = new Student("zhiwenhe", 32);
    p->display();
    delete p;

    int *a = new int[3];
    delete a;

    return 0;
}
