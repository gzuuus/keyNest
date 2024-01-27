// TODO! change this for a more flexible interface
export interface ProfileInterface {
	name: string;
	hexpub: string;
	xpub?: string;
	prvk?: string;
	level?: number;
	gap?: number;
	parent?: string;
	child_index?: number;
	comments?: string;
  }

export interface AppContext {
    fileList: string[] | undefined;
    currentDbname: string | undefined;
	sessionPass?: string | undefined;
}