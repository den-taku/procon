use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        packs: [(u64, u64); n]
    }
    let mut unions = std::collections::HashSet::new();
    let mut not = Vec::new();
    let mut cod = std::collections::BTreeSet::new();
    for &(a, b) in &packs {
        let mut ad = divior(a);
        let mut bd = divior(b);
        // let mut ad_not = Vec::new();
        let mut ad_not = std::collections::HashSet::new();
        // let mut bd_not = Vec::new();
        let mut bd_not = std::collections::HashSet::new();
        while !ad.is_empty() && !bd.is_empty() {
            let aone = ad[ad.len() - 1];
            let bone = bd[bd.len() - 1];
            if aone == bone {
                unions.insert(aone);
                ad.pop();
                bd.pop();
            } else if aone > bone {
                let one = ad.pop().unwrap();
                ad_not.insert(one);
                cod.insert(one);
            } else {
                let one = bd.pop().unwrap();
                bd_not.insert(one);
                cod.insert(one);
            }
            if !ad.is_empty() {
                for &e in &ad {
                    cod.insert(e);
                    ad_not.insert(e);
                }
                // ad_not.append(&mut ad);
            }
            if !bd.is_empty() {
                for &e in &bd {
                    cod.insert(e);
                    bd_not.insert(e);
                }
                // bd_not.append(&mut bd);
            }
        }
        not.push((ad_not, bd_not));
    }
    let mut valid = vec![false; n];
    let mut one = std::collections::HashSet::new();
    let mut two = std::collections::HashSet::new();
    for e in cod.iter().rev() {
        if valid.iter().all(|e| *e) {
            break;
        }
        if !unions.contains(&e) {
            let mut flag = false;
            if one.contains(&e) && two.contains(&e) {
                break;
            } else if one.contains(&e) {
                flag = true;
            }
            for i in 0..n {
                if !valid[i] {
                    if not[i].0.contains(e) {
                        if flag {
                            for e in not[i].0.iter() {
                                one.insert(e);
                            }
                            for e in not[i].1.iter() {
                                two.insert(e);
                            }
                        } else {
                            for e in not[i].0.iter() {
                                two.insert(e);
                            }
                            for e in not[i].1.iter() {
                                one.insert(e);
                            }
                        }
                        valid[i] = true;
                    }
                    if not[i].1.contains(e) {
                        if flag {
                            for e in not[i].1.iter() {
                                one.insert(e);
                            }
                            for e in not[i].0.iter() {
                                two.insert(e);
                            }
                        } else {
                            for e in not[i].1.iter() {
                                two.insert(e);
                            }
                            for e in not[i].0.iter() {
                                one.insert(e);
                            }
                        }
                        valid[i] = true;
                    }
                }
            }
        }
    }
    println!("one: {:?}", one);
    println!("two: {:?}", two);
    unimplemented!()
}

#[inline]
fn divior(mut num: u64) -> Vec<u64> {
    let mut set = std::collections::HashSet::new();
    let mut i = 2;
    while i * i <= num {
        while num % i == 0 {
            set.insert(i);
            num /= i;
        }
        i += 1;
    }
    if num != 1 {
        set.insert(num);
    }
    let mut v = set.iter().map(|e| *e).collect::<Vec<u64>>();
    v.sort();
    v.dedup();
    v
}
