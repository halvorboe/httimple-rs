use frame::{DATA, HEADERS, PRIORITY, RST_STREAM, SETTINGS, PING, GOAWAY, WINDOW_UPDATE, CONTINUATION};


pub fn pow(n: u32, t: u32) -> u32 {
    if t == 0 {
        1
    } else {
        let mut g = n;
        for _ in 0..t - 1 {
            g *= n;
        }
        g
    }
}

#[test]
fn do_pow() {
    assert_eq!(1, pow(4, 0));
    assert_eq!(4, pow(4, 1));
    assert_eq!(16, pow(4, 2));
    assert_eq!(64, pow(4, 3));
}

const PAD : &str = "0";

pub fn pad(s : String, len : usize) -> String{
    if s.len() < len {
        let i = len - s.len();
        let mut t = String::new();
        for _ in 0..i {
            t.push_str(PAD);
        }
        t.push_str(&s);
        t
    } else {
        s
    }
} 

#[test]
fn do_pad() {
    let mut s = String::from("1");
    let t = String::from("0001");
    assert_eq!(t, pad(s, 4));
    let mut s = String::from("1111");
    let t = String::from("1111");
    assert_eq!(t, pad(s, 4));
}

pub fn bin_to_vec(s : String) -> Vec<u8> {
    assert_eq!(0, s.len() % 8);
    let mut v = Vec::new();
    let len = s.len() / 8;
    println!("{}", len);
    let mut c = s.chars();
    for _ in 0..len {
        let mut i : u8 = 0;
        for y in 0..8 {
            if c.next().unwrap() == '1' {
                i += pow(2, 7 - y as u32) as u8;
            }
        }
        v.push(i);

    }
    v 
}


#[test]
fn do_bin_to_vec() {
    let t = String::from("00000001");
    let v = bin_to_vec(t);
    assert_eq!(1, v[0]);
    let t = String::from("11111111");
    let v = bin_to_vec(t);
    assert_eq!(255, v[0]);
}


pub fn get_type(i: u8) -> String {
   let r = {
       match i {
           DATA => "DATA",
           HEADERS => "HEADERS",
           PRIORITY => "PRIORITY",
           RST_STREAM => "RST_STREAM",
           SETTINGS => "SETTINGS",
           PING => "PING",
           GOAWAY => "GOAWAY",
           WINDOW_UPDATE => "WINDOW_UPDATE",
           CONTINUATION => "CONTINUATION",
           _ => "UNKNOWN"
       }
   };
   String::from(r)
}