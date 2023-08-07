extern int handshake_test();
extern void certificate_test();
extern int cli_test(int argc, char* argv[]);

int main(int argc, char* argv[]) {
	handshake_test();
	certificate_test();
	
	return cli_test(argc, argv);
}