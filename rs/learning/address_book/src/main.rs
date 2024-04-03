use chrono::prelude::*;

struct Profile {
    name: String,
    scripts_dir: String,
    backup_available: bool,
    modified_at: DateTime<Utc>, 
}

fn main() {

}

fn create_profile(name: String, scripts_dir: String, backup_available: bool) -> Profile {

   let utc_time: DateTime<Utc> = Utc::now();

    Profile {
        name=name,
        scripts_dir=scripts_dir,
        backup_available=backup_available, // OR VERIFY IF IT HAS BEEN DOWNLOADED
        modified_at=utc_time,
    }
}
