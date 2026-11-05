import { chromium,expect } from '@playwright/test';
import { createServer } from 'vite';
import fs from 'node:fs/promises';
const server=await createServer({server:{port:1422,host:'127.0.0.1'}});await server.listen();
const browser=await chromium.launch({channel:'msedge',headless:true});
try{
 const page=await browser.newPage({viewport:{width:1440,height:940}});const errors=[];page.on('pageerror',e=>errors.push(e.message));
 await page.goto('http://127.0.0.1:1422');await expect(page.getByRole('heading',{name:'A clearer view of every possibility.'})).toBeVisible();
 await fs.mkdir('docs/testing/screenshots',{recursive:true});await page.screenshot({path:'docs/testing/screenshots/welcome-dark.png'});
 await page.getByRole('button',{name:/^New session/}).click();
 const samples=await page.evaluate(()=>Array.from({length:4},(_,i)=>{const c=document.createElement('canvas');c.width=1200;c.height=900;const ctx=c.getContext('2d');ctx.fillStyle=['#485a69','#72536a','#4c6b60','#696044'][i];ctx.fillRect(0,0,1200,900);ctx.fillStyle='#e6d4ba';ctx.beginPath();ctx.arc(600+i*30,380,190,0,Math.PI*2);ctx.fill();ctx.fillStyle='#1b2330';ctx.fillRect(0,680,1200,220);ctx.fillStyle='#9fa2ae';for(let n=0;n<12;n++){ctx.fillRect(n*120,640-n%3*20,90,100);}return c.toDataURL('image/png').split(',')[1];}));
 await page.locator('input[type=file][multiple]').setInputFiles(samples.map((data,i)=>({name:`Variant ${i+1}.png`,mimeType:'image/png',buffer:Buffer.from(data,'base64')})));
 await expect(page.getByTestId('viewport')).toHaveCount(4);
 await page.getByRole('button',{name:'Rate 8',exact:true}).click();
 await page.getByRole('textbox',{name:'Notes',exact:true}).fill('Strong composition. Compare the lighting.');
 await page.getByRole('combobox',{name:'Tags',exact:true}).fill('lighting');await page.getByRole('combobox',{name:'Tags',exact:true}).press('Enter');
 await page.screenshot({path:'docs/testing/screenshots/workspace-dark.png'});
 await page.getByRole('button',{name:'A/B slider',exact:true}).click();await expect(page.locator('.divider-line')).toBeVisible();
 await page.screenshot({path:'docs/testing/screenshots/ab-dark.png'});
 await page.getByRole('button',{name:'Switch theme'}).click();await page.screenshot({path:'docs/testing/screenshots/ab-light.png'});
 await page.getByRole('button',{name:'Carousel',exact:true}).click();await page.getByRole('button',{name:'Next image',exact:true}).click();
 await page.getByRole('button',{name:'Grid',exact:true}).click();await page.locator('.thumbnail').nth(0).click();await page.locator('.thumbnail').nth(1).click({modifiers:['Control']});await page.getByRole('button',{name:'Rate 6',exact:true}).click();await expect(page.locator('.thumbnail-rating').filter({hasText:'6'})).toHaveCount(2);
 await page.getByRole('button',{name:/^Undo: Rating/}).click();await expect(page.locator('.thumbnail-rating').filter({hasText:'6'})).toHaveCount(0);
 await page.setViewportSize({width:960,height:640});await page.screenshot({path:'docs/testing/screenshots/workspace-compact.png'});
 if(errors.length)throw new Error(errors.join('\n'));console.log('UI smoke passed: import, grid, ratings, notes, tags, A/B, themes, carousel, batch undo, compact layout.');
}finally{await browser.close();await server.close();}
