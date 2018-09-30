#![allow(non_snake_case)]

fn main()
{
    // match_one
    assert_eq!(match_one("a", "a"), true);
    assert_eq!(match_one(".", "z"), true);
    assert_eq!(match_one("", "b"), true);
    assert_eq!(match_one("a", "b"), false);
    assert_eq!(match_one("p", ""), false);

    // match_multi
    assert_eq!(match_multi("aa", "aaa"), true);
    assert_eq!(match_multi("aa", "aab"), true);
    assert_eq!(match_multi("", "aaaaa"), true);
    assert_eq!(match_multi("aa", "aba"), false);
    assert_eq!(match_multi("aaa", "aa"), false);
    assert_eq!(match_multi("a.a", "aba"), true);
    assert_eq!(match_multi("a.c", "abc"), true);

    // $
    assert_eq!(search("c$", "abc"), true);
    assert_eq!(search("c$", "cab"), false);
    assert_eq!(search("d$", "abc"), false);

    // ^
    assert_eq!(search("^c", "ca"), true);
    assert_eq!(search("c$", "ca"), false);
    assert_eq!(search("c$", "abc"), true);

    // 部分一致
    assert_eq!(search("ddc", "adddc"), true);
    assert_eq!(search("adc", "addc"), false);
    assert_eq!(search("hogeg", "aaahogegaaa"), true);

    // ?
    // assert_eq!(search("ab?c", "ac"), true);
    // assert_eq!(search("ab?c", "abc"), true);
    // assert_eq!(search("a?b?c", "abc"), true);
    // assert_eq!(search("a?b?c?", ""), true);
    // assert_eq!(search("a?b?c", "c"), false);
    // assert_eq!(search("ab?c", "bc"), false);
}

fn match_one(pattern: &str, text: &str) -> bool {
    if pattern.is_empty() { return true; } // 空パターンは任意テキストと一致
    if text.is_empty() { return false; } // テキストが空なら、パターンに依らず一致しない

    if pattern == "." {
        // パターンが . のみの場合、任意テキストと一致
        return true;
    }

    pattern == text
}

fn match_multi(pattern: &str, text: &str) -> bool {
    if pattern.is_empty() { return true; } // 空パターンは任意テキストと一致

    if pattern == "$" && text.is_empty() { return true; } // $ 文字サポート

    if text.is_empty() { return false; } // テキストが空なら、パターンに依らず一致しない

    let (pattern_head, pattern_tail) = pattern.split_at(1);
    let (text_head, text_tail) = text.split_at(1);

    // パターンが2文字以上残っている場合は ? 文字チェックのため、パターンの次も見る
    if pattern_tail.len() > 1 {
        let (pattern_second_head, _) = pattern_tail.split_at(1);

        if pattern_second_head == "?" {
            return match_question(pattern, text);
        }
    }

    // ここに辿り着いたら通常の match
    return 
        // 頭1文字がマッチするかを検証
        match_one(pattern_head, text_head) &&
        // 後ろの文字列に対して再帰的に match_multi を呼び出す
        match_multi(pattern_tail, text_tail);
}

fn match_question(pattern: &str, text: &str) -> bool {
    // パターンを head (a?) と tail (?以降) に分割
    let (pattern_head, pattern_tail) = pattern.split_at(2);

    // head は更に "文字" と "?" に分割
    let (pattern_head, _) = pattern_head.split_at(1);

    // text の頭を抽出
    let (text_head, text_tail) = text.split_at(1);

    return 
        // ?より前の文字が一致しており、?以降も一致している
        (match_one(pattern_head, text_head) && match_multi(pattern_tail, text_tail)) ||
        // ?以降の文字が一致している
        match_multi(pattern_tail, text);
}

fn search(pattern: &str, text: &str) -> bool {
    let (pattern_head, pattern_tail) = pattern.split_at(1);

    if pattern_head == "^" {
        return match_multi(pattern_tail, text);
    } else {
        // ^ 文字で始まっていないパターンの場合は、
        // text の全地点を開始位置として検証
        let res = (0..text.len()).any(
            |index| -> bool {
                let (_, text_tail) = text.split_at(index);

                println!("[{}]: check match between {} and {}", index, pattern, text_tail);
                return match_multi(pattern, text_tail);
            }
        );

        if res {
            println!("Matched!");
        } else {
            println!("Do Not matched.");
        }
        res
    }
}