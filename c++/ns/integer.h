#ifndef INTEGER_H
#define INTEGER_H

namespace integer {
    enum sign { positive, negative };

    class Integer {
        int i;
        sign s;
    public:
        Integer(int ii = 0) : i(ii), s(ii > 0 ? positive : negative) {}
        sign getSign() const { return s; }
        void setSign(sign ss) { s = ss; }
        int getValue() const { return i; }
    };
}

#endif // INTEGER_H
