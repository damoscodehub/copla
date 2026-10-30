import { z } from 'zod';
import { initialViewport } from '../viewport/geometry';

export const viewportSchema = z.object({ x:z.number().finite(), y:z.number().finite(), zoom:z.number().min(.1).max(32), fit:z.enum(['fit','fill']) });
export const imageSchema = z.object({
  id:z.string(), name:z.string(), rating:z.number().int().min(1).max(10).nullable(), tags:z.array(z.string()), notes:z.string(),
  source:z.object({path:z.string(),absolutePath:z.string().optional(),size:z.number(),modified:z.string()}),
  width:z.number().positive(),height:z.number().positive(), metadata:z.record(z.string(),z.unknown()).default({}),
});
export const sessionSchema = z.object({
  schemaVersion:z.literal(1),sessionId:z.string(),title:z.string(),createdAt:z.string(),updatedAt:z.string(),
  images:z.array(imageSchema).max(10000),tags:z.record(z.string(),z.string()),comparison:z.array(z.string()).max(8),active:z.string().nullable(),
  view:z.object({mode:z.enum(['grid','carousel','ab']),pan:z.enum(['none','all','selection']),zoom:z.enum(['none','all','selection']),viewports:z.record(z.string(),viewportSchema),pair:z.array(z.string()).max(2),divider:z.number().min(0).max(100),horizontal:z.boolean()})
}).superRefine((doc,ctx)=>{
  const ids=new Set(doc.images.map(i=>i.id));
  if(ids.size!==doc.images.length)ctx.addIssue({code:'custom',message:'Duplicate image identifiers'});
  if(new Set(doc.comparison).size!==doc.comparison.length||doc.comparison.some(id=>!ids.has(id)))ctx.addIssue({code:'custom',message:'Invalid comparison set'});
  if(doc.images.some(i=>i.tags.some(tag=>!(tag in doc.tags))))ctx.addIssue({code:'custom',message:'Unknown tag reference'});
});
export type Session=z.infer<typeof sessionSchema>;
export type ImageRecord=z.infer<typeof imageSchema>;
export type FileEffect={id:string;before:string;after:string;fingerprint:string};
export type Entry={label:string;before:Session;after:Session;effect?:FileEffect;group?:string;time:number;started:number};
export type History={past:Entry[];future:Entry[]};
export function createSession():Session {const now=new Date().toISOString();return {schemaVersion:1,sessionId:crypto.randomUUID(),title:'Untitled session',createdAt:now,updatedAt:now,images:[],tags:{},comparison:[],active:null,view:{mode:'grid',pan:'none',zoom:'none',viewports:{},pair:[],divider:50,horizontal:false}};}
export function signature(doc:Session):string {const {view,active,updatedAt,...data}=doc;return JSON.stringify(data);}
export function reconcile(doc:Session):Session {
  const ids=new Set(doc.images.map(i=>i.id));const comparison=doc.comparison.filter(id=>ids.has(id)).slice(0,8);
  return {...doc,comparison,active:doc.active&&ids.has(doc.active)?doc.active:comparison[0]||doc.images[0]?.id||null,view:{...doc.view,pair:doc.view.pair.filter(id=>comparison.includes(id)),viewports:Object.fromEntries(doc.images.map(i=>[i.id,doc.view.viewports[i.id]||initialViewport()]))}};
}
export function appendHistory(history:History,entry:Entry):History {
  const past=[...history.past];const last=past.at(-1);
  if(entry.group&&last?.group===entry.group&&entry.time-last.time<750&&entry.time-last.started<5000){past[past.length-1]={...entry,before:last.before,started:last.started};}
  else past.push(entry);
  return {past:past.slice(-100),future:[]};
}
export function editImages(doc:Session,ids:string[],patch:(image:ImageRecord)=>ImageRecord):Session {const set=new Set(ids);return {...doc,images:doc.images.map(i=>set.has(i.id)?patch(i):i)};}
export const normalizedTag=(label:string)=>label.trim().normalize('NFKC').toLocaleLowerCase('en');
