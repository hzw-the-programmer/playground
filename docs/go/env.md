GOROOT is for compiler and tools that come from go installation and is used to find the standard libraries. It should always be set to the installation directory. 

If the environment variable is unset, GOPATH defaults to a subdirectory named “go” in the user’s home directory. 
so set it and add %GOPATH%\bin to PATH

If the GOBIN environment variable is set, commands are installed to the directory it names instead of DIR/bin. GOBIN must be an absolute path.