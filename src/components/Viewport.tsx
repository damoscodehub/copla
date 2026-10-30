import { useEffect,useRef,useState } from 'react';
import { useTranslation } from 'react-i18next';
import { clampView,dimensions,zoomAt,type Viewport as View } from '../viewport/geometry';
import type { ImageRecord } from '../domain/session';
import type { Asset } from '../services/native';

export function Viewport({image,asset,view,onChange}:{image:ImageRecord;asset?:Asset;view:View;onChange:(next:View,kind:'pan'|'zoom')=>void}) {
  const {t}=useTranslation();const root=useRef<HTMLDivElement>(null);const [box,setBox]=useState({width:1,height:1});const [error,setError]=useState(false);const [loaded,setLoaded]=useState(false);
  const latest=useRef({view,onChange,box,image});latest.current={view,onChange,box,image};
  const drag=useRef<{x:number;y:number;view:View}|null>(null);
  useEffect(()=>{setError(false);setLoaded(false);},[asset?.url]);
  useEffect(()=>{const el=root.current!;const observer=new ResizeObserver(entries=>{const {width,height}=entries[0].contentRect;setBox({width:Math.max(1,width),height:Math.max(1,height)});});observer.observe(el);const wheel=(event:WheelEvent)=>{event.preventDefault();const {view,onChange,box,image}=latest.current;const rect=el.getBoundingClientRect();onChange(zoomAt(view,Math.exp(-event.deltaY*.002),event.clientX-rect.left-box.width/2,event.clientY-rect.top-box.height/2,image.width,image.height,box.width,box.height),'zoom');};el.addEventListener('wheel',wheel,{passive:false});return()=>{observer.disconnect();el.removeEventListener('wheel',wheel);};},[]);
  const bounded=clampView(view,image.width,image.height,box.width,box.height);const size=dimensions(image.width,image.height,box.width,box.height,bounded);
  return <div ref={root} className="viewport" data-testid="viewport" onPointerDown={event=>{if(event.button!==0)return;event.currentTarget.setPointerCapture(event.pointerId);drag.current={x:event.clientX,y:event.clientY,view:bounded};}} onPointerMove={event=>{if(!drag.current)return;const start=drag.current;onChange(clampView({...start.view,x:start.view.x-(event.clientX-start.x)/size.width,y:start.view.y-(event.clientY-start.y)/size.height},image.width,image.height,box.width,box.height),'pan');}} onPointerUp={()=>{drag.current=null;}} onPointerCancel={()=>{drag.current=null;}}>
    {asset?.missing?<div className="image-state">{t('missing')}</div>:error?<div className="image-state">{t('imageError')}</div>:<>
      {!loaded&&<div className="image-state"><span className="spinner"/></div>}
      <img src={asset?.url||asset?.thumbnail} alt={image.name} draggable={false} onLoad={()=>setLoaded(true)} onError={()=>setError(true)} style={{width:size.width,height:size.height,transform:`translate(${box.width/2-bounded.x*size.width}px,${box.height/2-bounded.y*size.height}px)`}}/>
    </>}
  </div>;
}
