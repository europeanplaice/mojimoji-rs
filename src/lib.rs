//! Rust implementation of a fast converter between Japanese hankaku and zenkaku characters, [mojimoji](https://github.com/studio-ousia/mojimoji).

const ASCII_ZENKAKU_CHARS: [char; 86] = [
    'ａ', 'ｂ', 'ｃ', 'ｄ', 'ｅ', 'ｆ', 'ｇ', 'ｈ', 'ｉ', 'ｊ', 'ｋ',
    'ｌ', 'ｍ', 'ｎ', 'ｏ', 'ｐ', 'ｑ', 'ｒ', 'ｓ', 'ｔ', 'ｕ', 'ｖ',
    'ｗ', 'ｘ', 'ｙ', 'ｚ',
    'Ａ', 'Ｂ', 'Ｃ', 'Ｄ', 'Ｅ', 'Ｆ', 'Ｇ', 'Ｈ', 'Ｉ', 'Ｊ', 'Ｋ',
    'Ｌ', 'Ｍ', 'Ｎ', 'Ｏ', 'Ｐ', 'Ｑ', 'Ｒ', 'Ｓ', 'Ｔ', 'Ｕ', 'Ｖ',
    'Ｗ', 'Ｘ', 'Ｙ', 'Ｚ',
    '！', '”', '＃', '＄', '％', '＆', '’', '（', '）', '＊', '＋',
    '，', '－', '．', '／', '：', '；', '＜', '＝', '＞', '？', '＠',
    '［', '￥', '］', '＾', '＿', '‘', '｛', '｜', '｝', '～', '　',
    '＼'
];

const ASCII_HANKAKU_CHARS: [char; 86] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k',
    'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
    'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K',
    'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
    'W', 'X', 'Y', 'Z',
    '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+',
    ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@',
    '[', '¥', ']', '^', '_', '`', '{', '|', '}', '~', ' ',
    '\\'
];

const KANA_ZENKAKU_CHARS: [char; 63] = [
    'ア', 'イ', 'ウ', 'エ', 'オ', 'カ', 'キ', 'ク', 'ケ', 'コ',
    'サ', 'シ', 'ス', 'セ', 'ソ', 'タ', 'チ', 'ツ', 'テ', 'ト',
    'ナ', 'ニ', 'ヌ', 'ネ', 'ノ', 'ハ', 'ヒ', 'フ', 'ヘ', 'ホ',
    'マ', 'ミ', 'ム', 'メ', 'モ', 'ヤ', 'ユ', 'ヨ',
    'ラ', 'リ', 'ル', 'レ', 'ロ', 'ワ', 'ヲ', 'ン',
    'ァ', 'ィ', 'ゥ', 'ェ', 'ォ', 'ッ', 'ャ', 'ュ', 'ョ',
    '。', '、', '・', '゛', '゜', '「', '」', 'ー'
];

const KANA_HANKAKU_CHARS: [char; 63] = [
    'ｱ', 'ｲ', 'ｳ', 'ｴ', 'ｵ', 'ｶ', 'ｷ', 'ｸ', 'ｹ', 'ｺ',
    'ｻ', 'ｼ', 'ｽ', 'ｾ', 'ｿ', 'ﾀ', 'ﾁ', 'ﾂ', 'ﾃ', 'ﾄ',
    'ﾅ', 'ﾆ', 'ﾇ', 'ﾈ', 'ﾉ', 'ﾊ', 'ﾋ', 'ﾌ', 'ﾍ', 'ﾎ',
    'ﾏ', 'ﾐ', 'ﾑ', 'ﾒ', 'ﾓ', 'ﾔ', 'ﾕ', 'ﾖ',
    'ﾗ', 'ﾘ', 'ﾙ', 'ﾚ', 'ﾛ', 'ﾜ', 'ｦ', 'ﾝ',
    'ｧ', 'ｨ', 'ｩ', 'ｪ', 'ｫ', 'ｯ', 'ｬ', 'ｭ', 'ｮ',
    '｡', '､', '･', 'ﾞ', 'ﾟ', '｢', '｣', 'ｰ'
];

const DIGIT_ZENKAKU_CHARS: [char; 10] = [
    '０', '１', '２', '３', '４', '５', '６', '７', '８', '９'
];

const DIGIT_HANKAKU_CHARS: [char; 10] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
];

const KANA_TEN_MAP: [(char, char); 21] = [
    ('ガ', 'ｶ'), ('ギ', 'ｷ'), ('グ', 'ｸ'), ('ゲ', 'ｹ'), ('ゴ', 'ｺ'),
    ('ザ', 'ｻ'), ('ジ', 'ｼ'), ('ズ', 'ｽ'), ('ゼ', 'ｾ'), ('ゾ', 'ｿ'),
    ('ダ', 'ﾀ'), ('ヂ', 'ﾁ'), ('ヅ', 'ﾂ'), ('デ', 'ﾃ'), ('ド', 'ﾄ'),
    ('バ', 'ﾊ'), ('ビ', 'ﾋ'), ('ブ', 'ﾌ'), ('ベ', 'ﾍ'), ('ボ', 'ﾎ'),
    ('ヴ', 'ｳ')
];

const KANA_MARU_MAP: [(char, char); 5] = [
    ('パ', 'ﾊ'), ('ピ', 'ﾋ'), ('プ', 'ﾌ'), ('ペ', 'ﾍ'), ('ポ', 'ﾎ')
];

/// Converts Japanese zenkaku to hankaku.
/// # Arguments
/// * `text` - text to convert.
/// * `ascii` - indicates whether to convert ascii characters.
/// * `digit` - indicates whether to convert digits.
/// * `kana` - indicates whether to convert Japanese characters.
/// # Examples
/// ```rust
/// use mojimoji_rs::zen_to_han;
/// assert_eq!(zen_to_han("アイウａｂｃ０１２".to_string(), true, true, true), "ｱｲｳabc012".to_string());
/// assert_eq!(zen_to_han("アイウａｂｃ０１２".to_string(), true, true, false), "アイウabc012".to_string());
/// assert_eq!(zen_to_han("アイウａｂｃ０１２".to_string(), true, false, true), "ｱｲｳabc０１２".to_string());
/// assert_eq!(zen_to_han("アイウａｂｃ０１２".to_string(), false, true, true), "ｱｲｳａｂｃ012".to_string());
/// ```
pub fn zen_to_han(text: String, ascii: bool, digit: bool, kana: bool) -> String {
    let mut buf: String = String::with_capacity(text.len());
    let mut iter = text.chars();
    while let Some(c) = iter.next(){
        if ascii {
            match ASCII_ZENKAKU_CHARS.iter().position(|x| x == &c) {
                Some(idx) => {buf.push(ASCII_HANKAKU_CHARS[idx]); continue},
                _ => ()
            };
        };
        if digit {
            match DIGIT_ZENKAKU_CHARS.iter().position(|x| x == &c) {
                Some(idx) => {buf.push(DIGIT_HANKAKU_CHARS[idx]); continue},
                _ => ()
            };
        };
        if kana {
            match KANA_ZENKAKU_CHARS.iter().position(|x| x == &c) {
                Some(idx) => {buf.push(KANA_HANKAKU_CHARS[idx]); continue},
                _ => ()
            };
           match KANA_TEN_MAP.iter().position(|x| x.0 == c) {
                Some(idx) => {
                    buf.push(KANA_TEN_MAP[idx].1); 
                    buf.push('ﾞ'); 
                    continue
                },
                _ => ()
            };
           match KANA_MARU_MAP.iter().position(|x| x.0 == c) {
                Some(idx) => {
                    buf.push(KANA_MARU_MAP[idx].1); 
                    buf.push('ﾟ'); 
                    continue
                },
                _ => ()
            };
        };
        buf.push(c);
    };
    buf
}

/// Converts Japanese hankaku to zenkaku.
/// 
/// # Arguments
/// * `text` - text to convert.
/// * `ascii` - indicates whether to convert ascii characters.
/// * `digit` - indicates whether to convert digits.
/// * `kana` - indicates whether to convert Japanese characters.
/// # Examples
/// ```rust
/// use mojimoji_rs::han_to_zen;
/// assert_eq!(han_to_zen("ｱｲｳabc012".to_string(), true, true, true), "アイウａｂｃ０１２".to_string());
/// assert_eq!(han_to_zen("ｱｲｳabc012".to_string(), true, true, false), "ｱｲｳａｂｃ０１２".to_string());
/// assert_eq!(han_to_zen("ｱｲｳabc012".to_string(), true, false, true), "アイウａｂｃ012".to_string());
/// assert_eq!(han_to_zen("ｱｲｳabc012".to_string(), false, true, true), "アイウabc０１２".to_string());
/// ```
pub fn han_to_zen(text: String, ascii: bool, digit: bool, kana: bool) -> String {
    let mut buf: String = String::with_capacity(text.len());
    let mut iter = text.chars().peekable();
    while let Some(c) = iter.next(){
        if ascii {
            match ASCII_HANKAKU_CHARS.iter().position(|x| x == &c) {
                Some(idx) => {buf.push(ASCII_ZENKAKU_CHARS[idx]); continue},
                _ => ()
            };
        };
        if digit {
            match DIGIT_HANKAKU_CHARS.iter().position(|x| x == &c) {
                Some(idx) => {buf.push(DIGIT_ZENKAKU_CHARS[idx]); continue},
                _ => ()
            };
        };
        if kana {
           match KANA_TEN_MAP.iter().position(|x| x.1 == c) {
               Some(idx) => {
                   if let Some('ﾞ') = iter.peek(){
                        buf.push(KANA_TEN_MAP[idx].0); 
                        iter.next().unwrap();
                        continue
                    }
                },
                _ => ()
            };
           match KANA_MARU_MAP.iter().position(|x| x.1 == c) {
                Some(idx) => {
                    if let Some('ﾟ') = iter.peek(){
                        buf.push(KANA_MARU_MAP[idx].0);
                        iter.next().unwrap();
                        continue
                    }
                },
                _ => ()
            };
            match KANA_HANKAKU_CHARS.iter().position(|x| x == &c) {
                Some(idx) => {
                    buf.push(KANA_ZENKAKU_CHARS[idx]); 
                    continue
                },
                _ => ()
            };
        };
        buf.push(c);
    };
    buf
}

#[test]
fn test_zen_to_han(){
    let zen = "アイウエオ".to_string();
    let han = zen_to_han(zen, true, true, true);
    assert_eq!(han, "ｱｲｳｴｵ".to_string());

    let zen = "ガギグゲゴ".to_string();
    let han = zen_to_han(zen, true, true, true);
    assert_eq!(han, "ｶﾞｷﾞｸﾞｹﾞｺﾞ".to_string());

    let zen = "パピプペポ".to_string();
    let han = zen_to_han(zen, true, true, true);
    assert_eq!(han, "ﾊﾟﾋﾟﾌﾟﾍﾟﾎﾟ".to_string());

    let zen = "０１２３".to_string();
    let han = zen_to_han(zen, true, true, true);
    assert_eq!(han, "0123".to_string());

    let zen = "ａｂｃＡＢＣ".to_string();
    let han = zen_to_han(zen, true, true, true);
    assert_eq!(han, "abcABC".to_string());

    let zen = "＃？！￥".to_string();
    let han = zen_to_han(zen, true, true, true);
    assert_eq!(han, "#?!¥".to_string());

    let zen = "あいうえお".to_string();
    let han = zen_to_han(zen, true, true, true);
    assert_eq!(han, "あいうえお".to_string());

    let zen = "＼".to_string();
    let han = zen_to_han(zen, true, true, true);
    assert_eq!(han, "\\".to_string());
}

#[test]
fn test_han_to_zen(){
    let han = "ｱｲｳｴｵ".to_string();
    let zen = han_to_zen(han, true, true, true);
    assert_eq!(zen, "アイウエオ".to_string());

    let han = "ｶﾞｷﾞｸﾞｹﾞｺﾞ".to_string();
    let zen = han_to_zen(han, true, true, true);
    assert_eq!(zen, "ガギグゲゴ".to_string());

    let han = "ﾊﾟﾋﾟﾌﾟﾍﾟﾎﾟ".to_string();
    let zen = han_to_zen(han, true, true, true);
    assert_eq!(zen, "パピプペポ".to_string());

    let han = "0123".to_string();
    let zen = han_to_zen(han, true, true, true);
    assert_eq!(zen, "０１２３".to_string());

    let han = "abcABC".to_string();
    let zen = han_to_zen(han, true, true, true);
    assert_eq!(zen, "ａｂｃＡＢＣ".to_string());

    let han = "#?!¥".to_string();
    let zen = han_to_zen(han, true, true, true);
    assert_eq!(zen, "＃？！￥".to_string());

    let han = "あいうえお".to_string();
    let zen = han_to_zen(han, true, true, true);
    assert_eq!(zen, "あいうえお".to_string());

    let han = "ｱﾞｲﾞｳﾞｴﾞｵﾞ".to_string();
    let zen = han_to_zen(han, true, true, true);
    assert_eq!(zen, "ア゛イ゛ヴエ゛オ゛".to_string());

    let han = "\\".to_string();
    let zen = han_to_zen(han, true, true, true);
    assert_eq!(zen, "＼".to_string());
}
