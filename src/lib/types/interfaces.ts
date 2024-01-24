// TODO! change this for a more flexible interface
export interface ProfileInterface {
	name: string;
	npub: string;
	xpub?: string;
	prvk: string;
	level?: number;
	gap?: number;
	parent?: string;
  }

export interface AppContext {
    fileList: string[] | undefined;
    currentDbname: string | undefined;
}