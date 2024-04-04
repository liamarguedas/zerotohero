use chrono::prelude::*;

struct Profile {
    name: String,
    scripts_dir: String,
    backup_available: bool,
    modified_at: DateTime<Utc>, 
}

fn main() {

    let usuario1: Profile = create_profile(String::from("liam"), String::from("~/"), false);

    println!("Profile name: {prof_name}, scripts {dir}, modified {modif}, backup {back}",
        prof_name=usuario1.name,
        dir=usuario1.scripts_dir,
        modif=usuario1.modified_at,
        back=usuario1.backup_available);

}

fn create_profile(name: String, scripts_dir: String, backup_available: bool) -> Profile {

   let utc_time: DateTime<Utc> = Utc::now();

    Profile {
        name:name,
        scripts_dir:scripts_dir,
        backup_available:backup_available, // OR VERIFY IF IT HAS BEEN DOWNLOADED
        modified_at:utc_time,
    }
}
