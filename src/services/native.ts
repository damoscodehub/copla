import { convertFileSrc,invoke,isTauri } from '@tauri-apps/api/core';
import { open,save } from '@tauri-apps/plugin-dialog';
import { sessionSchema,type Session } from '../domain/session';
export const desktop=isTauri();
export type Asset={url:string;thumbnail:string;missing?:boolean;changed?:boolean};
export type Probe={path:string;name:string;width:number;height:number;size:number;modified:string;thumbnail:string;metadata:Record<string,unknown>};
export type Preferences={theme:'dark'|'light';recent:string[];tags:string[];background?:'neutral'|'checker';accent?:string};
export type Recovery={document:Session;path:string|null;revision:string|null};
export async function chooseImages(){if(!desktop)return [];const result=await open({multiple:true,filters:[{name:'Images',extensions:['png','jpg','jpeg','webp']}]});return Array.isArray(result)?result:result?[result]:[];}
export async function chooseSession(){return desktop?await open({multiple:false,filters:[{name:'Copla sessions',extensions:['copla','coplapack']}]}):null;}
export async function chooseSave(extension='copla',name='Untitled session'){return desktop?await save({defaultPath:`${name}.${extension}`,filters:[{name:'Copla',extensions:[extension]}]}):null;}
export async function inspect(path:string):Promise<Probe>{return invoke('inspect_image',{path});}
export const sourceUrl=(path:string)=>desktop?convertFileSrc(path):path;
export async function loadSession(path:string){const result=await invoke<{document:unknown;revision:string;path:string}>('open_session',{path});return {...result,document:sessionSchema.parse(result.document)};}
export async function saveSession(path:string,document:Session,revision:string|null){return invoke<string>('save_session',{path,document,revision});}
export const preferences=async()=>desktop?invoke<Preferences>('read_preferences'):JSON.parse(localStorage.getItem('copla-preferences')||'{"theme":"dark","recent":[],"tags":[]}') as Preferences;
export const storePreferences=async(value:Preferences)=>desktop?invoke('write_preferences',{preferences:value}):localStorage.setItem('copla-preferences',JSON.stringify(value));
export const writeRecovery=async(recovery:Recovery)=>desktop?invoke('write_recovery',{recovery}):localStorage.setItem('copla-recovery',JSON.stringify(recovery));
export async function readRecovery():Promise<Recovery|null>{const value=desktop?await invoke<Recovery|null>('read_recovery'):JSON.parse(localStorage.getItem('copla-recovery')||'null');if(value)value.document=sessionSchema.parse(value.document);return value;}
export const clearRecovery=async()=>desktop?invoke('clear_recovery'):localStorage.removeItem('copla-recovery');
export const releaseSessions=async()=>{if(desktop)await invoke('release_sessions');};
export const renameFile=(path:string,name:string,expected:string|null)=>invoke<{path:string;fingerprint:string}>('rename_image',{path,name,expected});
export async function scanFolder(folder:string,name:string):Promise<Array<{path:string;name:string;size:number;modified:string}>>{return invoke('scan_folder',{folder,name});}
