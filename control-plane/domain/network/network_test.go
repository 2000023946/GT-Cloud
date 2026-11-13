package network

import "testing"

func TestNetworkRule(t *testing.T) {
	rule := NetworkRule{
		Protocol:  "TCP",
		Port:      8080,
		Direction: Inbound,
	}

	if rule.Protocol != "TCP" {
		t.Errorf("expected protocol TCP, got %s", rule.Protocol)
	}

	if rule.Port != 8080 {
		t.Errorf("expected port 8080, got %d", rule.Port)
	}

	if rule.Direction != Inbound {
		t.Errorf("expected direction INBOUND, got %s", rule.Direction)
	}
}
