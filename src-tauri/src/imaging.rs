use crate::storage::{self, Result};
use base64::Engine;
use image::{ImageDecoder, ImageFormat, ImageReader};
use serde_json::{json, Value};
use std::{fs::{self, File}, io::{BufReader, Cursor, Read}, path::Path};
use tauri::Manager;

const MAX_IMAGE_BYTES:u64=256*1024*1024;
fn metadata(path:&Path)->Value {
    let mut values=serde_json::Map::new();
    if let Ok(file)=File::open(path) {
        let mut reader=BufReader::new(file);
        if let Ok(exif)=exif::Reader::new().read_from_container(&mut reader) {
            for field in exif.fields().take(100) { values.insert(field.tag.to_string(),json!(field.display_value().with_unit(&exif).to_string().chars().take(65536).collect::<String>())); }
        }
    }
    if let Ok(bytes)=storage::read_bounded(path,MAX_IMAGE_BYTES) {
        if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
            let mut offset=8;
            while offset+12<=bytes.len() {
                let length=u32::from_be_bytes(bytes[offset..offset+4].try_into().unwrap()) as usize;
                if length>bytes.len()-offset-12 {break}
                let kind=&bytes[offset+4..offset+8];let data=&bytes[offset+8..offset+8+length];
                if length<=1024*1024 && (kind==b"tEXt" || kind==b"zTXt" || kind==b"iTXt") {
                    if let Some(end)=data.iter().position(|b|*b==0) {
                        let key=String::from_utf8_lossy(&data[..end]).to_string();
                        let mut text=Vec::new();
                        if kind==b"tEXt" {text.extend_from_slice(&data[end+1..]);}
                        else if kind==b"zTXt" && end+2<=data.len() { let _=flate2::read::ZlibDecoder::new(&data[end+2..]).take(1024*1024).read_to_end(&mut text); }
                        else if kind==b"iTXt" && end+3<=data.len() {
                            let compressed=data[end+1]==1;let mut rest=&data[end+3..];
                            for _ in 0..2 {if let Some(n)=rest.iter().position(|b|*b==0){rest=&rest[n+1..];}else{rest=&[];break;}}
                            if compressed {let _=flate2::read::ZlibDecoder::new(rest).take(1024*1024).read_to_end(&mut text);}else{text.extend_from_slice(rest);}
                        }
                        values.insert(key,json!(String::from_utf8_lossy(&text).to_string()));
                    }
                }
                offset+=length+12;
            }
        }
    }
    Value::Object(values)
}
fn probe(app:&tauri::AppHandle,path:&str)->Result<Value> {
    let path=Path::new(path).canonicalize().map_err(|e|e.to_string())?;
    let stat=fs::metadata(&path).map_err(|e|e.to_string())?;
    if stat.len()>MAX_IMAGE_BYTES{return Err("Image exceeds 256 MB".into())}
    let mut reader=ImageReader::open(&path).map_err(|e|e.to_string())?.with_guessed_format().map_err(|e|e.to_string())?;
    if !matches!(reader.format(),Some(ImageFormat::Png|ImageFormat::Jpeg|ImageFormat::WebP)){return Err("Supported formats are PNG, JPEG, and WebP".into())}
    let mut limits=image::Limits::default();limits.max_image_width=Some(32768);limits.max_image_height=Some(32768);limits.max_alloc=Some(512*1024*1024);reader.limits(limits);
    let mut decoder=reader.into_decoder().map_err(|e|e.to_string())?;
    let orientation=decoder.orientation().map_err(|e|e.to_string())?;
    let mut decoded=image::DynamicImage::from_decoder(decoder).map_err(|e|e.to_string())?;
    decoded.apply_orientation(orientation);
    let (width,height)=(decoded.width(),decoded.height());
    let mut buffer=Cursor::new(Vec::new());decoded.thumbnail(320,240).write_to(&mut buffer,ImageFormat::Png).map_err(|e|e.to_string())?;
    app.asset_protocol_scope().allow_file(&path).map_err(|e|e.to_string())?;
    Ok(json!({"path":path.to_string_lossy(),"name":path.file_name().unwrap_or_default().to_string_lossy(),"width":width,"height":height,"size":stat.len(),"modified":stat.modified().ok().and_then(|t|t.duration_since(std::time::UNIX_EPOCH).ok()).map(|d|d.as_millis().to_string()).unwrap_or_default(),"thumbnail":format!("data:image/png;base64,{}",base64::engine::general_purpose::STANDARD.encode(buffer.into_inner())),"metadata":metadata(&path)}))
}
#[tauri::command]
pub async fn inspect_image(app:tauri::AppHandle,path:String)->Result<Value> {
    tauri::async_runtime::spawn_blocking(move||probe(&app,&path)).await.map_err(|e|e.to_string())?
}
#[tauri::command]
pub async fn rename_image(app:tauri::AppHandle,path:String,name:String,expected:Option<String>)->Result<Value> {
    tauri::async_runtime::spawn_blocking(move||{
        let source=Path::new(&path).canonicalize().map_err(|e|e.to_string())?;
        if name.is_empty()||name.trim()!=name||name.ends_with('.')||name.chars().any(|c|c<' '||"<>:\"/\\|?*".contains(c)){return Err("Invalid filename".into())}
        let stem=Path::new(&name).file_stem().and_then(|s|s.to_str()).unwrap_or("").to_uppercase();
        if ["CON","PRN","AUX","NUL","COM1","COM2","COM3","COM4","COM5","COM6","COM7","COM8","COM9","LPT1","LPT2","LPT3","LPT4","LPT5","LPT6","LPT7","LPT8","LPT9"].contains(&stem.as_str()){return Err("Reserved filename".into())}
        if source.extension()!=Path::new(&name).extension(){return Err("Keep the original file extension".into())}
        let destination=source.parent().unwrap().join(name);
        if destination.exists(){return Err("Destination already exists".into())}
        let digest=storage::fingerprint(&storage::read_bounded(&source,MAX_IMAGE_BYTES)?);
        if expected.is_some_and(|value|value!=digest){return Err("The original file changed; rename was not performed".into())}
        // Creating a hard link is a same-directory, no-clobber operation. Remove the old name only after success.
        fs::hard_link(&source,&destination).map_err(|e|format!("Safe rename unavailable on this filesystem: {e}"))?;
        if let Err(error)=fs::remove_file(&source){let _=fs::remove_file(&destination);return Err(error.to_string())}
        app.asset_protocol_scope().allow_file(&destination).map_err(|e|e.to_string())?;
        Ok(json!({"path":destination.to_string_lossy(),"fingerprint":digest}))
    }).await.map_err(|e|e.to_string())?
}
