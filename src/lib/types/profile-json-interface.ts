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

interface ScryptConfig {
	salt: string;
	n: number;
	r: number;
	p: number;
}

export interface RootPInterface {
	name: string;
	npub: string;
	xpub: string;
	prvk: string;
	level: number;
	scrypt: ScryptConfig;
	scope: number;
}

export interface ChildPInterface {
	name: string;
	npub: string;
	xpub: string;
	prvk: string;
	level: number;
	scrypt: ScryptConfig;
	scope: number;
}
