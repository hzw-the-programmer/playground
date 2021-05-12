#define FIELD(a) char *a##_string; int a##_size

class X {
    FIELD(one);
    FIELD(two);
};

int main(int argc, char* argv[]) {

}
