use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use zip::ZipArchive;

pub fn extract_mod_zip(zip_path: &Path, overlay_dir: &Path) -> Result<(), String> {
    let file = fs::File::open(zip_path).map_err(|e| e.to_string())?;
    let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        // Determine if this path belongs in the overlay
        // We look for directories like "ExtraContent", "PlatformContent", or "content"
        let mut target_path = None;
        let components: Vec<_> = outpath.components().collect();
        for (idx, comp) in components.iter().enumerate() {
            let comp_str = comp.as_os_str().to_string_lossy().to_lowercase();
            if comp_str == "extracontent" || comp_str == "platformcontent" || comp_str == "content" {
                // Keep the path from this component onwards
                let mut relative_path = PathBuf::new();
                for c in &components[idx..] {
                    relative_path.push(c);
                }
                target_path = Some(overlay_dir.join(relative_path));
                break;
            }
        }

        if let Some(target) = target_path {
            if file.is_dir() {
                fs::create_dir_all(&target).ok();
            } else {
                if let Some(parent) = target.parent() {
                    fs::create_dir_all(parent).ok();
                }
                let mut outfile = fs::File::create(&target).map_err(|e| e.to_string())?;
                io::copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
            }
        }
    }
    Ok(())
}
