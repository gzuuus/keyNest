// TODO! change this for a more flexible interface
export interface ProfileInterface {
	name: string;
	hexpub: string;
	xpub?: string;
	prvk?: string;
	level?: number;
	gap?: number;
	parent?: string;
	childIndex?: number;
  }

export interface AppContext {
    fileList: string[] | undefined;
    currentDbname: string | undefined;
}