package main

import (
	"bytes"
	"fmt"
	"image/jpeg"

	"github.com/nats-io/nats.go"
)

func cam() {
	nc, err := nats.Connect("nats://localhost:4222")
	if err != nil {
		fmt.Println("Error Connecting:", err)
		return
	}
	fmt.Println("Connected!")

	_, err = nc.Subscribe("camera.kitchen", imageHandler)
	if err != nil {
		fmt.Println("Error Subscribing:", err)
		return
	}

	select {} // block forever

	// sub.Unsubscribe()
}

func imageHandler(msg *nats.Msg) {
	img, err := jpeg.Decode(bytes.NewReader(msg.Data))
	if err != nil {
		fmt.Println("Failed to decode JPEG:", err)
		return
	}
	fmt.Printf("Received image: %dx%d\n", img.Bounds().Dx(), img.Bounds().Dy())
}
