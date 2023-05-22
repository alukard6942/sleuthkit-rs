/**
 * File: tests.rs
 * Author: xkoval18 <xkoval18@github>
 * Date: 20.10.2022
 * Last Modified Date: 20.10.2022
 */
use crate::{entry::Dir, error::TskResult, img::img_info::ImgInfo, fs::fs_info::FsInfo};

#[test]
fn extract_file() {
    let img = ImgInfo::new("testData/ntfs.img".to_string()) .unwrap();
    let fs =img.fs().unwrap();
    let dir = fs.dir_open_root() .unwrap();

    let file = {
        let mut file = None;
        for f in &dir {
            if f.name().unwrap() == "test.txt" {
                file = Some(f);
            }
        }

        file.unwrap()
    };

    // let text = String::from_utf8(file.contents()).unwrap();

    // assert_eq!(text, "hello word this is me!\nfuck you.\n");
}

#[test]
fn load_iso() -> TskResult<()> {
    let arg = "testData/test.iso".to_string();

    let _im = ImgInfo::new(arg)?;

    Ok(())
}

#[test]
fn imgimg() {
    let img = ImgInfo::new("testData/ntfs.img".to_string()).unwrap();
    println!("img type: {:?}", img);

    let fs = img.fs().unwrap();
    println!("fs: {:?}", fs);

    // let vs = img.vs_info().unwrap();
    // println!("vs: {:?}", vs);

    let r = fs.dir_open_root().unwrap();
    // let r = fs.open_dir("/").unwrap();
    println!("root: {:?}", r);

    for d in &r {
        println!("dir: {}", d)
    }
}

// first deep
fn recurse(d: &Dir, fs: &FsInfo, depth: usize) -> usize {
    let mut out = depth;

    for f in d {
        for _ in 0..depth {
            print!("\t")
        }

        println!("{}", f.name().unwrap());

        if let Some(n) = fs.dir_open_from_file(&f) {
            let out1 = recurse(&n, fs, depth + 1);
            if out1 > out {
                out = out1
            }
        }
    }
    //
    return out;
}

#[test]
fn tree() -> TskResult<()> {
    // let arg = env::args().nth(1).unwrap();
    let arg = "testData/test.iso".to_string();

    let im = ImgInfo::new(arg)?;

    let fs = im.fs()?;

    let root = fs.dir_open_root()?;

    let depth = recurse(&root, &fs, 0);

    assert_eq!(depth, 3);

    Ok(())
}
