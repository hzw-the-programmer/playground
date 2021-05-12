package rpc

import (
	"errors"
	"math/rand"
)

func SendVerifyCode(phoneNumber int64) error {
	if rand.Intn(2) == 0 {
		return errors.New("Send verify code rpc fail")
	}

	return nil
}
