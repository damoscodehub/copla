import { chromium } from '@playwright/test';
import fs from 'node:fs/promises';
import { createServer } from 'vite';
const server = await createServer({ server: { port: 1421, host: '127.0.0.1' } });
await server.listen();
const browser = await chromium.launch({ channel: 'msedge', headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 940 } });
await page.goto('http://127.0.0.1:1421');
const result = await page.evaluate(async () => {
  const canvas = document.createElement('canvas'); canvas.width = canvas.height = 4096;
  const ctx = canvas.getContext('2d');
  ctx.fillStyle = '#5a52ca'; ctx.fillRect(0,0,4096,4096);
  ctx.fillStyle = '#ffd090'; ctx.fillRect(600,600,1800,1800);
  const urls = await Promise.all(Array.from({length:8}, () => new Promise(resolve => canvas.toBlob(blob=>resolve(URL.createObjectURL(blob)), 'image/png'))));
  const stage = document.createElement('div'); stage.style.cssText = 'display:grid;grid-template-columns:repeat(4,1fr);gap:8px';
  const start = performance.now();
  const images = await Promise.all(urls.map(async url => { const img = new Image(); img.src=url; img.style.cssText='width:320px;height:300px;object-fit:contain'; await img.decode(); stage.append(img); return img; }));
  document.body.replaceChildren(stage);
  const loadMs = performance.now()-start;
  for (let i=0;i<60;i++) { await new Promise(requestAnimationFrame); images.forEach(img=>img.style.transform=`translate(${Math.sin(i/20)*10}px) scale(${1+i/1000})`); }
  const intervals=[]; let last=performance.now();
  for (let i=0;i<120;i++) { await new Promise(requestAnimationFrame); const now=performance.now(); intervals.push(now-last); last=now; images.forEach(img=>img.style.transform=`translate(${Math.sin(i/20)*10}px) scale(${1+i/1000})`); }
  return {images:8,width:4096,height:4096,decodeMs:Math.round(loadMs),meanFrameMs:intervals.reduce((a,b)=>a+b,0)/intervals.length,maxFrameMs:Math.max(...intervals),userAgent:navigator.userAgent};
});
await fs.mkdir('docs/testing', {recursive:true});
await fs.writeFile('docs/testing/browser-render-gate.json', JSON.stringify(result,null,2));
console.log(JSON.stringify(result));
await browser.close();
await server.close();
