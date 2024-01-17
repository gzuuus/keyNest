export interface ProfileJsonInterface {
    user: string;
    pubkey: string;
    npub: string;
    pkey: string;
    scrypt: {
        salt: string;
        n: number;
        r: number;
        p: number;
    };
}