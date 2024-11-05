use actix_files as fs;

pub fn static_files() -> fs::Files{
   let out =  fs::Files::new("/", "./static").prefer_utf8(true).show_files_listing();
   return  out;
}
