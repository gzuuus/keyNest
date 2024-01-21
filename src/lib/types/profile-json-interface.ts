export interface RootPInterface {
	name: string;
	npub: string;
	xpub: string;
	prvk: string;
	level: number;
	scope: number;
}

// TODO! change this for a more flexible interface
export interface ProfileInterface {
	name: string;
	npub: string;
	xpub?: string;
	prvk: string;
	level?: number;
	scope?: number;
  }
  