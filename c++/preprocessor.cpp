#define F(x) (x + 1)
#define G (x) (x + 1) 

int main(int argc, char* argv[]) {
    F(1);
    F (1);
    G(1);
    G (1);
}

// g++ -E preprocessor.cpp
// g++ -E -P preprocessor.cpp
