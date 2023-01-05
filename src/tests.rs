/**
 * File: tests.rs
 * Author: xkoval18 <xkoval18@github>
 * Date: 20.10.2022
 * Last Modified Date: 20.10.2022
 */
use crate::{entry::Dir, error::TskResult, img_info::ImgInfo};

#[test]
fn extract_file() {
    let dir = ImgInfo::new("testData/ntfs.img")
        .unwrap()
        .fs_info()
        .unwrap()
        .root()
        .unwrap();

    let file = {
        let mut file = None;
        for f in &dir {
            if f.name().unwrap() == "text.txt" {
                file = Some(f);
            }
        }

        file.unwrap()
    };

    let text = "";

    assert_eq!(text, "hello word this is me!\nfuck you.\n");
}

#[test]
fn load_iso() -> TskResult<()> {
    let arg = "testData/test.iso";

    let _im = ImgInfo::new(arg)?;

    Ok(())
}

#[test]
fn imgimg() {
    let img = ImgInfo::new("testData/ntfs.img").unwrap();
    println!("img type: {}", img.desc());

    let fs = img.fs_info().unwrap();
    println!("fs: {:?}", fs);

    // let vs = img.vs_info().unwrap();
    // println!("vs: {:?}", vs);

    let r = fs.root().unwrap();
    // let r = fs.open_dir("/").unwrap();
    println!("root: {:?}", r);

    for d in &r {
        println!("dir: {}", d)
    }
}

// first deep
fn recurse(d: &Dir, depth: usize) -> usize {
    let mut out = depth;

    for f in d {
        for _ in 0..depth {
            print!("\t")
        }

        println!("{}", f.name().unwrap());

        if let Some(n) = f.to_subdir() {
            let out1 = recurse(&n, depth + 1);
            if out1 > out {
                out = out1
            }
        }
    }

    return out;
}

#[test]
fn tree() -> TskResult<()> {
    // let arg = env::args().nth(1).unwrap();
    let arg = "testData/test.iso";

    let im = ImgInfo::new(arg)?;

    let fs = im.fs_info()?;

    let root = fs.root()?;

    let depth = recurse(&root, 0);

    assert_eq!(depth, 3);

    Ok(())
}
