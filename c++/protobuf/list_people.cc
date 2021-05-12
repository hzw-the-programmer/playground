#include <fstream>
#include <google/protobuf/util/time_util.h>
#include <iostream>

#include "addressbook.pb.h"

using namespace std;

using google::protobuf::util::TimeUtil;

// c++ list_people.cc addressbook.pb.cc -I/home/zhiwenhe/build/protobuf/include -std=c++11 -lprotobuf -L/home/zhiwenhe/build/protobuf/lib -lpthread -o list_people
// LD_LIBRARY_PATH=/home/zhiwenhe/build/protobuf/lib ./list_people

void ListPeople(const AddressBook& address_book) {
    for (int i = 0; i < address_book.people_size(); i++) {
        const Person& person = address_book.people(i);

        cout << "Person ID: " << person.id() << endl;
        cout << "  Name: " << person.name() << endl;
        if (person.email() != "") {
            cout << "  E-mail address: " << person.email() << endl;
        }

        for (int j = 0; j < person.phones_size(); j++) {
            const Person::PhoneNumber& phone_number = person.phones(j);

            switch (phone_number.type()){
                case Person::MOBILE:
                    cout << "  Mobile phone #: ";
                    break;
                case Person::HOME:
                    cout << "  Home phone #: ";
                    break;
                case Person::WORK:
                    cout << "  Work phone #: ";
                    break;
                default:
                    cout << "  Other phone #: ";
                    break;
            }
            cout << phone_number.number() << endl;
        }

        if (person.has_last_updated()) {
            cout << "  Updated: " << TimeUtil::ToString(person.last_updated()) << endl;
        }
    }
}

int main(int argc, char* argv[]) {
    GOOGLE_PROTOBUF_VERIFY_VERSION;

    if (argc != 2) {
        cerr << "Usage:  " << argv[0] << " ADDRESS_BOOK_FILE" << endl;
        return -1;
    }

    AddressBook address_book;

    {
        fstream input(argv[1], ios::in | ios::binary);
        if (!address_book.ParseFromIstream(&input)) {
            cerr << "Failed to parse address book." << endl;
            return -1;
        }
    }

    ListPeople(address_book);

    google::protobuf::ShutdownProtobufLibrary();

    return 0;
}