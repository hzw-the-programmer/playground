extern int handshake_test();
extern int handshake_test_2();
extern int handshake_test_3();
extern void certificate_test();
extern int cli_test(int argc, char* argv[]);

int main(int argc, char* argv[]) {
	//handshake_test();
	//handshake_test_2();
	handshake_test_3();
	//certificate_test();
	
	//return cli_test(argc, argv);
	return 0;
}