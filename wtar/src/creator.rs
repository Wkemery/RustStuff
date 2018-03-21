
const MAXNAMELEN:usize = 255;

use std::fs;
use std::io::Error;
use std::os::linux::fs::MetadataExt;


enum TypeFlag{
    Link,
    File,
    Directory,
}

struct ArchiveEntry {
    name: String,
    mode: String,
    uid: String,
    gid: String,
    size: String,
    mtime: String,
    checksum: i32,
    typeflag: TypeFlag,
    linkname: String,
    prefix: String,
}

impl ArchiveEntry {

    fn new(file_name: &str) -> Result<ArchiveEntry, Error> {
        
        let metadata = fs::metadata(file_name)?;
        
        let mode = metadata.st_mode().to_string();
        let uid = metadata.st_uid().to_string();
        let gid = metadata.st_gid().to_string();
        let size = metadata.st_size().to_string();
        let mtime = metadata.st_mtime().to_string();
        
        let name;
        let prefix;
        if file_name.len() > 100 {
                /*Need to split name*/
//                 &file_name[100..]
            name = String::from("name here");
            prefix = String::from("prefix here");
        }
        else {
            name = file_name.to_string();
            prefix = String::from("");
        }
        
        let typeflag = 
            if metadata.is_dir(){
                TypeFlag::Directory 
            }
            else if metadata.is_file(){
                TypeFlag::File
            }
            else{
                TypeFlag::Link
            };
        let linkname = 
            match typeflag {
                TypeFlag::Link => String::from("stuff"),
                _ => String::from("")
            };
        
        let checksum = 0;
        
        Ok(ArchiveEntry {
            name: name,
            mode: mode,
            uid: uid,
            gid: gid,
            size: size,
            mtime: mtime,
            checksum: checksum,
            typeflag: typeflag,
            linkname: linkname,
            prefix: prefix,
        })
    }
    
    fn write_out(&self)
    {
    
    }
    
}

pub fn create(ouput_file: &str, input_files: Vec<&str>)
{
    /*Get the stats on the input file*/
    

    for file_name in input_files{
        if(file_name.len() > MAXNAMELEN)
        {
            println!("File name {} is over the limit of {} characters", file_name, MAXNAMELEN);
        }
        if let Ok(archive_entry) = ArchiveEntry::new(file_name) {
            /*Write out the file*/
            archive_entry.write_out();
        }
        else
        {
            println!("there was an error, I'll figure out how to print it later");
        }
    }

}

