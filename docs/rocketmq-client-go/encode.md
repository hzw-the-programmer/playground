internal\remote\codec.go
encode
func encode(command *RemotingCommand) ([]byte, error)
	frameSize := 4 + len(header) + len(command.Body)
	//buf := bytes.NewBuffer(make([]byte, frameSize))
    buf := bytes.NewBuffer(make([]byte, frameSize + 4))
	buf.Reset()
