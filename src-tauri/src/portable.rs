use crate::storage::{self,Result,MAX_DOCUMENT};
use serde_json::{json,Value};
use std::{collections::HashSet,fs::{self,File},io::{Read,Write},path::{Component,Path,PathBuf}};
use zip::{ZipArchive,ZipWriter,write::SimpleFileOptions,CompressionMethod};

const MAX_ASSET:u64=256*1024*1024;
const MAX_TOTAL:u64=8*1024*1024*1024;
fn copy_entry_bounded<R: Read, W: Write>(reader: &mut R, output: &mut W, entry_limit: u64, total: &mut u64, total_limit: u64) -> Result<u64> {
    let mut copied = 0u64; let mut buffer = [0u8; 64 * 1024];
    loop {
        let remaining_entry = entry_limit.checked_sub(copied).ok_or("Expanded entry exceeds limits")?;
        let remaining_total = total_limit.checked_sub(*total).ok_or("Package exceeds extraction limits")?;
        let allowed = remaining_entry.min(remaining_total).min(buffer.len() as u64);
        if allowed == 0 { let mut extra = [0u8; 1]; if reader.read(&mut extra).map_err(|e| e.to_string())? != 0 { return Err(if remaining_total == 0 { "Package exceeds extraction limits" } else { "Expanded entry exceeds limits" }.into()); } return Ok(copied); }
        let count = reader.read(&mut buffer[..allowed as usize]).map_err(|e| e.to_string())?;
        if count == 0 { return Ok(copied); }
        output.write_all(&buffer[..count]).map_err(|e| e.to_string())?; copied += count as u64; *total += count as u64;
    }
}
pub fn export(path:&Path,mut document:Value)->Result<()> {
    storage::validate(&document)?;
    if path.exists(){return Err("Export destination already exists; choose a new filename".into())}
    let mut temp=tempfile::NamedTempFile::new_in(path.parent().ok_or("Invalid export path")?).map_err(|e|e.to_string())?;
    let mut manifest=Vec::new();let mut total=0;
    {
        let mut archive=ZipWriter::new(temp.as_file_mut());let options=SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
        for (index,image) in document["images"].as_array_mut().unwrap().iter_mut().enumerate() {
            let source=PathBuf::from(image["source"]["path"].as_str().unwrap());
            let extension=source.extension().and_then(|v|v.to_str()).unwrap_or("").to_lowercase();
            if !["png","jpg","jpeg","webp"].contains(&extension.as_str()){return Err("Unsupported asset extension".into())}
            let bytes=storage::read_bounded(&source,MAX_ASSET)?;total+=bytes.len() as u64;if total>MAX_TOTAL{return Err("Portable session exceeds 8 GB".into())}
            let name=format!("assets/{index:06}.{extension}");
            archive.start_file(&name,options).map_err(|e|e.to_string())?;archive.write_all(&bytes).map_err(|e|e.to_string())?;
            manifest.push(json!({"path":name,"sha256":storage::fingerprint(&bytes),"size":bytes.len()}));
            image["source"]["path"]=json!(name);image["source"].as_object_mut().unwrap().remove("absolutePath");
        }
        let entries=[("session.copla",document),("manifest.json",json!({"version":1,"assets":manifest}))];
        for (name,value) in entries {archive.start_file(name,options).map_err(|e|e.to_string())?;archive.write_all(&serde_json::to_vec_pretty(&value).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;}
        archive.finish().map_err(|e|e.to_string())?;
    }
    temp.as_file().sync_all().map_err(|e|e.to_string())?;temp.persist_noclobber(path).map_err(|e|e.to_string())?;Ok(())
}
fn safe_entry(name:&str)->bool {
    !name.contains('\\')&&!name.contains(':')&&Path::new(name).components().all(|part|matches!(part,Component::Normal(_)))
}
pub fn import(path:&Path,destination:&Path)->Result<String> {
    if !destination.is_dir()||fs::read_dir(destination).map_err(|e|e.to_string())?.next().is_some(){return Err("Choose an empty destination folder".into())}
    let mut archive=ZipArchive::new(File::open(path).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
    if archive.len()>10002{return Err("Too many package entries".into())}
    let staging=tempfile::tempdir_in(destination).map_err(|e|e.to_string())?;let mut seen=HashSet::new();let mut declared_total=0u64;let mut extracted_total=0u64;
    for index in 0..archive.len() {
        let mut file=archive.by_index(index).map_err(|e|e.to_string())?;let name=file.name().to_string();
        if !safe_entry(&name)||file.is_dir()||!seen.insert(name.to_lowercase())||file.unix_mode().is_some_and(|m|m&0o170000==0o120000){return Err("Unsafe or duplicate archive entry".into())}
        if name!="session.copla"&&name!="manifest.json"&&!name.starts_with("assets/"){return Err("Unexpected package entry".into())}
        let limit=if name.starts_with("assets/"){MAX_ASSET}else{MAX_DOCUMENT};declared_total=declared_total.checked_add(file.size()).ok_or("Package exceeds extraction limits")?;
        if file.size()>limit||declared_total>MAX_TOTAL||file.size()/file.compressed_size().max(1)>1000{return Err("Package exceeds extraction limits".into())}
        let target=staging.path().join(&name);fs::create_dir_all(target.parent().unwrap()).map_err(|e|e.to_string())?;
        let mut output=File::create(target).map_err(|e|e.to_string())?;
        copy_entry_bounded(&mut file,&mut output,limit,&mut extracted_total,MAX_TOTAL)?;
    }
    let manifest:Value=serde_json::from_slice(&storage::read_bounded(&staging.path().join("manifest.json"),MAX_DOCUMENT)?).map_err(|e|e.to_string())?;
    if manifest["version"]!=1{return Err("Unsupported package version".into())}
    let mut approved=HashSet::new();
    for asset in manifest["assets"].as_array().ok_or("Invalid package manifest")? {
        let name=asset["path"].as_str().ok_or("Missing asset path")?;
        if !safe_entry(name)||!name.starts_with("assets/")||!approved.insert(name.to_string()){return Err("Invalid manifest path".into())}
        let bytes=storage::read_bounded(&staging.path().join(name),MAX_ASSET)?;
        if asset["sha256"].as_str()!=Some(storage::fingerprint(&bytes).as_str())||asset["size"].as_u64()!=Some(bytes.len() as u64){return Err("Asset checksum mismatch".into())}
    }
    let document:Value=serde_json::from_slice(&storage::read_bounded(&staging.path().join("session.copla"),MAX_DOCUMENT)?).map_err(|e|e.to_string())?;storage::validate(&document)?;
    if seen.len()!=approved.len()+2{return Err("Package contains unlisted assets".into())}
    for image in document["images"].as_array().unwrap(){if !image["source"]["path"].as_str().is_some_and(|p|approved.contains(p))||image["source"].get("absolutePath").is_some(){return Err("Session references an asset outside the package".into())}}
    // All data is verified before moving any extracted content into the destination.
    let assets=staging.path().join("assets");if assets.exists(){fs::rename(&assets,destination.join("assets")).map_err(|e|e.to_string())?;}
    let session=destination.join("session.copla");fs::rename(staging.path().join("session.copla"),&session).map_err(|e|e.to_string())?;
    Ok(session.to_string_lossy().into_owned())
}
#[tauri::command]
pub async fn export_package(path:String,document:Value)->Result<()>{tauri::async_runtime::spawn_blocking(move||export(Path::new(&path),document)).await.map_err(|e|e.to_string())?}
#[tauri::command]
pub async fn import_package(path:String,destination:String)->Result<String>{tauri::async_runtime::spawn_blocking(move||import(Path::new(&path),Path::new(&destination))).await.map_err(|e|e.to_string())?}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn path_rules(){for name in ["../x","/x","C:/x","assets/../../x","assets\\x"]{assert!(!safe_entry(name));}assert!(safe_entry("assets/000001.png"));}
    #[test]fn package_round_trip(){let dir=tempfile::tempdir().unwrap();let image=dir.path().join("original.png");fs::write(&image,b"synthetic fixture").unwrap();let doc=json!({"schemaVersion":1,"sessionId":"x","images":[{"id":"a","rating":5,"source":{"path":image,"absolutePath":"private"}}]});let package=dir.path().join("test.coplapack");export(&package,doc).unwrap();assert!(export(&package,json!({})).is_err());let target=dir.path().join("output");fs::create_dir(&target).unwrap();let session=import(&package,&target).unwrap();let value:Value=serde_json::from_slice(&fs::read(session).unwrap()).unwrap();assert!(value["images"][0]["source"].get("absolutePath").is_none());assert_eq!(fs::read(target.join("assets/000000.png")).unwrap(),b"synthetic fixture");}
    #[test]fn actual_extraction_limit_is_enforced(){let mut source=std::io::Cursor::new(vec![1u8;11]);let mut output=Vec::new();let mut total=0;assert!(copy_entry_bounded(&mut source,&mut output,20,&mut total,10).is_err());assert_eq!(output.len(),10);assert_eq!(total,10);}
}
