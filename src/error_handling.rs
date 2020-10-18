pub mod unrecoveralble {

    pub fn index_out_of_bound() {
        let v = vec![4, 67, 12];
        // trying to access the index which doesnt exist in the vactor
        let item_100th = &v[99];
    }

}
pub mod recoveralble {
    use std::fs::File;
    use std::io::Read;

    pub fn file_not_found() {
        // this will return a Result enum, we need to match on that to get the actual file handler
        let f = File::open("hello.txt");
        match f {
            Ok(fh) => println!("{:?}", fh),
            Err(error) => println!("file could not be opened... {:?}", error)
        }
    }

    pub fn file_not_found_error_kind() {
        let f = File::open("somefile.csv");
        let f = match f {
            Ok(fh) => fh,
            Err(error) => match error.kind() {
                std::io::ErrorKind::NotFound => {
                    let f = File::create("hello.txt");
                    match f {
                        Ok(fh) => fh,
                        Err(error) => panic!("problem creating the file: {:?}", error)
                    }
                },
                abc => panic!("problem opening the file")
            } 
        };
    }

    pub fn unwrap_expect() {
        let f = File::open("somefile.csv");
        let f = match f {
            Ok(fh) => fh,
            Err(error) => panic!("file could not be opened: {:?}", error)
        };
        
        // the following the are shortcuts of the above code. unwrap and expect return the file
        // handler on success otherwise call panic
        let ff = File::open("somefile.csv").expect("file could not be opened");
        let fff = File::open("somefile.csv").unwrap();
    }

    pub fn propagate_errors() -> Result<String, std::io::Error> {
        let f = File::open("somefile.csv");

        let mut f = match f {
            Ok(fh) => fh,
            Err(error) => return Err(error),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(error) => Err(error),
        }
    }

    pub fn propagate_errors_shortcut() -> Result<String, std::io::Error>{
        // the ? will return the error to the calling funcation otherwise it just returns the
        // success value to variable hoook
        let mut f = File::open("somefile.csv")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }
}

