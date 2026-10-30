import { chromium } from '@playwright/test';
import { createServer } from 'vite';
import { spawn } from 'node:child_process';
import fs from 'node:fs/promises';
const server = await createServer({ server: { port: 1420, host: '127.0.0.1' } });
await server.listen();
const app = spawn('src-tauri/target/debug/copla.exe', [], { windowsHide: true, env: { ...process.env, WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS: '--remote-debugging-port=9223' } });
let browser;
try {
  for (let i=0;i<40;i++) {
    try { browser = await chromium.connectOverCDP('http://127.0.0.1:9223'); break; } catch { await new Promise(resolve=>setTimeout(resolve,500)); }
  }
  if (!browser) throw new Error('Native WebView debugging endpoint did not become available');
  const context = browser.contexts()[0];
  let page;
  for(let i=0;i<20;i++) { page=context.pages().find(p=>p.url().includes('1420')); if(page) break; await new Promise(resolve=>setTimeout(resolve,300)); }
  if(!page) throw new Error('Native application page did not load');
  await page.waitForSelector('h1');
  const result=await page.evaluate(()=>({title:document.title,heading:document.querySelector('h1')?.textContent,native:!!window.__TAURI_INTERNALS__,userAgent:navigator.userAgent}));
  await fs.mkdir('docs/testing',{recursive:true});
  await fs.writeFile('docs/testing/native-shell-gate.json',JSON.stringify(result,null,2));
  console.log(result);
} finally { if(browser) await browser.close(); app.kill(); await server.close(); }
