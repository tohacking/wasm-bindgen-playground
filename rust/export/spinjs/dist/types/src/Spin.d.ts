import { GamePlay, SpinGameInitArgs } from "./GamePlay.js";
import { ProveCredentials } from "./Proof.js";
interface SpinConstructor {
    cloudCredentials: ProveCredentials;
}
export declare class Spin {
    gamePlay: GamePlay;
    cloudCredentials: ProveCredentials;
    inputs: bigint[];
    witness: bigint[];
    constructor({ cloudCredentials }: SpinConstructor);
    newGame(): Promise<void>;
    add_public_input(input: bigint): void;
    add_private_input(input: bigint): void;
    step(command: bigint): void;
    getGameState(): import("game_logic_esm").SpinGameIntermediateStates;
    init_game(arg: SpinGameInitArgs): void;
    getGameID(): Promise<string>;
    generateProof(): Promise<{
        proof: BigInt[];
        verify_instance: BigInt[];
        aux: BigInt[];
        instances: BigInt[];
        status: import("zkwasm-service-helper").TaskStatus;
    } | null>;
    reset(): Promise<void>;
}
export {};
