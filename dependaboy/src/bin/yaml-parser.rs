mod utils;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[derive(Copy, Clone)]
struct Version {
    major: i32,
    minor: i32,
    patch: i32,
}

fn build_version(mut semver: &str) -> Version {
    let check = semver.chars().nth(0).unwrap();
    if check != 'v' {
        println!("Invalid version naming, check again!");
    }
    semver = &semver[1..];
    println!("semver check: {}", semver);
    let ver: Vec<&str> = semver.split(".").collect();
    // println!("hahah {}", ver[0]);
    // for v in ver {
    //     println!("hahah {}", v);
    // }
    // println!("hahah {}", ver[0]);
    print_type_of(&String::from(ver[0]));
    // let major: i32 = parse(String::from(ver[0]));
    // let minor: i32 = parse(String::from(ver[1]));
    // let patch: i32 = parse(String::from(ver[2]));
    // Version{major, minor, patch}
    Version{
        major: 1,
        minor: 0,
        patch: 0
    }
}

#[derive(Clone)]
struct Dependencies {
    name: String,
    src: String,
    version: Version,
}

fn parse(filename: String) -> Vec<Dependencies> {
    let mut vec = Vec::new();
    let mut buf = Dependencies{
        name: String::new(),
        src: String::new(),
        version: build_version("v0.0.0"),
    };
    if let Ok(lines) = utils::read_lines(filename) {
        for line in lines {
            if let Ok(mut test) = line {
                if test.len() == 0 {
                    continue;
                }
                let firstChar = test.chars().nth(0).unwrap();
                if firstChar == '-' {
                    let bufCopy = Dependencies{
                        name: buf.name.clone(),
                        src: buf.src.clone(),
                        version: buf.version.clone()
                    };
                    vec.push(bufCopy);
                    test = test.strip_prefix("- ").unwrap_or(&test).to_string();
                }
                test = test.trim().to_string();
                let tokens = test.split(":");
                let tokens: Vec<&str> = tokens.collect();
                let attr = tokens[0].trim();
                let val = tokens[1].trim();
                match attr {
                    "name" => {
                        buf.name = val.to_string();
                    },
                    "src" => {
                        buf.src = val.to_string();
                    },
                    "version" => {
                        buf.version = build_version(val);
                    },
                    &_ => {
                        println!("Error!");
                    }
                }
            }
        }
        vec.push(buf);
    }
    vec
}

fn main() {
    println!("Input the filename you want to read!");
    // let mut filename = String::new();
    let filename = String::from("requirements.yml");
    // io::stdin().read_line(&mut filename).expect("Error while reading filename");
    // filename = filename.trim().to_string();
    let _lol: Vec<Dependencies> = parse(filename);
}
