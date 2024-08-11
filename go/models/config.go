package models

import (
	"fmt"
	"os"
	"text/tabwriter"
)

type Config struct {
	Iface       string
	Snaplen     int32
	Promiscuous bool
	Url         string
	Bucket      string
	Token       string
}

// Returns new Config struct from params
func NewConfig(iface string, snaplen int32, promiscuous bool, url string, bucket string, token string) *Config {
	return &Config{
		Iface:       iface,
		Snaplen:     snaplen,
		Promiscuous: promiscuous,
		Url:         url,
		Bucket:      bucket,
		Token:       token,
	}
}

func (conf *Config) Contains() {
	w := tabwriter.NewWriter(os.Stdout, 0, 1, 1, ' ', 0)
	fmt.Fprintln(w, "Network")
	fmt.Fprintf(w, "\tInterface: %s\n", conf.Iface)
	fmt.Fprintf(w, "\tSnaplen: %d\n", conf.Snaplen)
	fmt.Fprintf(w, "\tPromiscuous: %v\n", conf.Promiscuous)
	fmt.Fprintln(w, "Database")
	fmt.Fprintf(w, "\tUrl: %s\n", conf.Url)
	fmt.Fprintf(w, "\tBucket: %s\n", conf.Bucket)
	fmt.Fprintf(w, "\tToken: %s\n", conf.Token)
	w.Flush()
}
