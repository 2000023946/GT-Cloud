package network

type NetworkRule struct {
	Protocol  string
	Port      int
	Direction NetworkDirection
}
