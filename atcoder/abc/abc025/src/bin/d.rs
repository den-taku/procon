use proconio::{fastout, input};
use std::collections::HashMap;

const MOD: u32 = 1_000_000_007;
const MAX: usize = 0b11111_11111_11111_11111_11111;

#[fastout]
fn main() {
    input! {
        x: [usize; 25]
    }
    // 初めから入っている場所を flag に
    let sat = x
        .iter()
        .enumerate()
        .fold(0, |s, (i, &e)| s | ((1 << i) * if e == 0 { 0 } else { 1 }));
    // 初めから入っている数字はアドレスを返す関数 set_map (ないときはNone)
    let set_map = {
        let mut tmp: Vec<(usize, usize)> = x.iter().cloned().enumerate().collect();
        tmp.sort();
        let tmp = tmp
            .iter()
            .cloned()
            .filter(|(_, e)| *e != 0)
            .map(|(i, e)| (e, i))
            .collect::<HashMap<_, _>>();
        let mut set_map = [None; 26];
        (1..26).fold((), |_, i| {
            if let Some(address) = tmp.get(&i) {
                set_map[i] = Some(*address);
            }
        });
        set_map
    };
    // 集合 usize に 1 から 25 まで入れていくときの場合の数
    let mut dp = HashMap::<usize, u32>::new();
    // 初期化
    dp.insert(0, 1);
    // MAX のときの入れ方が求める数
    println!("{}", rec(MAX, sat, &set_map, &mut dp));
}

#[inline]
fn rec(set: usize, sat: usize, set_map: &[Option<usize>], dp: &mut HashMap<usize, u32>) -> u32 {
    // すでに set に関して計算済みのとき
    if let Some(e) = dp.get(&set) {
        *e
    // これから計算する
    } else {
        // その集合に最後に入れる数字は何か
        let count = (0..25).fold(0, |s, i| s + (set >> i & 1));
        // 入れる数字が初めから入っているものの時
        if let Some(address) = set_map[count] {
            // set のうちそのアドレスに何かが入りうる時
            if set >> address & 1 == 1
                // set から address を除いた場所に count を入れても大丈夫なら
                && valid(
                    set - (1 << address) as usize,
                    address,
                    count,
                    set_map,
                    sat - (1 << address),
                )
            {
                // set から address を除いた場所に 1~count-1 まで入れる場合の数
                let pattern = rec(set - (1 << address) as usize, sat, set_map, dp) % MOD;
                dp.insert(set, pattern);
                pattern
            } else {
                // そこにないならないですね
                dp.insert(set, 0);
                0
            }
        // 初めから入っていない，すなわち 0 とされている候補を入れる時
        } else {
            // set のうちどれかをその候補にかえ，可能ならそこを置き換えたときの場合の数を足し合わせる(日本語力...)
            let sum = (0..25).fold(0, |s, i| {
                if set >> i & 1 == 1 && valid(set - (1 << i), i, count, set_map, sat) {
                    (s + (rec(set - (1 << i), sat, set_map, dp) % MOD)) % MOD
                } else {
                    s
                }
            });
            dp.insert(set, sum % MOD);
            sum % MOD
        }
    }
}

#[inline]
fn valid(set: usize, index: usize, _count: usize, _set_map: &[Option<usize>], sat: usize) -> bool {
    // 元からは入っていなかった候補を set の index に入れても大丈夫か
    // なんか元から入ってそう
    if sat >> index & 1 == 1 {
        false
    // 四つ角は無条件
    } else if index == 0 || index == 4 || index == 20 || index == 24 {
        true
    // 最上段 or 最下段なら左右のチェックだけ
    } else if index < 4 || 20 < index {
        let l = set >> (index - 1) & 1;
        let r = set >> (index + 1) & 1;
        l + r != 1
    // 最左段 or 最右段なら上下のチェックだけ
    } else if index % 5 == 0 || (index + 1) % 5 == 0 {
        let u = set >> (index - 5) & 1;
        let d = set >> (index + 5) & 1;
        u + d != 1
    // それ以外(うちの9マス)は上下左右のチェック
    } else {
        let u = set >> (index - 5) & 1;
        let d = set >> (index + 5) & 1;
        let l = set >> (index - 1) & 1;
        let r = set >> (index + 1) & 1;
        u + d != 1 && l + r != 1
    }
}
