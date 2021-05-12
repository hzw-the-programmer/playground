#include <iostream>

using namespace std;

class GameBoard {
public:
    GameBoard() {
        cout << "GameBoard()\n";
    }
    GameBoard(const GameBoard&) {
        cout << "GameBoard(const GameBoard&)\n";
    }
    GameBoard& operator=(const GameBoard&) {
        cout << "GameBoard::operator=()\n";
    }
};

class Game {
    GameBoard gb;
public:
    Game() {
        cout << "Game()\n";
    }
    /*
    Game(const Game&) {
        cout << "Game(const Game&)\n";
    }
    */
    Game(const Game& g) : gb(g.gb) {
        cout << "Game(const Game&)\n";
    }
    Game(int) {
        cout << "Game(int)\n";
    }
    /*
    Game& operator=(const Game&) {
        cout << "Game::operator=()\n";
    }
    */
    Game& operator=(const Game& g) {
        gb = g.gb;
        cout << "Game::operator=()\n";
        return *this;
    }

    class Hzw {
        int i;
    public:
        Hzw(int i) :i (i) {
            cout << "Hzw()\n";
        }
        Hzw(const Hzw&) {
            cout << "Hzw(const Hzw&)\n";
        }
        void display() {
            cout << i << endl;
        }
    };

   operator Hzw() {
       cout << "Game::operator Hzw()\n";
       return Hzw(123);
   }
};

class Chess : public Game {};

class Checkers : public Game {
public:
    Checkers() {
        cout << "Checkers()\n";
    }
    /*
    Checkers(const Checkers&) {
        cout << "Checkers(const Checkers&)\n";
    }
    */
    Checkers(const Checkers& ck) : Game(ck) {
        cout << "Checkers(const Checkers&)\n";
    }
    /*
    Checkers& operator=(const Checkers&) {
        cout << "Checkers::operator=()\n";
    }
    */
   Checkers& operator=(const Checkers& ck) {
       Game::operator=(ck);
       cout << "Checkers::operator=()\n";
       return *this;
   }
};

void f(Game::Hzw hzw) {
    hzw.display();
}

int main(int argc, char* argv[]) {
    Game g1;
    Game g2 = g1;
    g2 = g1;

    cout << "******************" << endl;

    Chess c1;
    Chess c2(c1);
    //Chess c3(1);
    c2 = c1;

    cout << "******************" << endl;

    Checkers ck1, ck2(ck1);
    ck2 = ck1;

    cout << "******************" << endl;

    f(c1);

    return 0;
}