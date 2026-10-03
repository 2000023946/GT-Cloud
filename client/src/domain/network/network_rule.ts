import { NetworkDirection } from "./network_direction";

export interface NetworkRule {
    protocol: string;
    port: number;
    direction: NetworkDirection;
}