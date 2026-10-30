use fs2::FileExt;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::HashMap, fs::{self, File, OpenOptions}, io::{Read, Write}, path::{Path, PathBuf}, sync::Mutex};
use tauri::Manager;

pub const MAX_DOCUMENT: u64 = 32 * 1024 * 1024;
pub type Result<T> = std::result::Result<T, String>;
#[derive(Default)]
pub struct Locks(pub Mutex<HashMap<PathBuf, File>>);
pub fn fingerprint(bytes: &[u8]) -> String { format!("{:x}", Sha256::digest(bytes)) }
pub fn read_bounded(path: &Path, limit: u64) -> Result<Vec<u8>> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    if file.metadata().map_err(|e| e.to_string())?.len() > limit { return Err("File exceeds the supported size limit".into()); }
    let mut bytes = Vec::new(); file.take(limit + 1).read_to_end(&mut bytes).map_err(|e| e.to_string())?;
    if bytes.len() as u64 > limit { return Err("File exceeds the supported size limit".into()); }
    Ok(bytes)
}
pub fn atomic_write(path: &Path, bytes: &[u8]) -> Result<()> {
    let parent = path.parent().ok_or("Invalid destination")?;
    let mut temp = tempfile::NamedTempFile::new_in(parent).map_err(|e| e.to_string())?;
    temp.write_all(bytes).map_err(|e| e.to_string())?;
    temp.as_file().sync_all().map_err(|e| e.to_string())?;
    temp.persist(path).map_err(|e| e.to_string())?;
    Ok(())
}
pub fn validate(doc: &Value) -> Result<()> {
    if doc["schemaVersion"].as_u64() != Some(1) { return Err("Unsupported session schema version".into()); }
    if doc["sessionId"].as_str().is_none() { return Err("Missing session identifier".into()); }
    let images = doc["images"].as_array().ok_or("Invalid images list")?;
    if images.len() > 10000 { return Err("Session exceeds 10,000 image records".into()); }
    let mut ids = std::collections::HashSet::new();
    for image in images {
        let id = image["id"].as_str().ok_or("Missing image identifier")?;
        if !ids.insert(id) { return Err("Duplicate image identifier".into()); }
        if image["source"]["path"].as_str().is_none() { return Err("Missing image path".into()); }
        if !image["rating"].is_null() && !image["rating"].as_u64().is_some_and(|v| (1..=10).contains(&v)) { return Err("Invalid image rating".into()); }
    }
    Ok(())
}
fn lock(path: &Path, locks: &Locks) -> Result<()> {
    let key = path.parent().ok_or("Invalid session path")?.canonicalize().map_err(|e|e.to_string())?.join(path.file_name().ok_or("Invalid filename")?);
    let mut held = locks.0.lock().map_err(|e|e.to_string())?;
    if held.contains_key(&key) { return Ok(()); }
    let lock_path = key.with_extension("copla.lock");
    let file = OpenOptions::new().create(true).truncate(false).read(true).write(true).open(&lock_path).map_err(|e|e.to_string())?;
    file.try_lock_exclusive().map_err(|_|"This session is already open in another Copla window".to_string())?;
    held.insert(key, file); Ok(())
}
pub fn load(path: &Path, locks: &Locks) -> Result<Value> {
    let path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let bytes = read_bounded(&path, MAX_DOCUMENT)?;
    let mut doc: Value = serde_json::from_slice(&bytes).map_err(|e|e.to_string())?; validate(&doc)?;
    lock(&path, locks)?;
    let base = path.parent().unwrap_or_else(|| Path::new("."));
    for image in doc["images"].as_array_mut().unwrap() {
        let source = PathBuf::from(image["source"]["path"].as_str().unwrap());
        let relative = if source.is_absolute() { source.clone() } else { base.join(&source) };
        let absolute = image["source"]["absolutePath"].as_str().map(PathBuf::from);
        let relative_exists = relative.exists();
        let mut resolved = if relative_exists { relative.clone() }
            else if let Some(abs) = absolute.as_ref() { if abs.exists() { abs.clone() } else { relative.clone() } }
            else { relative.clone() };
        // If the resolved path does not exist, search the session directory tree for a
        // file with the same basename (handles images moved into subfolders).
        if !resolved.exists() {
            let target_name = source.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
            if !target_name.is_empty() {
                let mut matches: Vec<PathBuf> = Vec::new();
                let mut stack: Vec<(PathBuf, u32)> = vec![(base.to_path_buf(), 0)];
                let mut visited = 0u32;
                while let Some((dir, depth)) = stack.pop() {
                    if visited > 5000 || depth > 4 { continue; }
                    if let Ok(entries) = fs::read_dir(&dir) {
                        for entry in entries.flatten() {
                            visited += 1;
                            if visited > 5000 { break; }
                            let p = entry.path();
                            if p.is_dir() {
                                if depth < 4 { stack.push((p, depth + 1)); }
                            } else if p.file_name().map(|n| n == source.file_name().unwrap()).unwrap_or(false) {
                                matches.push(p);
                                if matches.len() > 1 { break; }
                            }
                        }
                    }
                }
                if matches.len() == 1 {
                    resolved = matches[0].clone();
                }
            }
        }
        let resolved = resolved.canonicalize().unwrap_or(resolved);
        image["source"]["path"] = json!(resolved.to_string_lossy());
        if image["source"].get("absolutePath").is_none() || !PathBuf::from(image["source"]["absolutePath"].as_str().unwrap_or("")).exists() {
            image["source"]["absolutePath"] = json!(resolved.to_string_lossy());
        }
    }
    Ok(json!({"document":doc,"revision":fingerprint(&bytes),"path":path.to_string_lossy()}))
}

pub fn save(path: &Path, mut doc: Value, revision: Option<String>, locks: &Locks) -> Result<String> {
    validate(&doc)?; lock(path, locks)?;
    if path.exists() {
        let bytes = read_bounded(path, MAX_DOCUMENT)?;
        if revision.as_ref() != Some(&fingerprint(&bytes)) { return Err("The destination already exists or changed outside Copla. Use Save As with a new filename.".into()); }
    } else if revision.is_some() { return Err("The saved session was removed outside Copla. Use Save As.".into()); }
    for image in doc["images"].as_array_mut().unwrap() {
        let source = PathBuf::from(image["source"]["path"].as_str().unwrap());
        image["source"]["absolutePath"] = json!(source.to_string_lossy());
        if let Some(relative) = pathdiff::diff_paths(&source, path.parent().unwrap()) { image["source"]["path"] = json!(relative.to_string_lossy()); }
    }
    let bytes=serde_json::to_vec_pretty(&doc).map_err(|e|e.to_string())?;
    if bytes.len() as u64 > MAX_DOCUMENT {return Err("Session exceeds the document size limit".into())}
    atomic_write(path,&bytes)?; Ok(fingerprint(&bytes))
}
pub fn data_dir(app: &tauri::AppHandle) -> Result<PathBuf> {
    let path=app.path().app_data_dir().map_err(|e|e.to_string())?;
    fs::create_dir_all(&path).map_err(|e|e.to_string())?; Ok(path)
}
#[tauri::command]
pub fn open_session(path:String, locks:tauri::State<Locks>) -> Result<Value> { load(Path::new(&path),&locks) }
#[tauri::command]
pub fn save_session(path:String, document:Value, revision:Option<String>, locks:tauri::State<Locks>) -> Result<String> { save(Path::new(&path),document,revision,&locks) }
#[tauri::command]
pub fn release_sessions(locks:tauri::State<Locks>) { if let Ok(mut held)=locks.0.lock() { held.clear(); } }
#[tauri::command]
pub fn read_preferences(app:tauri::AppHandle) -> Result<Value> {
    let path=data_dir(&app)?.join("preferences.json");
    if !path.exists() {return Ok(json!({"recent":[],"tags":[],"theme":"dark"}));}
    let mut prefs:Value=serde_json::from_slice(&read_bounded(&path,MAX_DOCUMENT)?).map_err(|e|e.to_string())?;
    if let Some(recent)=prefs["recent"].as_array_mut() {recent.retain(|entry|entry.as_str().is_some_and(|s|Path::new(s).exists()));recent.truncate(10);}
    Ok(prefs)
}
#[tauri::command]
pub fn write_preferences(app:tauri::AppHandle, preferences:Value) -> Result<()> { atomic_write(&data_dir(&app)?.join("preferences.json"),&serde_json::to_vec(&preferences).map_err(|e|e.to_string())?) }
#[tauri::command]
pub fn write_recovery(app:tauri::AppHandle, recovery:Value) -> Result<()> {
    validate(&recovery["document"])?;
    let dir=data_dir(&app)?; let path=dir.join("recovery.json");
    if path.exists() { let old=read_bounded(&path,MAX_DOCUMENT)?;atomic_write(&dir.join("recovery.previous.json"),&old)?; }
    atomic_write(&path,&serde_json::to_vec(&recovery).map_err(|e|e.to_string())?)
}
#[tauri::command]
pub fn read_recovery(app:tauri::AppHandle) -> Result<Option<Value>> {
    let dir=data_dir(&app)?;
    for name in ["recovery.json","recovery.previous.json"] {let path=dir.join(name);if path.exists() {if let Ok(bytes)=read_bounded(&path,MAX_DOCUMENT) {if let Ok(value)=serde_json::from_slice::<Value>(&bytes) { if validate(&value["document"]).is_ok() {return Ok(Some(value));} }}}}
    Ok(None)
}
#[tauri::command]
pub fn clear_recovery(app:tauri::AppHandle) -> Result<()> {
    for name in ["recovery.json","recovery.previous.json"] {let path=data_dir(&app)?.join(name);if path.exists(){fs::remove_file(path).map_err(|e|e.to_string())?;}} Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn doc()->Value {json!({"schemaVersion":1,"sessionId":"test","images":[]})}
    #[test] fn replacement_and_conflicts() {
        let dir=tempfile::tempdir().unwrap();let path=dir.path().join("test.copla");let locks=Locks::default();
        let rev=save(&path,doc(),None,&locks).unwrap();
        assert!(save(&path,doc(),None,&locks).is_err());
        assert!(save(&path,doc(),Some(rev),&locks).is_ok());
        assert!(load(&path,&Locks::default()).is_err());
        assert_eq!(load(&path,&locks).unwrap()["document"],doc());
    }
    #[test] fn rejects_unsupported_schema_and_duplicate_ids(){assert!(validate(&json!({"schemaVersion":2})).is_err());assert!(validate(&json!({"schemaVersion":1,"sessionId":"x","images":[{"id":"a","source":{"path":"x"},"rating":11}]})).is_err());}
    #[test] fn load_finds_images_in_subfolders(){
        let dir=tempfile::tempdir().unwrap();let sessdir=dir.path().join("Character Bible");fs::create_dir_all(&sessdir).unwrap();let imgdir=sessdir.join("Approved");fs::create_dir_all(&imgdir).unwrap();let img=imgdir.join("pic.png");fs::write(&img,b"x").unwrap();
        let locks=Locks::default();
        let doc=json!({"schemaVersion":1,"sessionId":"t","images":[{"id":"a","name":"pic.png","rating":null,"tags":[],"notes":"","source":{"path":"pic.png","size":1,"modified":""},"width":1,"height":1,"metadata":{}}],"tags":{},"comparison":[],"active":null,"view":{"mode":"grid","pan":"all","zoom":"all","viewports":{},"pair":[],"divider":50,"horizontal":false,"gridCols":0,"gridRows":0,"dividerLine":true,"dividerHandler":true,"handlerReposition":false,"handlerOffset":0,"dividerColor":"#e7dfff"}});
        let session_path=sessdir.join("s.copla");
        save(&session_path,doc,None,&locks).unwrap();
        let loaded=load(&session_path,&locks).unwrap();
        let resolved=loaded["document"]["images"][0]["source"]["path"].as_str().unwrap();
        println!("resolved: {}", resolved);
        assert!(Path::new(resolved).exists(), "resolved path should exist: {}", resolved);
    }
    #[test] fn load_resolves_relative_and_absolute_paths(){
        let dir=tempfile::tempdir().unwrap();let sessdir=dir.path().join("sessions");fs::create_dir(&sessdir).unwrap();let imgdir=dir.path().join("images");fs::create_dir(&imgdir).unwrap();let img=imgdir.join("a.png");fs::write(&img,b"x").unwrap();
        let locks=Locks::default();
        let doc=json!({"schemaVersion":1,"sessionId":"t","images":[{"id":"a","name":"a","rating":null,"tags":[],"notes":"","source":{"path":img.to_string_lossy(),"size":1,"modified":""},"width":1,"height":1,"metadata":{}}],"tags":{},"comparison":[],"active":null,"view":{"mode":"grid","pan":"all","zoom":"all","viewports":{},"pair":[],"divider":50,"horizontal":false}});
        let session_path=sessdir.join("s.copla");
        let _rev=save(&session_path,doc,None,&locks).unwrap();
        // saved path should be relative
        let saved:Value=serde_json::from_slice(&fs::read(&session_path).unwrap()).unwrap();
        println!("saved source path: {}", saved["images"][0]["source"]["path"]);
        assert!(saved["images"][0]["source"]["absolutePath"].as_str().unwrap().contains("a.png"));
        // load resolves to absolute existing path
        let loaded=load(&session_path,&locks).unwrap();
        let resolved=loaded["document"]["images"][0]["source"]["path"].as_str().unwrap();
        println!("resolved: {}", resolved);
        assert!(Path::new(resolved).exists());
        assert!(Path::new(resolved).is_absolute());
    }
}
